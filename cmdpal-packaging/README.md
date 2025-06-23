# cmdpal-packaging

A simple build-script helper crate for [`cmdpal-rs`](https://crates.io/crates/cmdpal).

## Explanation

Getting your home-baked (using Rust in this case) Command Palette extension to work with Windows is a bit of a hassle.

First, you have to obtain an `AppxManifest.xml` file to instruct the system to register your COM extension GUIDs,
and invoke your server.

At runtime, a `Microsoft.CommandPalette.Extensions.winmd` binary file is required to make Command Palette program communicate with your extension.

This crate provides helpers to generate the two files.

# Usage

To use this crate, add it to your `Cargo.toml`:

```toml
[build-dependencies]
cmdpal-packaging = "0.2"
```

Then, in your `build.rs`:

```rust
fn main() {
    cmdpal_packaging::generate_winmd().unwrap();
    cmdpal_packaging::AppxManifestBuilder::new()
        .id("YourName.YourExtension")
        .display_name("A Cool Extension")
        .publisher_display_name("CN=xxxxxxxx")
        .class_u128(GUID, None)
        .executable("your_cool_extension.exe")
        .build()
        .write_xml()
        .unwrap();
}
```

After building, you can run `Add-AppxPackage -Path ./AppxManifest.xml -Register` in PowerShell, under your extension's target directory,
to register your extension and test it out.

If you are getting rust-analyzer warnings, it is likely because the winmd file is occupied when the extension is running.
You can safely ignore errors in that case. 

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>