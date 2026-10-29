# Changelog

All notable changes to ext-template are documented here. The format is
based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Extensions pick changes up with `bin/sync`; the version records which
scaffold an extension was synced from.

## [0.2.2] - 2026-10-29

### Added

- **Stubs check.** `.github/scripts/stubs-check.php` prints a canonical
  signature listing of the built extension and of `stubs/<file>` (classes,
  finality, parents, interfaces, constants, method modifiers, parameters,
  defaults, return types); the managed CI diffs the two on every leg and
  `make stubs-check` runs the same locally. Curated stubs can no longer
  drift from the registered classes. Skipped with a warning until an
  extension has a stubs file.

### Changed

- Actions: `actions/checkout` v7, `configure-pages` v6, `deploy-pages` v5,
  `upload-pages-artifact` v5, `softprops/action-gh-release` v3 (Node 24
  runtime) — the first Dependabot batch.

## [0.2.1] - 2026-10-29

### Fixed

- Release builds export `GGML_NATIVE=OFF` on x86_64 legs, so a vendored
  ggml (whisper.cpp) is built for ggml's portable AVX2 baseline instead
  of the runner's CPU. The ubuntu-26.04 runners have AVX-512, and
  ext-whisper 0.1.1's x86_64 binaries crashed with SIGILL on AVX2-only
  CPUs.

## [0.2.0] - 2026-10-29

### Added

- Weekly scheduled CI and `workflow_dispatch` in the managed `ci.yml`.
- ZTS test legs (Linux, PHP 8.4 / 8.5), `continue-on-error` for now,
  with a step that asserts the runner's `PHP_ZTS` matches the leg.
- Managed `.github/dependabot.yml`: grouped weekly cargo minor/patch,
  ignoring the versions.conf pins and each extension's primary upstream
  crate.
- ext-template's own Dependabot config for GitHub Actions, covering
  `template/managed/.github/workflows`.
- `bin/upstream-check` + `template/upstream.conf` + the weekly
  `upstream-watch` workflow: one `upstream` issue per exact pin that
  falls behind crates.io.
- Seeds: README Status section, stack table and "Writing about this"
  slot; PLAN.md as design notes; CHANGELOG.md.

### Changed

- `EXT_PHP_RS_PIN` `=0.15.13` → `=0.16.3`; `THISERROR_PIN` `=2.0.18` →
  `=2.0.21`.
- `make stubs` writes to `target/` for a diff against the curated
  stubs, and takes `STUBS_PRELOAD` for Linux.

### Fixed

- Linux CI and release builds on ubuntu-26.04 runners: bindgen's
  libclang is pinned to libclang-dev's LLVM (the image mixes clang 20,
  21 and 22, which breaks ext-php-rs's bindings generation).
- `make stubs` passed a `--stubs` flag cargo-php never had.
- setup-php is unpinned (`@v2`) and the PHP 8.5 × macos-14 leg is back
  (#1, July 2026), now synced into every extension.

## [0.1.0] - 2026-06-12

First release: the `template/managed/` and `template/seed/` trees,
POSIX `bin/render` and `bin/sync`, per-repo `.ext-template.conf`, and
the `versions.conf` pin cascade.

[0.2.2]: https://github.com/DisplaceTech/ext-template/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/DisplaceTech/ext-template/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/DisplaceTech/ext-template/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/DisplaceTech/ext-template/releases/tag/v0.1.0
