use crate::class::{max_arg_count, min_arg_count};
use crate::warning::WarningType;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref SPACE_BETWEEN: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("space_between.ron")).unwrap();
}

pub fn parse_space_between(
    class_name: &str,
    args: &[&str; 3],
    warnings: &mut Vec<WarningType>,
) -> Option<Vec<String>> {
    max_arg_count(class_name, args, 2, warnings);

    if min_arg_count(args, 2, warnings) {
        let negative = class_name.starts_with('-');

        match args[0] {
            "x" => {
                if let Some(value) = get_value(args[1], negative) {
                    return Some(vec![format!("margin-left: {}", value)]);
                }

                if args[1] == "reverse" {
                    return Some(vec!["--tw-space-x-reverse: 1".into()]);
                }

                warnings.push(WarningType::ValueNotFound(
                    format!("{}-{}", class_name, args[0]),
                    args[0].into(),
                ))
            }
            "y" => {
                if let Some(value) = get_value(args[1], negative) {
                    return Some(vec![format!("margin-top: {}", value)]);
                }

                if args[1] == "reverse" {
                    return Some(vec!["--tw-space-y-reverse: 1".into()]);
                }

                warnings.push(WarningType::ValueNotFound(
                    format!("{}-{}", class_name, args[0]),
                    args[0].into(),
                ))
            }
            _ => warnings.push(WarningType::InvalidArg(
                format!("{}-{}", class_name, args[0]),
                vec!["x", "y"],
            )),
        }
    }

    None
}

fn get_value(arg: &str, negative: bool) -> Option<String> {
    if let Some(value) = SPACE_BETWEEN.get(arg) {
        return Some(format!("{}{}", if negative { "-" } else { "" }, value));
    }
    None
}
