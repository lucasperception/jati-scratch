from typing import Literal
import matplotlib.pyplot as plt
import matplotlib
import pandas as pd
import numpy as np

type CostBenchStats = tuple[tuple[int, int], float, Literal["AA", "DNA"]]
cost_dims_with_reduction: CostBenchStats = [
    ((12, 73), 1.80, "AA"),
    ((45, 223), -0.03, "AA"),
    ((27, 632), -3.67, "AA"),
    ((128, 688), -10.73, "DNA"),
    ((46, 16250), -1.95, "DNA"),
    ((33, 4455), -5.81, "DNA"),
]


def mem_usage(model: Literal["AA", "DNA"], n_seqs: int, m: int) -> int:
    node_count = n_seqs * 2 - 1
    n = 5 if model == "DNA" else 21
    surv_ins_len = node_count
    c0_len = 2 * node_count

    model_len = n * n
    ftilde_len = n * m
    anc_len = 3 * m
    pnu_len = m
    return (
        np.array([model_len, ftilde_len, anc_len, pnu_len]).sum() * node_count
        + surv_ins_len
        + c0_len
    )


def dim_key(
    dim_with_reduction: CostBenchStats,
) -> int:
    dim = dim_with_reduction[0]
    return mem_usage(dim_with_reduction[2], dim[0], dim[1])


cost_dims_with_reduction.sort(key=dim_key)

# [((12, 73), 1.8), ((45, 223), -0.03), ((27, 632), -3.67), ((128, 688), -10.73), ((33, 4455), -5.81), ((46, 16250), -1.95)]
print(cost_dims_with_reduction)

cost_dimprod_with_reduction = list(
    map(
        lambda dim_with_reduction: (dim_key(dim_with_reduction), dim_with_reduction[1]),
        cost_dims_with_reduction,
    )
)
cost_dimprods, cost_reductions = zip(*cost_dimprod_with_reduction)


oneshot_taxa_len = 500
oneshot_taxa = pd.read_csv("../../testbench/results/one-shot-taxa.csv")
oneshot_taxa_prod = list(
    mem_usage("DNA", n, oneshot_taxa_len) for n in oneshot_taxa["n"]
)
oneshot_taxa_reduction = (
    (oneshot_taxa["opt+cachlocality"] - oneshot_taxa["cluster-par-regraft+zero-alloc"])
    / oneshot_taxa["cluster-par-regraft+zero-alloc"]
    * 100.0
)


oneshot_len_taxa = 10
oneshot_len = pd.read_csv("../../testbench/results/one-shot-length.csv")
oneshot_len_prod = list(mem_usage("DNA", oneshot_len_taxa, m) for m in oneshot_len["m"])
oneshot_len_reduction = (
    (oneshot_len["opt+cachlocality"] - oneshot_len["cluster-par-regraft+zero-alloc"])
    / oneshot_len["cluster-par-regraft+zero-alloc"]
    * 100.0
)


# print(dimprods)
# print(reductions)
# some NixOS/wayland shenanigans
matplotlib.use("TkAgg")
plt.xlabel("(log) PIPCost cache size [8-Byte Floating Points]")
plt.ylabel("Relative change in performance [%]")
_ = plt.semilogx(cost_dimprods, cost_reductions, label="PIP Cost Benchmarks")
_ = plt.semilogx(oneshot_taxa_prod, oneshot_taxa_reduction, label="One-Shot (Taxa)")
_ = plt.semilogx(
    oneshot_len_prod,
    oneshot_len_reduction,
    label="One-Shot (Sequence Length)",
)
_ = plt.legend()

_ = plt.show()
