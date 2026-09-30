/*use crate::minecraft;
slint::include_modules!();
pub struct App {
    pub ui: MainWindow,
}
pub enum PlayStatus{
    Play,
    Offline,
    Update,
    Loading,
}
impl PlayStatus {
    fn ui_data(self) -> (&'static str, slint::Color) {
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

        Self { ui }
    }

    pub fn set_status(&self, text: &str) {
            self.ui.global::<Logic>().set_button_play_status(text.into());
    }

    pub fn play_status(&self, status: PlayStatus) {
        let logic = self.ui.global::<Logic>();
        let color = self.ui.global::<Color>();

        let (text, color_value) = status.ui_data();

        logic.set_button_play_status(text.into());
        color.set_button_play(color_value);
    }

    pub fn play_enabled(&self, enabled: bool) {
        let akt = self.ui.global::<Enabled>();
        akt.set_button_play(enabled)
    }



    //не передаёт ui крашит процес токио
    pub fn minecraft_run(&self) {
        let logic = self.ui.global::<Logic>();
        let mw_weak = self.ui.as_weak(); //----
        logic.on_play_minecrat_process(move || {

            let mw_weak = mw_weak.clone(); //----
            tokio::spawn(async move {
                minecraft::minecraft_run(mw_weak).await;
            });
        });
    }

    pub fn run(&self) -> Result<(), slint::PlatformError> {
        self.ui.run()
    }*/