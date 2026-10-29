# ext-{{NAME}} — design notes

Design notes and decision history: why the surface looks the way it
does, what was tried and rejected, and the constraints that bind future
work. It is not a to-do list.

- Status and surface: `README.md`
- What shipped, per release: `CHANGELOG.md`
- Remaining work: [`future` issues](https://github.com/DisplaceTech/ext-{{NAME}}/issues?q=is%3Aissue+is%3Aopen+label%3Afuture)
  (one issue per idea, with the reasoning and any locked constraint)
- How to cut a release: `RELEASE.md`

## Before building

Propose the file tree, dependency list, and Rust-side ownership design
(umbrella CLAUDE.md ground rule 5) and get it approved before building
the surface.

## Decisions

| Date | Decision | Why |
| ---- | -------- | --- |
