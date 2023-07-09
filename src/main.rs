mod onnurmark;

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::onnurmark::parser;
use crate::onnurmark::to_html;
use crate::onnurmark::{CompileResult, Constructs, Options, ParseOptions};

fn to_html(value: &str) -> Option<(String, CompileResult)> {
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

    Some(to_html::compile(
        &events,
        parse_state.bytes,
        &options.compile,
    ))
}

fn write_html(
    directory_path: &PathBuf,
    onm_path: &PathBuf,
    language: &String,
    html: String,
    frontmatter: &HashMap<String, String>,
    category: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    // Make directories recursively in '_build' directory
    fs::create_dir_all(&directory_path)?;

    // Write html file from the template file
    let template = fs::read_to_string("./_template/entry_".to_string() + &language + ".html")?;

    let final_html = template
        .replace(
            "{{title}}",
            &frontmatter.get("title").unwrap_or(&"Untitled".to_string()),
        )
        .replace(
            "{{subtitle}}",
            &frontmatter.get("subtitle").unwrap_or(&"".to_string()),
        )
        .replace(
            "{{created_at}}",
            &frontmatter
                .get("created_at")
                .unwrap_or(&"unknown".to_string()),
        )
        .replace(
            "{{updated_at}}",
            &frontmatter
                .get("updated_at")
                .unwrap_or(&"unknown".to_string()),
        )
        .replace("{{language}}", &language)
        .replace("{{content}}", &html)
        .replace("{{category}}", &category)
        .replace(
            "{{onm_path}}",
            onm_path
                .strip_prefix("./")
                .unwrap_or(onm_path)
                .to_str()
                .ok_or("Failed to convert file name to string.")?,
        );

    fs::write(directory_path.join("index.html"), final_html).unwrap();

    Ok(())
}

fn main() {
    let mut directory_queue: Vec<(PathBuf, String)> = Vec::new();

    for entry in fs::read_dir("./_wiki").unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            directory_queue.push((
                path.clone(),
                path.file_name().unwrap().to_str().unwrap().to_string(),
            ));
        }
    }

    // Create '_build' directory if not exists, and clear it.
    if fs::metadata("./_build").is_ok() {
        fs::remove_dir_all("./_build").unwrap();
    }
    fs::create_dir("./_build").unwrap();

    // Copy all files from '_static' into '_build'
    for entry in fs::read_dir("./_static").unwrap() {
        let path = entry.unwrap().path();
        fs::copy(
            &path,
            PathBuf::from("./_build/").join(&path.file_name().unwrap()),
        )
        .unwrap();
    }

    let mut category_map: HashMap<PathBuf, String> = HashMap::new();

    // Iterate over all files in the directory.
    while let Some((directory_pathbuf, language)) = directory_queue.pop() {
        let index_pathbuf = directory_pathbuf.join("_index.onm");

        let index_content = fs::read_to_string(&index_pathbuf).unwrap();
        let (html, compile_result) = to_html(&index_content).unwrap();

        let index_entry_filename = directory_pathbuf.file_name().unwrap().to_str().unwrap();
        let mut index_entry_directory = PathBuf::from("./_build/");
        index_entry_directory.push(&language);
        if index_entry_filename != language {
            index_entry_directory.push(&index_entry_filename);
        }

        println!("index : {}", &index_pathbuf.to_str().unwrap());

        let parent_directory = directory_pathbuf.parent().unwrap();
        let index_category = category_map
            .get(parent_directory)
            .unwrap_or(&"".to_string())
            .clone();
        let mut current_category = String::new();

        if index_entry_filename != language {
            if !index_category.is_empty() {
                current_category.push_str(&index_category);
                current_category.push_str(" &gt; ");
            }
            current_category.push_str("<a href=\"/");
            current_category.push_str(&language);
            current_category.push_str("/");
            current_category.push_str(index_entry_filename);
            current_category.push_str("/\">");
            current_category.push_str(
                &compile_result
                    .frontmatter
                    .get("title")
                    .unwrap_or(&"Untitled".to_string()),
            );
            current_category.push_str("</a>");
        }

        write_html(
            &index_entry_directory,
            &index_pathbuf,
            &language,
            html,
            &compile_result.frontmatter,
            &index_category,
        )
        .unwrap();

        category_map.insert(directory_pathbuf.clone(), current_category.clone());

        let dir = fs::read_dir(&directory_pathbuf).unwrap();

        for dir_entry in dir {
            let path = dir_entry.unwrap().path();
            if path.is_dir() {
                directory_queue.push((path, language.clone()));
                continue;
            }
            if path.extension().unwrap() != "onm" {
                continue;
            }
            if path.file_name().unwrap() == "_index.onm" {
                continue;
            }

            println!("onm : {}", path.to_str().unwrap());
            let entry_filename = path.file_stem().unwrap().to_str().unwrap();
            let entry_directory = PathBuf::from("./_build/")
                .join(&language)
                .join(entry_filename);

            let content = fs::read_to_string(&path).unwrap();
            let (html, compile_result) = to_html(&content).unwrap();
            write_html(
                &entry_directory,
                &path,
                &language,
                html,
                &compile_result.frontmatter,
                &current_category,
            )
            .unwrap();
        }
    }
}
