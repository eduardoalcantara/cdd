pub const HELP: &str = r#"cdd — Change Directory Directly
2026 (c) Eduardo Alcantara

USAGE:
    cdd <query> [queries...] [options]
    cdd -h | --help

SEARCH:
    Queries perform partial matching, without autocorrection.
    * matches zero or more characters in a directory name.
    ? matches exactly one character in a directory name.
    The final query identifies the target directory itself; its descendants
    are not included just because they inherit the path match.

    In Bash/Zsh, quote wildcards to prevent the shell
    from expanding them before cdd: cdd 'proj*' 'app?'

OPTIONS:
    -h, --help       Show this help
    -l, -1           Select the first result without opening the menu
    -2 ... -20       Set the number of visible lines in the list (default: -10)
    -oa              Ascending alphabetical order
    -od              Descending alphabetical order
    -of              Order found during scan (default)
    -qs              Queries in sequential order (default)
    -qi              Queries in inverse order
    -qa              Queries in any order
    -ci              Case insensitive matching
    -cr              Respect case matching
    --               End options; allows queries starting with a hyphen

PERSISTENT CONFIGURATION:
    Append :on to persist an option and :off to remove it.
    Examples: -l:on, -l:off, -15:on, -oa:on, -qa:off, -ci:on
    File: system config directory/cdd/cdd.json

EXAMPLES:
    cdd www app
    cdd 'proj*' 'app?'
    cdd docs -qa -oa
    cdd cache -l

The command must be sourced by the Bash/Zsh or PowerShell wrapper so that
the current terminal directory is changed."#;
