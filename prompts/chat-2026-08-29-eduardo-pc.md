# CCIA: Chat History (2026-08-29, eduardo-pc)

## Prompt #44
**Contexto/Pergunta:** O usuário informou que o comando `cdd` funciona bem, mas ainda não é reconhecido no CMD. Pediu para resolver, recompilar e reinstalar.

**Ação/Resposta:**
1. **Diagnóstico:** O instalador de desenvolvimento (`scripts/setup/install.ps1`) só injetava o wrapper PowerShell no `$PROFILE`. O CMD depende de `cdd.cmd` estar no `PATH` do usuário, o que não era feito nesse fluxo.
2. **Correção do instalador:** Atualizado `install.ps1` para, após `cargo build --release`, copiar `cdd-bin.exe` para `scripts/shell/`, adicionar essa pasta ao `PATH` do usuário (suporte CMD) e remover essa entrada no `--uninstall`.
3. **Reinstalação:** Executado `.\scripts\setup\install.ps1 -Quiet` com sucesso (compilação release, cópia do binário, PATH atualizado).
4. **Validação:** Testado `cdd --help` via `cmd.exe` com o PATH atualizado; saída de ajuda exibida corretamente.

## Prompt #45
**Contexto/Pergunta:** O usuário perguntou se há forma de acelerar a busca do `cdd`, sugerindo um índice persistente: buscar primeiro no arquivo indexador, ir ao disco só se não encontrar, e atualizar o índice ao descobrir pastas novas.

**Ação/Resposta:**
1. **Novo módulo `index.rs`:** Criado índice persistente em `~/.config/cdd/index.json`, organizado por raiz de busca (drive ou `/`).
2. **Fluxo de busca:** Com índice ativo (padrão), filtra entradas em memória; se não houver match, varre o disco, grava todas as pastas encontradas no índice e retorna o resultado.
3. **Manutenção:** Entradas removidas do disco são podadas automaticamente; deduplicação ao mesclar novos caminhos.
4. **Novas flags:** `-ix` (padrão, sticky `:on`/`:off`), `-in` (ignora índice na execução), `-ir` (reconstrói índice da raiz atual).
5. **Validação:** `cargo test` com 27 testes passando; reinstalação local via `install.ps1 -Quiet`.

## Prompt #46
**Contexto/Pergunta:** O usuário relatou que `cdd linux` funciona na raiz do drive (`G:\`), mas dentro de uma subpasta (ex.: após entrar em `...\scripts\linux`) o mesmo comando retorna "no directory found". Suspeita de que a busca não cobre o drive inteiro.

**Ação/Resposta:**
1. **Diagnóstico:** Bug clássico do Windows: `G:` significa "diretório atual no drive G", enquanto `G:\` é a raiz do drive. O código usava o prefixo `G:` retornado por `current_dir()`, então dentro de subpastas a varredura (e o índice) ficava limitada à árvore abaixo do cwd.
2. **Correção:** Criada `windows_drive_root_from()` que sempre normaliza para `X:\` com barra invertida.
3. **Teste:** Adicionado teste unitário Windows para garantir que caminhos profundos ainda resolvem para `G:\`.
4. **Reinstalação:** Recompilado e reinstalado via `install.ps1 -Quiet`.

## Prompt #47
**Contexto/Pergunta:** Commit + Push de todas as alterações pendentes.

**Ação/Resposta:**
1. Commit com índice persistente, correção `G:\` vs `G:`, suporte CMD no install dev e logs CCIA.
2. Excluído `scripts/shell/cdd-bin.exe` (artefato compilado local).
3. Push para `origin/main`.

## Prompt #48
**Contexto/Pergunta:** Nova release no GitHub.

**Ação/Resposta:**
1. Criada e enviada a tag `v0.1.2` para disparar o workflow `.github/workflows/release.yml`.
2. Release inclui: índice persistente, correção `G:\` vs `G:`, suporte CMD no install dev.
