# status.md — snapshot do estado atual

## Última atualização

2026-08-26

## Resumo do estado atual

O repositório transitou de um template de inicialização para o projeto oficial do comando CLI `cdd` (Change Directory Directly). As especificações fundamentais de arquitetura (Rust Binary Core + Shell Wrappers), configuração de usuário persistente e UI foram documentadas.

## Tarefas concluídas

- [x] Criar arquivos de governança da raiz
- [x] Criar documentos operacionais e wireframes
- [x] Atualizar governança de scripts e regras de automação
- [x] Converter o repositório-base em domínio `cdd` (readme, spec-root)
- [x] Documentar especificação técnica da core feature do `cdd`

## Tarefas pendentes

- [ ] Implementar a estrutura avançada de Match Ranking da busca baseada em algoritmos fuzzy.

## Próximos passos

1. Preparar documentação do end-user em `/docs`.
2. Publicar a primeira Release (via GitHub Actions ou local com os scripts `build-dist`).

## Mudanças recentes

- Atualização integral do escopo do repositório de "Template" para "Projeto cdd".
- Especificação técnica inicial incluída em `specs/to-do/SPEC-cdd-core.md` resolvendo dúvidas iniciais sobre wildcard, Rust TUI e cross-platform wrappers.
