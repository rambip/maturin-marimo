# maturin-marimo
Test setup for marimo loading extension compiled by maturin

Minimal setup with:

- uv-managed Python project metadata in [pyproject.toml](pyproject.toml)
- a Rust extension module built with maturin and PyO3 in [Cargo.toml](Cargo.toml) and [src/lib.rs](src/lib.rs)
- marimo as a Python dev dependency

Quick start:

```bash
uv sync --dev
uv run maturin develop
uv run python -c "import maturin_marimo; print(maturin_marimo.hello())"
```
