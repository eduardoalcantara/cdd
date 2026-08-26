# status.md — snapshot do estado atual

## Última atualização

2026-07-26

## Resumo do estado atual

Template universal implementado conforme `spec-project-bootstrap.md`. Estrutura raiz, pastas operacionais, governança do agente e rastreamento de prompts estão prontos para uso via GitHub Template Repository.

## Tarefas concluídas

- [x] Criar arquivos de governança da raiz
- [x] Criar documentos operacionais e wireframes
- [x] Criar `spec-template.md` e `rules-scripts.md`
- [x] Criar estrutura de pastas (`docs/`, `ideas/`, `specs/`, `core/`, etc.)
- [x] Incluir scripts base de validação da estrutura
- [x] Padronizar nomes de arquivos com hífen (`-`) em vez de underscore (`_`)

## Tarefas pendentes

- [ ] Marcar o repositório como **Template repository** nas Settings do GitHub
- [ ] Ao criar um projeto real: substituir placeholders `[NOME DO PROJETO]` e afins
- [ ] Definir o domínio do projeto e popular `/core`
- [ ] Escrever a primeira spec formal em `specs/to-do/`

## Riscos

- Placeholders não preenchidos podem gerar ambiguidade se o template for usado sem personalização.
- Scripts de domínio ainda não existem; apenas validação estrutural está disponível.

## Próximos passos

1. Ativar o repo como template no GitHub.
2. Gerar um repositório de teste a partir do template.
3. Personalizar `spec-root.md` e `readme.md` para o domínio do primeiro projeto.

## Mudanças recentes

- Fundação documental e operacional criada a partir de `spec-project-bootstrap.md`.
