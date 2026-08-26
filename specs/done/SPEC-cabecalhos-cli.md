# Especificação: Cabeçalhos CLI em Scripts PowerShell

## 1. Objetivo

Padronizar a exibição de cabeçalhos de entrada nos scripts de linha de comando (CLI) do TRE-PA. O objetivo é fornecer uma apresentação visual consistente, limpa e que funcione independentemente do ambiente (desde terminais Windows modernos até ambientes limitados como o WinPE).

## 2. Exemplo Visual

Ao executar um script que implementa este padrão, o terminal deverá apresentar uma tabela em cor `Cyan` contendo as metainformações do script, com suporte para uma linha divisória que separa os metadados fixos (versão, compilação) de dados dinâmicos da execução (caminhos, parâmetros lidos).

```text
┌──────────────────┬────────────────────────────────────────────────────────┐
│ TRE-PA           │ Nome do Projeto / Modulo                               │
│ Versao           │ 12                                                     │
│ Compilado em     │ 2026-08-18 15:53                                       │
│ Funcao           │ Breve descricao da acao principal                      │
├──────────────────┼────────────────────────────────────────────────────────┤
│ Pasta do script  │ C:\Caminho\Para\O\Script                               │
│ Arquivo de Log   │ C:\Caminho\Para\O\Log\execucao.log                     │
└──────────────────┴────────────────────────────────────────────────────────┘
```

## 3. Características Técnicas

Para garantir compatibilidade universal (especialmente no WinPE que pode corromper caracteres UTF-8 de caixas de texto), o cabeçalho baseia-se nas seguintes regras:

1. **Codificação ASCII/OEM (Code Page 437):** Utiliza-se a invocação direta aos bytes dos caracteres de tabela padrão MS-DOS. Isso evita letras acentuadas bizarras ou caracteres ilegíveis em consolas sem suporte a Unicode.
2. **Cores:** A tabela inteira deve ser renderizada na cor ciano (`Cyan`) usando `Write-Host -ForegroundColor Cyan`.
3. **Colunas de Tamanho Fixo:** 
   - A coluna da esquerda (Chaves/Labels) tem largura padrão inicial de **18 caracteres**, expandindo-se dinamicamente (até um limite máximo, ex: 28) se alguma chave (label) for maior.
   - A coluna da direita (Valores) tem largura padrão de **56 caracteres**, quebrando (truncando) o que ultrapassar este valor, para garantir que o formato quadrado não "desabe" em ecrãs menores.
4. **Divisória Horizontal (Split):** O código permite definir após qual linha deve surgir o separador horizontal intermediário (`├─┼─┤`). Ideal para dividir o bloco em "Identificação do Software" e "Contexto da Execução atual".

## 4. Código de Referência (Implementação)

Abaixo estão as três funções genéricas (onde o prefixo `App` pode ser substituído pela sigla do projeto, como `Sis` no Configurar-SIS) que compõem o motor de renderização da tabela. Estas funções devem ser incluídas no ficheiro base do script.

```powershell
function Get-AppOemBoxChar {
    param([byte]$Code)
    # 437 = OEM United States (garante os caracteres de caixa originais do DOS)
    return [System.Text.Encoding]::GetEncoding(437).GetString([byte[]]@($Code))
}

function Format-AppBoxCell {
    param([string]$Text, [int]$Width)
    if ($null -eq $Text) { $Text = '' }
    if ($Text.Length -gt $Width) { 
        # Trunca se for maior que a largura
        return $Text.Substring(0, $Width) 
    }
    # Preenche com espacos se for menor
    return $Text.PadRight($Width)
}

function Write-AppBoxTable {
    param(
        [Parameter(Mandatory)][string[]]$Labels,
        [Parameter(Mandatory)][string[]]$Values,
        [int]$SplitAfter = 0
    )
    $leftW = 18
    $rightW = 56
    
    # Auto-ajuste de largura baseado na maior chave (ate 28 chars)
    foreach ($lab in $Labels) {
        if ($lab.Length -gt $leftW) { $leftW = [Math]::Min(28, $lab.Length) }
    }
    
    # Definicao dos caracteres OEM (borda simples)
    $tl = Get-AppOemBoxChar 218 # ┌
    $tr = Get-AppOemBoxChar 191 # ┐
    $bl = Get-AppOemBoxChar 192 # └
    $br = Get-AppOemBoxChar 217 # ┘
    $h  = Get-AppOemBoxChar 196 # ─
    $v  = Get-AppOemBoxChar 179 # │
    $ml = Get-AppOemBoxChar 195 # ├
    $mr = Get-AppOemBoxChar 180 # ┤
    
    # Linhas de Topo, Meio e Base pre-computadas
    $ruleTop = $tl + ($h * $leftW) + $h + ($h * $rightW) + $tr
    $ruleMid = $ml + ($h * $leftW) + $h + ($h * $rightW) + $mr
    $ruleBot = $bl + ($h * $leftW) + $h + ($h * $rightW) + $br
    
    Write-Host $ruleTop -ForegroundColor Cyan
    $n = $Labels.Count
    
    for ($i = 0; $i -lt $n; $i++) {
        # Monta cada linha de texto da tabela
        $line = $v + (Format-AppBoxCell -Text ([string]$Labels[$i]) -Width $leftW) + 
                $v + (Format-AppBoxCell -Text ([string]$Values[$i]) -Width $rightW) + $v
        
        Write-Host $line -ForegroundColor Cyan
        
        # Insere a divisoria caso chegue na linha indicada
        if ($SplitAfter -gt 0 -and $i -eq ($SplitAfter - 1) -and $i -lt ($n - 1)) {
            Write-Host $ruleMid -ForegroundColor Cyan
        }
    }
    
    Write-Host $ruleBot -ForegroundColor Cyan
    Write-Host '' # Linha em branco para espacamento apos o cabecalho
}
```

## 5. Exemplo de Utilização (Write-Header)

A implementação destas funções fica oculta. O orquestrador / script chama a formatação utilizando arrays de descrições e valores (usualmente contendo metadados extraídos de um `VERSION.ini`).

```powershell
function Write-AppHeader {
    Write-AppBoxTable -SplitAfter 4 -Labels @(
        'TRE-PA',
        'Versao',
        'Compilado em',
        'Funcao',
        'Pasta do script',
        'Arquivo de Log'
    ) -Values @(
        'Projeto Exemplo TRE-PA',
        '1.0',
        '2026-08-26 14:00',
        'Realizar rotina administrativa',
        $PSScriptRoot,
        'C:\Logs\exemplo.log'
    )
}

# Invocacao no inicio do script
Write-AppHeader
```
