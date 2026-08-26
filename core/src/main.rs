mod args;
mod config;
mod search;
mod tui;

fn main() {
    let cfg = config::Config::load();
    let app_args = args::parse_args(cfg);

    if app_args.config_changed {
        eprintln!("(filtros atualizados na configuração)");
    }

    if app_args.queries.is_empty() {
        eprintln!("cdd: uso: cdd <query> [opcoes]");
        std::process::exit(1);
    }

    // A busca será implementada em search::find_directories
    let results = search::find_directories(&app_args);

    if results.is_empty() {
        eprintln!("cdd: nenhum diretorio encontrado");
        std::process::exit(1);
    } else {
        let selected = if results.len() == 1 || app_args.lucky_pick {
            results[0].clone()
        } else {
            if let Some(choice) = tui::select_directory(results, app_args.list_size as usize) {
                choice
            } else {
                std::process::exit(1);
            }
        };

        if let Some(out_path) = app_args.out_file {
            let _ = std::fs::write(out_path, &selected);
        } else {
            println!("{}", selected);
        }
    }
}
