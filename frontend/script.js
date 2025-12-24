import { Actor, HttpAgent } from "https://cdn.jsdelivr.net/npm/@dfinity/agent@2.3.0/+esm";

// Minimal IDL factory (handwritten) pro náš canister
const idlFactory = ({ IDL }) => {
  const SymmetryReport = IDL.Record({
    n: IDL.Nat32,
    p_max: IDL.Nat32,
    alpha: IDL.Float64,
    beta: IDL.Float64,
    seed: IDL.Nat64,
    trials: IDL.Nat32,
    sym_abs_mean: IDL.Float64,
    sym_abs_max: IDL.Float64,
    hx_norm_mean: IDL.Float64,
    x_norm_mean: IDL.Float64,
  });

  return IDL.Service({
    get_version: IDL.Func([], [IDL.Text], ["query"]),
    matvec: IDL.Func([IDL.Nat32, IDL.Nat32, IDL.Float64, IDL.Float64, IDL.Vec(IDL.Float64)], [IDL.Vec(IDL.Float64)], []),
    symmetry_probe: IDL.Func([IDL.Nat32, IDL.Nat32, IDL.Float64, IDL.Float64, IDL.Nat64, IDL.Nat32], [SymmetryReport], []),
  });
};

// Pozn.: Po deploy se sem doplní canisterId (dfx generuje canister_ids.json).
// V dev režimu můžeš natvrdo opsat ID ze `dfx canister id smrk_probe`.
const CANISTER_ID = window.CANISTER_ID_SMRK_PROBE || "";

function val(id) {
  return document.getElementById(id).value;
}

document.getElementById("run").addEventListener("click", async () => {
  const out = document.getElementById("out");
  out.textContent = "Running...\n";

  if (!CANISTER_ID) {
    out.textContent = "Missing CANISTER_ID. Set window.CANISTER_ID_SMRK_PROBE in index.html or hardcode it in main.js.\n";
    return;
  }

  const agent = new HttpAgent({ host: window.location.origin });
  // In local dev you may need: await agent.fetchRootKey();
  try { await agent.fetchRootKey(); } catch (_) {}

  const actor = Actor.createActor(idlFactory, { agent, canisterId: CANISTER_ID });

  const n = Number(val("n"));
  const pmax = Number(val("pmax"));
  const alpha = Number(val("alpha"));
  const beta = Number(val("beta"));
  const seed = BigInt(val("seed"));
  const trials = Number(val("trials"));

  const rep = await actor.symmetry_probe(n, pmax, alpha, beta, seed, trials);

  out.textContent =
    `SMRK Spectral Probe v0.1\n` +
    `N=${rep.n}, Pmax=${rep.p_max}, alpha=${rep.alpha}, beta=${rep.beta}\n` +
    `seed=${rep.seed}, trials=${rep.trials}\n\n` +
    `sym_abs_mean = ${rep.sym_abs_mean}\n` +
    `sym_abs_max  = ${rep.sym_abs_max}\n` +
    `hx_norm_mean = ${rep.hx_norm_mean}\n` +
    `x_norm_mean  = ${rep.x_norm_mean}\n`;
});
