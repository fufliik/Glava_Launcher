mod gui;
mod minecraft;

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    let ui = gui::App::new();
    eprintln!("start");
    ui.play_status(gui::PlayStatus::Play);

    ui.minecraft_run();
    ui.run()

}
