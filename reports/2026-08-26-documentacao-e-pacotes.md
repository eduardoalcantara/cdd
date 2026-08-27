# Relatório — documentação e pacotes do cdd

## Entrega

- Criados `docs/HOW_TO_USE.md`, `docs/HOW_TO_INSTALL.md` e `docs/HOW_IT_WORKS.md`.
- Atualizados `readme.md`, `setup.md`, `status.md` e `timeline.md`.
- Scripts de distribuição passaram a incluir documentação e checksums SHA-256.
- Linux passou a usar target `x86_64-unknown-linux-musl` para produzir binário static-pie.

## Artefatos

- `dist/cdd-linux-x86_64.tar.gz`
- `dist/cdd-linux-x86_64.tar.gz.sha256`
- `dist/cdd-windows-x86_64.zip`
- `dist/cdd-windows-x86_64.zip.sha256`

`dist/` permanece ignorado pelo Git; artefatos são saídas locais de build, não arquivos-fonte versionados.

## Validação

### Linux

- Checksum SHA-256 aprovado.
- Conteúdo do `tar.gz` inspecionado.
- Executável identificado como ELF x86-64 static-pie.

### Windows

- Target Rust `x86_64-pc-windows-gnu`.
- Cross-compilação com toolchain MinGW oficial do Arch Linux, pacotes verificados por assinatura.
- Executável identificado como PE32+ console x86-64.
- Conteúdo do ZIP e checksum SHA-256 validados.
- Execução do `cdd.exe` e do instalador PowerShell não realizada neste host Linux; permanece necessária validação funcional em Windows.
