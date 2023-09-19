mod link;
use link::{Circle, Link};
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

struct Model {
    // window: Window,
    egui: Egui,
    angle: f32,
    link1: Link,
    link2: Link,
    link3: Link,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    Model {
        egui,
        angle: 0.0,
        link1: Link::from(vec2(-100.0, 0.0), vec2(-50.0, 0.0), 50.0),
        link2: Link::from(vec2(0.0, 0.0), vec2(50.0, 0.0), 150.0),
        link3: Link::from(vec2(50.0, 0.0), vec2(0.0, 0.0), 60.0),
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    egui.set_elapsed_time(update.since_start);

    let ctx = egui.begin_frame();

    egui::Window::new("Rum window").show(&ctx, |ui| {
        ui.label("res");
    });
    model.angle += 0.1 / PI;
    let angle_point = Link::angle_radius(model.link1.origin, model.angle, model.link1.radius);
      
    Link::link_to_point(angle_point, &mut model.link1.p2, &mut model.link2.origin);

    let intersection = model.link2.radius_intersection(&model.link3).1.expect("msg");

    Link::link_to_point(intersection, &mut model.link2.p2, &mut model.link3.p2)
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    model.link1.draw(&draw);
    model.link2.draw(&draw);
    model.link3.draw(&draw);

    let c2 = Circle::from(model.link2.origin, model.link2.radius);
    let c3 = Circle::from(model.link3.origin, model.link3.radius);

    // c2.draw(&draw);
    // c3.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
