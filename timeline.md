# timeline.md — histórico evolutivo (cronológico reverso)

## 2026-08-26 — Documentação consolidada e pacotes atualizados

- **Evento:** Criação dos guias de uso, instalação e arquitetura/algoritmo.
- **Impacto:** Usuários, desenvolvedores e agentes de IA passam a ter contratos operacionais separados. Pacotes Linux e Windows incluem os guias e checksums SHA-256; Linux usa binário musl static-pie e Windows contém executável PE32+ x86_64 cross-compilado.
- **Arquivos afetados:** `docs/HOW_TO_USE.md`, `docs/HOW_TO_INSTALL.md`, `docs/HOW_IT_WORKS.md`, `readme.md`, `scripts/setup/build-dist.*`, `status.md`.

## 2026-08-26 — Filtro literal e estado vazio na TUI

- **Evento:** Correção do filtro interno do `inquire`, que ainda aplicava fuzzy matching.
- **Impacto:** O texto digitado agora exige substring literal case-insensitive. Um filtro impossível mostra `Nenhum caminho encontrado com o filtro.` em vez de caminhos aproximados.
- **Validação:** 22 testes Rust e clippy estrito, incluindo scorer literal e estado vazio.
- **Arquivos afetados:** `core/src/tui.rs`, `docs/usage.md`, `specs/done/SPEC-cdd-core.md`, `status.md`.

## 2026-08-26 — Filtro textual sobre o conjunto completo

- **Evento:** Separação entre limite visual e conjunto filtrável da TUI.
- **Impacto:** A lista continua mostrando 10 linhas por padrão (configurável entre 2 e 20), mas o filtro digitado pelo usuário pesquisa em todos os diretórios encontrados.
- **Arquivos afetados:** `core/src/tui.rs`, `core/src/help.rs`, `docs/usage.md`, `specs/done/SPEC-cdd-core.md`, `status.md`.

## 2026-08-26 — Correção dos resultados terminais e foco da TUI

- **Evento:** Ajuste após teste real em console externo.
- **Impacto:** Uma query só retorna diretórios cujo próprio nome corresponde ao termo final efetivo; descendentes deixaram de aparecer apenas porque um ancestral corresponde. O item focado passou a usar texto preto sobre fundo cinza.
- **Validação:** 20 testes Rust e clippy estrito; incluídos testes específicos contra vazamento de descendentes nos modos sequencial e `-qa`.
- **Arquivos afetados:** `core/src/search.rs`, `core/src/tui.rs`, `docs/usage.md`, `specs/done/SPEC-cdd-core.md`, `status.md`.

## 2026-08-26 — Conclusão funcional do core

- **Evento:** Implementação de curingas `*`/`?`, ajuda `-h`/`--help`, validação de flags, filtros sticky visíveis e exclusões seguras na varredura Linux.
- **Impacto:** O contrato foi corrigido para busca parcial literal e glob explícito, sem fuzzy/autocorreção. Configurações antigas são migradas, flags inválidas falham com código `2` e o wrapper PowerShell preserva o exit code.
- **Validação:** 20 testes Rust, `cargo fmt --check`, `cargo check`, clippy estrito, build release e smoke test Bash. PowerShell ficou pendente porque `pwsh` não está disponível neste host.
- **Arquivos afetados:** `core/src/`, `core/Cargo.toml`, `core/Cargo.lock`, `scripts/shell/cdd.*`, `spec-root.md`, `readme.md`, `docs/`, `status.md`, `specs/done/SPEC-cdd-core.md`.

## 2026-08-26 — `.prompt-status` registra só o início

- **Evento:** Correção da governança de `.prompt-status`. O arquivo deixou de receber escrita de fim (`end_time`, duração do prompt atual, `success`/`blocked`/`failed`).
- **Impacto:** Commit/push no meio ou no fim do prompt não deixa o rastreio "aberto". O Tempo do rodapé da resposta é `now() - current_prompt_start_time`. O prompt anterior só é arquivado em `[last]` no **início** do prompt seguinte.
- **Arquivos afetados:** `.cursorrules`, `rules.md`, `flow.md`, `spec-root.md`, `spec-project-bootstrap.md`, `spec-template.md`, `setup.md`, `.prompt-status`, `status.md`.

## 2026-08-26 — Orquestração de Distribuição End-User

- **Evento:** Desenvolvimento do fluxo de empacotamento para distribuição autônoma (zero dependências) do comando cdd.
- **Impacto:** Geração das rotinas `build-dist.*` que empacotam o binário com o script `install-user.*`. Isso dispensa o usuário final de instalar a suíte Rust, democratizando a adoção do `cdd` via `.zip` ou `.tar.gz`.
- **Arquivos afetados:** `scripts/setup/build-dist.*`, `scripts/setup/install-user.*`.

## 2026-08-26 — Implementação do Core e Shell Wrappers

- **Evento:** Desenvolvimento do motor Rust em `core/` e wrappers de shell (`cdd.sh` / `cdd.ps1`).
- **Impacto:** O comando base `cdd` foi testado e compilado com busca parcial, TUI e mudança efetiva do diretório pai. Esta etapa criou o MVP; curingas, help, validações completas e testes automatizados foram concluídos posteriormente. A afirmação original de validação cross-platform completa era excessiva.
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
