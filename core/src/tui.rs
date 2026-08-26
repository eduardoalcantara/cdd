use inquire::Select;

pub fn select_directory(mut matches: Vec<String>, list_size: usize) -> Option<String> {
    if matches.len() > list_size {
        matches.truncate(list_size);
    }
    
    let ans = Select::new("Selecione o diretório alvo:", matches)
        .with_page_size(list_size)
        .prompt();

    match ans {
        Ok(choice) => Some(choice),
        Err(_) => None, // Cancelado pelo usuário
    }
}
