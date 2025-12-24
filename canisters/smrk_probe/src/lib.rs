use candid::{CandidType, Deserialize};
use ic_cdk_macros::{query, update};
use serde::Serialize;

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct SymmetryReport {
    pub n: u32,
    pub p_max: u32,
    pub alpha: f64,
    pub beta: f64,
    pub seed: u64,
    pub trials: u32,

    pub sym_abs_mean: f64,
    pub sym_abs_max: f64,
    pub hx_norm_mean: f64,
    pub x_norm_mean: f64,
}

#[query]
fn get_version() -> String {
    "smrk_probe v0.1 (matvec + symmetry_probe)".to_string()
}

/// --------- Core math helpers (deterministic, no floats tricks) ----------

fn dot(a: &[f64], b: &[f64]) -> f64 {
    // Basic dot product. Deterministic given same inputs (IEEE-754), but note
    // that different hardware can still differ in last bits. Good enough for v0.1.
    let mut s = 0.0f64;
    for (x, y) in a.iter().zip(b.iter()) {
        s += x * y;
    }
    s
}

fn norm2(a: &[f64]) -> f64 {
    dot(a, a).sqrt()
}

/// Simple deterministic RNG (splitmix64) for reproducible probes.
fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

fn rand_f64(state: &mut u64) -> f64 {
    // Uniform in [0,1)
    let u = splitmix64(state);
    // take top 53 bits
    let x = (u >> 11) as u64;
    (x as f64) / ((1u64 << 53) as f64)
}

fn rand_signed(state: &mut u64) -> f64 {
    // Simple symmetric distribution in [-1, 1)
    2.0 * rand_f64(state) - 1.0
}

/// --------- Number theory helpers (v0.1 simple, later sieve) ----------

fn primes_up_to(pmax: u32) -> Vec<u32> {
    if pmax < 2 {
        return vec![];
    }
    let n = pmax as usize;
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut p = 2usize;
    while p * p <= n {
        if is_prime[p] {
            let mut k = p * p;
            while k <= n {
                is_prime[k] = false;
                k += p;
            }
        }
        p += 1;
    }
    let mut ps = Vec::new();
    for i in 2..=n {
        if is_prime[i] {
            ps.push(i as u32);
        }
    }
    ps
}

/// von Mangoldt Λ(n): log p if n=p^k, else 0
fn von_mangoldt(n: u32) -> f64 {
    if n < 2 {
        return 0.0;
    }
    // trial divide to find smallest prime factor p
    let mut m = n;
    let mut p = 2u32;
    while (p as u64) * (p as u64) <= (m as u64) {
        if m % p == 0 {
            // check if m is power of p only
            while m % p == 0 {
                m /= p;
            }
            if m == 1 {
                return (p as f64).ln();
            } else {
                return 0.0;
            }
        }
        p += if p == 2 { 1 } else { 2 }; // 2,3,5,...
    }
    // n is prime
    (n as f64).ln()
}

fn log_n(n: u32) -> f64 {
    if n <= 1 { 0.0 } else { (n as f64).ln() }
}

/// --------- H operator: y = Hx (truncated) ----------
/// Indices represent n=1..=N mapped to vec indices 0..N-1.
///
/// Hx(n) = sum_{p<=Pmax} (1/p) [ x(pn) + 1_{p|n} x(n/p) ] + (alpha Λ(n) + beta log n) x(n)
///
/// Note:
/// - forward term x(pn) contributes only if pn<=N.
/// - backward term x(n/p) contributes only if p|n.
/// - diagonal is always present.
///
/// Complexity ~ N * pi(Pmax) + some divisibility checks.
/// For v0.1 we do straightforward loops; later we optimize.
fn h_matvec(n: u32, pmax: u32, alpha: f64, beta: f64, x: &[f64]) -> Vec<f64> {
    let n_usize = n as usize;
    assert_eq!(x.len(), n_usize, "x length must equal N");

    let primes = primes_up_to(pmax);
    let mut y = vec![0.0f64; n_usize];

    // Precompute diagonal terms
    for i in 0..n_usize {
        let nn = (i as u32) + 1;
        let diag = alpha * von_mangoldt(nn) + beta * log_n(nn);
        y[i] += diag * x[i];
    }

    // Prime shift terms
    for &p in primes.iter() {
        let invp = 1.0f64 / (p as f64);

        // forward: x(pn)
        // for n_idx representing n = i+1, target is pn = p*(i+1)
        // add invp * x[pn] into y[n]
        for i in 0..n_usize {
            let nn = (i as u32) + 1;
            let pn = (p as u64) * (nn as u64);
            if pn <= n as u64 {
                let j = (pn as usize) - 1;
                y[i] += invp * x[j];
            } else {
                // since nn increases, pn increases: we can break early for speed
                break;
            }
        }

        // backward: if p|n then add invp * x[n/p]
        // For each multiple n = p*k <= N: y[n] += invp*x[k]
        let mut m = p;
        while m <= n {
            let idx_m = (m as usize) - 1;
            let k = m / p;
            let idx_k = (k as usize) - 1;
            y[idx_m] += invp * x[idx_k];
            m += p;
        }
    }

    y
}

#[update]
fn matvec(n: u32, p_max: u32, alpha: f64, beta: f64, x: Vec<f64>) -> Vec<f64> {
    // Soft guardrails for v0.1 (avoid blowing cycles)
    if n == 0 || n > 50_000 {
        ic_cdk::trap("N must be in 1..=50000 for v0.1");
    }
    if p_max > 1_000_000 {
        ic_cdk::trap("Pmax too large for v0.1");
    }
    if x.len() != n as usize {
        ic_cdk::trap("x length must equal N");
    }

    h_matvec(n, p_max, alpha, beta, &x)
}

#[update]
fn symmetry_probe(
    n: u32,
    p_max: u32,
    alpha: f64,
    beta: f64,
    seed: u64,
    trials: u32,
) -> SymmetryReport {
    if n == 0 || n > 20_000 {
        ic_cdk::trap("N must be in 1..=20000 for symmetry_probe v0.1");
    }
    if trials == 0 || trials > 200 {
        ic_cdk::trap("trials must be in 1..=200");
    }

    let mut rng = seed;
    let n_usize = n as usize;

    let mut sym_abs_sum = 0.0;
    let mut sym_abs_max = 0.0;
    let mut hx_norm_sum = 0.0;
    let mut x_norm_sum = 0.0;

    for _t in 0..trials {
        // deterministic pseudo-random x,y
        let mut x = vec![0.0f64; n_usize];
        let mut y = vec![0.0f64; n_usize];
        for i in 0..n_usize {
            x[i] = rand_signed(&mut rng);
            y[i] = rand_signed(&mut rng);
        }

        // compute Hx, Hy
        let hx = h_matvec(n, p_max, alpha, beta, &x);
        let hy = h_matvec(n, p_max, alpha, beta, &y);

        // symmetry defect: <x,Hy> - <Hx,y>
        let lhs = dot(&x, &hy);
        let rhs = dot(&hx, &y);
        let defect = (lhs - rhs).abs();

        sym_abs_sum += defect;
        if defect > sym_abs_max {
            sym_abs_max = defect;
        }

        hx_norm_sum += norm2(&hx);
        x_norm_sum += norm2(&x);
    }

    let t = trials as f64;
    SymmetryReport {
        n,
        p_max,
        alpha,
        beta,
        seed,
        trials,
        sym_abs_mean: sym_abs_sum / t,
        sym_abs_max,
        hx_norm_mean: hx_norm_sum / t,
        x_norm_mean: x_norm_sum / t,
    }
}
