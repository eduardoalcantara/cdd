# Relatório — implementação do template universal

**Data:** 2026-07-26  
**Base normativa:** `spec-project-bootstrap.md`  
**Status:** concluído

## Objetivo

Materializar a estrutura raiz definida pela spec de bootstrap para que este repositório funcione como template eficaz de novos projetos Cursor.

## O que foi entregue

- Arquivos de governança da raiz (`.gitignore`, `.cursorrules`, `.prompt-status`, documentos operacionais).
- Wireframes preenchíveis com placeholders para personalização.
- Pastas operacionais com `readme.md` de propósito.
- Scripts de validação estrutural (Linux e Windows).
- Prompt reutilizável de personalização em `prompts/`.

## Validação

- Executar `scripts/validation/validate-structure.ps1` (Windows) ou `validate-structure.sh` (Linux).
- Conferir presença de todos os itens listados na seção 2 de `spec-project-bootstrap.md`.

## Pendências

- Marcar o repositório como Template no GitHub Settings.
- Personalizar placeholders ao gerar o primeiro projeto derivado.

## Próximo passo

Ativar **Template repository** nas Settings do GitHub e gerar um repo de teste.
