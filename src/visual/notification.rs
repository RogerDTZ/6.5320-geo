use egui_macroquad::egui;

struct Notification {
    pub text: String,
    pub color: egui::Color32,
    pub timer: Option<f32>,
}

impl Notification {
    fn default_color() -> egui::Color32 {
        egui::Color32::from_hex("#FFFFFF").unwrap()
    }

    pub fn set_timer(&mut self, timer: f32) -> &Self {
        self.timer = Some(timer);
        self
    }

    pub fn update_timer(&mut self, dt: f32) {
        self.timer = self.timer.map(|t| (t - dt).max(0.0));
    }

    pub fn visible(&self) -> bool {
        self.timer.is_none_or(|t| t > 0.0)
    }
}

impl From<String> for Notification {
    fn from(text: String) -> Self {
        Self { text, color: Self::default_color(), timer: None }
    }
}
