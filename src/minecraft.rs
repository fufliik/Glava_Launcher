use crate::gui::{MainWindow, Logic};
use slint::{ComponentHandle, Weak, };
use crate::gui::App;
use crate::dir;
use lyceris::minecraft::loader::neoforge::NeoForge;
//use lyceris::minecraft::config::Memory;
use lyceris::minecraft::{
    config::ConfigBuilder,
    emitter::{Emitter, Event},
    install::install,
    launch::launch,
};
pub async fn minecraft_run(ui_weak: Weak<MainWindow>) -> Result<(), Box<dyn std::error::Error>> {
    let emitter = Emitter::default();

    //загрузка w1 под вопросом можно зделать в гуи чтобы вызывать при зрагрузки модов как перечисление 
    let w1 = ui_weak.clone();
    emitter
        .on(
            Event::MultipleDownloadProgress,
            move |(path, current, total): (String, u64, u64)| {
                let percent = (current as f64 / total as f64) * 100.0;
                let res = format!("{:.1} / 100 %", percent);
                println!("Downloading {} - {}/{}", path, current, total);
                let w1 = w1.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(w1) = w1.upgrade() {
                        w1.global::<Logic>().set_login_information_output(res.into());
                    }
                });
            },
        )
        .await;

    let w2 = ui_weak.clone();
    emitter
        .on(Event::Console, move |line: String| {
            println!("Line: {}", line);
            let  w2 =  w2.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(w2) =  w2.upgrade() {
                    w2.global::<Logic>().set_login_information_output(line.into());
                }
            });
        })
        .await;

    let dir = dir::launcher()?;
    let config = ConfigBuilder::new(
        &dir,
        "1.21.1".into(),
        lyceris::auth::AuthMethod::Offline {
            username: "Lyceris".into(),
            uuid: None,
        },
    )
        //.memory(Memory::Gigabyte(data.memory))
        .loader(NeoForge("21.1.233".to_string()).into())
        .build();

    install(&config, Some(&emitter)).await?;

    launch(&config, Some(&emitter)).await?.wait().await?;

    Ok(())
}