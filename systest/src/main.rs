use cmdpal::prelude::WinResult as Result;
use cmdpal::prelude::*;
use windows::Win32;

//255c6090-dbec-4008-b865-3f08765e727b
//0x255c6090_dbec_4008_b865_3f08765e727b
const EXTENSION_GUID: windows_core::GUID =
    windows_core::GUID::from_u128(0x255c6090_dbec_4008_b865_3f08765e727b);

const MD_CONTENT: &str = include_str!("../../README.md");

fn com_main() -> Result<()> {
    tracing::info!("Hello, world!");

    let mut settings =
        JsonCommandSettings::new("D:/Projects/cmdpal/target/debug/settings.json".into());
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
            .add_choice("gpt-3.5-turbo".into())
            .add_choice("gpt-4".into())
            .add_choice("gpt-4o".into())
            .default("gpt-3.5-turbo".into())
            .caption("Model"),
    );

    let md_box: ComObject<_> = cmdpal::content::markdown::MarkdownContent::new("");
    let form_box = cmdpal::content::form::FormContentBuilder::new()
        .template_json(include_str!("./template.json"))
        .submit(|_, inputs, data| {
            tracing::info!("Form submitted with inputs: {}, data: {}", inputs, data);
            // Here you can process the form inputs and data
            // For now, just return KeepOpen to keep the form open
            Ok(CommandResult::KeepOpen)
        })
        .build();
    let task_box = md_box.clone();
    // start a thread to update the content to current time
    let _handle = std::thread::spawn(move || unsafe {
        loop {
            let time = Win32::System::SystemInformation::GetLocalTime();
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
                    token.lock().ok().map(|v| v.clone()).flatten(),
                    temperature.lock().ok().map(|v| v.clone()).flatten(),
                    toggle.lock().ok().map(|v| v.clone()).flatten(),
                    model.lock().ok().map(|v| v.clone()).flatten()
                )
                .into();
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
    let copy_sample_item: ComObject<CommandItem> = CommandItemBuilder::try_new(
        cmdpal::cmd::common::copy_text::CopyTextCommandBuilder::new(
            h!("This is a sample text to copy to clipboard").clone(),
        )
        .build()
        .to_interface(),
    )?
    .title("Copy Sample Text")
    .build();
    let copy_time_box = md_box.clone();
    let copy_time_item: ComObject<CommandItem> = CommandItemBuilder::try_new(
        cmdpal::cmd::common::copy_text::CopyTextCommandBuilder::new_dyn(Box::new(move || {
            if let Ok(body) = copy_time_box.body() {
                body.clone()
            } else {
                "Failed to get current time".into()
            }
        }))
        .build()
        .to_interface(),
    )?
    .title("Copy Current Time Markdown")
    .build();
    let open_nonebot_dev_item = CommandItemBuilder::try_new(
        cmdpal::cmd::common::open_url::OpenUrlCommandBuilder::new(
            "https://nonebot.dev".to_string(),
        )
        .build()
        .to_interface(),
    )?
    .title("Open nonebot.dev")
    .build();
    let reveal_file_item = CommandItemBuilder::try_new(
        cmdpal::cmd::common::reveal_file::RevealFileCommandBuilder::new(std::path::PathBuf::from(
            "D:/Projects/cmdpal/README.md",
        ))
        .build()
        .to_interface(),
    )?
    .title("Reveal README.md in Explorer")
    .build();
    let cmd = cmdpal::page::content::ContentPageBuilder::new(
        cmdpal::page::BasePageBuilder::new(
            cmdpal::cmd::BaseCommandBuilder::new()
                .name("Example Page")
                .icon(IconInfo::new(IconData::from(h!("\u{f6fa}").clone())))
                .id("BlueG.PEP.ExamplePage")
                .build(),
        )
        .loading(false)
        .title("PEP Example Page")
        .build(),
    )
    .details(
        DetailsBuilder::new()
            .title("Details Title")
            .body("Details Body")
            .build(),
    )
    .add_content(md_box)
    .add_content(MarkdownContent::new(MD_CONTENT))
    .add_content(form_box)
    .add_context_item(CommandContextItemBuilder::new(copy_sample_item).build())
    .add_context_item(CommandContextItemBuilder::new(copy_time_item).build())
    .add_context_item(CommandContextItemBuilder::new(open_nonebot_dev_item).build())
    .add_context_item(CommandContextItemBuilder::new(reveal_file_item).build())
    .build();
    let provider = CommandProviderBuilder::new()
        .id("BlueG.PEP")
        .display_name("PEP Viewer")
        .icon(IconInfo::new(IconData::from(h!("\u{e8a5}").clone())))
        .frozen(true)
        .add_top_level(
            CommandItemBuilder::try_new(cmd.to_interface())?
                .icon(IconInfo::new(IconData::from(h!("\u{f6fa}").clone())))
                .title("View PEP")
                .subtitle("Open a PEP by number")
                .build()
                .to_interface(),
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
    let file = tracing_appender::rolling::daily("D:/Projects/cmdpal/target/debug/", "cmdpal.log");
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
