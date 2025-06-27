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
    let md_box: ComObject<_> = cmdpal::content::markdown::MarkdownContent::new("".into()).into();
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
                *body = format!("# Current Time\n{}", time).into();
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
    .title("Copy Sample Text") // TODO: The display text is determined by InvokableCommand's title?
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
                .icon(IconInfo::from(IconData::from(h!("\u{f6fa}").clone())).into())
                .id("BlueG.PEP.ExamplePage")
                .build(),
        )
        .loading(false)
        .title("PEP Example Page")
        .build(),
    )
    .details(
        cmdpal::details::DetailsBuilder::new()
            .title("Details Title")
            .body("Details Body")
            .build(),
    )
    .try_add_content(cmdpal::content::markdown::MarkdownContent::new(MD_CONTENT.into()).into())?
    .try_add_content(md_box.to_interface())?
    .try_add_command(
        CommandContextItemBuilder::new(copy_sample_item)
            .build()
            .to_interface(),
    )?
    .try_add_command(
        CommandContextItemBuilder::new(copy_time_item)
            .build()
            .to_interface(),
    )?
    .try_add_command(
        CommandContextItemBuilder::new(open_nonebot_dev_item)
            .build()
            .to_interface(),
    )?
    .try_add_command(
        CommandContextItemBuilder::new(reveal_file_item)
            .build()
            .to_interface(),
    )?
    .build();
    let provider = cmdpal::cmd_provider::CommandProviderBuilder::new()
        .id("BlueG.PEP")
        .display_name("PEP Viewer")
        .icon(IconInfo::from(IconData::from(h!("\u{e8a5}").clone())).into())
        .frozen(true)
        .add_top_level(
            cmdpal::cmd_item::CommandItemBuilder::try_new(cmd.to_interface())?
                .icon(IconInfo::from(IconData::from(h!("\u{f6fa}").clone())).into())
                .title("View PEP")
                .subtitle("Open a PEP by number")
                .build()
                .to_interface(),
        )
        .build();
    ExtRegistry::new()
        .register(EXTENSION_GUID, cmdpal::ext::Extension::from(&*provider))
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
