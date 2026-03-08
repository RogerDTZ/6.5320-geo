extern crate rand;
use rand::prelude::*;

use egui_macroquad::macroquad;
use egui_macroquad::egui;
use macroquad::prelude::*;

use geo::point::Point;
use geo::closest_pair;

mod visual;

const SPACE: f32 = 1000.0;
const CANVAS_MARGIN: f32 = 50.0;
const CANVAS_MARGIN_TOP: f32 = 150.0;
const CANVAS_PADDING: f32 = 5.0;

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
    let mut num_rand_points: usize = 100;
    let mut animated_run = false;

    let mut result: Option<closest_pair::Pair> = None;
    let play: Option<visual::FrameManager> = None;

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
        }

        // Drawing
        draw_canvas_rect();

        for point in &points {
            let color = if result.as_ref().is_some_and(|pair| pair.0 == *point || pair.1 == *point) { RED } else { BLACK };
            draw_circle(point.x as f32, point.y as f32, visual::adaptive::point_radius(points.len()), color);
        }
        if let Some(pair)  = &result {
            draw_line(pair.0.x as f32, pair.0.y as f32, pair.1.x as f32, pair.1.y as f32, 3.0, RED);
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
                                let x = rng.random_range(0.0..SPACE as f64);
                                let y = rng.random_range(0.0..SPACE as f64);
                                points.push(Point::new(x, y));
                            }
                            // notification = Some(Notification::new(format!("Added {} random points", num_rand_points)));
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
                                        // notification = Some(Notification::build(
                                        //     format!("Closest pair: ({:.2}, {:.2}) and ({:.2}, {:.2}), distance {:.4}",
                                        //         pair.0.x, pair.0.y, pair.1.x, pair.1.y, pair.dist()),
                                        //     5.0,
                                        //     egui::Color32::from_hex("#00591b").unwrap(),
                                        // ));
                                    }
                                    Err(e) => {
                                        // notification = Some(Notification::build(
                                        //     format!("Error: {}", e),
                                        //     5.0,
                                        //     egui::Color32::from_hex("#840000").unwrap(),
                                        // ));
                                    }
                                }
                            }
                        });
                        ui.checkbox(&mut animated_run, "Animated");
                    });
                });
        });

        egui_macroquad::draw();
        next_frame().await;
    }
}
