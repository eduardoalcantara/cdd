# Especificação concluída: Core do cdd

## 1. Objetivo

Implementar uma CLI cross-platform que encontre diretórios por termos parciais ou curingas explícitos e, através de wrappers, altere o diretório do shell atual.

## 2. Contrato de busca

- Termos sem curingas: correspondência parcial case-insensitive em nomes de diretórios.
- `*`: zero ou mais caracteres dentro de um nome.
- `?`: exatamente um caractere dentro de um nome.
- Curingas não atravessam separadores.
- Não há fuzzy matching, autocorreção nem escolha aproximada.
- No Bash/Zsh, padrões devem ser citados (`cdd 'proj*'`) para evitar expansão pelo shell.
- O último termo na ordem efetiva deve corresponder ao nome do diretório de destino; correspondência apenas em um ancestral não inclui seus descendentes.

### Modos

- `-qs`: termos na ordem hierárquica informada (padrão).
- `-qi`: termos na ordem inversa.
- `-qa`: todos os termos existem, independentemente da ordem.

### Raiz

- Linux/WSL: `/`, sem descer em `/proc`, `/sys`, `/dev` e `/run`.
- Windows: drive atual; uma letra como primeiro termo seleciona outro drive.

## 3. Arquitetura

1. O binário Rust interpreta argumentos, lê configuração, pesquisa e abre a TUI.
2. O caminho escolhido é escrito no arquivo indicado por `--cdd-out-file`.
3. O wrapper Bash/Zsh ou PowerShell lê esse arquivo e executa `cd`/`Set-Location`.

## 4. Configuração

Arquivo: `cdd/cdd.json` no diretório de configuração do sistema.

- `:on` persiste uma opção.
- `:off` remove a persistência e restaura o default daquele grupo.
- O formato inclui metadados sticky com `serde(default)` e migra configurações antigas por seus valores não default.
- Execuções afetadas mostram `(filtros ativos: ...)`.

## 5. Opções

- `-h`, `--help`: ajuda.
- `-l`, `-1`: primeiro resultado, sem TUI.
- `-2` até `-20`: quantidade de linhas visíveis; uma ocorrência explícita desativa lucky pick na execução.
- `-oa`, `-od`, `-of`: ordem alfabética ascendente, descendente ou ordem da varredura.
- `-qs`, `-qi`, `-qa`: modo dos termos.
- `--`: encerra opções.

Flags de comportamento aceitam `:on` e `:off`. Flags desconhecidas e tamanhos fora da faixa retornam código `2`.

## 6. Interface

- Um resultado: seleção direta.
- Vários resultados: menu `inquire`, com o item focado em texto preto sobre fundo cinza.
- O filtro textual do menu opera sobre todos os resultados encontrados, com correspondência parcial literal e case-insensitive; `-2` até `-20` limita somente as linhas visíveis.
- Um filtro sem correspondências mostra `Nenhum caminho encontrado com o filtro.` e não permite selecionar um caminho aproximado.
- Nenhum resultado: código `1`.
- Ajuda: código `0`.

## 7. Critérios de aceite

- [x] Core Rust e wrappers Bash/PowerShell.
- [x] Configuração persistente com indicação dos filtros ativos.
- [x] Correspondência parcial e curingas `*`/`?`.
- [x] Modos sequencial, inverso e qualquer ordem.
- [x] Lista limitada a `-2` até `-20`.
- [x] Ajuda `-h`/`--help`.
- [x] Exclusões seguras na raiz Linux.
- [x] Testes automatizados de parser, config, matcher e busca.
- [x] Documentação de uso.

## 8. Validação

Validado em Linux:

- `cargo fmt --check`
- `cargo check`
- `cargo test` — 22 testes
- `cargo clippy --all-targets -- -D warnings`
- build release
- smoke test do wrapper Bash para `--help` e erro de opção

PowerShell não foi executado neste ambiente por ausência de `pwsh`; o wrapper foi revisado para normalizar o caminho e preservar `$LASTEXITCODE`.
