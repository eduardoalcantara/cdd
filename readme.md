# [NOME DO PROJETO]

> Template universal de repositório para projetos implementados com Cursor IDE.

## Resumo

Este repositório é a fundação documental e operacional para novos projetos. Ele padroniza estrutura, governança, fluxo do agente, rastreamento de prompts e o núcleo específico do projeto em `/core`.

Substitua os placeholders `[NOME DO PROJETO]`, `[RESUMO]`, `[PÚBLICO]` e demais marcações ao iniciar um projeto real a partir deste template.

## Público e objetivo

- **Público:** pessoas e agentes que criam e mantêm projetos no Cursor.
- **Objetivo:** nascer com contexto suficiente para o Cursor trabalhar com consistência desde o primeiro prompt.

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

1. Crie um repositório novo a partir deste template no GitHub (**Use this template**).
2. Clone o repositório e abra no Cursor.
3. Leia, nesta ordem: `spec-root.md` → `rules.md` → `flow.md` → `setup.md`.
4. Preencha os placeholders do domínio do projeto.
5. Coloque o conteúdo específico em `/core`.
6. Atualize `status.md` e `timeline.md` a cada entrega relevante.

Detalhes de ambiente: ver `setup.md`, `tools-linux.md` e `tools-windows.md`.

## Documentação relacionada

- Fundação do template: `spec-project-bootstrap.md`
- Fluxo do agente: `flow.md`
- Regras permanentes: `rules.md`
- Regras de scripts: `rules-scripts.md`
- Molde de specs: `spec-template.md`

## Estado atual

Ver `status.md` para o snapshot vivo do projeto.
