mod args;
mod config;
mod help;
mod pattern;
mod search;
mod tui;

fn main() {
    let cfg = config::Config::load();
    let app_args = match args::parse_args(cfg) {
        Ok(args) => args,
        Err(error) => {
            eprintln!("cdd: {error}");
            eprintln!("Use 'cdd --help' to see available options.");
            std::process::exit(2);
        }
    };

    if app_args.show_help {
        println!("{}", help::HELP);
        return;
    }

    if app_args.config_changed {
        eprintln!("(filters updated in configuration)");
    }

    if !app_args.active_filters.is_empty() {
        eprintln!("(active filters: {})", app_args.active_filters.join(", "));
    }

    if app_args.queries.is_empty() {
        eprintln!("cdd: usage: cdd <query> [queries...] [options]");
        eprintln!("Use 'cdd --help' to see examples.");
        std::process::exit(2);
    }

    let results = match search::find_directories(&app_args) {
        Ok(results) => results,
        Err(error) => {
            eprintln!("cdd: {error}");
            std::process::exit(2);
        }
    };

    if results.is_empty() {
        eprintln!("cdd: no directory found");
        std::process::exit(1);
    } else {
        let selected = if results.len() == 1 || app_args.lucky_pick {
            results[0].clone()
        } else if let Some(choice) = tui::select_directory(results, app_args.list_size as usize, app_args.case_sensitivity) {
            choice
        } else {
            std::process::exit(1);
        };

        if let Some(out_path) = app_args.out_file {
            if let Err(error) = std::fs::write(out_path, &selected) {
                eprintln!("cdd: could not write result: {error}");
                std::process::exit(1);
            }
        } else {
            println!("{}", selected);
        }
    }
}
