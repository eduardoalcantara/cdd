# tools-linux.md — ferramentas e comandos (Linux)

## Ferramentas necessárias

| Ferramenta | Uso |
|---|---|
| `git` | Versionamento |
| Cursor IDE | Implementação assistida por agente |
| `bash` | Scripts em `scripts/` |
| Ferramentas de domínio | Conforme o projeto em `/core` |

## Comandos úteis

```bash
# Clonar e entrar no projeto
git clone <URL> && cd <repo>

# Validar estrutura do template
./scripts/validation/validate-structure.sh

# Ver status git
git status

# Dar permissão de execução a scripts
chmod +x scripts/**/*.sh
```

## Dependências do sistema

Instale conforme a distribuição. Exemplos genéricos:

```bash
# Debian/Ubuntu
sudo apt update
sudo apt install -y git curl

# Fedora
sudo dnf install -y git curl
```

Dependências de domínio (Node, Python, Docker, etc.) devem ser documentadas em `docs/` ou em `core/` quando específicas do projeto.

## Uso recomendado

- Preferir scripts em `scripts/` com descoberta da raiz do repositório.
- Manter equivalência comportamental com os scripts Windows descritos em `tools-windows.md` e `rules-scripts.md`.
- Não versionar artefatos de build nem segredos.

## Observações específicas de Linux

- Caminhos usam `/`.
- Scripts shell devem falhar cedo (`set -euo pipefail` quando aplicável).
- Permissões de execução (`chmod +x`) são necessárias após o clone se o Git não as preservou.
