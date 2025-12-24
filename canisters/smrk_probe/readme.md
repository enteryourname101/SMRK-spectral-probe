# Spectral Probe Experiment

This directory documents the **Spectral Probe experiment** used to study
structural properties of an arithmetic operator acting on ℓ²(ℕ).
The experiment is designed as a *measurement protocol*, not as a proof.

---

## Experiment Objective

The purpose of the Spectral Probe is to measure and monitor **structural stability**
of a truncated arithmetic operator under controlled conditions.

In particular, the experiment focuses on:

- symmetry leakage introduced by finite truncation,
- stability of quadratic form measurements,
- dependence on truncation parameters and arithmetic weights.

The experiment does **not** attempt to:
- compute the full spectrum in the infinite-dimensional limit,
- verify the Riemann Hypothesis,
- establish self-adjointness numerically.

---

## Operator Under Study

The operator (informally referred to as the *SMRK operator*) acts on vectors
indexed by natural numbers and combines:

1. **Prime-indexed shift terms**
   - forward shift: `n → p·n`
   - backward shift: `n → n/p` (if divisible)

2. **Diagonal arithmetic terms**
   - logarithmic weight `log(n)`
   - von Mangoldt weight `Λ(n)`

In truncated form, the operator acts on ℓ²({1,…,N}) with prime sums
restricted to `p ≤ Pmax`.

---

## Measured Quantities

For each experiment run, the following quantities are measured:

### 1. Symmetry Leakage

The primary observable is the symmetry defect:

