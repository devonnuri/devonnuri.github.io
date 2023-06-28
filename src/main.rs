mod markdown;

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::ffi::OsStr;

use crate::markdown::{Options, ParseOptions, Constructs};
use crate::markdown::parser;
use crate::markdown::to_html;

fn to_html(value: &str) -> (String, HashMap<String, String>) {
    let options = Options {
        parse: ParseOptions {
            constructs: Constructs {
                frontmatter: true,
                // math_flow: true,  // due to mathjax
                // math_text: true,
                ..Constructs::default()
            },
            ..ParseOptions::default()
        },
        ..Options::default()
    };

    let (events, parse_state) = parser::parse(value, &options.parse).unwrap();
    // println!("{:#?}", events);

    Ok::<(String, HashMap<String, String>), (String, HashMap<String, String>)>(to_html::compile(
        &events,
        parse_state.bytes,
        &options.compile,
    )).unwrap()
}

fn write_html(directory_path: &String, html: String, frontmatter: &HashMap<String, String>, category: String) {
    // Make directories recursively in '_build' directory
    fs::create_dir_all(&directory_path).unwrap();

    // Write html file from the template file
    let template = fs::read_to_string("./_template/entry.html").unwrap();
    let html = template
        .replace("{{title}}", &frontmatter.get("title").unwrap_or(&"".to_string()))
        .replace("{{subtitle}}", &frontmatter.get("subtitle").unwrap_or(&"".to_string()))
        .replace("{{created_at}}", &frontmatter.get("created_at").unwrap_or(&"".to_string()))
        .replace("{{updated_at}}", &frontmatter.get("updated_at").unwrap_or(&"".to_string()))
        .replace("{{content}}", &html)
        .replace("{{category}}", &category);

    fs::write(directory_path.clone() + "/index.html", html).unwrap();
}

fn main() {
    let mut directory_queue = Vec::new();
    directory_queue.push("./_wiki".to_string());

    // Create '_build' directory if not exists, and clear it.
    if fs::metadata("./_build").is_ok() {
        fs::remove_dir_all("./_build").unwrap();
    }
    fs::create_dir("./_build").unwrap();

    // Copy all files from '_static' into '_build'
    for entry in fs::read_dir("./_static").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let new_path = "./_build/".to_string() + file_name;
        fs::copy(path, new_path).unwrap();
    }

    let mut category_map: HashMap<String, String> = HashMap::new();

    // Iterate over all files in the directory.
    while let Some(directory_path) = directory_queue.pop() {
        let index_path = directory_path.clone() + "/_index.md";

        println!("found index : {}", &index_path);

        let index_content = fs::read_to_string(&index_path).unwrap();
        let (html, frontmatter) = to_html(&index_content);

        let directory_pathbuf = PathBuf::from(directory_path.clone());
        let index_entry_filename = directory_pathbuf.file_name().unwrap().to_str().unwrap();
        let index_entry_directory = if index_entry_filename == "_wiki" { 
            "./_build/".to_string()
        } else {
            "./_build/".to_string() + index_entry_filename + "/"
        };

        let parent_directory = directory_pathbuf.parent().unwrap().file_name().unwrap_or(OsStr::new("")).to_str().unwrap();
        let index_category = category_map.get(parent_directory).map(String::as_str).unwrap_or("").to_string();
        let current_category = if index_category == "" {
            frontmatter.get("title").unwrap_or(&"".to_string()).clone()
        } else {
            index_category.clone() + " > " + frontmatter.get("title").unwrap_or(&"".to_string())
        };

        write_html(&index_entry_directory, html, &frontmatter, index_category.clone());

        category_map.insert(index_entry_filename.to_string(), current_category.clone());

        println!("parent: {}, current: {}", parent_directory, index_entry_filename);

        let dir = fs::read_dir(&directory_path).unwrap();

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
            let entry_filename = path.file_stem().unwrap().to_str().unwrap();
            let entry_directory = "./_build/".to_string() + entry_filename + "/";

            let content = fs::read_to_string(path.clone()).unwrap();
            let (html, frontmatter) = to_html(&content);
            write_html(&entry_directory, html, &frontmatter, current_category.clone());
        }
    }
}