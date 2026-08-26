# status.md — snapshot do estado atual

## Última atualização

2026-08-26

## Resumo do estado atual

Template universal implementado conforme `spec-project-bootstrap.md`. Estrutura raiz e de templates foi refinada com a adoção de novas regras para automação de scripts (modos interativo/silencioso) e layout de cabeçalhos CLI (Ciano/OEM437). O repositório está se transformando no projeto do comando de console Linux `cdd`.

## Tarefas concluídas

- [x] Criar arquivos de governança da raiz
- [x] Criar documentos operacionais e wireframes
- [x] Criar `spec-template.md` e `rules-scripts.md`
- [x] Criar estrutura de pastas (`docs/`, `ideas/`, `specs/`, `core/`, etc.)
- [x] Incluir scripts base de validação da estrutura
- [x] Padronizar nomes de arquivos com hífen (`-`) em vez de underscore (`_`)
- [x] Atualizar governança de scripts (automação SSH/Quiet/Force, UI de tabelas ASCII/OEM Ciano).

## Tarefas pendentes

- [ ] Definir o domínio do projeto e popular `/core` como `cdd` (comando Linux)
- [ ] Escrever as especificações iniciais para as features do `cdd` em `specs/to-do/`
- [ ] Atualizar arquivos genéricos (`readme.md`, `spec-root.md`) para o contexto `cdd`.

## Riscos

- Adequação das ferramentas e scripts de validação aos novos modos de automação recém-documentados (`--quiet`, `--ssh`).

## Próximos passos

1. Personalizar `spec-root.md` e `readme.md` para o domínio exato do `cdd`.
2. Modelar a arquitetura e fluxo de comandos do `cdd`.

## Mudanças recentes

- Atualização de `rules-scripts.md`, `rules.md` e `spec-project-bootstrap.md` incorporando padrões de terminal interativo (cabeçalhos tabelados e numerados) e modos de automação e acesso remoto (`--quiet`, `--ssh`, `--force`, `--log`).
