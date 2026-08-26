# spec-root.md — autoridade arquitetural

## Propósito do projeto

[NOME DO PROJETO] existe para [PROPÓSITO EM UMA FRASE].

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
6. **`.prompt-status` é obrigatório** — todo prompt é rastreado do início ao fim.

## Escopo

- Manter a estrutura documental e operacional do template.
- Permitir adaptação por domínio (software, hardware, SO, impressão 3D, híbridos).
- Garantir que o Cursor consiga operar sem ambiguidade sobre regras, validação e documentação.

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
- Não tratar prompt como concluído sem atualizar `.prompt-status`.
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
- como rastrear cada prompt em `.prompt-status`.

## Conclusão normativa

Este arquivo é a autoridade arquitetural máxima. Specs em `specs/` detalham entregas; se houver conflito, `spec-root.md` prevalece até ser atualizado formalmente.
