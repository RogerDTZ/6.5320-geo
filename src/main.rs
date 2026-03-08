extern crate rand;
use rand::prelude::*;

use egui_macroquad::macroquad;
use egui_macroquad::egui;
use macroquad::prelude::*;

use geo::point::Point;
use geo::closest_pair;
use geo::visual;

const SPACE: f32 = 1000.0;
const CANVAS_MARGIN: f32 = 50.0;
const CANVAS_MARGIN_TOP: f32 = 150.0;
const CANVAS_PADDING: f32 = 0.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Closest Pair Demo".to_string(),
        window_width: SPACE as i32,
        window_height: SPACE as i32,
        window_resizable: false,
        high_dpi: true,
        ..Default::default()
    }
}

fn get_view_camera() -> Camera2D {
    Camera2D::from_display_rect(Rect::new(-CANVAS_MARGIN, -CANVAS_MARGIN, SPACE + 2.0 * CANVAS_MARGIN, SPACE + CANVAS_MARGIN + CANVAS_MARGIN_TOP))
}

fn draw_canvas_rect() {
    draw_rectangle_lines(-CANVAS_PADDING, -CANVAS_PADDING, SPACE + CANVAS_PADDING * 2.0, SPACE + CANVAS_PADDING * 2.0, 5.0, BLACK);
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut pointer_over_egui = false;
    let mut x_var_rng = rand::rng();
    let camera = get_view_camera();

    let mut points: Vec<Point> = Vec::new();
    let mut num_rand_points: usize = 10;
    let mut animated = false;

    let mut player: Option<visual::Player> = None;
    let mut playback_speed = 1.0;
    let mut notif = visual::Notification::empty();

    loop {
        if screen_height() != SPACE || screen_width() != SPACE {
            request_new_screen_size(SPACE, SPACE);
        }
        clear_background(WHITE);

        // World positioning
        set_camera(&camera);

        // Input handling
        if is_mouse_button_pressed(MouseButton::Left) && !pointer_over_egui {
            let (mx, my) = mouse_position();
            let p = camera.screen_to_world(Vec2::new(mx, my));
            // Randomly perturb x coordinate
            points.push(Point::new(p.x as f64 + x_var_rng.random_range(-1e-5..1e-5), p.y as f64));
            player = None;
            notif.set(format!("Added point ({:.2}, {:.2})", p.x, p.y), Some(2.0), None);
        }
        if is_mouse_button_pressed(MouseButton::Right) && !pointer_over_egui {
            let (mx, my) = mouse_position();
            let p = camera.screen_to_world(Vec2::new(mx, my));
            if let Some(idx) = points.iter().position(|pt| {
                let dx = pt.x as f32 - p.x;
                let dy = pt.y as f32 - p.y;
                (dx * dx + dy * dy).sqrt() < 10.0
            }) {
                points.remove(idx);
                player = None;
                notif.set(format!("Removed point ({:.2}, {:.2})", p.x, p.y), Some(2.0), None);
            }
        }

        // Update
        let dt = get_frame_time();
        notif.update_timer(dt);

        // Drawing
        draw_canvas_rect();
        for point in &points {
            draw_circle(point.x as f32, point.y as f32, visual::adaptive::point_radius(points.len()), BLACK);
        }
        if let Some(ref mut player) = player {
            player.update(dt * playback_speed);
            player.get_shapes().iter().for_each(|shape| { shape.render(points.len(), SPACE) });
        }

        // UI positioning
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
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                if ui.button("Clear points").clicked() {
                                    points.clear();
                                    player = None;
                                    notif.set("Cleared all points".into(), Some(2.0), None);
                                }
                            });
                            ui.horizontal(|ui| {
                                if ui.button("Random points").clicked() {
                                    points.clear();
                                    points.reserve(num_rand_points);
                                    player = None;
                                    let mut rng = rand::rng();
                                    for _ in 0..num_rand_points {
                                        let x = rng.random_range(0.0..SPACE as f64);
                                        let y = rng.random_range(0.0..SPACE as f64);
                                        points.push(Point::new(x, y));
                                    }
                                    notif.set(format!("Added {} random points", num_rand_points), Some(2.0), None);
                                }
                                ui.add(egui::Slider::new(&mut num_rand_points, 10..=100000).text("Number of random points"));
                            });
                            ui.horizontal(|ui| {
                                ui.scope(|ui| {
                                    ui.visuals_mut().widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(180, 60, 60);
                                    ui.visuals_mut().widgets.hovered.weak_bg_fill  = egui::Color32::from_rgb(210, 80, 80);
                                    ui.visuals_mut().widgets.active.weak_bg_fill   = egui::Color32::from_rgb(140, 40, 40);
                                    if ui.button("Run").clicked() {
                                        let mut fman = visual::FrameManager::with_arena_capacity(points.len());
                                        let start_time = std::time::Instant::now();
                                        let result = closest_pair::closest_pair(points.clone(), &mut fman, animated);
                                        let elapsed = start_time.elapsed();
                                        match result {
                                            Ok(pair) => {
                                                player = Some(fman.into());
                                                notif.set(
                                                    format!(
                                                        "Algorithm finished in {:.2?}.\nClosest pair: {}, {}\nDistance: {:.2}",
                                                        elapsed,
                                                        pair.0,
                                                        pair.1,
                                                        pair.dist()
                                                    ),
                                                    Some(2.0),
                                                    Some(egui::Color32::GREEN)
                                                );
                                            }
                                            Err(e) => {
                                                player = None;
                                                println!("Error: {}", e);
                                                notif.set(format!("Error: {}", e), Some(2.0), Some(egui::Color32::RED));
                                            }
                                        }
                                    }
                                });
                                ui.checkbox(&mut animated, "Animated");
                                if points.len() > 1000 && animated {
                                    notif.set("Warning: Animation is slow with too many points, disabled".into(), Some(3.0), Some(egui::Color32::YELLOW));
                                    animated = false;
                                }
                                ui.add(egui::Slider::new(&mut playback_speed, 0.1..=10.0).text("Playback speed"));
                            });
                        });

                        ui.separator();

                        ui.vertical(|ui| {
                            ui.set_min_width(400.0);
                            ui.label(egui::RichText::new(format!("Points placed: {}", points.len())).color(egui::Color32::WHITE));
                            ui.label(egui::RichText::new(notif.text.clone()).color(if notif.visible() { notif.color } else { notif.inactive_color() }));
                        });
                    });
                });
        });

        egui_macroquad::draw();
        next_frame().await;
    }
}
