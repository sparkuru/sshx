# ADR 0006: No required remote Agent; sourced optional integration later

Date: 2026-10-22. Status: **basic boundary decided; precise hooks/Agent deferred**.

## Decision and context

Standard SSH login requires no remote SSHX Agent and no SFTP subsystem. Image/file transfer separately requires an actually available SFTP service. Outer Session identity never proves a nested ssh/sudo/container/tmux destination or the current Shell cwd/edit position. Unknown OS/Shell/readiness/cwd stays unknown; prompt text, titles and arbitrary OSC are not authenticated facts.

Core must not inject probe commands into a live interactive Shell to manufacture context. A later supported hook/editor integration may report structured context over an SSH-protected separate channel, with Session/generation, source, observation time, freshness and trust. Remote reports remain untrusted data; schema validation and local effect authorization are still required. No remote output can grant local clipboard-read, file upload or port-open permission.

## Alternatives and consequences

Compare no Agent, a temporary on-demand component and a persistent Agent only when a concrete capability needs them. Prefer the smallest deployment and explicitly consent to any remote change. Never silently append permanent hooks to user Shell profiles or listen on public interfaces. No Agent approach can simply infer reliable command text/exit status from raw keystrokes.

WezTerm documents Shell integration markers such as cwd and prompt/command boundaries. Such markers are a reference for future adapters, not proof of complete command history or SSHX authenticity. [Official Shell integration documentation](https://wezterm.org/shell-integration.html), reviewed 2026-10-22.

Until reliable mapping/edit context exists, Image Paste retains a transfer result and offers explicit local preview/copy. That is an intentional MVP result, not a false automatic-insertion claim. History is default-off, supported-Shell structured data only, filtered before persistence with pause/exclude/retention/delete; command fields can be unknown and filtering cannot promise complete desensitization. Command Reference uses licensed offline examples; History access needs separate permission. Clipboard Bridge authorizes read and write separately. Port Forward keeps resource mechanisms in Core, policy in plugins, loopback default, and separate process/listening/reachability states.

## Evidence and reopening gate

No hook/Agent/history/forwarding capability is implemented. E7 validates unknown-context refusal and malicious-remote isolation with synthetic contracts. Phase 2 defines forwarding and Phase 3 selects Shell-specific hook/editor contracts, source authenticity and freshness, opt-in persistence/deletion and dedicated auxiliary interactive-auth UX. Reopen only for an approved capability whose evidence shows the no-component route insufficient; document installation/teardown and trust boundaries before execution.
