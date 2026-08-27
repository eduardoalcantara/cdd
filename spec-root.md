# cdd cli command (change directory directly) - a Linux evolution of cd for faster users

## Propósito do projeto

O comando `cdd` (Change Directory Directly) existe para acelerar a navegação de diretórios via terminal, permitindo encontrar e mudar para pastas por correspondência parcial literal, curingas explícitos ou parâmetros sequenciais, sem exigir o path completo.

Este repositório nasce a partir do template universal definido em `spec-project-bootstrap.md`, com estrutura documental completa e orientação explícita para o Cursor AI.

## Visão geral

- O projeto é governado pelos documentos da raiz.
- O agente opera segundo `flow.md`, `rules.md` e `.cursorrules`.
- O conteúdo específico do domínio vive em `/core`.
- O progresso é rastreado em `status.md`, `timeline.md` e `.prompt-status`.

## Princípios fundacionais

1. **Repositório nasce documentado** — contexto mínimo antes de código solto.
2. **A raiz orienta** — cada arquivo da raiz tem função operacional.
3. **Contexto explícito para o agente** — regras, fluxo, estado e histórico visíveis.
4. **Formato universal, conteúdo adaptável** — espinha dorsal fixa; interior muda por domínio.
5. **`/core` concentra o específico** — salvo quando o padrão da tecnologia exigir outro local.
6. **`.prompt-status` é obrigatório** — todo prompt é rastreado no **início**; o fim (duração) é calculado na resposta como `now() - current_prompt_start_time` e **não** é gravado no arquivo.

## Escopo

- Implementar a CLI `cdd` cross-platform (binário compilado em Rust + Shell Wrappers em Bash/PowerShell).
- Suportar busca parcial case-insensitive, curingas nativos `*`/`?` por nome de diretório e menus interativos customizados.
- Não realizar fuzzy matching, autocorreção nem seleção por aproximação: uma query incorreta deve falhar sem sugerir ou escolher outro diretório.
- Oferecer ajuda integrada por `-h` e `--help`.
- Manter o comportamento das flags configuráveis (sticky settings com modo `:on` e `:off`).
- Manter a estrutura documental e operacional do template original para garantir que o Cursor consiga operar sem ambiguidade.

## Fora de escopo

- Implementação de domínio específico antes de specs em `specs/to-do/`.
- Ferramentas, serviços ou integrações não documentadas.
- Alterações que violem a hierarquia normativa sem atualização explícita de `spec-root.md` e `rules.md`.

## Arquitetura de alto nível

```text
raiz documental (governança)
├── fluxo e regras do agente
├── estado e histórico
├── specs / docs / prompts
├── scripts / reports / resources
└── core/  ← domínio do projeto
```

## Contratos centrais

| Contrato | Documento |
|---|---|
| O que o projeto é | `spec-root.md`, `readme.md` |
| Como o agente age | `flow.md`, `.cursorrules` |
| O que é permitido | `rules.md` |
| Como scripts funcionam | `rules-scripts.md` |
| Onde está o domínio | `/core` |
| Como se rastreia prompts | `.prompt-status` |

## Regras permanentes

- Não implementar fora do escopo confirmado.
- `.prompt-status` registra só o início do prompt atual; não gravar fim nem status final no arquivo.
- Não colocar conteúdo específico do projeto fora de `/core` sem justificativa normativa.
- Mudanças relevantes atualizam `status.md` e `timeline.md`.

## Critérios de sucesso

O repositório está pronto quando o Cursor responde sem ambiguidade:

- o que o projeto é;
- quais são as regras;
- como operar e validar;
- como documentar progresso;
- onde ficam referências e o núcleo em `/core`;
- como o agente se comporta;
- como rastrear o **início** de cada prompt em `.prompt-status` (a duração sai no rodapé: `now() - current_prompt_start_time`).

## Conclusão normativa

Este arquivo é a autoridade arquitetural máxima. Specs em `specs/` detalham entregas; se houver conflito, `spec-root.md` prevalece até ser atualizado formalmente.
