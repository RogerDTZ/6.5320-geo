extern crate rand;
use rand::prelude::*;

use egui_macroquad::macroquad;
use egui_macroquad::egui;
use macroquad::prelude::*;

use geo::point::Point;
use geo::closest_pair;
use geo::visual;

const WINDOW_SIZE: i32 = 1000;
const SPACE: f32 = 1000.0;
const CANVAS_MARGIN: f32 = 50.0;
const CANVAS_MARGIN_TOP: f32 = 150.0;
const CANVAS_PADDING: f32 = 0.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Closest Pair Demo".to_string(),
        window_width: WINDOW_SIZE,
        window_height: WINDOW_SIZE,
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
    let mut num_rand_points: usize = 30;
    let mut animated = false;
    let mut fix_thr = 30.0;

    let mut player: Option<visual::Player> = None;
    let mut playback_speed = 1.0;
    let mut notif = visual::Notification::empty();

    loop {
        if screen_height() != WINDOW_SIZE as f32 || screen_width() != WINDOW_SIZE as f32 {
            request_new_screen_size(WINDOW_SIZE as f32, WINDOW_SIZE as f32);
        }
        clear_background(WHITE);

        // World positioning
        set_camera(&camera);

        // Input handling
        if is_mouse_button_pressed(MouseButton::Left) && !pointer_over_egui {
            let (mx, my) = mouse_position();
            let p = camera.screen_to_world(Vec2::new(mx, my));
            // Randomly perturb x coordinate
            // if x and y in range 0..SPACE
            let range = 0.0..=(SPACE as f32);
            if range.contains(&p.x) && range.contains(&p.y) {
                points.push(Point::new(p.x as f64 + x_var_rng.random_range(-1e-5..1e-5), p.y as f64));
                player = None;
                notif.set(format!("Added point ({:.2}, {:.2})", p.x, p.y), Some(2.0), None);
            }
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
            for shape in player.get_shapes() {
                shape.render(points.len(), SPACE);
            }
        }

        // UI positioning
        set_default_camera();
        egui_macroquad::ui(|egui_ctx| {
            egui_ctx.set_pixels_per_point(screen_dpi_scale());
            pointer_over_egui = egui_ctx.is_pointer_over_area();

            egui::Window::new("Controls")
                .title_bar(false)
                .collapsible(false)
                .anchor(egui::Align2::CENTER_TOP, egui::vec2(0.0, 10.0))
                .resizable(false)
                .show(egui_ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.set_min_width(500.0);
                            ui.horizontal(|ui| {
                                ui.scope(|ui| {
                                    ui.visuals_mut().widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(180, 60, 60);
                                    ui.visuals_mut().widgets.hovered.weak_bg_fill  = egui::Color32::from_rgb(210, 80, 80);
                                    ui.visuals_mut().widgets.active.weak_bg_fill   = egui::Color32::from_rgb(140, 40, 40);
                                    if ui.button("Run").clicked() {
                                        let mut fman = visual::FrameManager::with_arena_capacity(points.len().min(256));
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
                                let anim_enabled = points.len() <= 1000;
                                if !anim_enabled && animated {
                                    notif.set("Warning: Animation is slow with too many points, disabled".into(), Some(3.0), Some(egui::Color32::YELLOW));
                                    animated = false;
                                }
                                ui.add_enabled(anim_enabled, egui::Checkbox::new(&mut animated, "Animated"));
                                ui.add(egui::Slider::new(&mut playback_speed, 1.0..=1000.0).suffix("x").text("Playback speed").logarithmic(true));
                            });
                            if ui.button("Clear points").clicked() {
                                points.clear();
                                points.shrink_to_fit();
                                player = None;
                                notif.set("Cleared all points".into(), Some(2.0), None);
                            }
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
                                ui.add(egui::Slider::new(&mut num_rand_points, 10..=1000000).text("Number of random points").clamping(egui::SliderClamping::Never).logarithmic(true));
                            });
                            ui.add_enabled_ui(points.len() <= 3000,|ui| { ui.horizontal(|ui| {
                                if ui.button("Interesting Distribution").clicked() {
                                    let mut cnt = 0;
                                    while points.len() > 1 {
                                        let result = closest_pair::closest_pair(points.clone(), &mut visual::NoRecord, false).unwrap();
                                        if result.dist() < fix_thr {
                                            points.remove(points.iter().position(|p| p == &result.0).unwrap());
                                            cnt += 1;
                                        } else {
                                            break;
                                        }
                                    }
                                    if cnt > 0 {
                                        notif.set(format!("Removed {} points to create an interesting distribution", cnt), Some(3.0), None);
                                        player = None;
                                    } else {
                                        notif.set("No points removed, distribution is already interesting".into(), Some(3.0), None);
                                    }
                                }
                                ui.add(egui::Slider::new(&mut fix_thr, 10.0..=100.0).text("Distance threshold").clamping(egui::SliderClamping::Never));
                            })});
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
