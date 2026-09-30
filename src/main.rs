use crate::gui::PlayStatus::{Offline, Play, Update};

mod gui;
mod minecraft;
mod dir;

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    let ui = gui::App::new();
    eprintln!("start");
    ui.play_status(Play, true);
    ui.minecraft_run();
    ui.run()


}
