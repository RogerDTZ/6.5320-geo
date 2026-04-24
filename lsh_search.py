#!/usr/bin/env python3

# written with assistance from LLM

import subprocess
import re
from typing import Annotated
import typer
import matplotlib.pyplot as plt
import seaborn as sns
import pandas as pd

app = typer.Typer(add_completion=False)


def run_lsh(binary: str, d: int, n: int, m: int, k: int, num_l: int, r: int, threads: int) -> float | None:
    result = subprocess.run(
        [binary, str(d), str(n), str(m), str(k), str(num_l), str(r),
         "--threads", str(threads)],
        capture_output=True, text=True,
    )
    match = re.search(r"Correct rate: ([\d.]+)%", result.stdout)
    return float(match.group(1)) if match else None


def best_k_for_l(binary: str, d: int, n: int, m: int, num_l: int, r: int, threads: int, max_k: int) -> tuple[int | None, float]:
    best_k, best_acc = None, -1.0
    for k in range(1, max_k + 1):
        acc = run_lsh(binary, d, n, m, k, num_l, r, threads)
        if acc is None:
            typer.echo(f"  [L={num_l}, k={k}] binary error", err=True)
            continue
        if acc > best_acc:
            best_acc, best_k = acc, k
    return best_k, best_acc


def plot_results(coarse: list[dict], fine: list[dict], threshold: float, out: str):
    df = pd.DataFrame(sorted(coarse + fine, key=lambda x: x["L"]))

    sns.set_theme(style="darkgrid")
    _, ax1 = plt.subplots(figsize=(10, 5))
    ax2 = ax1.twinx()

    # grid below everything: only ax1 draws grid, ax2 is transparent to it
    ax1.set_axisbelow(True)
    ax2.set_axisbelow(True)
    ax2.grid(False)

    color_acc = "#1f77b4"
    color_k   = "#ff7f0e"

    ax1.plot(df["L"], df["accuracy"], marker="o", color=color_acc, label="Accuracy (%)", zorder=3)
    ax2.plot(df["L"], df["best_k"],   marker="s", color=color_k,   label="Best k", linestyle="--", zorder=3)
    ax1.axhline(threshold, color="red", linestyle="--", linewidth=1, label=f"{threshold}% threshold", zorder=3)

    ax2.set_ylim(1, 15)
    ax2.set_yticks(range(1, 16))

    ax1.set_xlabel("L", color="black")
    ax1.set_ylabel("Accuracy (%)", color="black")
    ax2.set_ylabel("Best k", color="black")
    ax1.tick_params(axis="both", labelcolor="black")
    ax2.tick_params(axis="y", labelcolor="black")

    lines1, labels1 = ax1.get_legend_handles_labels()
    lines2, labels2 = ax2.get_legend_handles_labels()
    leg = ax1.legend(lines1 + lines2, labels1 + labels2, loc="lower right")
    leg.set_zorder(5)

    plt.tight_layout()
    plt.savefig(out, dpi=150)
    typer.echo(f"Plot saved to {out}")
    plt.close()


@app.command()
def main(
    d: int,
    n: int,
    m: int,
    r: int,
    threads:   Annotated[int,   typer.Option(help="Worker threads (0 = all cores)")] = 0,
    max_l:     Annotated[int,   typer.Option(help="Maximum L (grid doubles from 1)")] = 64,
    max_k:     Annotated[int,   typer.Option(help="Maximum k to scan")] = 20,
    threshold: Annotated[float, typer.Option(help="Accuracy threshold (%) triggering fine search")] = 50.0,
    binary:    Annotated[str,   typer.Option(help="Path to lsh binary")] = "./target/release/lsh",
    out:       Annotated[str,   typer.Option(help="Output plot filename")] = "lsh_search.pdf",
):
    typer.echo(f"Grid search: d={d} n={n} m={m} r={r} | threads={threads} | L=1..{max_l} k=1..{max_k} threshold={threshold}%")
    typer.echo("-" * 60)

    coarse: list[dict] = []
    fine:   list[dict] = []
    fine_done = False
    prev_l: int | None = None

    def scan(num_l: int, store: list[dict]):
        bk, ba = best_k_for_l(binary, d, n, m, num_l, r, threads, max_k)
        tag = "(fine)" if store is fine else ""
        if bk is None:
            typer.echo(f"L={num_l:5d}: all runs failed {tag}")
        else:
            typer.echo(f"L={num_l:5d}: best k={bk:3d}, accuracy={ba:.2f}% {tag}")
        store.append({"L": num_l, "best_k": bk, "accuracy": ba})
        return ba

    num_l = 1
    while num_l <= max_l:
        acc = scan(num_l, coarse)

        if not fine_done and acc >= threshold and prev_l is not None:
            typer.echo(f"\n--- accuracy crossed {threshold}%, fine search L={prev_l+1}..{num_l-1} ---")
            for mid in range(prev_l + 1, num_l):
                scan(mid, fine)
            typer.echo("--- fine search complete ---\n")
            fine_done = True

        prev_l = num_l
        num_l *= 2

    plot_results(coarse, fine, threshold, out)


if __name__ == "__main__":
    app()
