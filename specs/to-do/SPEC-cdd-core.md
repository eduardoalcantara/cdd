# Especificação: Core do cdd (Change Directory Directly)

## 1. Objetivo

Implementar a funcionalidade central do comando `cdd`, um utilitário de navegação de diretórios por linha de comando otimizado para usuários rápidos. O `cdd` permite buscas nebulosas (fuzzy/partial match) por sequências de parâmetros sem exigir o caminho completo.

## 2. Visão Geral do Funcionamento

O usuário invoca o comando com um ou mais argumentos parciais de diretórios:
```bash
cdd www app1
```

O utilitário pesquisa o sistema de arquivos partindo de raízes adequadas ao sistema operacional:
- **Linux:** O algoritmo inicia a varredura a partir do diretório raiz `/`.
- **Windows:** A varredura começa na raiz do drive atual (ex: `C:\`). Caso o primeiro argumento seja uma letra única (ex: `d`, `e`), o sistema interpreta como a unidade a ser pesquisada (ex: `D:\`) em vez de usar o drive atual.

Em seguida, o comportamento se baseia no número de resultados:
1. **Se encontrar 1 match:** Muda imediatamente o diretório do terminal atual para o caminho encontrado (ex: `/var/www/applications/app1`).
2. **Se encontrar mais de 1 match:** Exibe uma interface gráfica de terminal (TUI) com os diretórios listados, onde o usuário pode navegar (Setas Direcionais, Page Up/Down, Home/End) e selecionar com ENTER o diretório alvo.

## 3. Arquitetura (O Problema do `cd`)

**Importante:** Um processo filho (como um binário Rust ou script Python) não pode alterar o diretório de trabalho do processo pai (o shell do usuário). 
Para que o `cdd` funcione de fato, ele usará o padrão de **Shell Wrapper**:

1. **O Binário Core (Rust):** Faz o trabalho pesado. Interpreta os parâmetros, lê os arquivos de configuração, realiza a busca rápida (suporta regex/glob/wildcards) no disco e gerencia a interface visual TUI se houver mais de um resultado. Ao final, ele apenas *imprime* o caminho absoluto escolhido no `stdout` (ou escreve em um arquivo temporário).
2. **O Shell Wrapper (Bash/PowerShell):** É uma função injetada no perfil do usuário (`.bashrc`, `$PROFILE`). O usuário, ao digitar `cdd`, está chamando esse wrapper. O wrapper executa o binário Core e intercepta o caminho impresso para executar de fato o comando `cd <caminho>`.

## 4. Arquivo de Configuração do Usuário

O comportamento do `cdd` pode ser customizado e persistido num arquivo de configuração na home do usuário (ex: `~/.config/cdd/cdd.conf` ou equivalente).
Todas as opções podem ser sobrescritas temporariamente em tempo de execução, ou fixadas (sticky) se acompanhadas do modificador `on` ou `off`.

### Sintaxe de Opções e Persistência
- Uso normal (temporário na chamada, faz o bypass da configuração global): `-l`
- Fixar/Ativar na configuração pessoal: `-l:on`
- Remover/Desativar da configuração pessoal: `-l:off`
*Toda vez que o comando rodar afetado por um parâmetro salvo na configuração, ele deve exibir no cabeçalho/prompt: `(filtros ativos: -1=on)`.*

## 5. Opções Suportadas

### 5.1 Lucky Pick (`-l` ou `-1`)
- **Ação:** O comando seleciona o *primeiro* diretório que satisfaça as condições da query e aplica o `cd` imediatamente, ignorando se havia outras correspondências. Pula a interface visual TUI.
- **Opções Sticky:** `-l:on`, `-l:off`, `-1:on`, `-1:off`.

### 5.2 List Size (`-2` até `-20`)
- **Ação:** Define o tamanho máximo da lista interativa exibida caso haja múltiplas opções.
- **Default:** `-10`.
- **Nota:** Substitui a restrição da opção `-l`/`-1`.
- **Opções Sticky:** `-<N>:on`, `-<N>:off`.

### 5.3 List Order (`-oa`, `-od`, `-of`)
- **Ação:** Ordem de exibição dos resultados.
  - `a`: Ascendente (Alfabética A-Z).
  - `d`: Descendente (Alfabética Z-A).
  - `f`: Ordem de *Find* / de busca (ordem com que o algoritmo encontrou no disco) - **Padrão**.
- **Opções Sticky:** `-o<ordem>:on`, `-o<ordem>:off`.

### 5.4 Query Order (`-qs`, `-qi`, `-qa`)
- **Ação:** Regras lógicas para tratar as palavras pesquisadas.
  - `s` (Sequential): O argumento 1 deve estar antes (num nível mais alto na árvore de pastas) do argumento 2. Ex: `cdd www app` (onde `app` é uma subpasta de `www`). **Padrão**.
  - `i` (Inverse): Ordem inversa. `cdd app www` encontraria `/var/www/app`.
  - `a` (Any): A ordem não importa. Basta que ambos os termos existam no caminho do diretório.
- **Opções Sticky:** `-q<modo>:on`, `-q<modo>:off`.

## 6. Dúvidas Técnicas Esclarecidas

1. **Uso de Coringas (`*`, `?`):** Totalmente factível. A implementação do buscador (Core Rust) suportará expansão por Glob e Regex nativos.
2. **Mudança de Diretório em Rust vs Shell:** Resolvido pela Arquitetura de "Shell Wrapper + Binary Core".
3. **Menus em Bash vs Rust:** Embora o Bash permita menus através do `select` ou comandos interativos como o `dialog` ou lendo inputs de teclado (tecla a tecla com ANSI escapes), é extremamente frágil, lento e difícil de manter cross-platform. **O Rust cuidará da TUI** (usando bibliotecas como `ratatui`, `crossterm` ou `inquire`), onde renderização de cores, fundo invertido para item focado e capturas de setas do teclado funcionam excepcionalmente bem.
4. **Cross-Platform:** Rust compila perfeitamente tanto para binários Linux quanto Windows. O único componente que precisa de versões separadas são os Wrappers (função `.sh` para Bash/Zsh no Linux e uma função em `.ps1` para o PowerShell no Windows).

## 7. Próximos Passos
- Inicializar projeto Rust em `/core`.
- Implementar o parser de argumentos em CLI (suporte à sintaxe `:on/:off`).
- Implementar o buscador de arquivos concorrente com motor de Glob.
- Implementar as funções Wrapper (`cdd.sh` e `cdd.ps1`).