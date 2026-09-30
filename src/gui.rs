use crate::minecraft;
use slint::Weak;
slint::include_modules!();
pub struct App {
    pub ui: MainWindow,
    pub ui_weak: Weak<MainWindow>,
}
pub enum PlayStatus{
    Play,
    Offline,
    Update,
    Loading,
}

impl PlayStatus{
    pub fn data(self) -> (&'static str, slint::Color) {
        match self {
            PlayStatus::Play => (
                "Играть",
                slint::Color::from_rgb_u8(35, 134, 54),
            ),
            PlayStatus::Offline => (
                "Оффлайн",
                slint::Color::from_rgb_u8(150, 150, 150),
            ),
            PlayStatus::Update => (
                "Обновить",
                slint::Color::from_rgb_u8(150, 150, 150),
            ),
            PlayStatus::Loading => (
                "Загрузка",
                slint::Color::from_rgb_u8(255, 99, 99),
            ),
        }
    }
}

impl App {
    pub fn new() -> Self {
        let ui = MainWindow::new().unwrap();
        let ui_weak = ui.as_weak();
        Self { ui, ui_weak }
    }
//global Logic
/*    pub fn set_button_play_status(&self, text: &str) {
            self.ui.global::<Logic>().set_button_play_status(text.into());
    }*/
    pub fn set_login_information_output(&self, text: &str) {
        self.ui.global::<Logic>().set_login_information_output(text.into());
    }
    pub fn play_status(&self, status: PlayStatus, enabled: bool) {
        let (output, color) = status.data();
        self.ui.global::<Logic>().set_button_play_status(output.into());
        self.ui.global::<Color>().set_button_play(color.into());
        self.ui.global::<Enabled>().set_button_play(enabled.into());
    }
    pub fn username(&self, username: &str){
        self.ui.global::<Logic>().set_username(username.into());
    }
    pub fn password(&self, password: &str){
        self.ui.global::<Logic>().set_password(password.into());
    }
/*
    //выдод в интерфейс с процент газрузки
    pub fn emitter_mdp_minecraft(ui_weak: self::Weak<MainWindow>, text: String ) {
        let _ =  slint::invoke_from_event_loop( move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.global::<Logic>().set_login_information_output(text.into());
            }
        });
    }*/
    /*
    pub fn emitter_console_minecraft(ui_weak: Weak<MainWindow>, text: String ) {
        let _ =  slint::invoke_from_event_loop( move || {})
    }*/

//global Enabled

    pub fn minecraft_run(&self){
        let ui_weak = self.ui.as_weak();
        self.ui.global::<Logic>().on_play_minecrat_process(move || {
            let ui_weak = ui_weak.clone();
            tokio::spawn(async move {
                let _ = minecraft::minecraft_run(ui_weak).await;
            });
        });
        self.ui.global::<Logic>().set_login_information_output("".into());
    }


//runtime
    pub fn run(&self) -> Result<(), slint::PlatformError> {
        self.ui.run()
    }
}