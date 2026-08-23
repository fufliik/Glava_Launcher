use std::env;
use crate::gui::MainWindow;

use lyceris::minecraft::{
    config::ConfigBuilder,
    emitter::{Emitter, Event},
    install::{install},
    launch::launch,
};
use crate::gui;
pub async fn minecraft_run(mw_weak: slint::Weak<MainWindow>) -> Result<(), Box<dyn std::error::Error>> {
    let emitter = Emitter::default();



    emitter
        .on(
            Event::MultipleDownloadProgress,
            |(path, current, total): (String, u64, u64)| {
                println!("Downloading {} - {}/{}", path, current, total);
            },
        )
        .await;

    emitter
        .on(Event::Console, |line: String| {
            println!("Line: {}", line);
        })
        .await;

    let current_dir = env::current_dir()?;
    let config = ConfigBuilder::new(
        current_dir.join("game"),
        "1.21.4".into(),
        lyceris::auth::AuthMethod::Offline {
            username: "Lyceris".into(),
            // If none given, it will be generated.
            uuid: None,
        },
    )
        .build();

    install(&config, Some(&emitter)).await?;

    launch(&config, Some(&emitter)).await?.wait().await?;

    Ok(())
}