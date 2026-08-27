pub const HELP: &str = r#"cdd — Change Directory Directly

USO:
    cdd <termo> [termos...] [opções]
    cdd -h | --help

BUSCA:
    Termos comuns fazem correspondência parcial, sem autocorreção.
    * corresponde a zero ou mais caracteres no nome de um diretório.
    ? corresponde a exatamente um caractere no nome de um diretório.
    O termo final identifica o próprio diretório de destino; seus descendentes
    não são incluídos apenas por herdarem o match no caminho.

    No Bash/Zsh, coloque curingas entre aspas para impedir que o shell
    os expanda antes do cdd: cdd 'proj*' 'app?'

OPÇÕES:
    -h, --help       Mostra esta ajuda
    -l, -1           Seleciona o primeiro resultado, sem abrir o menu
    -2 ... -20       Define as linhas visíveis da lista (padrão: -10)
    -oa              Ordem alfabética ascendente
    -od              Ordem alfabética descendente
    -of              Ordem encontrada na varredura (padrão)
    -qs              Termos na ordem informada (padrão)
    -qi              Termos na ordem inversa
    -qa              Termos em qualquer ordem
    --               Encerra as opções; permite termos iniciados por hífen

CONFIGURAÇÃO PERSISTENTE:
    Acrescente :on para persistir uma opção e :off para removê-la.
    Exemplos: -l:on, -l:off, -15:on, -oa:on, -qa:off
    Arquivo: diretório de configuração do sistema/cdd/cdd.json

EXEMPLOS:
    cdd www app
    cdd 'proj*' 'app?'
    cdd docs -qa -oa
    cdd cache -l

O comando deve ser carregado pelo wrapper Bash/Zsh ou PowerShell para que
o diretório do terminal atual seja alterado."#;
