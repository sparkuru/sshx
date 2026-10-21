# Design Source Provenance

The user initially named root `design.txt`, which was read completely during planning. Between planning and execution it was no longer present in the workspace. The user then supplied `/tmp/tmp/design.txt` and explicitly requested adding it to Trellis spec and mainline. The main session reread that file completely and retained its bytes at:

`.trellis/spec/trellis-plus/sshx-design.txt`

- Imported on: 2026-10-21
- Length: 30,205 bytes
- SHA-256: `d115212b36001c4f0f83f128fb0af5f15623d20121bc0e5820d23da76893d4ff`
- Transformation: none; exact bytes, no translated or rewritten source
- Ownership: user-supplied project requirements, not Trellis/UUPM source
- Canonical source for future tasks: repository-local spec copy; no future dependency on the temporary path
- Scope authority: source import and initialization approved; future product implementation still follows normal guided task selection and planning

The design is in Chinese and retained in its original language for fidelity. New project policy and task evidence are in English, following the existing spec documentation convention. Technical links in the design are source references, not claims that their current APIs were independently researched during initialization.

The discovery record's root-file references describe the planning-time baseline. For every later requirement lookup use the canonical spec copy and its numbered sections.
