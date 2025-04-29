use cmdpal::icon::{IconData, IconInfo};
use cmdpal::*;
use windows::Win32;
use windows::Win32::System::Com::*;
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, MSG, TranslateMessage,
};
use windows::core::{Result, h};
use windows_core::{ComObject, Interface};

//255c6090-dbec-4008-b865-3f08765e727b
//0x255c6090_dbec_4008_b865_3f08765e727b
const EXTENSION_GUID: windows_core::GUID =
    windows_core::GUID::from_u128(0x255c6090_dbec_4008_b865_3f08765e727b);

const MD_CONTENT: &str = include_str!("./../README.md");

fn com_main() -> Result<()> {
    tracing::info!("Hello, world!");
    let md_box: ComObject<_> = cmdpal::content::markdown::MarkdownContent::new("".into()).into();
    let task_box = md_box.clone();
    // start a thread to update the content to current time
    let _handle = std::thread::spawn(move || unsafe {
        loop {
            let time = Win32::System::SystemInformation::GetLocalTime();
            let time = format!(
                "{}-{}-{} {}:{}:{}",
                time.wYear, time.wMonth, time.wDay, time.wHour, time.wMinute, time.wSecond
            );
            if let Ok(mut body) = task_box.body_mut() {
                *body = format!("# Current Time\n{}", time).into();
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;
        let global_options: IGlobalOptions =
            CoCreateInstance(&CLSID_GlobalOptions, None, CLSCTX_INPROC_SERVER)?;
        global_options.Set(COMGLB_RO_SETTINGS, COMGLB_FAST_RUNDOWN.0 as usize)?;
        let factory: IClassFactory = cmdpal::ext_factory::ExtensionClassFactory(
            cmdpal::ext::Extension {
                cmd_provider: cmdpal::provider::cmd::CommandProvider::new(
                    "BlueG.PEP",
                    "PEP Viewer",
                    Some(IconInfo::from(IconData::from(h!("\u{e8a5}").clone())).into()),
                    None,
                    true,
                    vec![
                        cmdpal::cmd_item::CommandItem::new(
                            Some(IconInfo::from(IconData::from(h!("\u{f6fa}").clone())).into()),
                            "View PEP",
                            "Open a PEP by number",
                            cmdpal::page::content::ContentPage::new(
                                vec![],
                                vec![
                                    cmdpal::content::markdown::MarkdownContent::new(
                                        MD_CONTENT.into(),
                                    )
                                    .into(),
                                    md_box.to_interface(),
                                ],
                                Some(
                                    cmdpal::details::Details::new(
                                        None,
                                        "Details Title!",
                                        "Details body!",
                                        vec![],
                                    )
                                    .into(),
                                ),
                                cmdpal::page::BasePage::new(
                                    "Search for PEP",
                                    false,
                                    None,
                                    cmdpal::cmd::BaseCommand::new("Page", "", None).into(),
                                )
                                .into(),
                            )
                            .into(),
                            vec![],
                        )
                        .into(),
                    ],
                    vec![],
                )
                .into(),
            }
            .into(),
        )
        .into();
        CoRegisterClassObject(
            &EXTENSION_GUID,
            &factory,
            CLSCTX_LOCAL_SERVER,
            REGCLS_MULTIPLEUSE | REGCLS_SUSPENDED,
        )?;
        CoResumeClassObjects()?;
        tracing::info!("Class object registered");
        let ext: crate::bindings::IExtension = CoCreateInstance(&EXTENSION_GUID, None, CLSCTX_ALL)?;
        tracing::info!("Extension created");
        let provider: crate::bindings::ICommandProvider = ext
            .GetProvider(crate::bindings::ProviderType::Commands)?
            .cast()?;
        tracing::info!("Provider created");
        let id = provider.Id()?;
        tracing::info!("Provider ID: {:?}", id);
        let mut msg: MSG = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            if TranslateMessage(&msg).as_bool() {
                DispatchMessageW(&msg);
            }
        }
    }
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
