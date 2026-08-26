# rules.md — regras permanentes de governança

## Hierarquia normativa

1. `spec-root.md` — autoridade arquitetural máxima
2. `rules.md` — este arquivo (governança permanente)
3. `.cursorrules` — comportamento do agente
4. `flow.md` — sequência operacional
5. `rules-scripts.md` — governança de scripts
6. Specs em `specs/` — entregas formais
7. Docs, ideas e references — apoio (não normativo por padrão)

Em conflito: nível superior prevalece. Se irresolvido → pause e peça esclarecimento.

## Regras gerais permanentes

- Não inventar requisitos.
- Não extrapolar escopo.
- Ser honesto sobre limites, dúvidas e validações.
- Manter a raiz documental viva e coerente.
- Conteúdo específico do projeto vive em `/core` sempre que aplicável.

## Regras de escopo

- Implementar somente o que estiver confirmado em `spec-root.md` ou em specs ativas.
- Não misturar grupos/temas sem autorização.
- Adaptações por domínio (software, hardware, SO, 3D, híbridos) devem preservar a raiz documental.

## Regras de qualidade

- Preferir mudanças pequenas, claras e justificadas.
- Não deixar código/documentação órfãos sem referência.
- Nomear arquivos e pastas de forma previsível.
- Evitar duplicar normas: referenciar o documento canônico.

## Regras de documentação

- Mudanças relevantes atualizam `status.md` e `timeline.md`.
- Entregas materiais geram relatório em `reports/`.
- Specs novas usam `spec-template.md`.
- Specs concluídas migram de `specs/to-do/` para `specs/done/`.

## Regras de validação

- Validar o que for possível no ambiente.
- Não afirmar sucesso sem evidência.
- Registrar o que ficou sem validação.

## Regras de bloqueio

Pause e peça esclarecimento quando:

- faltar contexto essencial;
- houver contradição documental;
- o escopo estiver indefinido;
- a ação for destrutiva ou irreversível sem confirmação.

## Regras de atualização

- Alterar `spec-root.md` somente com intenção explícita de mudar a arquitetura.
- Ao mudar regras permanentes, atualizar este arquivo e refletir em `.cursorrules` / `flow.md` se necessário.
- Ao concluir prompts, atualizar `.prompt-status` sem perder o histórico recente.

## Regras de scripts (resumo)

Detalhes em `rules-scripts.md`:

- instalação com `--uninstall` reversível;
- input numerado;
- `0` = não, `1` = sim;
- Enter = default documentado.
