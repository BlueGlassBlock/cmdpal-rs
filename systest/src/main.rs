use cmdpal::prelude::*;

//255c6090-dbec-4008-b865-3f08765e727b
//0x255c6090_dbec_4008_b865_3f08765e727b
const EXTENSION_GUID: windows_core::GUID =
    windows_core::GUID::from_u128(0x255c6090_dbec_4008_b865_3f08765e727b);

const MD_CONTENT: &str = include_str!("../../README.md");

fn com_main() -> Result<()> {
    tracing::info!("Hello, world!");

    let exe_path = std::env::current_exe()?;
    tracing::info!("Current exe path: {:?}", exe_path);

    let mut settings = JsonCommandSettings::new(exe_path.parent().unwrap().join("settings.json"));
    let token = settings.add_setting(
        TextSetting::new("llm-token")
            .placeholder("Bring Your Own Key")
            .caption("Token"),
    );
    let temperature = settings.add_setting(
        NumberSetting::new("llm-temperature")
            .default(0.7)
            .min(0.0)
            .max(1.0)
            .caption("Temperature"),
    );
    let toggle = settings.add_setting(
        ToggleSetting::new("llm-toggle")
            .default(true)
            .caption("Enable LLM"),
    );
    let model = settings.add_setting(
        ChoiceSetSetting::<&str>::new("llm-model")
            .add_choice("gpt-3.5-turbo")
            .add_choice("gpt-4")
            .add_choice("gpt-4o")
            .default("gpt-3.5-turbo")
            .caption("Model"),
    );

    let md_box = cmdpal::content::markdown::MarkdownContent::new("");
    let form_box = cmdpal::content::form::FormContent::builder()
        .template_json(include_str!("./template.json"))
        .submit(|_, inputs, data| {
            tracing::info!("Form submitted with inputs: {}, data: {}", inputs, data);
            // Here you can process the form inputs and data
            // For now, just return KeepOpen to keep the form open
            Ok(CommandResult::KeepOpen)
        });
    let task_box = md_box.clone();
    // start a thread to update the content to current time
    let _handle = std::thread::spawn(move || {
        loop {
            let time = unsafe { windows::Win32::System::SystemInformation::GetLocalTime() };
            let time = format!(
                "{}-{}-{} {:02}:{:02}:{:02}",
                time.wYear, time.wMonth, time.wDay, time.wHour, time.wMinute, time.wSecond
            );
            if let Ok(mut body) = task_box.body_mut() {
                *body = format!(
                    r#"
# Current Time
{}

# Configurations
- Token: {:?}
- Temperature: {:?}
- Toggle: {:?}
- Model: {:?}
"#,
                    time,
                    token.lock().ok().as_deref(),
                    temperature.lock().ok(),
                    toggle.lock().ok(),
                    model.lock().ok(),
                )
                .into();
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
    let copy_sample_item = cmdpal::cmd::common::copy_text::CopyTextCommand::new(
        h!("This is a sample text to copy to clipboard").clone(),
    )
    .item()?
    .title("Copy Sample Text");
    let copy_time_box = md_box.clone();
    let copy_time_item =
        cmdpal::cmd::common::copy_text::CopyTextCommand::new_dyn(Box::new(move || {
            if let Ok(body) = copy_time_box.body() {
                body.clone()
            } else {
                "Failed to get current time".into()
            }
        }))
        .item()?
        .title("Copy Current Time Markdown");
    let open_nonebot_dev_item =
        cmdpal::cmd::common::open_url::OpenUrlCommand::new("https://nonebot.dev")
            .item()?
            .title("Open nonebot.dev");
    let reveal_file_item = cmdpal::cmd::common::reveal_file::RevealFileCommand::new(
        exe_path
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("README.md"),
    )
    .item()?
    .title("Reveal README.md in Explorer");
    let cmd = cmdpal::cmd::BaseCommand::builder()
        .name("Example Page")
        .icon(IconInfo::new(IconData::from_glyph_and_family(
            "M",
            "Wingdings",
        ))) // M is a bomb in Wingdings
        .id("BlueG.PEP.ExamplePage")
        .page()
        .loading(false)
        .title("PEP Example Page")
        .content()
        .details(
            Details::builder()
                .title("Details Title")
                .body("Details Body"),
        )
        .add_content(md_box)
        .add_content(MarkdownContent::new(MD_CONTENT))
        .add_content(form_box)
        .add_context_item(copy_sample_item.context())
        .add_context_item(copy_time_item.context())
        .add_context_item(open_nonebot_dev_item.context())
        .add_context_item(reveal_file_item.context());

    let provider = CommandProvider::builder()
        .id("BlueG.PEP")
        .display_name("PEP Viewer")
        .icon(IconInfo::new(IconData::from(h!("\u{e8a5}").clone())))
        .frozen(true)
        .add_top_level(
            cmd.item()?
                .icon(IconInfo::new(IconData::from(h!("\u{f6fa}").clone())))
                .title("View PEP")
                .subtitle("Open a PEP by number"),
        )
        .settings(settings.into())
        .build();
    ExtRegistry::new()
        .register(EXTENSION_GUID, Extension::from(&*provider))
        .serve()?;
    tracing::info!("Exiting...");
    Ok(())
}

fn main() {
    use tracing_subscriber::prelude::*;
    let file = tracing_appender::rolling::daily(
        std::env::current_exe().unwrap().parent().unwrap(),
        "cmdpal.log",
    );
    let (non_blocking, _guard) = tracing_appender::non_blocking(file);
    // log to stdout and file
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_thread_ids(true)
                .with_line_number(true),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false)
                .with_line_number(true),
        )
        .init();
    
    com_main().unwrap_or_else(|e| {
        tracing::error!("Error: {:?}", e);
        std::process::exit(1);
    });
}
