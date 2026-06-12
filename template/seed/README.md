# ext-{{NAME}}

{{DESCRIPTION}}

Scaffolded from [ext-template](https://github.com/DisplaceTech/ext-template);
the files listed in its `template/managed/` tree are kept in sync with
`bin/sync` — edit those upstream, not here. Everything else is this
extension's own.

House conventions (quality bar, layout, release flow):
[ext-infer](https://github.com/DisplaceTech/ext-infer) is the worked
reference. Every README in this family carries a **deliberately out of
scope** section — write yours early.

## License

MIT © Eric Mann / Displace Technologies. If release binaries statically
link third-party code, add `THIRD-PARTY-NOTICES.md` (see ext-turbovec for
the pattern); the release workflow already attaches a transitive
`cargo about` manifest.
