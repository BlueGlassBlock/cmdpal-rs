//! ```cargo
//! [package]
//! name = "update_winmd"
//! version = "0.1.0"
//! edition = "2024"
//! [dependencies]
//! reqwest = { version = "0.12", features = ["blocking", "json"] }
//! zip = "2.6"
//! windows-bindgen = "0.61"
//! ```
//! This script downloads the Microsoft.CommandPalette.Extensions.winmd from NuGet, and generates bindings for it.

use std::io::{Read, Write};
use reqwest::blocking::Client;
use zip::ZipArchive;
use std::collections::HashMap;
use windows_bindgen;

const PACKAGE_NAME: &str = "Microsoft.CommandPalette.Extensions";

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
    let nupkg = client.get(format!(
        "https://api.nuget.org/v3-flatcontainer/{}/{}/{}.{}.nupkg",
        PACKAGE_NAME.to_lowercase(),
        version,
        PACKAGE_NAME.to_lowercase(),
        version
    )).send()?.bytes()?;
    println!("Downloading {} version {}", PACKAGE_NAME, version);
    println!("Size: {} bytes", nupkg.len());
    let mut archive = ZipArchive::new(std::io::Cursor::new(nupkg))?;
    let mut buf = Vec::new();
    let mut file = std::fs::File::create("winmd/Microsoft.CommandPalette.Extensions.winmd")?;
    archive
        .by_name("winmd/Microsoft.CommandPalette.Extensions.winmd")?
        .read_to_end(&mut buf)?;
    file.write_all(&buf)?;
    file.flush()?;
    let metadata_dir = format!("{}\\System32\\WinMetadata", env!("WINDIR"));
    windows_bindgen::bindgen(
        [
            "--in",
            "winmd/Microsoft.CommandPalette.Extensions.winmd",
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
        ]
    );
    // Do a dirty replace: replace `pub trait` with `#[ambassador::delegateable_trait]\npub trait`
    let mut bindings = std::fs::read_to_string("src/bindings.rs")?;
    bindings = bindings.replace("pub trait", "#[ambassador::delegatable_trait]\npub trait");
    std::fs::write("src/bindings.rs", bindings)?;
    Ok(())
}