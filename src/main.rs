mod markdown;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

use crate::markdown::parser;
use crate::markdown::to_html;
use crate::markdown::{Constructs, Options, ParseOptions};

fn to_html(value: &str) -> (String, HashMap<String, String>) {
    let options = Options {
        parse: ParseOptions {
            constructs: Constructs {
                frontmatter: true,
                gfm_footnote_definition: true,
                gfm_label_start_footnote: true,
                gfm_strikethrough: true,
                gfm_table: true,
                gfm_task_list_item: true,
                ..Constructs::default()
            },
            ..ParseOptions::default()
        },
        ..Options::default()
    };

    let (events, parse_state) = parser::parse(value, &options.parse).unwrap();

    Ok::<(String, HashMap<String, String>), (String, HashMap<String, String>)>(to_html::compile(
        &events,
        parse_state.bytes,
        &options.compile,
    ))
    .unwrap()
}

fn write_html(
    directory_path: &String,
    md_path: &String,
    language: &String,
    html: String,
    frontmatter: &HashMap<String, String>,
    category: String,
) {
    // Make directories recursively in '_build' directory
    fs::create_dir_all(&directory_path).unwrap();

    // Write html file from the template file
    let template =
        fs::read_to_string("./_template/entry_".to_string() + &language + ".html").unwrap();

    let category_html = if category.is_empty() {
        "".to_string()
    } else {
        category.clone()
    };

    let html = template
        .replace(
            "{{title}}",
            &frontmatter.get("title").unwrap_or(&"".to_string()),
        )
        .replace(
            "{{subtitle}}",
            &frontmatter.get("subtitle").unwrap_or(&"".to_string()),
        )
        .replace(
            "{{created_at}}",
            &frontmatter.get("created_at").unwrap_or(&"".to_string()),
        )
        .replace(
            "{{updated_at}}",
            &frontmatter.get("updated_at").unwrap_or(&"".to_string()),
        )
        .replace("{{language}}", &language)
        .replace("{{content}}", &html)
        .replace("{{category}}", &category_html)
        .replace("{{md_path}}", &md_path[2..]); // remove './' from the path

    fs::write(directory_path.clone() + "/index.html", html).unwrap();
}

fn main() {
    let mut directory_queue: Vec<(String, String)> = Vec::new();
    directory_queue.push(("./_wiki/en".to_string(), "en".to_string()));
    directory_queue.push(("./_wiki/ko".to_string(), "ko".to_string()));

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
    while let Some((directory_path, language)) = directory_queue.pop() {
        let index_path = directory_path.clone() + "/_index.md";

        let index_content = fs::read_to_string(&index_path).unwrap();
        let (html, frontmatter) = to_html(&index_content);

        let directory_pathbuf = PathBuf::from(directory_path.clone());
        let index_entry_filename = directory_pathbuf.file_name().unwrap().to_str().unwrap();
        let index_entry_directory = if index_entry_filename == language.as_str() {
            "./_build/".to_string() + language.as_str() + "/"
        } else {
            "./_build/".to_string() + language.as_str() + "/" + index_entry_filename + "/"
        };

        println!("index : {}", &index_path);

        let parent_directory = directory_pathbuf
            .parent()
            .unwrap()
            .file_name()
            .unwrap_or(OsStr::new(""))
            .to_str()
            .unwrap();
        let index_category = category_map
            .get(parent_directory)
            .map(String::as_str)
            .unwrap_or("")
            .to_string();
        let current_category = if index_entry_filename == language.as_str() {
            "".to_string() // Root directory
        } else if index_category == "" {
            "<a href=\"/".to_string()
                + index_entry_filename
                + "/\">"
                + frontmatter.get("title").unwrap_or(&"".to_string())
                + "</a>"
        } else {
            index_category.clone()
                + " &gt; <a href=\"/"
                + index_entry_filename
                + "/\">"
                + frontmatter.get("title").unwrap_or(&"".to_string())
                + "</a>"
        };

        write_html(
            &index_entry_directory,
            &index_path,
            &language,
            html,
            &frontmatter,
            index_category.clone(),
        );

        category_map.insert(index_entry_filename.to_string(), current_category.clone());

        let dir = fs::read_dir(&directory_path).unwrap();

        for dir_entry in dir {
            let path = dir_entry.unwrap().path();
            if path.is_dir() {
                directory_queue.push((path.to_str().unwrap().to_string(), language.clone()));
                continue;
            }
            if path.extension().unwrap() != "md" {
                continue;
            }
            if path.file_name().unwrap() == "_index.md" {
                continue;
            }

            println!("md : {}", path.to_str().unwrap());
            let entry_filename = path.file_stem().unwrap().to_str().unwrap();
            let entry_directory =
                "./_build/".to_string() + language.as_str() + "/" + entry_filename + "/";

            let content = fs::read_to_string(path.clone()).unwrap();
            let (html, frontmatter) = to_html(&content);
            write_html(
                &entry_directory,
                &path.to_str().unwrap().to_string(),
                &language,
                html,
                &frontmatter,
                current_category.clone(),
            );
        }
    }
}
