extern crate rand;
use rand::prelude::*;

use egui_macroquad::macroquad;
use egui_macroquad::egui;
use macroquad::prelude::*;

use geo::point::Point;
use geo::closest_pair;

const SPACE_SIZE: i32 = 1000;

struct Notification {
    text: String,
    timer: f32,
    color: egui::Color32,
}

impl Notification {
    fn new(text: String) -> Self {
        Self { text, timer: 2.0, color: egui::Color32::from_hex("#101010").unwrap() }
    }

    fn build(text: String, timer: f32, color: egui::Color32) -> Self {
        Self { text, timer, color }
    }

    fn alpha(&self) -> f32 {
        self.timer.clamp(0.0, 1.0).sqrt()
    }
}

fn point_radius(n: usize) -> f32 {
    (100.0 / (n as f32).sqrt()).clamp(0.5, 5.0)
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Closest Pair Demo".to_string(),
        window_width: SPACE_SIZE,
        window_height: SPACE_SIZE,
        window_resizable: false,
        high_dpi: true,
        ..Default::default()
    }
}

fn get_camera() -> Camera2D {
    let size = SPACE_SIZE as f32;
    let margin = 100.0;
    Camera2D::from_display_rect(Rect::new(-margin, size + margin * 0.5, size + margin * 2.0, -size - margin * 2.0))
}

fn draw_canvas_rect() {
    let size = SPACE_SIZE as f32;
    let margin = 5.0;
    draw_rectangle_lines(-margin, -margin, size + margin * 2.0, size + margin * 2.0, 5.0, BLACK);
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut camera = get_camera();
    let mut points: Vec<Point> = Vec::new();
    let mut num_rand_points: usize = 100;
    let mut notification: Option<Notification> = None;
    let mut pointer_over_egui = false;
    let mut animated_run = false;
    let mut result : Option<closest_pair::Pair> = None;
    let mut x_var_rng = rand::rng();

    loop {
        let dt = get_frame_time();
        clear_background(WHITE);

        if is_mouse_button_pressed(MouseButton::Left) && !pointer_over_egui {
            let (mx, my) = mouse_position();
            notification = Some(Notification::new(format!("Added point ({:.0}, {:.0})", mx, my)));
            let wp = camera.screen_to_world(Vec2::new(mx, my));
            points.push(Point::new(wp.x as f64 + x_var_rng.random_range(-1e-5..1e-5), wp.y as f64));
        }

        if screen_height() != SPACE_SIZE as f32 || screen_width() != SPACE_SIZE as f32 {
            request_new_screen_size(SPACE_SIZE as f32, SPACE_SIZE as f32);
        }

        let speed = 200.0 * get_frame_time();
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up)    { camera.target.y -= speed; }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down)  { camera.target.y += speed; }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left)  { camera.target.x -= speed; }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) { camera.target.x += speed; }

        set_camera(&camera);
        draw_canvas_rect();

        for point in &points {
            let color = if result.as_ref().is_some_and(|pair| pair.0 == *point || pair.1 == *point) { RED } else { BLACK };
            draw_circle(point.x as f32, point.y as f32, point_radius(points.len()), color);
        }
        if let Some(pair)  = &result {
            draw_line(pair.0.x as f32, pair.0.y as f32, pair.1.x as f32, pair.1.y as f32, 3.0, RED);
        }

        set_default_camera();

        egui_macroquad::ui(|egui_ctx| {
            egui_ctx.set_pixels_per_point(1.5);

            pointer_over_egui = egui_ctx.is_pointer_over_area();

            egui::Window::new("Controls")
                .title_bar(false)
                .collapsible(false)
                .anchor(egui::Align2::LEFT_TOP, egui::vec2(10.0, 10.0))
                .resizable(false)
                .show(egui_ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Clear points").clicked() {
                            points.clear();
                            result = None;
                        }
                        ui.label(format!("Points placed: {}", points.len()));
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Random points").clicked() {
                            points.clear();
                            result = None;
                            points.reserve(num_rand_points);
                            let mut rng = rand::rng();
                            for _ in 0..num_rand_points {
                                let x = rng.random_range(0.0..SPACE_SIZE as f64);
                                let y = rng.random_range(0.0..SPACE_SIZE as f64);
                                points.push(Point::new(x, y));
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
                                match closest_pair::closest_pair(points.clone()) {
                                    Ok(pair) => {
                                        let pair = pair.unwrap();
                                        result = Some(pair);
                                        notification = Some(Notification::build(
                                            format!("Closest pair: ({:.2}, {:.2}) and ({:.2}, {:.2}), distance {:.4}",
                                                pair.0.x, pair.0.y, pair.1.x, pair.1.y, pair.dist()),
                                            5.0,
                                            egui::Color32::from_hex("#00591b").unwrap(),
                                        ));
                                    }
                                    Err(e) => {
                                        notification = Some(Notification::build(
                                            format!("Error: {}", e),
                                            5.0,
                                            egui::Color32::from_hex("#840000").unwrap(),
                                        ));
                                    }
                                }
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
                    .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-10.0, 10.0))
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
