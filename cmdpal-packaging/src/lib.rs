//! This module provides functions to make your extension installable and usable by the Command Palette.
//! You should invoke these functions in `build.rs` to generate the necessary files for your extension.
//! The generated files will be placed in the cargo artifacts directory, alongside the final binary.

const WINMD_NAME: &str = "Microsoft.CommandPalette.Extensions.winmd";
const WINMD_DATA: &[u8] = include_bytes!("Microsoft.CommandPalette.Extensions.winmd");

/// An workaround function to get the cargo final artifact directory.
///
/// Taken and modified from https://github.com/rust-lang/cargo/issues/9661#issuecomment-1812847609.
///
/// Should get replaced when https://github.com/rust-lang/cargo/issues/13663 lands.
fn get_cargo_artifact_dir() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let skip_triple = std::env::var("TARGET")? == std::env::var("HOST")?;
    let skip_parent_dirs = if skip_triple { 3 } else { 4 };

    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR")?);
    let mut current = out_dir.as_path();
    for _ in 0..skip_parent_dirs {
        current = current.parent().ok_or("not found")?;
    }

    Ok(std::path::PathBuf::from(current))
}

/// Generates the `Microsoft.CommandPalette.Extensions.winmd` file alongside the final binary.
/// This file is necessary for interoperability with the Command Palette.
pub fn generate_winmd() -> Result<(), Box<dyn std::error::Error>> {
    let artifact_dir = get_cargo_artifact_dir()?;
    let winmd_path = std::path::Path::new(&artifact_dir).join(WINMD_NAME);
    std::fs::write(&winmd_path, WINMD_DATA)?;
    Ok(())
}

pub struct AppxManifest {
    id: String,
    publisher_id: String,
    version: String,
    logo: String,
    display_name: String,
    publisher_display_name: String,
    description: String,
    executable: String,
    arguments: String,
    classes: Vec<(String, String)>, // (ClassId, DisplayName)
}

impl AppxManifest {
    pub fn generate_xml(&self) -> String {
        let com_classes: Vec<String> = self
            .classes
            .iter()
            .map(|(class_id, display)| {
                format!(
                    r#"<com:Class Id="{}" DisplayName="{}" />"#,
                    class_id, display
                )
            })
            .collect();
        let activation_classes: Vec<String> = self
            .classes
            .iter()
            .map(|(class_id, _)| format!(r#"<CreateInstance ClassId="{}" />"#, class_id))
            .collect();

        format!(
            r#"<?xml version="1.0" encoding="utf-8"?>

<Package
  xmlns="http://schemas.microsoft.com/appx/manifest/foundation/windows10"
  xmlns:uap="http://schemas.microsoft.com/appx/manifest/uap/windows10"
  xmlns:uap3="http://schemas.microsoft.com/appx/manifest/uap/windows10/3"
  xmlns:com="http://schemas.microsoft.com/appx/manifest/com/windows10"
  xmlns:rescap="http://schemas.microsoft.com/appx/manifest/foundation/windows10/restrictedcapabilities"
  IgnorableNamespaces="uap uap3 rescap">

  <Identity
    Name="{id}"
    Publisher="{publisher_id}"
    Version="{version}" />

  <Properties>
    <DisplayName>{display_name}</DisplayName>
    <PublisherDisplayName>{publisher_display_name}</PublisherDisplayName>
    <Logo>{logo}</Logo>
  </Properties>

  <Dependencies>
    <TargetDeviceFamily Name="Windows.Universal" MinVersion="10.0.17763.0" MaxVersionTested="10.0.19041.0" />
    <TargetDeviceFamily Name="Windows.Desktop" MinVersion="10.0.17763.0" MaxVersionTested="10.0.19041.0" />
  </Dependencies>

  <Applications>
    <Application Id="App"
      Executable="{executable}"
      EntryPoint="Windows.FullTrustApplication">
      <uap:VisualElements
        DisplayName="{display_name}"
        Description="{description}"
        BackgroundColor="transparent"
        Square150x150Logo="Assets\Square150x150Logo.png"
        Square44x44Logo="Assets\Square44x44Logo.png">
      </uap:VisualElements>
      <Extensions>
        <com:Extension Category="windows.comServer">
          <com:ComServer>
            <com:ExeServer Executable="{executable}" Arguments="{arguments}" DisplayName="{display_name}">
              {com_classes}
            </com:ExeServer>
          </com:ComServer>
        </com:Extension>
        <uap3:Extension Category="windows.appExtension">
          <uap3:AppExtension Name="com.microsoft.commandpalette"
            Id="PG-SP-ID"
            PublicFolder="Public"
            DisplayName="{display_name}"
            Description="{description}">
            <uap3:Properties>
              <CmdPalProvider>
                <Activation>
                  {activation_classes}
                </Activation>
                <SupportedInterfaces>
                  <Commands/>
                </SupportedInterfaces>
              </CmdPalProvider>
            </uap3:Properties>
          </uap3:AppExtension>
        </uap3:Extension>
      </Extensions>
    </Application>
  </Applications>

  <Capabilities>
    <rescap:Capability Name="runFullTrust" />
  </Capabilities>
</Package>
"#,
            id = self.id,
            publisher_id = self.publisher_id,
            display_name = self.display_name,
            publisher_display_name = self.publisher_display_name,
            version = self.version,
            description = self.description,
            logo = self.logo,
            executable = self.executable,
            arguments = self.arguments,
            com_classes = com_classes.join("\n"),
            activation_classes = activation_classes.join("\n"),
        )
    }

    pub fn write_xml(&self) -> Result<(), Box<dyn std::error::Error>> {
        let artifact_dir = get_cargo_artifact_dir()?;
        let manifest_path = artifact_dir.join("AppxManifest.xml");
        std::fs::write(&manifest_path, self.generate_xml())?;
        Ok(())
    }
}

#[derive(Default)]
pub struct AppxManifestBuilder {
    id: Option<String>,
    publisher_id: Option<String>,
    version: Option<String>,
    logo: Option<String>,
    display_name: Option<String>,
    publisher_display_name: Option<String>,
    description: Option<String>,
    executable: Option<String>,
    arguments: Option<String>,
    classes: Vec<(String, Option<String>)>, // (ClassId, DisplayName)
}

impl AppxManifestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn publisher_id(mut self, publisher_id: impl Into<String>) -> Self {
        self.publisher_id = Some(publisher_id.into());
        self
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn logo(mut self, logo: impl Into<String>) -> Self {
        self.logo = Some(logo.into());
        self
    }

    pub fn display_name(mut self, display_name: impl Into<String>) -> Self {
        self.display_name = Some(display_name.into());
        self
    }

    pub fn publisher_display_name(mut self, publisher_display_name: impl Into<String>) -> Self {
        self.publisher_display_name = Some(publisher_display_name.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn executable(mut self, executable: impl Into<String>) -> Self {
        self.executable = Some(executable.into());
        self
    }

    pub fn arguments(mut self, arguments: impl Into<String>) -> Self {
        self.arguments = Some(arguments.into());
        self
    }

    pub fn class(mut self, class_id: impl Into<String>, display_name: Option<&str>) -> Self {
        let display_name = display_name.map(|d| d.into());
        self.classes.push((class_id.into(), display_name));
        self
    }

    pub fn class_u128(self, class_id: u128, display_name: Option<&str>) -> Self {
        let class_id = format!(
            "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
            (class_id >> 96) as u32,
            (class_id >> 80) as u16,
            (class_id >> 64) as u16,
            (class_id >> 48) as u16,
            class_id & 0xFFFFFFFFFFFF
        );
        self.class(class_id, display_name)
    }

    fn infer_executable() -> String {
        std::env::var("CARGO_BIN_NAME").unwrap_or_else(|_| {
            println!("cargo::warning=CARGO_BIN_NAME is not set, using 'cmdpal-extension'");
            "cmdpal-extension".into()
        }) + ".exe"
    }

    fn infer_version() -> String {
        let version = std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| {
            println!("cargo::warning=CARGO_PKG_VERSION is not set, using '0.1.0' as base");
            "0.1.0".into()
        });

        version
            .split_once('-')
            .map_or_else(|| version.as_str(), |(v, _)| v)
            .split('.')
            .map(|s| s.to_string()) // handle cases like "0.1.0-alpha" or "1.2.3-beta"
            .collect::<Vec<String>>()
            .join(".")
            + ".0" // Convert x.x.x  to x.x.x.0
    }

    pub fn build(self) -> AppxManifest {
        let id = self.id.expect("id is required");
        let publisher_id = self.publisher_id.unwrap_or_else(|| {
            println!("cargo::warning=publisher_id is not set, using default 'CN=Unknown'");
            "CN=Unknown".into()
        });
        let version = self.version.unwrap_or_else(Self::infer_version);
        let logo = self.logo.unwrap_or("Assets\\StoreLogo.png".into());
        let display_name = self.display_name.unwrap_or_else(|| id.clone());
        let publisher_display_name = self.publisher_display_name.unwrap_or("Unknown".into());
        let description = self.description.unwrap_or_else(|| display_name.clone());
        let executable = self.executable.unwrap_or_else(Self::infer_executable);
        let arguments = self.arguments.unwrap_or("-RegisterAsComServer".into());
        let classes: Vec<(String, String)> = self
            .classes
            .into_iter()
            .map(|(class_id, display)| {
                let display = display.unwrap_or_else(|| display_name.clone());
                (class_id, display)
            })
            .collect();

        AppxManifest {
            id,
            publisher_id,
            version,
            logo,
            display_name,
            publisher_display_name,
            description,
            executable,
            arguments,
            classes,
        }
    }
}
