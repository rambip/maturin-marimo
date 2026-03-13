# maturin-marimo
Test setup for marimo loading extension compiled by maturin

Minimal setup with:

- uv-managed Python project metadata in [pyproject.toml](pyproject.toml)
- a Rust extension module built with maturin and PyO3 in [Cargo.toml](Cargo.toml) and [src/lib.rs](src/lib.rs)
- marimo as a Python dev dependency

Install:

```bash
uv sync
uv run maturin develop
uv run python -c "import maturin_marimo; print(maturin_marimo.magic_number())"
```

To easily recompile the source and to bump the magic number:
```
sh recompile.sh
```

And then, check if marimo was able to reload it
