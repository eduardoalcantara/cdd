# setup.md — preparação do ambiente

## Pré-requisitos

- Conta GitHub (para criar o repo a partir do template)
- Git
- Cursor IDE
- Ferramentas de domínio conforme `tools-linux.md` ou `tools-windows.md`

## Instalação

1. No GitHub, use **Use this template** (ou selecione este template em **Start with a template**).
2. Clone o repositório gerado:

```bash
git clone <URL_DO_NOVO_REPO>
cd <NOME_DO_REPO>
```

3. Abra a pasta no Cursor.

## Bootstrap

1. Leia `readme.md`, `spec-root.md`, `rules.md` e `flow.md`.
2. Substitua placeholders (`[NOME DO PROJETO]`, `[PROPÓSITO]`, etc.).
3. Confirme que `.prompt-status` existe na raiz.
4. Valide a estrutura:

```bash
# Linux / macOS
./scripts/validation/validate-structure.sh

# Windows (PowerShell)
.\scripts\validation\validate-structure.ps1
```

5. Coloque o conteúdo específico do projeto em `/core`.

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
3. Ao final, confira atualizações em `.prompt-status`, `status.md` e `timeline.md`.

## Solução de problemas

| Problema | Ação |
|---|---|
| Falta pasta ou arquivo da raiz | Recriar a partir deste template ou restaurar do commit de fundação |
| Script de validação falha | Conferir caminhos e permissões de execução |
| Agente ignora regras | Reforçar leitura de `.cursorrules`, `rules.md` e `flow.md` |
| Segredos commitados por engano | Rotacionar credenciais; remover do histórico se necessário |
