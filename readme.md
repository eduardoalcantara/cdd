# cdd (Change Directory Directly)

> Evolução do comando `cd` do Linux projetada para usuários velozes de linha de comando.

## Resumo

O `cdd` encontra diretórios por trechos do nome ou por curingas explícitos e muda o diretório do shell atual através de um wrapper Bash/Zsh ou PowerShell.

```bash
cdd www app1
cdd 'proj*' 'app?'
cdd --help
```

Termos comuns usam correspondência parcial case-insensitive. `*` corresponde a zero ou mais caracteres e `?` a exatamente um caractere no nome de um diretório. O projeto não usa fuzzy matching nem autocorreção.

No Bash/Zsh, coloque curingas entre aspas para impedir a expansão antecipada pelo shell.

## Principais recursos

- Busca a partir de `/` no Linux/WSL ou do drive atual no Windows.
- Queries sequenciais, inversas ou em qualquer ordem.
- Curingas explícitos `*` e `?`.
- Lista interativa com 10 linhas visíveis por padrão.
- Filtro textual literal sobre todos os resultados encontrados.
- Configurações persistentes com `:on` e `:off`.
- Ajuda integrada por `cdd --help`.
- Instalação e desinstalação reversíveis.

## Instalação rápida

- Desenvolvimento (compila com Rust): `scripts/setup/install.sh` ou `install.ps1`.
- Pacote pré-compilado: extraia o `.tar.gz`/`.zip` e execute o `install.sh`/`install.ps1` incluído.
- Distribuição: `scripts/setup/build-dist.sh` ou `build-dist.ps1`.

Instruções completas: [`docs/HOW_TO_INSTALL.md`](docs/HOW_TO_INSTALL.md).

## Uso rápido

```bash
cdd linux
cdd projetos api
cdd 'proj*' 'app?'
cdd linux -20
cdd --help
```

Guia completo: [`docs/HOW_TO_USE.md`](docs/HOW_TO_USE.md).

## Estrutura de alto nível

| Caminho | Função |
|---|---|
| `spec-root.md` | Autoridade arquitetural do projeto |
| `rules.md` / `.cursorrules` | Governança e comportamento do agente |
| `flow.md` | Ordem de leitura e execução |
| `.prompt-status` | Rastreamento de cada prompt |
| `status.md` / `timeline.md` | Estado atual e histórico |
| `setup.md` / `tools-*.md` | Ambiente e ferramentas |
| `specs/` | Especificações formais (`to-do/`, `done/`) |
| `core/` | Conteúdo específico do projeto |
| `scripts/` | Automações e bootstrap |
| `docs/`, `ideas/`, `references/`, `reports/`, `prompts/`, `resources/` | Apoio documental e operacional |

## Como iniciar

1. Clone o repositório.
2. Certifique-se de possuir o toolchain do **Rust** instalado (`cargo`).
3. Execute `cd core && cargo test`.
4. Execute o instalador da sua plataforma.
5. Consulte `cdd --help`.

Detalhes de ambiente: ver `setup.md`, `tools-linux.md` e `tools-windows.md`.

## Documentação relacionada

- Uso: [`docs/HOW_TO_USE.md`](docs/HOW_TO_USE.md)
- Instalação: [`docs/HOW_TO_INSTALL.md`](docs/HOW_TO_INSTALL.md)
- Arquitetura e algoritmo: [`docs/HOW_IT_WORKS.md`](docs/HOW_IT_WORKS.md)
- Fundação do template: `spec-project-bootstrap.md`
- Fluxo do agente: `flow.md`
- Regras permanentes: `rules.md`
- Regras de scripts: `rules-scripts.md`
- Molde de specs: `spec-template.md`

## Estado atual

Ver `status.md` para o snapshot vivo do projeto.
