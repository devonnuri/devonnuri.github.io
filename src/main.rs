mod markdown;

use std::fs;
use crate::markdown::Options;
use crate::markdown::parser;
use crate::markdown::to_html;

fn to_html(value: &str) -> String {
    let mut options = Options::default();
    options.parse.constructs.frontmatter = true;

    let (events, parse_state) = parser::parse(value, &options.parse).unwrap();
    Ok::<String, String>(to_html::compile(
        &events,
        parse_state.bytes,
        &options.compile,
    )).unwrap()
}

fn main() {
    let mut directory_queue = Vec::new();
    directory_queue.push("./_wiki".to_string());

    while let Some(directory_path) = directory_queue.pop() {
        let index_path = directory_path.clone() + "/_index.md";
        if fs::metadata(index_path.clone()).is_ok() {
            println!("found index : {}", index_path);

            let index_content = fs::read_to_string(index_path).unwrap();
            let html = to_html(&index_content);
            println!("{}", html);
        }

        let dir = fs::read_dir(directory_path).unwrap();

        for dir_entry in dir {
            let path = dir_entry.unwrap().path();
            if path.is_dir() {
                directory_queue.push(path.to_str().unwrap().to_string());
                continue;
            }
            if path.extension().unwrap() != "md" {
                continue;
            }
            if path.file_name().unwrap() == "_index.md" {
                continue;
            }

            println!("found md file : {}", path.to_str().unwrap());
        }
    }
}