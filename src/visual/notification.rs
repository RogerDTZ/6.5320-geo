use egui_macroquad::egui;

pub struct Notification {
    pub text: String,
    pub color: egui::Color32,
    timer: Option<f32>,
}

impl Notification {
    pub fn empty() -> Self {
        Self { text: String::new(), color: Self::default_color(), timer: None }
    }

    fn default_color() -> egui::Color32 {
        egui::Color32::WHITE
    }

    pub fn inactive_color(&self) -> egui::Color32 {
        let color = self.color;
        egui::Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 80)
    }

    pub fn set(&mut self, text: String, timer: Option<f32>, color: Option<egui::Color32>) -> &Self {
        self.text = text;
        self.timer = timer;
        self.color = color.unwrap_or_else(Self::default_color);
        self
    }

    pub fn update_timer(&mut self, dt: f32) {
        self.timer = self.timer.map(|t| (t - dt).max(0.0));
    }

    pub fn visible(&self) -> bool {
        !self.text.is_empty() && self.timer.is_none_or(|t| t > 0.0)
    }
}