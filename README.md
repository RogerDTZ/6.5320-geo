# MIT 6.5320

**PS1:** An interactive visualization of the closest pair of points algorithm.

**PS2:** An implementation of LSH.

## Build

Install Rust via [rustup](https://rust-lang.org/tools/install/).

## PS1

```bash
cargo run --release --bin geo
```

Left click to add a point, right click to remove a point.

Controls:
- **Run** — execute the closest pair algorithm on current points
- **Animated** — if enabled (≤1000 points), records and plays back the algorithm step by step
- **Clear points** — remove all points from the canvas
- **Random points** — generate N random points uniformly over the canvas
- **Interesting Distribution** — repeatedly removes the closest pair until all remaining pairs exceed a distance threshold, producing a well-spread point set


## PS2

Implementation of LSH as introduced in lecture, with parallel query support.

```bash
cargo run --release --bin lsh -- <D> <N> <M> <K> <L> <R>
```

Use `-t` or `--threads` to specify parallelism. Defaults to all cores.

The search over `L` and `k` is done by `lsh_search.py`.
