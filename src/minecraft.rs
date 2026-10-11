use crate::gui::MainWindow;
use slint::Weak;
use crate::gui::App;
use crate::auth::User;
use crate::dir;
use lyceris::minecraft::loader::neoforge::NeoForge;
//use lyceris::minecraft::config::Memory;
use lyceris::minecraft::{
    config::ConfigBuilder,
    emitter::{Emitter, Event},
    install::install,
    launch::launch,
};
use crate::gui::PlayStatus;

pub async fn minecraft_run(ui_weak: Weak<MainWindow>) -> Result<(), Box<dyn std::error::Error>> {
    let emitter = Emitter::default();
    //загрузка w1 под вопросом можно зделать в гуи чтобы вызывать при зрагрузки модов как перечисление 
    let w1 = ui_weak.clone();
    let _ = App::play_status(w1.clone(), PlayStatus::Game, false);

    emitter
        .on(
            Event::MultipleDownloadProgress,
            move |(path, current, total): (String, u64, u64)| {
                let percent = (current as f64 / total as f64) * 100.0;
                let res = format!("{:.1} / 100 %", percent);
                println!("Downloading {} - {}/{}", path, current, total);
                let _ = App::emitter_mdp_minecraft(w1.clone(), res);
                let _ = App::hotbar_enabled(w1.clone(), true);



            },
        )
        .await;

    let w2 = ui_weak.clone();
    emitter
        .on(Event::Console, move |line: String| {
            println!("Line: {}", line);
            let _ = App::emitter_console_minecraft(w2.clone(), line);
        })
        .await;

    let dir = dir::launcher()?;
    let data = User::load()?;
    let config = ConfigBuilder::new(
        &dir,
        "1.21.1".into(),
        lyceris::auth::AuthMethod::Offline {
            username: data.username,
            uuid: None,
        },
    )
        //.memory(Memory::Gigabyte(data.memory))
        .loader(NeoForge("21.1.233".to_string()).into())
        .build();

    install(&config, Some(&emitter)).await?;
    let _ = App::hotbar_enabled(ui_weak.clone(), false);
    launch(&config, Some(&emitter)).await?.wait().await?;

    Ok(())
}