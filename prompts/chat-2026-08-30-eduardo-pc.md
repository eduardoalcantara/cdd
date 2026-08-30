# CCIA: Chat History (2026-08-30, eduardo-pc)

## Prompt #49
**Contexto/Pergunta:** Mesmo com índice, a busca ainda parece lenta. Usuário sugeriu animação estilo Pac-Man com frutinhas CP437 (chr 248/208), caroço (chr 250) e boca `<` / ò (chr 242) na linha abaixo do comando, enquanto aguarda.

**Ação/Resposta:**
1. **Opinião:** Boa ideia para percepção de espera; não acelera I/O, mas melhora UX em varreduras longas.
2. **Implementação:** Novo módulo `wait_animation.rs` — linha cheia de °/Ð, Pac-Man `<`/`ò` comendo e deixando ·, loop até fim da busca.
3. **Integração:** Animação só quando o índice não resolve e o disco é varrido (sem flicker em cache hit); stderr + `\r`, só em TTY.
4. **Validação:** 30 testes passando; reinstalação via `install.ps1 -Quiet`.

## Prompt #50
**Contexto/Pergunta:** Usuário quer fruta `ð` (não `Ð`); index ainda lento como disco; animação deve aparecer em busca no index e no disco.

**Ação/Resposta:**
1. **Bug do índice:** `prune_missing` e `search` chamavam `is_dir()` em *cada* entrada do index — equivalente a varrer o disco. Removido do hot path; `is_dir()` só nos matches finais.
2. **Fruta:** `FRUIT_B` alterado para `ð` (U+00F0).
3. **Animação:** Pac-Man inicia antes do index e continua no disco se necessário.
4. **Prune:** Somente em `-ir` após rebuild.

## Prompt #51
**Contexto/Pergunta:** Ajustar boca do Pac-Man: aberta `<`, fechada `c`.

**Ação/Resposta:** Alterado `MOUTH_CLOSED` de `ò` para `c` em `wait_animation.rs`; reinstalado.

## Prompt #52
**Contexto/Pergunta:** Animação rápida demais; padrão único `c · · · · ð ...`; ciclo de comer c → C+ð → @ → c; mais lento.

**Ação/Resposta:** Reescrito `wait_animation.rs` com spacing 5, FRUIT só ð, 220ms/frame, máquina de estados Approach/Open/Bite/Swallow.

## Prompt #53
**Contexto/Pergunta:** `-ix:on` falha no PowerShell com `unknown option: -ix:`; dúvida se index está desabilitado por padrão.

**Ação/Resposta:**
1. Esclarecido: index **habilitado** por padrão (`use_index: true`).
2. **Causa:** PowerShell interpreta `:on` antes do binário, enviando `-ix:` sem o `on`.
3. **Fix:** Novos aliases `-ixon` / `-ixoff`; mensagem de erro orienta aspas ou aliases; help atualizado.

## Prompt #54
**Contexto/Pergunta:** `cdd linux -ix:on` falha; `cdd '-ix:on'` sem query retorna usage error após salvar config.

**Ação/Resposta:**
1. **`cdd.ps1`:** `Normalize-CddArg` converte `-ix:` → `-ixon` e `-ix:off` → `-ixoff` antes de chamar o binário.
2. **`main.rs`:** invocação só com flags sticky (`cdd -ixon`, `cdd '-ix:on'`) sai 0 após salvar config, sem exigir query.
3. **Help:** documentado `Sticky-only update (no search): cdd -ixon`.
4. **Teste:** `sticky_only_invocation_has_no_queries`; binário `-ixon` → exit 0.

**Formas válidas no PowerShell:**
- `cdd -ixon` ou `cdd -ixoff` (recomendado)
- `cdd '-ix:on'` ou `cdd linux '-ix:on'`
- `cdd linux -ix:on` (corrigido pelo wrapper após `. $PROFILE`)

## Prompt #55
**Contexto/Pergunta:** Como instalar novamente no CMD?

**Ação/Resposta:** Instruções para `scripts\setup\install.cmd` (dev) ou `install-user.cmd` (pacote); PATH em `scripts\shell`; abrir novo CMD após instalar.

## Prompt #56
**Contexto/Pergunta:** Erro ao rodar `install-user.cmd` no repo: `cdd-bin.exe` não encontrado em `scripts\setup\`.

**Ação/Resposta:** `install-user` é só para o ZIP de release; no repo usar `install.cmd`. Guard clause adicionada em `install-user.ps1` com mensagem orientativa.
