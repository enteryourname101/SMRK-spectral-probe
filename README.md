# SMRK Spectral Probe

This repository contains a reproducible experimental framework for studying
operator-theoretic constructions motivated by the Hilbert–Pólya program.
The focus is not on proving the Riemann Hypothesis, but on building and measuring
well-defined arithmetic operators and their structural properties.

The project combines:
- explicit operator definitions on ℓ²(ℕ),
- deterministic numerical experiments,
- reproducible deployment on the Internet Computer (ICP),
- optional cryptographic timestamping of experimental artifacts.

---

## Purpose

The primary goals of this repository are:

- to define **explicit arithmetic operators** acting on ℓ²(ℕ),
- to measure **structural properties** (symmetry leakage, stability under truncation),
- to provide a **transparent and reproducible experimental pipeline**,
- to create auditable experimental artifacts that can be independently verified.

This is an **experimental and exploratory** project.
Numerical results are treated as measurements, not proofs.

---

## Conceptual Background

The operator studied here (referred to informally as the *SMRK operator*)
combines two key components:

1. **Diagonal arithmetic terms**
   - logarithmic weights `log(n)`
   - von Mangoldt weights `Λ(n)`

2. **Prime-indexed shift operators**
   - forward shifts `n → p·n`
   - backward shifts `n → n/p` (when divisible)

This places the construction in the broader context of:
- arithmetic quantum systems on ℓ²(ℕ),
- divisibility and prime-scale dynamics,
- operator-theoretic approaches inspired by the Hilbert–Pólya idea.

The project explicitly acknowledges related prior work
(e.g. Bost–Connes-type systems),
while exploring a **distinctly non-diagonal, dynamical operator regime**.

---

## Repository Structure

.
├─ canisters/
│ └─ smrk_probe/
│ ├─ src/
│ │ └─ lib.rs # Rust canister implementing matvec + probes
│ ├─ Cargo.toml
│ └─ smrk_probe.did # Candid interface
│
├─ frontend/
│ ├─ index.html # Minimal web UI
│ ├─ main.js # ICP agent + experiment runner
│ └─ style.css
│
├─ experiments/
│ └─ spectral_probe/
│ ├─ README.md # Experiment description
│ ├─ schema.json # Canonical experiment schema
│ └─ results/ # Local / off-chain results
│
├─ dfx.json
├─ Cargo.toml # Workspace definition
└─ README.md # This file

yaml
Zkopírovat kód

---

## What Is Implemented (v0.1)

- Rust-based ICP canister providing:
  - deterministic `matvec` operation: `y = Hx`,
  - symmetry-leakage probes using random test vectors (with fixed seed),
- minimal web frontend hosted on ICP,
- deterministic numerical outputs suitable for hashing and timestamping.

Not implemented (yet):
- full eigenvalue solvers on-chain,
- claims about spectral identification with zeta zeros,
- any proof-level statements.

---

## Reproducibility

All experiments are:
- deterministic (given fixed parameters and seed),
- versioned (canister WASM hash),
- reproducible by third parties using the same inputs.

Numerical experiments are designed to be run:
- via the ICP-hosted frontend,
- or locally via direct canister calls.

---

## Data and Timestamping

Experimental outputs may optionally be:
- hashed (SHA-256),
- timestamped or committed to permanent storage systems (e.g. Arweave),
- stored locally or off-chain prior to public release.

The project supports a **hash-first** workflow:
only cryptographic commitments are published initially,
with full data released later if desired.

---

## Scope and Limitations

- This repository does **not** claim to prove the Riemann Hypothesis.
- Numerical results are **not evidence of truth**, only structural measurements.
- Finite truncations necessarily introduce boundary effects.
- Operator self-adjointness is a theoretical question, not resolved numerically.

The intent is clarity, not overreach.

---

## Status

Active experimental research project.
The framework is expected to evolve as:
- truncation strategies improve,
- analytical understanding deepens,
- and numerical methods are refined.

---

## License

Open research code.
Licensing terms will be specified as the project matures.
Until then, the repository is intended for study, experimentation,
and academic discussion.

---

## Contact / Attribution

Project author: **Enter Yourname**

This repository is part of a broader research program exploring
operator-based arithmetic dynamics and reproducible computational experiments.
