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
| `template/managed/` | Makefile, build.rs, about.toml/hbs, CI/release/docs workflows | **Owned by the template.** Re-rendered into extensions by `bin/sync`; never hand-edit in a child repo. |
| `template/seed/` | Cargo.toml, composer.json, `.cargo/`, toolchain pin, src skeleton, docs skeleton, PLAN/RELEASE | **Rendered once** at creation, then owned by the extension. Sync never touches them. |

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
