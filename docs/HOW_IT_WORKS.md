# Como o cdd funciona

Este documento descreve o fluxo técnico e os contratos que devem ser preservados por desenvolvedores e agentes de IA.

## Por que existe um wrapper

Um processo filho não consegue alterar o diretório de trabalho do shell que o iniciou. Por isso, o `cdd` é dividido em:

1. **Core Rust:** pesquisa e escolhe um caminho absoluto.
2. **Wrapper do shell:** executa o core e aplica `cd`/`Set-Location` no shell atual.

O wrapper cria um arquivo temporário e chama:

```text
cdd <argumentos> --cdd-out-file <arquivo-temporário>
```

O core escreve somente o caminho escolhido nesse arquivo. Bash/Zsh lê o caminho e executa `cd`; PowerShell usa `Set-Location`.

## Fluxo principal

```text
Usuário
  └─> função cdd do wrapper
       ├─> cria arquivo temporário
       └─> executa o core Rust
            ├─> carrega configuração
            ├─> interpreta argumentos
            ├─> escolhe raiz de pesquisa
            ├─> percorre diretórios
            ├─> aplica queries
            ├─> ordena resultados
            ├─> seleciona diretamente ou abre TUI
            └─> grava caminho escolhido
       └─> wrapper valida o caminho
            └─> cd / Set-Location
```

## Módulos do core

| Arquivo | Responsabilidade |
|---|---|
| `src/main.rs` | Orquestra configuração, ajuda, busca, TUI e saída |
| `src/args.rs` | Parser, validação de flags e aplicação sticky |
| `src/config.rs` | Configuração JSON e migração retrocompatível |
| `src/pattern.rs` | Termos literais e curingas `*`/`?` |
| `src/search.rs` | Raiz, varredura, query order e ordenação |
| `src/tui.rs` | Lista interativa, filtro literal e seleção |
| `src/help.rs` | Texto de `-h`/`--help` |

## Parsing e configuração

`args.rs` recebe a configuração carregada e produz `AppArgs`.

Regras importantes:

- flags desconhecidas retornam erro;
- list size só aceita `2..=20`;
- uma list size explícita desativa lucky pick naquela execução;
- `--` encerra o parsing de opções;
- `--cdd-out-file` é interno aos wrappers.

As opções com `:on` alteram o valor e ativam metadados em `StickyState`. `:off` restaura o default do grupo e remove o metadado.

Configurações antigas, sem `sticky`, são migradas por valores não default. O arquivo é salvo em `cdd/cdd.json` dentro do diretório de configuração do sistema.

## Raiz e varredura

`search.rs` usa `jwalk` para percorrer diretórios concorrentemente:

- Linux/WSL: raiz `/`;
- Windows: drive atual;
- Windows: uma letra como primeiro termo muda o drive.

Ao pesquisar `/` no Linux, a leitura não desce em `/proc`, `/sys`, `/dev` e `/run`.

Erros individuais de acesso são ignorados; diretórios acessíveis continuam sendo processados.

## Compilação das queries

`pattern.rs` transforma cada termo em `QueryPattern`:

- sem `*`/`?`: substring literal case-insensitive;
- com `*`/`?`: regex ancorada ao nome completo do componente;
- `*` vira zero ou mais caracteres;
- `?` vira exatamente um caractere;
- demais metacaracteres são escapados.

Não introduza fuzzy matching ou autocorreção nessa etapa.

## Correspondência no caminho

Cada path é convertido em componentes de diretório.

### Sequential (`-qs`)

Os termos devem aparecer na ordem informada. O último termo deve corresponder ao próprio diretório de destino.

```text
queries: www, app
path:    /var/www/app
```

### Inverse (`-qi`)

Inverte os termos antes de aplicar a regra sequencial.

```text
queries: app, www
path:    /var/www/app
```

### Any (`-qa`)

Todos os termos devem existir em algum componente e pelo menos um deles deve corresponder ao diretório de destino.

Essa exigência terminal impede que um match em um ancestral inclua todos os descendentes.

## Ordenação e lucky pick

- `-of`: mantém a ordem da varredura;
- `-oa`: ordena ascendente;
- `-od`: ordena descendente;
- `-l`/`-1`: interrompe a varredura no primeiro match.

Sem lucky pick:

- zero resultados: erro;
- um resultado: seleção direta;
- vários resultados: TUI.

## TUI e filtro textual

`tui.rs` passa todos os resultados ao `inquire::Select`. `list_size` controla apenas `page_size`, portanto o filtro considera o conjunto completo.

O scorer padrão fuzzy do `inquire` não é usado. Um scorer customizado:

1. converte o filtro para lowercase;
2. exige `path_lowercase.contains(filter)`;
3. preserva a ordem original;
4. oculta caminhos sem substring literal.

Uma entrada sentinela `NoResults` é exibida somente quando o filtro não vazio não corresponde a nenhum caminho:

```text
Nenhum caminho encontrado com o filtro.
```

Selecionar essa entrada não produz destino.

O item focado usa foreground preto e background cinza via `RenderConfig`.

## Comunicação com o wrapper

Ao concluir:

1. o core grava o caminho sem newline em `--cdd-out-file`;
2. o wrapper verifica o exit code;
3. o wrapper confirma que o destino é um diretório;
4. o wrapper muda o diretório;
5. o arquivo temporário é removido.

Sem wrapper, o core pode imprimir o caminho, mas não consegue mudar o diretório do terminal pai.

## Testes e validação

```bash
cd core
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets -- -D warnings
cargo build --release
```

Os testes cobrem parser, sticky settings, migração, curingas, modos de query, exclusões Linux, prevenção de descendentes indevidos, filtro literal e estado vazio da TUI.

## Invariantes para futuras mudanças

- Não reintroduzir fuzzy/autocorreção.
- O diretório final precisa satisfazer o termo terminal efetivo.
- `list_size` limita linhas visíveis, não candidatos filtráveis.
- O filtro da TUI precisa ser literal e usar todos os resultados.
- O core nunca tenta executar `cd`.
- Wrappers precisam preservar paths com espaços e exit codes.
- Instalação deve continuar reversível.
- Mudanças de comportamento exigem testes, atualização da spec, `status.md` e `timeline.md`.
