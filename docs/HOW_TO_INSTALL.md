# Como instalar o cdd

## Pacote pré-compilado

O usuário final não precisa instalar Rust.

### Linux / WSL

1. Extraia `cdd-linux-x86_64.tar.gz`.
2. Entre na pasta extraída.
3. Execute:

```bash
bash install.sh
```

4. Abra outro terminal ou recarregue o profile:

```bash
source ~/.bashrc
```

O pacote instala o binário e o wrapper em `~/.local/share/cdd` e adiciona uma entrada ao `.bashrc` e/ou `.zshrc`.

Desinstalação:

```bash
bash install.sh --uninstall
```

### Windows / PowerShell

1. Extraia `cdd-windows-x86_64.zip`.
2. Abra o PowerShell na pasta extraída.
3. Execute:

```powershell
.\install.ps1
```

4. Abra outro terminal ou recarregue o profile:

```powershell
. $PROFILE
```

O pacote instala `cdd.exe` e o wrapper em `$LOCALAPPDATA\cdd`.

Desinstalação:

```powershell
.\install.ps1 -Uninstall
```

## Instalação para desenvolvimento

Esta modalidade compila o checkout atual e faz o profile apontar para o wrapper dentro do repositório.

Pré-requisito: toolchain Rust com `cargo`.

Linux:

```bash
bash scripts/setup/install.sh --quiet
```

PowerShell:

```powershell
.\scripts\setup\install.ps1 -Quiet
```

Essa instalação deixa de funcionar se o repositório for movido ou removido. Para distribuição, prefira os pacotes pré-compilados.

## Gerar pacotes

Linux:

```bash
bash scripts/setup/build-dist.sh
```

Windows:

```powershell
.\scripts\setup\build-dist.ps1
```

Os artefatos são gravados em `dist/`:

- `cdd-linux-x86_64.tar.gz`;
- `cdd-linux-x86_64.tar.gz.sha256`;
- `cdd-windows-x86_64.zip`;
- `cdd-windows-x86_64.zip.sha256`.

Cada pacote inclui `README.md`, `HOW_TO_USE.md`, `HOW_TO_INSTALL.md` e `HOW_IT_WORKS.md`.

Verifique o pacote Linux:

```bash
cd dist
sha256sum -c cdd-linux-x86_64.tar.gz.sha256
```

No PowerShell:

```powershell
Get-FileHash .\dist\cdd-windows-x86_64.zip -Algorithm SHA256
```

O pacote Windows deve ser compilado em Windows ou em ambiente cross-compilation capaz de produzir um `cdd.exe` válido. Não substitua o executável Windows pelo binário Linux.

## Verificação

Abra um terminal novo:

```bash
cdd --help
```

No PowerShell:

```powershell
cdd --help
```

Se o comando não existir, confirme que o profile correto contém `CDD_INSTALL_MARKER` e recarregue-o.
