# Retained Third-Party Material

This inventory supplements existing upstream headers. Project-authored policy in `.trellis/spec/trellis-plus/` is independent text; the imported SSHX design is user-supplied project material. No application dependency, UUPM artifact, Docker wrapper template or preview renderer was added by this initialization.

| Component | Version / source | Retained paths | Exact notice |
| --- | --- | --- | --- |
| `@mindfoldhq/trellis` | 0.6.17 installed npm distribution; upstream `https://github.com/mindfold-ai/trellis`, package metadata `license: AGPL-3.0-only`, author Mindfold LLC | Existing `.trellis/` runtime, agents, configuration, generated workflow/spec scaffolds; managed `AGENTS.md` block; local generated `.agents/` and `.codex/` integrations | [trellis-0.6.17-LICENSE](trellis-0.6.17-LICENSE) |

## Collection evidence

- Source artifact: `LICENSE` from the installed `@mindfoldhq/trellis` 0.6.17 npm package resolved through the existing `trellis` executable. It was copied without modification; the installed package and repository `.trellis/.version` agree.
- The installed package root has `LICENSE` and no separate `NOTICE` or `COPYRIGHT` file. No notice text was invented or substituted.
- License SHA-256: `d8a6cc31abc16b6748c7a21f21611f5a1ec33f67d22ca23d7da1c19b95496bee`.
- At collection, all 87 existing template-metadata entries matched their files. All 28 `.trellis/scripts/**/*.py` files also matched the installed distribution's corresponding template bytes.
- The upstream distribution is evidence for the retained initialization files; its CLI runtime and dependency tree were not vendored. Ordinary future dependencies retain their own package notices and should use the project's eventual dependency-license checks.
- Personal integrations remain local and are excluded from staging. Existing protected runtime/templates were not changed or added to the Trellis Plus commit plan.

## Maintenance

When retained third-party content changes, verify its actual version/source, collect exact applicable license/notice texts, update this inventory and preserve inline notices. If provenance cannot be verified, record `license-notice-needed` and the affected paths; do not replace the root project license or present unresolved material as cleared for distribution. A root SSHX software license has not been selected by this documentation task.
