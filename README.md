# lib-serialize

`serialize` — the format-agnostic serialization port: `Serializer`/`Encoder`/`Decoder` traits + `serialize.serialize`/`deserialize`.

Rule B: this crate carries **no formats**; `lib-json` (and later `lib-toml`/`lib-yaml`) register themselves.

```text
src/index.hnl   the interface
src/{lib,serializer,traits,error}.rs   the traits
src/ops.rs      the two ops + `serialize_op_registry_with(registry)`
```

## License

MIT.
