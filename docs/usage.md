# Guia de uso do cdd

## Pré-requisito

O `cdd` precisa do wrapper instalado no perfil do shell. O binário Rust encontra o destino; a função Bash/Zsh ou PowerShell executa a mudança de diretório no terminal atual.

Depois da instalação, abra outro terminal ou recarregue o perfil:

```bash
source ~/.bashrc
```

No PowerShell:

```powershell
. $PROFILE
```

## Busca

```bash
cdd www app
```

Termos comuns fazem correspondência parcial case-insensitive em nomes de diretórios. Os termos seguem a hierarquia do caminho por padrão (`-qs`) e o último termo efetivo deve corresponder ao próprio diretório de destino. Um ancestral correspondente não transforma seus descendentes em resultados.

O `cdd` não corrige termos nem seleciona nomes aproximados.

### Curingas

- `*`: zero ou mais caracteres dentro de um nome de diretório.
- `?`: exatamente um caractere dentro de um nome de diretório.

```bash
cdd 'proj*' 'app?'
```

As aspas são necessárias no Bash/Zsh para que o próprio shell não expanda os curingas antes de chamar o `cdd`. Curingas não atravessam separadores de caminho.

## Opções

| Opção | Efeito |
|---|---|
| `-h`, `--help` | Mostra a ajuda |
| `-l`, `-1` | Usa o primeiro resultado e pula o menu |
| `-2` até `-20` | Quantidade de linhas visíveis da lista interativa |
| `-oa`, `-od`, `-of` | Ordem ascendente, descendente ou encontrada |
| `-qs`, `-qi`, `-qa` | Queries sequenciais, inversas ou em qualquer ordem |
| `--` | Encerra opções; permite um termo iniciado por hífen |

Uma opção explícita de tamanho (`-2` até `-20`) desativa o lucky pick naquela execução para que a lista seja exibida.

O limite visual não reduz o conjunto de candidatos: ao digitar na lista, o filtro textual pesquisa em todos os diretórios encontrados usando correspondência parcial literal e case-insensitive. Se nada corresponder, a lista mostra `Nenhum caminho encontrado com o filtro.`.

## Configuração persistente

Acrescente `:on` para persistir e `:off` para remover uma configuração:

```bash
cdd projeto -15:on
cdd projeto -oa:on
cdd projeto -l:off
```

Quando uma execução é afetada por configurações persistentes, o `cdd` mostra:

```text
(filtros ativos: -15=on, -oa=on)
```

O arquivo é `cdd/cdd.json` dentro do diretório de configuração do sistema (`~/.config` no Linux, normalmente).

## Raiz da busca

- Linux/WSL: `/`, excluindo `/proc`, `/sys`, `/dev` e `/run`.
- Windows: raiz do drive atual.
- Windows com letra inicial: `cdd d projeto` pesquisa em `D:\`.

## Resultados

- Nenhum resultado: mensagem de erro e código de saída `1`.
- Um resultado: muda diretamente.
- Vários resultados: abre o menu interativo; o item focado usa texto preto sobre fundo cinza.
- Opção inválida: mensagem de erro e código de saída `2`.
