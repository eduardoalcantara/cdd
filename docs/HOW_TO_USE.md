# Como usar o cdd

## Conceito

O `cdd` localiza um diretório sem exigir seu caminho completo. O usuário informa partes dos nomes das pastas; o binário pesquisa, apresenta os destinos válidos e o wrapper altera o diretório do shell atual.

```bash
cdd projetos meu-app
```

## Busca textual

Termos comuns usam correspondência parcial literal e case-insensitive:

```bash
cdd linux
cdd projetos api
```

O termo final efetivo deve corresponder ao próprio diretório de destino. Um match em um ancestral não inclui automaticamente todos os seus descendentes.

O `cdd` não corrige termos nem seleciona nomes aproximados.

## Curingas

- `*`: zero ou mais caracteres dentro do nome de um diretório.
- `?`: exatamente um caractere.

No Bash/Zsh, use aspas para impedir que o shell expanda o padrão:

```bash
cdd 'proj*' 'app?'
```

## Lista interativa

Quando há vários destinos, a TUI permite:

- `↑` / `↓`: mover o foco;
- `Page Up` / `Page Down`: mudar de página;
- `Home` / `End`: primeiro ou último resultado;
- digitação: filtrar todos os resultados por substring literal;
- `Enter`: selecionar;
- `Esc`: cancelar.

O item focado usa texto preto sobre fundo cinza. A lista mostra 10 linhas por padrão, mas o filtro textual atua sobre todos os resultados. Quando o filtro não encontra nada, aparece:

```text
Nenhum caminho encontrado com o filtro.
```

## Opções

| Opção | Comportamento |
|---|---|
| `-h`, `--help` | Exibe a ajuda |
| `-l`, `-1` | Usa o primeiro resultado sem abrir a TUI |
| `-2` até `-20` | Define a quantidade de linhas visíveis |
| `-oa` | Ordem alfabética ascendente |
| `-od` | Ordem alfabética descendente |
| `-of` | Ordem da varredura, padrão |
| `-qs` | Termos na ordem hierárquica informada, padrão |
| `-qi` | Termos na ordem inversa |
| `-qa` | Termos em qualquer ordem |
| `--` | Encerra opções |

Exemplos:

```bash
cdd linux -oa
cdd app www -qi
cdd projeto -20
cdd cache -l
```

## Configuração persistente

Use `:on` para persistir uma opção e `:off` para removê-la:

```bash
cdd projeto -15:on
cdd projeto -oa:on
cdd projeto -qa:off
```

Filtros persistentes ativos são mostrados antes da busca:

```text
(filtros ativos: -15=on, -oa=on)
```

O arquivo é `cdd/cdd.json` dentro do diretório de configuração do sistema.

## Raízes de pesquisa

- Linux/WSL: `/`, sem entrar em `/proc`, `/sys`, `/dev` e `/run`.
- Windows: drive atual.
- Windows com letra inicial: `cdd d projeto` pesquisa em `D:\`.

## Códigos de saída

- `0`: seleção ou ajuda bem-sucedida;
- `1`: nenhum diretório, cancelamento ou falha operacional;
- `2`: opção inválida.
