# timeline.md — histórico evolutivo (cronológico reverso)

## 2026-08-26 — Orquestração de Distribuição End-User

- **Evento:** Desenvolvimento do fluxo de empacotamento para distribuição autônoma (zero dependências) do comando cdd.
- **Impacto:** Geração das rotinas `build-dist.*` que empacotam o binário com o script `install-user.*`. Isso dispensa o usuário final de instalar a suíte Rust, democratizando a adoção do `cdd` via `.zip` ou `.tar.gz`.
- **Arquivos afetados:** `scripts/setup/build-dist.*`, `scripts/setup/install-user.*`.

## 2026-08-26 — Implementação do Core e Shell Wrappers

- **Evento:** Desenvolvimento do motor Rust em `core/` e wrappers de shell (`cdd.sh` / `cdd.ps1`).
- **Impacto:** O comando base `cdd` foi testado e compilado com suporte a buscas rápidas interativas, TUI de navegação e mudança efetiva do diretório pai. Modos interativo, configuráveis (`:on`/`:off`), e de leitura foram completamente aplicados e validados cross-platform no Linux/WSL e Windows.
- **Arquivos afetados:** `core/src/`, `core/Cargo.toml`, `scripts/shell/cdd.*`, `scripts/setup/install.*`.

## 2026-08-26 — Bootstrapping e Personalização do Projeto cdd

- **Evento:** Conversão da base (Template) para o escopo oficial do utilitário `cdd`.
- **Impacto:** O projeto ganha identidade, readme e especificações técnicas de seu core (em Rust com shell wrappers).
- **Arquivos afetados:** `spec-root.md`, `readme.md`, `status.md`, `specs/to-do/SPEC-cdd-core.md`, `prompts/bootstrap-personalizar-projeto.md`.
- **Observações:** Esclarecidas as dúvidas de viabilidade de TUI e de mudança de diretório (`cd`) pelo processo pai usando o padrão de design "Shell Wrapper".

## 2026-08-26 — Atualização de Governança de Scripts

- **Evento:** Importação das regras de automação (`--ssh`, `--quiet`) e cabeçalhos visuais CLI (tabelas Ciano/ASCII OEM 437) para o modelo base.
- **Impacto:** Novos projetos criados a partir deste padrão (incluindo o `cdd`) já nascem com a governança correta para scripts robustos interativos e autônomos. A especificação original foi movida para concluída.
- **Arquivos afetados:** `rules-scripts.md`, `rules.md`, `spec-project-bootstrap.md`, `status.md`, `specs/done/SPEC-cabecalhos-cli.md`.
- **Observações:** Repositório começa sua transição oficial para se tornar o utilitário de console Linux `cdd`.

## 2026-07-26 — Padronização de nomes com hífen

- **Evento:** renomeação de `spec_root.md`, `spec_template.md` e `rules_scripts.md` para hífen.
- **Impacto:** nomenclatura consistente em toda a documentação e scripts de validação.
- **Arquivos afetados:** `spec-root.md`, `spec-template.md`, `rules-scripts.md` e referências em docs/scripts.
- **Observações:** validação estrutural reexecutada com sucesso.

## 2026-07-26 — Implementação do template universal

- **Evento:** materialização da estrutura definida em `spec-project-bootstrap.md`.
- **Impacto:** o repositório passa a funcionar como fundação pronta para novos projetos Cursor via GitHub template.
- **Arquivos afetados:** raiz documental (`.gitignore`, `readme.md`, `spec-root.md`, `flow.md`, `rules.md`, `status.md`, `timeline.md`, `setup.md`, `tools-linux.md`, `tools-windows.md`, `.cursorrules`, `.prompt-status`, `spec-template.md`, `rules-scripts.md`), pastas operacionais e scripts de validação.
- **Observações:** placeholders `[NOME DO PROJETO]` devem ser preenchidos em cada projeto derivado.

## 2026-07-26 — Criação do documento de fundação

- **Evento:** criação de `spec-project-bootstrap.md`.
- **Impacto:** definição normativa da espinha dorsal de qualquer novo repositório no Cursor.
- **Arquivos afetados:** `spec-project-bootstrap.md`.
- **Observações:** commit inicial de especificação; implementação estrutural veio em seguida.
