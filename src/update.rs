use self_update::cargo_crate_version;

const REPO_OWNER: &str = "fufliik";
const REPO_NAME: &str = "Glava_Launcher";
const BIN_NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn check() -> Result<(), Box<dyn std::error::Error>> {
    let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner(REPO_OWNER)
        .repo_name(REPO_NAME)
        .build()?
        .fetch()?
        .with_current_version(VERSION);

    if releases.is_update_available()? {
        if let Some(latest) = releases.latest() {
            println!("Найдено новое обновление: {} -> {}", VERSION, latest.version());
        }
        run()?;
    }
    Ok(())
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner(REPO_OWNER)
        .repo_name(REPO_NAME)
        .bin_name(BIN_NAME)
        .show_download_progress(true)
        .show_output(false)
        .no_confirm(true)
        .target("x86_64-pc-windows-msvc")
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    println!("Обновлено до {}", status.version());
    Ok(())
}