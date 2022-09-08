use class::spacing::{margin::Margin, padding::Padding, space_between::SpaceBetween};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::class::Class;

pub mod class;

lazy_static! {
    pub static ref STYLE_REGEX: Regex =
        Regex::new(r#"(?:class|className)=(?:["']\W+\s*(?:\w+)\()?["']([^'"]+)['"]"#).unwrap();
}

pub fn parse_html(input: &Path, output: &Path) -> Result<(), String> {
    let html = match fs::read_to_string(input) {
        Ok(h) => h,
        Err(e) => return Err(e.to_string()),
    };

    let mut classes: Vec<&str> = Vec::new();
    for capture in STYLE_REGEX.captures_iter(&html) {
        if let Some(group) = capture.get(1) {
            for cap in group.as_str().split(" ") {
                classes.push(cap)
            }
        }
    }

    let mut compiled_css = fs::read_to_string("preflight.css").unwrap();

    for class in classes {
        // padding, margin, spacing...
        if class.contains('-') {
            // if class contains one dash `-`
            match class.chars().filter(|c| *c == '-').count() {
                1 => {
                    let mut split = class.split("-");
                    let before_dash = split.nth(0).unwrap();
                    let after_dash = split.nth(0).unwrap();

                    let class = match before_dash.len() {
                        1 => match before_dash {
                            "m" => Class::Margin(Margin::new(before_dash, after_dash)),
                            "p" => Class::Padding(Padding::new(before_dash, after_dash)),
                            _ => continue,
                        },
                        2 => match before_dash.chars().nth(0).unwrap() {
                            'm' => Class::Margin(Margin::new(before_dash, after_dash)),
                            'p' => Class::Padding(Padding::new(before_dash, after_dash)),
                            _ => continue,
                        },
                        _ => continue,
                    };

                    compiled_css.push_str(&class.to_css());
                }
                2 => {
                    let mut split = class.split("-");
                    let before_dash = split.nth(0).unwrap();
                    let mid_dash = split.nth(0).unwrap();
                    let after_dash = split.nth(0).unwrap();

                    let class = match before_dash {
                        "space" => Class::SpaceBetween(SpaceBetween::new(mid_dash, after_dash)),
                        _ => continue,
                    };

                    compiled_css.push_str(&class.to_css());
                }
                _ => (),
            }
        }
    }

    let mut file = match File::create(output) {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };

    match file.write_all(compiled_css.as_bytes()) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e.to_string()),
    }
}
