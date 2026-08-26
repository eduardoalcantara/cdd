# tools-windows.md — ferramentas e comandos (Windows)

## Ferramentas necessárias

| Ferramenta | Uso |
|---|---|
| Git for Windows | Versionamento |
| Cursor IDE | Implementação assistida por agente |
| PowerShell 7+ (recomendado) | Scripts em `scripts/` |
| Ferramentas de domínio | Conforme o projeto em `/core` |

## Comandos úteis

```powershell
# Clonar e entrar no projeto
git clone <URL>
cd <repo>

# Validar estrutura do template
.\scripts\validation\validate-structure.ps1

# Ver status git
git status

# Política de execução (se necessário, escopo do usuário)
Set-ExecutionPolicy -Scope CurrentUser RemoteSigned
```

## Dependências do sistema

- Instalar Git e Cursor.
- Usar `winget`, Chocolatey ou instaladores oficiais conforme política local.
- Dependências de domínio (Node, Python, Docker Desktop, etc.) devem ser documentadas em `docs/` ou em `core/`.

## Uso recomendado

- Preferir scripts PowerShell espelhando o comportamento dos scripts Linux.
- Seguir `rules-scripts.md` para input numerado, cabeçalho e `--uninstall`.
- Evitar misturar caminhos Unix e Windows no mesmo script sem normalização.

## Observações específicas de Windows

- Caminhos usam `\` (ou `/` no PowerShell moderno).
- Preferir PowerShell para automação neste template.
- Atenção a políticas de execução e antivírus ao rodar scripts locais.
- Google Drive / OneDrive podem atrasar I/O; evite builds pesados em pastas sincronizadas se houver problemas.
