# setup.md — preparação do ambiente

## Pré-requisitos

- Git
- Cursor IDE
- Rust / Cargo para desenvolvimento e geração de pacotes
- Ferramentas de domínio conforme `tools-linux.md` ou `tools-windows.md`

## Instalação

Usuário final: siga [`docs/HOW_TO_INSTALL.md`](docs/HOW_TO_INSTALL.md) e prefira o pacote pré-compilado da plataforma.

Desenvolvimento Linux:

```bash
bash scripts/setup/install.sh --quiet
```

Desenvolvimento Windows:

```powershell
.\scripts\setup\install.ps1 -Quiet
```

## Desenvolvimento

1. Leia `readme.md`, `spec-root.md`, `rules.md`, `flow.md` e `docs/HOW_IT_WORKS.md`.
2. Valide a estrutura:

```bash
# Linux / macOS
./scripts/validation/validate-structure.sh

# Windows (PowerShell)
.\scripts\validation\validate-structure.ps1
```

3. Valide o core:

```bash
cd core
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets -- -D warnings
```

4. Gere pacotes com `scripts/setup/build-dist.sh` (Linux) ou `build-dist.ps1` (Windows).

## Variáveis de ambiente

- Use `.env.example` (quando existir) como modelo.
- Nunca versionar `.env` nem credenciais.
- Documente variáveis novas em `docs/` ou neste arquivo.

## Verificações

- [ ] Estrutura raiz presente
- [ ] `.prompt-status` inicializado
- [ ] `status.md` e `timeline.md` atualizáveis
- [ ] Script de validação executa com sucesso
- [ ] Placeholders principais revisados

## Execução inicial

1. Crie a primeira spec em `specs/to-do/` usando `spec-template.md`.
2. Peça ao Cursor para seguir `flow.md` e implementar somente o escopo da spec.
3. O Tempo de cada resposta do agente aparece no rodapé do chat (`now() - current_prompt_start_time`); `.prompt-status` guarda só o início. Confira também `status.md` e `timeline.md`.

## Solução de problemas

| Problema | Ação |
|---|---|
| Falta pasta ou arquivo da raiz | Recriar a partir deste template ou restaurar do commit de fundação |
| Script de validação falha | Conferir caminhos e permissões de execução |
| Agente ignora regras | Reforçar leitura de `.cursorrules`, `rules.md` e `flow.md` |
| Segredos commitados por engano | Rotacionar credenciais; remover do histórico se necessário |
