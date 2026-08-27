# status.md — snapshot do estado atual

## Última atualização

2026-08-26

## Resumo do estado atual

O core funcional do `cdd` está implementado: busca parcial literal, curingas `*`/`?`, modos de query, TUI, configuração sticky visível, ajuda, wrappers e distribuição. Fuzzy/autocorreção foi removido formalmente do escopo por risco de selecionar diretórios incorretos.

## Tarefas concluídas

- [x] Criar arquivos de governança da raiz
- [x] Criar documentos operacionais e wireframes
- [x] Atualizar governança de scripts e regras de automação
- [x] Converter o repositório-base em domínio `cdd` (readme, spec-root)
- [x] Documentar especificação técnica da core feature do `cdd`
- [x] MVP: parser, config JSON, busca substring com `jwalk`, TUI, wrappers, install, build-dist
- [x] `.prompt-status` passa a registrar só o início do prompt (duração = `now() - start` no rodapé)
- [x] Curingas `*` e `?` por componente de diretório, sem fuzzy/autocorreção
- [x] Ajuda integrada `-h` / `--help`
- [x] Header de filtros sticky ativos e migração de configurações antigas
- [x] Validação da faixa `-2` até `-20` e rejeição de flags desconhecidas
- [x] Exclusão de `/proc`, `/sys`, `/dev` e `/run` na varredura Linux
- [x] 22 testes Rust, clippy estrito, build release e smoke test Bash
- [x] Resultados limitados ao diretório final correspondente, sem listar descendentes por match no ancestral
- [x] Item focado da TUI com texto preto sobre fundo cinza
- [x] Filtro textual da TUI aplicado a todos os resultados, mantendo 10 linhas visíveis por padrão
- [x] Filtro da TUI literal/case-insensitive, sem fuzzy, com mensagem explícita quando vazio
- [x] Guias `HOW_TO_USE`, `HOW_TO_INSTALL` e `HOW_IT_WORKS`
- [x] Pacote Linux estático com documentação e checksum SHA-256
- [x] Pacote Windows x86_64 cross-compilado, validado como PE32+ e acompanhado de checksum SHA-256
- [x] Guia end-user em `docs/usage.md`
- [x] Spec do core concluída em `specs/done/SPEC-cdd-core.md`

## Tarefas pendentes

- [ ] Executar a matriz de smoke tests do wrapper no Windows/PowerShell.
- [ ] Publicar os pacotes de release.

## Próximos passos

1. Validar o pacote Windows em uma máquina com PowerShell.
2. Gerar e publicar a primeira release com os scripts `build-dist`.

## Mudanças recentes

- Regras de `.prompt-status`: gravar só o início; Tempo do rodapé calculado com `now()`.
- Escopo alinhado: correspondência parcial + curingas explícitos; fuzzy/autocorreção removido.
- Core, ajuda, sticky metadata, validações, testes e documentação concluídos.
- Busca terminal e foco visual corrigidos após teste em console externo.
- Limite visual da TUI separado do conjunto completo usado pelo filtro textual.
- Fuzzy interno do `inquire` substituído por scorer literal para impedir falsos positivos.
- Documentação de usuário, instalação e arquitetura consolidada em `/docs`.
