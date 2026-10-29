<h1 align="center">ext-template</h1>

<p align="center">
  <strong>The shared scaffold for Displace PHP extensions.</strong><br>
  Rust + ext-php-rs build glue, Makefile, PHPT harness, CI/release/docs workflows — rendered for new extensions, synced into existing ones.
</p>

<p align="center">
  <a href="https://github.com/DisplaceTech/ext-template/actions/workflows/ci.yml"><img src="https://github.com/DisplaceTech/ext-template/actions/workflows/ci.yml/badge.svg" alt="CI" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-green" alt="MIT License" /></a>
</p>

---

## Why this exists

[ext-infer](https://github.com/DisplaceTech/ext-infer),
[ext-turbovec](https://github.com/DisplaceTech/ext-turbovec), and
[ext-whisper](https://github.com/DisplaceTech/ext-whisper) share ~90% of
their scaffold, and every convention fix used to be hand-mirrored three
times (the cargo-about `--features cli` fix was re-learned per repo —
that's the bug this repo retires). The scaffold now lives here once;
extensions render from it and re-sync when it improves.

## Two trees, two lifecycles

| Tree | Files | Lifecycle |
|---|---|---|
| `template/managed/` | Makefile, build.rs, about.toml/hbs, rust-toolchain.toml, CI/release/docs workflows, `.github/dependabot.yml` | **Owned by the template.** Re-rendered into extensions by `bin/sync`; never hand-edit in a child repo. |
| `template/seed/` | Cargo.toml, composer.json, `.cargo/`, src skeleton, docs skeleton, README/PLAN/CHANGELOG/RELEASE | **Rendered once** at creation, then owned by the extension. Sync never touches them. |

**Version bumps cascade.** `template/versions.conf` is the single
source of truth for the Rust channel, the declared MSRV, and the crate
pins every extension shares (`ext-php-rs`, `thiserror`). Bump a value
there, run `bin/sync` against each extension, and the change lands in
each child's `rust-toolchain.toml` (managed) and Cargo.toml (only those
exact pin lines — the rest of Cargo.toml stays extension-owned). The
floor is set by the most demanding extension; everyone rides it.

Per-repo variance lives in each extension's `.ext-template.conf`
(see [`ext-template.conf.example`](ext-template.conf.example)): name,
namespace, docs host, apt/brew deps, and any codegen flags shipped
binaries must keep (`X86_RUSTFLAGS` — e.g. ext-turbovec's
`-C target-cpu=x86-64-v3`).

## Creating a new extension

```sh
cp ext-template.conf.example /tmp/foo.conf   # edit values
bin/render /tmp/foo.conf ../ext-foo
cd ../ext-foo && git init -b main && cargo build && make test
```

The rendered skeleton compiles and passes its PHPT immediately — the
domain work starts from a green baseline. Remember the house rule:
propose the file tree + dependency list + Rust ownership design before
building the surface.

## Syncing a fix into existing extensions

Change the file under `template/managed/`, then for each extension:

```sh
bin/sync ../ext-infer
git -C ../ext-infer diff      # review — sync output is a proposal, not a commit
```

Commit in the child as `chore: sync scaffold from ext-template@<sha>` so
the provenance is greppable.

## Dependency updates and scheduled CI

Naive Dependabot would fight `bin/sync`: a child-side PR bumping a
managed workflow or a versions.conf pin is reverted by the next sync. So
each kind of dependency has exactly one owner:

| Dependency | Owner | How it moves |
| ---------- | ----- | ------------ |
| GitHub Actions (this repo's workflows **and** `template/managed/.github/workflows`) | ext-template's [`.github/dependabot.yml`](.github/dependabot.yml) | Weekly grouped PR here; merge, then `bin/sync` each extension. |
| `ext-php-rs`, `thiserror`, Rust channel | [`template/versions.conf`](template/versions.conf) | Deliberate bump here, cascaded by `bin/sync`; surfaced by the upstream watcher. |
| Each extension's primary upstream crate (`llama-cpp-2`, `turbovec`, `whisper-rs` and their `-sys` crates) | the extension's `Cargo.toml` | Deliberate, tested bump in the extension (format/behaviour checks per its RELEASE.md); surfaced by the upstream watcher. |
| Every other crate in an extension | the managed [`dependabot.yml`](template/managed/.github/dependabot.yml) synced into each child | Weekly grouped minor/patch cargo PR in the child; the classes above are ignored, and `github-actions` is not enabled there. |

**Dependabot and the template path.** Dependabot scans a non-root
`directory` for workflow YAML directly (dependabot-core's
`github_actions/file_fetcher.rb` uses `workflows_dir = "."` for any
directory other than `/`), and the unrendered templates parse as YAML
(`{{PLACEHOLDER}}` is a valid flow mapping; checked with Ruby's
`YAML.safe_load`, the parser Dependabot uses), so action bumps for the
managed workflows land here once.

**Upstream watcher.** [`bin/upstream-check`](bin/upstream-check) reads
[`template/upstream.conf`](template/upstream.conf) and compares each
exact pin Dependabot is told to ignore against crates.io's latest stable
release. The weekly [`upstream-watch`](.github/workflows/upstream-watch.yml)
workflow runs it with `--issues`: one `upstream`-labelled issue per
lagging crate in this repo, kept current as crates.io moves and closed
when the pin catches up. Run `bin/upstream-check` locally for a status
table.

**Scheduled CI.** The managed `ci.yml` runs weekly (Mondays) and on
`workflow_dispatch`, so a green badge means "green this week". GitHub
disables scheduled workflows in public repos after 60 days with no
repository activity; the weekly Dependabot PRs, and merging them, are
what keep the schedules alive. If one goes dormant, re-enable it from
the Actions tab.

**ZTS.** The managed CI matrix adds thread-safe PHP 8.4 and 8.5 legs on
Linux (setup-php `phpts: ts`), `continue-on-error` until they have a
track record. Release binaries stay NTS-only.

## Stubs

Committed stubs are curated (generics, `@throws`, prose), so
`make stubs` writes cargo-php's output to `target/<stubs file>` for a
signature diff instead of overwriting them. On Linux, cargo-php's
`dlopen()` needs PHP's embed library preloaded
(`make stubs STUBS_PRELOAD=/usr/lib/libphp8.4.so`); macOS needs nothing.

## Placeholder convention

Uppercase `{{LIKE_THIS}}`, substituted from `.ext-template.conf` —
deliberately disjoint from GitHub Actions' `${{ lowercase }}`
expressions so workflow files render safely. Rendering hard-fails if any
placeholder survives.

## Deliberately out of scope

**A cargo-generate/cookiecutter dependency** — two POSIX sh scripts and
sed are auditable and run anywhere · **syncing seed files** — an
extension's Cargo.toml, sources, and docs diverge legitimately; pushing
template versions over them would destroy real differences (ext-turbovec's
1.89 toolchain floor and x86-64-v3 baseline are the proof cases) ·
**Windows scaffolding** — out of scope platform-wide.

## License

[MIT](LICENSE) &copy; 2026 Eric Mann / Displace Technologies
