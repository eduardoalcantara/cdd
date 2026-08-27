# core/

Binário Rust do `cdd`.

## Desenvolvimento

```bash
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
```

Ajuda:

```bash
cargo run -- --help
```

## Módulos

- `args.rs`: parser, validação e sticky settings.
- `config.rs`: configuração JSON retrocompatível.
- `pattern.rs`: correspondência parcial literal e curingas `*`/`?`.
- `search.rs`: varredura concorrente, modos de query e ordenação.
- `tui.rs`: seleção interativa.
- `help.rs`: ajuda integrada.

O argumento interno `--cdd-out-file` é usado exclusivamente pelos wrappers para comunicar o diretório escolhido sem capturar a saída da TUI.
