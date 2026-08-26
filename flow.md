# flow.md — fluxo operacional do agente

## Ordem de leitura obrigatória

Antes de implementar ou alterar o repositório:

1. `spec-root.md`
2. `rules.md`
3. `.cursorrules`
4. `.prompt-status`
5. `rules-scripts.md` — se a tarefa envolver scripts
6. `status.md` e `timeline.md`
7. Specs e docs relevantes em `specs/` e `docs/`

Não pular a leitura obrigatória. Não começar implementação sem contexto.

## Fluxo de execução do agente

1. Ler os documentos-raiz na ordem acima.
2. Identificar o escopo confirmado da tarefa.
3. Planejar a entrega (arquivos, validações, impactos).
4. Atualizar `.prompt-status` no início (`current_prompt_status = running`).
5. Implementar ou documentar somente o escopo confirmado.
6. Validar o que foi feito.
7. Atualizar `status.md`.
8. Atualizar `timeline.md`.
9. Produzir relatório em `reports/` quando a entrega for material.
10. Atualizar `.prompt-status` ao concluir (`success`, `blocked` ou `failed`).
11. Registrar próximos passos na resposta e em `status.md`.

## Ordem de decisões

1. Há contexto suficiente? Se não → pause e peça esclarecimento.
2. Há contradição entre documentos? Se sim → pause; não invente resolução.
3. A tarefa cabe no escopo de `spec-root.md` / specs ativas? Se não → não implemente.
4. A tarefa envolve scripts? Se sim → aplicar `rules-scripts.md`.
5. O conteúdo é específico do projeto? Se sim → preferir `/core`.

## Gate de confirmação

Antes de mudanças amplas (refactors, remoções, mudança de arquitetura):

- Confirmar escopo com o usuário quando houver risco ou ambiguidade.
- Não misturar grupos ou temas sem autorização.

## Checklist de execução

- [ ] Documentos-raiz lidos
- [ ] `.prompt-status` atualizado no início
- [ ] Escopo confirmado
- [ ] Implementação limitada ao escopo
- [ ] Validações executadas ou declaradas como pendentes
- [ ] `status.md` atualizado
- [ ] `timeline.md` atualizado
- [ ] Relatório criado (se aplicável)
- [ ] `.prompt-status` atualizado no fim

## Passos de validação

- Conferir se arquivos exigidos pela entrega existem e estão coerentes.
- Executar scripts/testes aplicáveis ao domínio.
- Declarar explicitamente o que não foi possível validar.

## Passos de encerramento

1. Atualizar `.prompt-status` com fim, duração, LLM, status e resumo.
2. Responder com: alterações, validações, pendências, arquivos impactados, próximo passo, documentos que justificam a ação.
3. Nenhum prompt deve ser tratado como concluído sem atualizar `.prompt-status`.

## Leitura e atualização de `.prompt-status`

| Momento | Ação |
|---|---|
| Antes de iniciar | Ler o arquivo |
| No início | Preencher seção `[current]` com status `running` |
| No fim | Mover/atualizar `[last]`, limpar ou fechar `[current]`, atualizar `[totals]` |
