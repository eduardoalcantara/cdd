# cdd (Change Directory Directly)

> Evolução do comando `cd` do Linux projetada para usuários velozes de linha de comando.

## Resumo

O `cdd` é um utilitário CLI que revoluciona a forma de navegar em diretórios no terminal. Diferente do comando `cd` padrão que demanda o caminho exato (`cd /var/www/applications/app1`), o `cdd` aceita parâmetros de pesquisa nebulosos (fuzzy/parciais). 

Basta digitar `cdd www app1` e, se esse for o único resultado plausível no sistema, ele te leva para lá instantaneamente. Se houver múltiplos diretórios correspondentes, uma lista interativa minimalista será exibida para você navegar (usando setas) e selecionar com ENTER o destino.

## Público e objetivo

- **Público:** Desenvolvedores, sysadmins e power users que trabalham extensivamente em terminais no Linux (Bash/Zsh) ou Windows (PowerShell).
- **Objetivo:** Fornecer navegação super rápida através de buscas indexadas de glob patterns, suportando flags flexíveis que podem ser persistidas no perfil do usuário.

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
3. O código fonte está contido em `/core`. Para desenvolver as implementações da TUI e do sistema de busca, navegue até esta pasta.
4. Para as documentações e diretrizes de contribuição, leia nesta ordem: `spec-root.md` → `rules.md` → `flow.md`.
5. Detalhes de arquitetura de software estão em `specs/`.

Detalhes de ambiente: ver `setup.md`, `tools-linux.md` e `tools-windows.md`.

## Documentação relacionada

- Fundação do template: `spec-project-bootstrap.md`
- Fluxo do agente: `flow.md`
- Regras permanentes: `rules.md`
- Regras de scripts: `rules-scripts.md`
- Molde de specs: `spec-template.md`

## Estado atual

Ver `status.md` para o snapshot vivo do projeto.
