//! ```cargo
//! [package]
//! name = "update_winmd"
//! version = "0.1.0"
//! edition = "2024"
//! [dependencies]
//! reqwest = { version = "0.12", features = ["blocking", "json"] }
//! regex = "1.11"
//! zip = "4.2"
//! windows-bindgen = "0.61"
//! ```
//! This script downloads the Microsoft.CommandPalette.Extensions.winmd from NuGet, and generates bindings for it.

use reqwest::blocking::Client;
use std::collections::HashMap;
use std::io::{Read, Write};
use regex::Regex;
use windows_bindgen;
use zip::ZipArchive;

const PACKAGE_NAME: &str = "Microsoft.CommandPalette.Extensions";

fn document_bindings(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let lines = content.lines().map(str::to_string).collect::<Vec<_>>();

    let struct_re = Regex::new(r"^(\s*)(pub\s+)?struct\s+(\w+)\s*[({;]")?;
    let enum_re = Regex::new(r"^(\s*)(pub\s+)?enum\s+(\w+)\s*[{]")?;
    let impl_re = Regex::new(r"^(\s*)impl(?:\s+[^<\s]+)?\s+(\w+)\s*[{]")?;
    let field_re = Regex::new(r"^(\s*)(pub\s+)?(\w+)\s*:\s*[^,]+,?\s*$")?;
    let variant_re = Regex::new(r"^(\s*)(\w+)\s*[,{(]")?;
    let fn_re = Regex::new(r"^(\s*)(pub\s+)?fn\s+(\w+)\s*[<(]")?;
    let const_re = Regex::new(r"^(\s*)(pub\s+)?const\s+(\w+)\s*:\s*[^,]+,?\s*$")?;

    let mut output = Vec::new();
    let mut paths = Vec::new();
    let mut i = 0;
    let mut brace_stack = 0;

    while i < lines.len() {
        let line = &lines[i];

        brace_stack += line.matches('{').count();
        brace_stack -= line.matches('}').count();

        if line.contains("_") {
            output.push(line.clone());
            i += 1;
            continue; // Skip lines with underscores in their names
        }

        if let Some(caps) = struct_re.captures(line) {
            let indent = &caps[1];
            let name = &caps[3];
            let doc_path = format!("./bindings_docs/{}.md", name);

            if i == 0 || !lines[i - 1].contains("include_str!") {
                paths.push(doc_path.clone());
                output.push(format!("{indent}#[doc = include_str!(\"{doc_path}\")]"));
            }
            output.push(line.clone());
            i += 1;

            while i < lines.len() {
                let current = &lines[i];

                if let Some(field_caps) = field_re.captures(current) {
                    let f_indent = &field_caps[1];
                    let f_name = &field_caps[3];
                    let f_doc_path = format!("./bindings_docs/{}/{}.md", name, f_name);

                    if i == 0 || !lines[i - 1].contains(&f_doc_path) {
                        paths.push(f_doc_path.clone());
                        output.push(format!("{f_indent}#[doc = include_str!(\"{f_doc_path}\")]"));
                    }
                }

                if let Some(const_caps) = const_re.captures(current) {
                    let const_indent = &const_caps[1];
                    let const_name = &const_caps[3];
                    let const_doc_path = format!("./bindings_docs/{}/{}.md", name, const_name);

                    if i == 0 || !lines[i - 1].contains(&const_doc_path) {
                        paths.push(const_doc_path.clone());
                        output.push(format!(
                            "{const_indent}#[doc = include_str!(\"{const_doc_path}\")]"
                        ));
                    }
                }

                output.push(current.clone());
                i += 1;

                brace_stack += current.matches('{').count();
                brace_stack -= current.matches('}').count();
                if brace_stack == 0 {
                    break;
                }
            }
        } else if let Some(caps) = enum_re.captures(line) {
            let indent = &caps[1];
            let name = &caps[3];
            let doc_path = format!("./bindings_docs/{}.md", name);

            if i == 0 || !lines[i - 1].contains("include_str!") {
                paths.push(doc_path.clone());
                output.push(format!("{indent}#[doc = include_str!(\"{doc_path}\")]"));
            }

            output.push(line.clone());
            i += 1;

            while i < lines.len() {
                let current = &lines[i];

                if let Some(variant_caps) = variant_re.captures(current) {
                    let v_indent = &variant_caps[1];
                    let v_name = &variant_caps[2];
                    let v_doc_path = format!("./bindings_docs/{}/{}.md", name, v_name);

                    if i == 0 || !lines[i - 1].contains(&v_doc_path) {
                        paths.push(v_doc_path.clone());
                        output.push(format!("{v_indent}#[doc = include_str!(\"{v_doc_path}\")]"));
                    }
                }

                output.push(current.clone());
                i += 1;

                brace_stack += current.matches('{').count();
                brace_stack -= current.matches('}').count();
                if brace_stack == 0 {
                    break;
                }
            }
        } else if let Some(caps) = impl_re.captures(line) {
            let name = &caps[2];
            output.push(line.clone());
            i += 1;

            while i < lines.len() {
                let current = &lines[i];

                if let Some(fn_caps) = fn_re.captures(current) {
                    let fn_indent = &fn_caps[1];
                    let fn_name = &fn_caps[3];
                    let fn_doc_path = format!("./bindings_docs/{}/{}.md", name, fn_name);

                    if i == 0 || !lines[i - 1].contains(&fn_doc_path) {
                        paths.push(fn_doc_path.clone());
                        output.push(format!(
                            "{fn_indent}#[doc = include_str!(\"{fn_doc_path}\")]"
                        ));
                    }
                } else if let Some(const_caps) = const_re.captures(current) {
                    let const_indent = &const_caps[1];
                    let const_name = &const_caps[3];
                    let const_doc_path = format!("./bindings_docs/{}/{}.md", name, const_name);

                    if i == 0 || !lines[i - 1].contains(&const_doc_path) {
                        paths.push(const_doc_path.clone());
                        output.push(format!(
                            "{const_indent}#[doc = include_str!(\"{const_doc_path}\")]"
                        ));
                    }
                }

                output.push(current.clone());
                i += 1;

                brace_stack += current.matches('{').count();
                brace_stack -= current.matches('}').count();
                if brace_stack == 0 {
                    break;
                }
            }
        } else {
            output.push(line.clone());
            i += 1;
        }
    }
    for path in paths {
        // Add "src" before the path, then create the dir and file if it doesn't exist
        let full_path = format!("src/{}", path);
        let path = std::path::Path::new(&full_path);
        let dir = path.parent().ok_or("Failed to get parent directory")?;
        std::fs::create_dir_all(dir)?;
        if !path.exists() {
            std::fs::File::create(path)?;
        }
    }
    Ok(output.join("\n"))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let version = client
        .get(format!(
            "https://api.nuget.org/v3-flatcontainer/{}/index.json",
            PACKAGE_NAME.to_lowercase()
        ))
        .header("Accept", "application/json")
        .send()?
        .json::<HashMap<String, Vec<String>>>()?["versions"]
        .iter()
        .last()
        .ok_or("Failed to get the latest version")?
        .clone();
    let nupkg = client
        .get(format!(
            "https://api.nuget.org/v3-flatcontainer/{}/{}/{}.{}.nupkg",
            PACKAGE_NAME.to_lowercase(),
            version,
            PACKAGE_NAME.to_lowercase(),
            version
        ))
        .send()?
        .bytes()?;
    println!("Downloading {} version {}", PACKAGE_NAME, version);
    println!("Size: {} bytes", nupkg.len());
    let mut archive = ZipArchive::new(std::io::Cursor::new(nupkg))?;
    let mut buf = Vec::new();
    let mut file =
        std::fs::File::create("cmdpal-packaging/src/Microsoft.CommandPalette.Extensions.winmd")?;
    archive
        .by_name("winmd/Microsoft.CommandPalette.Extensions.winmd")?
        .read_to_end(&mut buf)?;
    file.write_all(&buf)?;
    file.flush()?;
    let metadata_dir = format!("{}\\System32\\WinMetadata", env!("WINDIR"));
    windows_bindgen::bindgen([
        "--in",
        "cmdpal-packaging/src/Microsoft.CommandPalette.Extensions.winmd",
        &metadata_dir,
        "--filter",
        "Microsoft.CommandPalette.Extensions",
        "--reference",
        "windows,skip-root,Windows.Foundation",
        "windows,skip-root,TypedEventHandler",
        "windows,skip-root,IClosable",
        "windows,skip-root,IRandomAccessStreamReference",
        "windows,skip-root,VirtualKeyModifiers",
        "windows,skip-root,Uri",
        "--out",
        "src/bindings.rs",
        "--flat",
        "--implement",
    ]);
    let mut bindings = std::fs::read_to_string("src/bindings.rs")?;
    bindings = String::from("//! Raw bindings for `Microsoft.CommandPalette.Extensions`\n") + &bindings;
    bindings = bindings.replace("windows_core::imp::define_interface!", "crate::_define_windows_core_interface_with_bindings_docs!");
    bindings = document_bindings(&bindings)?;
    std::fs::write("src/bindings.rs", bindings)?;
    Ok(())
}
