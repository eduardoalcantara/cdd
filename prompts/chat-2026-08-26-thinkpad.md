# CCIA: Chat History (2026-08-26, thinkpad)

## Prompt #19
**Contexto/Pergunta:** O usuário aprovou o plano “Completar funcionalidades do cdd” e pediu sua implementação integral, sem editar o arquivo do plano. O contrato confirmado remove fuzzy/autocorreção e mantém busca parcial literal com curingas explícitos `*` e `?`.

**Ação/Resposta:**
1. Implementados curingas por nome de diretório, modos sequencial/inverso/qualquer ordem e exclusão de `/proc`, `/sys`, `/dev` e `/run` na varredura Linux.
2. Adicionados `-h`/`--help`, validação estrita de `-2` até `-20`, erro para flags desconhecidas e suporte a `--`.
3. Configuração ganhou metadados sticky retrocompatíveis; filtros ativos são exibidos e list size explícito prevalece sobre lucky pick.
4. Wrappers foram robustecidos: Bash trata arquivo temporário/path; PowerShell normaliza path e preserva `$LASTEXITCODE`.
5. Adicionados 18 testes Rust. Validados `cargo fmt --check`, `cargo check`, `cargo test`, clippy estrito, build release, estrutura do repositório e smoke tests Bash (help, erro e persistência sticky).
6. PowerShell não foi executado porque `pwsh` não está disponível neste host.
7. Documentação e governança alinhadas; fuzzy foi removido de `spec-root.md`; a spec foi concluída em `specs/done/SPEC-cdd-core.md`; criado `docs/usage.md`.
8. `.prompt-status` foi atualizado somente no início deste prompt, conforme a regra vigente.

## Prompt #20
**Contexto/Pergunta:** O usuário perguntou se o `cdd` já foi instalado neste PC e se o limite padrão solicitado de até 10 linhas (metade do máximo de 20) foi respeitado.

**Ação/Resposta:**
1. Verificado que o `cdd` não está instalado de forma persistente: não há marcador nos profiles nem diretório em `~/.local/share/cdd`; a função disponível na sessão atual veio do smoke test com `source`.
2. Confirmado no código que `Config::default().list_size` é `10` e a TUI trunca os resultados para esse valor antes de configurar `page_size`.
3. Nenhuma instalação foi executada sem autorização explícita.

## Prompt #21
**Contexto/Pergunta:** O usuário autorizou a instalação do `cdd` neste PC para teste em um console externo.

**Ação/Resposta:**
1. Executado `scripts/setup/install.sh --quiet`.
2. Binário release compilado e wrapper adicionado ao `/home/edu/.bashrc`.
3. Validado em uma nova instância interativa do Bash: `cdd` foi carregado como função e `cdd --help` retornou com sucesso.
4. Instalação reversível por `bash scripts/setup/install.sh --uninstall`.

## Prompt #22
**Contexto/Pergunta:** Após teste externo com `cdd linux`, o usuário apontou dois desvios: o item focado mudava apenas a cor do texto, em vez de inverter fundo/texto; e todos os descendentes de um diretório correspondente apareciam como resultados.

**Ação/Resposta:**
1. Confirmada a causa da busca: qualquer componente do caminho podia satisfazer a query, fazendo todos os descendentes herdarem o match.
2. Corrigido o contrato: o termo final efetivo deve corresponder ao próprio diretório de destino. Em `-qa`, todos os termos devem existir e pelo menos um deve corresponder ao destino.
3. Configurado o `inquire::RenderConfig` para item focado com foreground preto e background cinza.
4. Adicionados testes contra descendentes indevidos nos modos sequencial e `-qa`; total passou para 20 testes.
5. Documentação, spec, status e timeline atualizados.
6. Validado `cdd linux -l` em novo Bash: destino `/home/edu/projects/os/linux_dev_workstation`, sem cair em `docs` ou outro descendente.
7. Release recompilada; instalação existente já usa o binário corrigido.

## Prompt #23
**Contexto/Pergunta:** O usuário elogiou o filtro textual da lista e perguntou se ele pesquisa nos 20 resultados máximos ou somente nas 10 linhas padrão.

**Ação/Resposta:**
1. Confirmado no código que a lista é truncada antes de ser entregue ao `inquire`.
2. Com o default `-10`, o filtro atua somente sobre os primeiros 10 candidatos; com `-20`, sobre os primeiros 20.
3. Esclarecido que `list_size` hoje acumula dois papéis (quantidade visível e tamanho do conjunto filtrável), e que seria melhor separar page size do conjunto completo de candidatos se o usuário quiser filtragem global.

## Prompt #24
**Contexto/Pergunta:** O usuário aprovou separar o limite visual da lista do conjunto usado pelo filtro textual.

**Ação/Resposta:**
1. Removido o truncamento dos candidatos antes de criar o `inquire::Select`.
2. `list_size` permanece como `page_size`: 10 linhas visíveis por padrão, configurável entre 2 e 20.
3. O filtro textual agora opera sobre todos os diretórios encontrados.
4. Ajuda, guia, spec, status e timeline atualizados.
5. Validados 20 testes, `cargo check`, clippy estrito e build release; o binário usado pela instalação local foi recompilado.

## Prompt #25
**Contexto/Pergunta:** O usuário relatou que um filtro impossível não mostrava mensagem e que `docss` ainda retornava vários caminhos sem essa substring.

**Ação/Resposta:**
1. Identificada a causa: o `Select` do `inquire` usa fuzzy matching por padrão, contrariando o contrato literal do `cdd`.
2. Adicionado scorer customizado case-insensitive por substring, preservando a ordem original e eliminando resultados aproximados.
3. Adicionada opção de estado vazio, exibida somente quando nenhuma substring corresponde: `Nenhum caminho encontrado com o filtro.`.
4. Adicionados dois testes do scorer e estado vazio; total passou para 22 testes.
5. Documentação, spec, status e timeline atualizados.
6. Validados `cargo fmt --check`, `cargo check`, 22 testes, clippy estrito e build release. A instalação local já aponta para o release corrigido.

## Prompt #26
**Contexto/Pergunta:** O usuário solicitou três documentos em `/docs` (`HOW_TO_USE.md`, `HOW_TO_INSTALL.md`, `HOW_IT_WORKS.md`), atualização do `readme.md`, regeneração dos pacotes ZIP/TAR-GZ e commit/push.

**Ação/Resposta:**
1. Criados os três guias; `HOW_IT_WORKS.md` documenta wrapper, parser, configuração, varredura, patterns, query modes, TUI, scorer literal, comunicação e invariantes.
2. Atualizados `readme.md`, `docs/readme.md`, `setup.md`, `status.md` e `timeline.md`.
3. Scripts `build-dist.sh`/`.ps1` passaram a incluir README/guias e gerar checksum SHA-256; Linux usa target musl static-pie.
4. Gerado e validado `cdd-linux-x86_64.tar.gz`.
5. Instalado target Rust Windows e usado toolchain MinGW portátil oficial, com pacotes verificados por assinatura, para gerar um `cdd.exe` PE32+ válido; ZIP e checksum validados.
6. Criado relatório `reports/2026-08-26-documentacao-e-pacotes.md`. Os artefatos em `dist/` permanecem ignorados pelo Git.
7. Execução funcional do pacote Windows permanece não validada neste host Linux.
