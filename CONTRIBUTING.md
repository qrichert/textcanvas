# Contributing to TextCanvas

Thanks for your interest in contributing to TextCanvas.

TextCanvas has both a Rust implementation and a Python implementation. Shared
features should stay aligned across both codebases.

## Prerequisites

Install the following tools before contributing:

- Rust (edition 2024 toolchain)
- Python 3.12+
- [`uv`](https://docs.astral.sh/uv/)
- [`pre-commit`](https://pre-commit.com/)
- [`cargo-tarpaulin`](https://github.com/xd009642/tarpaulin)

## Get started

1. Fork and clone the repository.
2. Run the full project checks once to verify your environment:

   ```sh
   make check
   ```

3. Use `make help` to see all available development targets.

## Development workflow

The main day-to-day commands are:

- `make lint` — run pre-commit hooks (formatting, linting, docs, and type checks)
- `make rust-test` — run Rust unit tests
- `make python-test` — run Python unit tests
- `make rust-coverage` / `make rust-coverage-pct` — generate or enforce Rust coverage
- `make python-coverage` / `make python-coverage-pct` — generate or enforce Python coverage
- `make check` — run the full quality gate used by CI

The `make check` target includes formatting, documentation generation, strict
Clippy checks, tests, coverage enforcement, and pre-commit validation.

## Project structure

- `src/` — Rust implementation
- `textcanvas/` — Python implementation
- `tests/` — Python tests
- Rust tests live inline with the Rust source

## Code standards

- Keep the project dependency-light. Runtime dependencies should remain minimal.
- Rust changes must pass strict Clippy checks with warnings denied.
- Python changes must pass the configured pre-commit hooks, including Ruff,
  formatting, doc formatting, and Pyright.
- Coverage standards are intentionally high: Rust coverage is enforced at 99%
  and Python coverage at 100%.

## Pull requests

Before opening a pull request:

- make sure all CI-equivalent checks pass locally
- keep Rust and Python behavior in sync when changing shared features
- keep changes focused and well-tested

If you are unsure which command to run, start with:

```sh
make check
```
