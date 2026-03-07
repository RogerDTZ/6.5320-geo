extern crate rand;

use egui_macroquad::macroquad;
use egui_macroquad::egui;
use macroquad::prelude::*;

use geo::point::Point;

struct Notification {
    text: String,
    timer: f32,
    color: egui::Color32,
}

impl Notification {
    fn new(text: String) -> Self {
        Self { text, timer: 2.0, egui::Color32::BLACK }
    }

    fn build(text: String, timer: f32, color: egui::Color) -> Self {
        Self { text, timer, color }
    }

    fn alpha(&self) -> f32 {
        self.timer.clamp(0.0, 1.0).sqrt()
    }
}

fn point_radius(n: usize) -> f32 {
    match n {
        0..100 => 6.0,
        100..1000 => 3.0,
        1000..10000 => 2.0,
        10000..100000 => 1.0,
        _ => 0.5,
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Closest Pair Demo".to_string(),
        window_width: 1280,
        window_height: 1280,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut points: Vec<Point> = Vec::new();
    let mut num_rand_points: usize = 10;
    let mut notification: Option<Notification> = None;
    let mut pointer_over_egui = false;
    let mut animated_run = false;

    loop {
        let dt = get_frame_time();
        clear_background(WHITE);

        if is_mouse_button_pressed(MouseButton::Left) && !pointer_over_egui {
            let (mx, my) = mouse_position();
            notification = Some(Notification::new(format!("Added point ({:.0}, {:.0})", mx, my)));
            points.push(Point::new(mx as f64, my as f64));
        }

        if screen_height() != 1280.0 || screen_width() != 1280.0 {
            request_new_screen_size(1280.0, 1280.0);
        }

        // Draw all points
        for point in &points {
            draw_circle(point.x as f32, point.y as f32, point_radius(points.len()), BLUE);
        }

        set_default_camera();

        egui_macroquad::ui(|egui_ctx| {
            egui_ctx.set_pixels_per_point(1.2);

            pointer_over_egui = egui_ctx.is_pointer_over_area();

            egui::Window::new("Controls")
                .anchor(egui::Align2::LEFT_TOP, egui::vec2(10.0, 10.0))
                .resizable(false)
                .show(egui_ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Clear points").clicked() {
                            points.clear();
                        }
                        ui.label(format!("Points placed: {}", points.len()));
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Random points").clicked() {
                            use rand::prelude::*;
                            points.clear();
                            points.reserve(num_rand_points);
                            let mut rng = rand::rng();
                            for _ in 0..num_rand_points {
                                let x = rng.random_range(0.0..screen_width());
                                let y = rng.random_range(0.0..screen_height());
                                points.push(Point::new(x as f64, y as f64));
                            }
                            notification = Some(Notification::new(format!("Added {} random points", num_rand_points)));
                        }
                        ui.add(egui::Slider::new(&mut num_rand_points, 1..=10000).text("Number of random points"));
                    });
                    ui.horizontal(|ui| {
                        ui.scope(|ui| {
                            ui.visuals_mut().widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(180, 60, 60);
                            ui.visuals_mut().widgets.hovered.weak_bg_fill  = egui::Color32::from_rgb(210, 80, 80);
                            ui.visuals_mut().widgets.active.weak_bg_fill   = egui::Color32::from_rgb(140, 40, 40);
                            if ui.button("Run").clicked() {

                            }
                        });
                        ui.checkbox(&mut animated_run, "Animated");
                    });
                });

            // Toast notification anchored to bottom-center
            if let Some(notif) = &mut notification {
                notif.timer -= dt;
                let alpha = notif.alpha();

                egui::Window::new("##notification")
                    .title_bar(false)
                    .resizable(false)
                    .collapsible(false)
                    .anchor(egui::Align2::CENTER_BOTTOM, egui::vec2(0.0, -20.0))
                    .frame(
                        egui::Frame::window(&egui_ctx.style())
                            .fill(notif.color.gamma_multiply(alpha))
                            .multiply_with_opacity(alpha),
                    )
                    .show(egui_ctx, |ui| {
                        ui.label(
                            egui::RichText::new(&notif.text)
                                .color(egui::Color32::WHITE.gamma_multiply(alpha)),
                        );
                    });

                if notif.timer <= 0.0 {
                    notification = None;
                }
            }
        });

        egui_macroquad::draw();
        next_frame().await;
    }
}
