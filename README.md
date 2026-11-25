# lib-serialize

`serialize` — the format-agnostic serialization port: `Serializer`/`Encoder`/
`Decoder` traits + `serialize.serialize`/`deserialize`.

Rule B: this crate carries **no formats**; `lib-json` (and later
`lib-toml`/`lib-yaml`) register themselves.

```text
src/index.hnl              the interface
src/serialize.hnl          the .hnl sources + tests
src/{lib,serializer,traits,error}.rs   the traits
```

## Layout

```text
src/        the .hnl interface + the Rust trait crate
pkg.hnl     package manifest — modules, deps, description, license
workflows/gate.yaml       bupha entry (handl-gates) → scripts/hnl-gate.sh
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
cargo xtask gate  # full gate (bupha entry)
```

## Development

```bash
cargo xtask gate        # fmt --check → clippy -D warnings → test → doc
```

`cargo xtask` wraps the repo's `xtask/` crate (`fmt`, `clippy`, `test`, `doc`,
`check`, `gate`).

## License

MIT.
