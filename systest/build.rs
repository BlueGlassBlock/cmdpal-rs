fn main() {
    let manifest = cmdpal_packaging::AppxManifestBuilder::new()
        .class_u128(
            0x255c6090_dbec_4008_b865_3f08765e727b,
            Some("PEP Viewer"),
        )
        .id("BlueG.PEP")
        .build();
    manifest.write_xml().ok();
    cmdpal_packaging::generate_winmd().ok();
}
