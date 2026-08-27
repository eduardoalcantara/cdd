use crate::config::{Config, ListOrder, QueryOrder};
use std::env;

#[derive(Debug, Clone)]
pub struct AppArgs {
    pub queries: Vec<String>,
    pub lucky_pick: bool,
    pub list_size: u32,
    pub list_order: ListOrder,
    pub query_order: QueryOrder,
    pub config_changed: bool,
    pub out_file: Option<String>,
    pub show_help: bool,
    pub active_filters: Vec<String>,
}

pub fn parse_args(config: Config) -> Result<AppArgs, String> {
    let args = env::args().skip(1);
    let (app_args, config) = parse_from(config, args)?;

    if app_args.config_changed {
        config
            .save()
            .map_err(|error| format!("could not save configuration: {error}"))?;
    }

    Ok(app_args)
}

fn parse_from<I, S>(mut config: Config, args: I) -> Result<(AppArgs, Config), String>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut queries = Vec::new();
    let args: Vec<String> = args.into_iter().map(Into::into).collect();

    if args.iter().any(|arg| arg == "-h" || arg == "--help") {
        let app_args = AppArgs {
            queries,
            lucky_pick: config.lucky_pick,
            list_size: config.list_size,
            list_order: config.list_order,
            query_order: config.query_order,
            config_changed: false,
            out_file: None,
            show_help: true,
            active_filters: config.active_filter_labels(),
        };
        return Ok((app_args, config));
    }

    let mut lucky_pick = config.lucky_pick && !config.sticky.list_size;
    let mut list_size = config.list_size;
    let mut list_order = config.list_order;
    let mut query_order = config.query_order;
    let mut config_changed = false;
    let mut out_file = None;
    let mut options_ended = false;
    let mut list_size_explicit = false;

    let mut skip_next = false;
    for (i, arg) in args.iter().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }

        if !options_ended && arg == "--" {
            options_ended = true;
            continue;
        }

        if !options_ended && arg == "--cdd-out-file" {
            let path = args
                .get(i + 1)
                .ok_or_else(|| "internal option --cdd-out-file requires a path".to_string())?;
            out_file = Some(path.clone());
            skip_next = true;
            continue;
        }

        if !options_ended && arg.starts_with('-') {
            let (core_arg, sticky) = if arg.ends_with(":on") {
                (&arg[..arg.len() - 3], Some(true))
            } else if arg.ends_with(":off") {
                (&arg[..arg.len() - 4], Some(false))
            } else {
                (arg.as_str(), None)
            };

            match core_arg {
                "-l" | "-1" => match sticky {
                    Some(true) => {
                        config.lucky_pick = true;
                        config.sticky.lucky_pick = true;
                        config_changed = true;
                        lucky_pick = true;
                    }
                    Some(false) => {
                        config.lucky_pick = false;
                        config.sticky.lucky_pick = false;
                        config_changed = true;
                        lucky_pick = false;
                    }
                    None => lucky_pick = true,
                },
                "-oa" => apply_list_order(
                    ListOrder::Ascending,
                    sticky,
                    &mut config,
                    &mut list_order,
                    &mut config_changed,
                ),
                "-od" => apply_list_order(
                    ListOrder::Descending,
                    sticky,
                    &mut config,
                    &mut list_order,
                    &mut config_changed,
                ),
                "-of" => apply_list_order(
                    ListOrder::Find,
                    sticky,
                    &mut config,
                    &mut list_order,
                    &mut config_changed,
                ),
                "-qs" => apply_query_order(
                    QueryOrder::Sequential,
                    sticky,
                    &mut config,
                    &mut query_order,
                    &mut config_changed,
                ),
                "-qi" => apply_query_order(
                    QueryOrder::Inverse,
                    sticky,
                    &mut config,
                    &mut query_order,
                    &mut config_changed,
                ),
                "-qa" => apply_query_order(
                    QueryOrder::Any,
                    sticky,
                    &mut config,
                    &mut query_order,
                    &mut config_changed,
                ),
                _ => {
                    if let Some(raw_size) = core_arg.strip_prefix('-')
                        && raw_size.chars().all(|character| character.is_ascii_digit())
                    {
                        let size = raw_size
                            .parse::<u32>()
                            .map_err(|_| format!("invalid list size: {core_arg}"))?;
                        if !(2..=20).contains(&size) {
                            return Err(format!(
                                "list size out of range -2 to -20: {core_arg}"
                            ));
                        }

                        list_size_explicit = true;
                        match sticky {
                            Some(true) => {
                                config.list_size = size;
                                config.sticky.list_size = true;
                                config_changed = true;
                                list_size = size;
                            }
                            Some(false) => {
                                config.list_size = 10;
                                config.sticky.list_size = false;
                                config_changed = true;
                                list_size = 10;
                            }
                            None => list_size = size,
                        }
                    } else {
                        return Err(format!("unknown option: {arg}"));
                    }
                }
            }
        } else {
            queries.push(arg.clone());
        }
    }

    if list_size_explicit {
        lucky_pick = false;
    }

    let app_args = AppArgs {
        queries,
        lucky_pick,
        list_size,
        list_order,
        query_order,
        config_changed,
        out_file,
        show_help: false,
        active_filters: config.active_filter_labels(),
    };

    Ok((app_args, config))
}

fn apply_list_order(
    requested: ListOrder,
    sticky: Option<bool>,
    config: &mut Config,
    effective: &mut ListOrder,
    config_changed: &mut bool,
) {
    match sticky {
        Some(true) => {
            config.list_order = requested;
            config.sticky.list_order = true;
            *effective = requested;
            *config_changed = true;
        }
        Some(false) => {
            config.list_order = ListOrder::Find;
            config.sticky.list_order = false;
            *effective = ListOrder::Find;
            *config_changed = true;
        }
        None => *effective = requested,
    }
}

fn apply_query_order(
    requested: QueryOrder,
    sticky: Option<bool>,
    config: &mut Config,
    effective: &mut QueryOrder,
    config_changed: &mut bool,
) {
    match sticky {
        Some(true) => {
            config.query_order = requested;
            config.sticky.query_order = true;
            *effective = requested;
            *config_changed = true;
        }
        Some(false) => {
            config.query_order = QueryOrder::Sequential;
            config.sticky.query_order = false;
            *effective = QueryOrder::Sequential;
            *config_changed = true;
        }
        None => *effective = requested,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(arguments: &[&str]) -> Result<(AppArgs, Config), String> {
        parse_from(Config::default(), arguments.iter().copied())
    }

    #[test]
    fn help_is_handled_without_a_query() {
        let (args, _) = parse(&["--help"]).unwrap();
        assert!(args.show_help);
        assert!(args.queries.is_empty());
    }

    #[test]
    fn list_size_accepts_only_two_through_twenty() {
        assert_eq!(parse(&["query", "-2"]).unwrap().0.list_size, 2);
        assert_eq!(parse(&["query", "-20"]).unwrap().0.list_size, 20);
        assert!(parse(&["query", "-0"]).is_err());
        assert!(parse(&["query", "-21"]).is_err());
    }

    #[test]
    fn explicit_list_size_disables_lucky_pick_for_the_invocation() {
        let config = Config {
            lucky_pick: true,
            ..Config::default()
        };
        let (args, _) = parse_from(config, ["query", "-12"]).unwrap();
        assert!(!args.lucky_pick);
    }

    #[test]
    fn persisted_list_size_overrides_persisted_lucky_pick() {
        let config = Config {
            lucky_pick: true,
            sticky: crate::config::StickyState {
                lucky_pick: true,
                list_size: true,
                ..Default::default()
            },
            ..Config::default()
        };

        let (args, _) = parse_from(config, ["query"]).unwrap();
        assert!(!args.lucky_pick);
    }

    #[test]
    fn sticky_filters_are_tracked_even_when_the_value_is_default() {
        let (args, config) = parse(&["query", "-10:on", "-of:on", "-qs:on"]).unwrap();

        assert!(config.sticky.list_size);
        assert!(config.sticky.list_order);
        assert!(config.sticky.query_order);
        assert_eq!(args.active_filters, vec!["-10=on", "-of=on", "-qs=on"]);
    }

    #[test]
    fn sticky_off_resets_value_and_metadata() {
        let config = Config {
            lucky_pick: true,
            sticky: crate::config::StickyState {
                lucky_pick: true,
                ..Default::default()
            },
            ..Config::default()
        };

        let (args, config) = parse_from(config, ["query", "-l:off"]).unwrap();
        assert!(!args.lucky_pick);
        assert!(!config.sticky.lucky_pick);
        assert!(args.active_filters.is_empty());
    }

    #[test]
    fn unknown_option_is_an_error() {
        assert_eq!(
            parse(&["query", "-xyz"]).unwrap_err(),
            "unknown option: -xyz"
        );
    }

    #[test]
    fn double_dash_allows_a_query_starting_with_hyphen() {
        let (args, _) = parse(&["--", "-archive"]).unwrap();
        assert_eq!(args.queries, vec!["-archive"]);
    }
}
