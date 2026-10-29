# ext-{{NAME}}

{{DESCRIPTION}}

## Status

<!-- Keep this section near the top and keep it true. Before the first
     release, say "In development" instead. Fill in real values from the
     last full verification run (CI matrix + any model/fixture tier). -->

Feature-complete for its scope. Maintained: dependency updates and
new-model support ship as patch/minor releases — see the
[CHANGELOG](CHANGELOG.md). Last verified: PHP 8.3/8.4/8.5,
`<upstream crate> <x.y.z>`, `<YYYY-MM-DD>`.

Remaining ideas are tracked as
[`future` issues](https://github.com/DisplaceTech/ext-{{NAME}}/issues?q=is%3Aissue+is%3Aopen+label%3Afuture).
**Co-maintainers welcome** — if you run this in production and want a
say in where it goes, open an issue or a PR.

Scaffolded from [ext-template](https://github.com/DisplaceTech/ext-template);
the files listed in its `template/managed/` tree are kept in sync with
`bin/sync` — edit those upstream, not here. Everything else is this
extension's own.

House conventions (quality bar, layout, release flow):
[ext-infer](https://github.com/DisplaceTech/ext-infer) is the worked
reference. Every README in this family carries a **deliberately out of
scope** section — write yours early.

## Part of a stack

Transcribe → chunk → embed → search → answer, entirely on your hardware:

| Stage | Package | What it does |
| ----- | ------- | ------------ |
| Transcribe | [ext-whisper](https://github.com/DisplaceTech/ext-whisper) | Speech-to-text over whisper.cpp |
| Chunk | [ai-toolkit](https://github.com/DisplaceTech/ai-toolkit) | Text chunkers and packed-vector math, pure PHP |
| Embed · rerank · answer | [ext-infer](https://github.com/DisplaceTech/ext-infer) | Local LLM chat, embeddings and reranking over llama.cpp |
| Search | [ext-turbovec](https://github.com/DisplaceTech/ext-turbovec) | Compressed in-memory vector search (TurboQuant) |
| Integrate | [ai-contracts](https://github.com/DisplaceTech/ai-contracts) | The interfaces frameworks code against |
| Reference app | [rag-over-markdown](https://github.com/DisplaceTech/rag-over-markdown) | The whole pipeline over a folder of markdown |

## Writing about this

<!-- Link the posts that explain this extension; check they resolve. -->

## License

MIT © Eric Mann / Displace Technologies. If release binaries statically
link third-party code, add `THIRD-PARTY-NOTICES.md` (see ext-turbovec for
the pattern); the release workflow already attaches a transitive
`cargo about` manifest.
