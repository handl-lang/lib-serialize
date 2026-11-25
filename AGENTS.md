# lib-serialize

`serialize` — the format-agnostic serialization **port**:
`Serializer`/`Encoder`/`Decoder` traits + `serialize.serialize` /
`deserialize`.

## Layout

| path | role |
| --- | --- |
| `src/index.hnl`, `src/index_flat.hnl` | the .hnl interface |
| `src/serialize.hnl`, `serialize.test.hnl`, `probe.test.hnl` | .hnl surface + tests |
| `src/{lib,serializer,traits,error}.rs` | the Rust trait crate |
| `Cargo.toml`, `xtask/` | crate manifest + `cargo xtask` gate |
| `pkg.hnl`, `scripts/hnl-gate.sh`, `workflows/gate.yaml` | manifest / hnl gate / bupha entry |

## Contract

- Rule B: this crate carries **no formats** — `lib-json` (and later
  `lib-toml`/`lib-yaml`) register themselves. Backends:
  `lib-serialize-toml-c`, `lib-serialize-yaml-c`.

## Verification

```bash
cargo xtask gate        # fmt --check → clippy -D warnings → test → doc
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
./ci.sh                # full gate (bupha entry)
```

## License

MIT.

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
