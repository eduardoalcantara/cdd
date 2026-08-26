use crate::config::{Config, ListOrder, QueryOrder};
use std::env;

#[derive(Debug)]
pub struct AppArgs {
    pub queries: Vec<String>,
    pub lucky_pick: bool,
    pub list_size: u32,
    pub list_order: ListOrder,
    pub query_order: QueryOrder,
    pub config_changed: bool,
    pub out_file: Option<String>,
}

pub fn parse_args(mut config: Config) -> AppArgs {
    let mut queries = Vec::new();
    let args: Vec<String> = env::args().skip(1).collect();
    
    let mut lucky_pick = config.lucky_pick;
    let mut list_size = config.list_size;
    let mut list_order = config.list_order.clone();
    let mut query_order = config.query_order.clone();
    let mut config_changed = false;
    let mut out_file = None;

    let mut skip_next = false;
    for (i, arg) in args.iter().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }

        if arg == "--cdd-out-file" {
            if let Some(path) = args.get(i + 1) {
                out_file = Some(path.clone());
                skip_next = true;
                continue;
            }
        }
        if arg.starts_with('-') {
            // Check for sticky
            let (core_arg, sticky) = if arg.ends_with(":on") {
                (&arg[..arg.len() - 3], Some(true))
            } else if arg.ends_with(":off") {
                (&arg[..arg.len() - 4], Some(false))
            } else {
                (arg.as_str(), None)
            };

            // Parse argument
            match core_arg {
                "-l" | "-1" => {
                    if let Some(true) = sticky {
                        config.lucky_pick = true;
                        config_changed = true;
                        lucky_pick = true;
                    } else if let Some(false) = sticky {
                        config.lucky_pick = false;
                        config_changed = true;
                        lucky_pick = false;
                    } else {
                        lucky_pick = true;
                    }
                }
                "-oa" => {
                    if let Some(true) = sticky {
                        config.list_order = ListOrder::Ascending;
                        config_changed = true;
                        list_order = ListOrder::Ascending;
                    } else if let Some(false) = sticky {
                        // Removing sticky, reset to default? We can just reset to Find
                        config.list_order = ListOrder::Find;
                        config_changed = true;
                        list_order = ListOrder::Find;
                    } else {
                        list_order = ListOrder::Ascending;
                    }
                }
                "-od" => {
                    if let Some(true) = sticky {
                        config.list_order = ListOrder::Descending;
                        config_changed = true;
                        list_order = ListOrder::Descending;
                    } else if let Some(false) = sticky {
                        config.list_order = ListOrder::Find;
                        config_changed = true;
                        list_order = ListOrder::Find;
                    } else {
                        list_order = ListOrder::Descending;
                    }
                }
                "-of" => {
                    if let Some(true) = sticky {
                        config.list_order = ListOrder::Find;
                        config_changed = true;
                        list_order = ListOrder::Find;
                    } else if let Some(false) = sticky {
                        config.list_order = ListOrder::Find;
                        config_changed = true;
                        list_order = ListOrder::Find;
                    } else {
                        list_order = ListOrder::Find;
                    }
                }
                "-qs" => {
                    if let Some(true) = sticky {
                        config.query_order = QueryOrder::Sequential;
                        config_changed = true;
                        query_order = QueryOrder::Sequential;
                    } else if let Some(false) = sticky {
                        config.query_order = QueryOrder::Sequential;
                        config_changed = true;
                        query_order = QueryOrder::Sequential;
                    } else {
                        query_order = QueryOrder::Sequential;
                    }
                }
                "-qi" => {
                    if let Some(true) = sticky {
                        config.query_order = QueryOrder::Inverse;
                        config_changed = true;
                        query_order = QueryOrder::Inverse;
                    } else if let Some(false) = sticky {
                        config.query_order = QueryOrder::Sequential;
                        config_changed = true;
                        query_order = QueryOrder::Sequential;
                    } else {
                        query_order = QueryOrder::Inverse;
                    }
                }
                "-qa" => {
                    if let Some(true) = sticky {
                        config.query_order = QueryOrder::Any;
                        config_changed = true;
                        query_order = QueryOrder::Any;
                    } else if let Some(false) = sticky {
                        config.query_order = QueryOrder::Sequential;
                        config_changed = true;
                        query_order = QueryOrder::Sequential;
                    } else {
                        query_order = QueryOrder::Any;
                    }
                }
                _ => {
                    // Try to parse list size (-2 to -20)
                    if core_arg.starts_with('-') && core_arg.len() > 1 && core_arg.chars().nth(1).unwrap().is_ascii_digit() {
                        if let Ok(size) = core_arg[1..].parse::<u32>() {
                            if let Some(true) = sticky {
                                config.list_size = size;
                                config_changed = true;
                                list_size = size;
                            } else if let Some(false) = sticky {
                                config.list_size = 10;
                                config_changed = true;
                                list_size = 10;
                            } else {
                                list_size = size;
                            }
                        }
                    }
                }
            }
        } else {
            queries.push(arg.clone());
        }
    }

    if config_changed {
        let _ = config.save();
    }

    AppArgs {
        queries,
        lucky_pick,
        list_size,
        list_order,
        query_order,
        config_changed,
        out_file,
    }
}
