mod link;
use link::{Circle, Link};
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

struct Model {
    // window: Window,
    egui: Egui,
    angle: f32,
    show_radii: bool,
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
        show_radii: false,
        angle: 0.0,
        link1: Link::from(vec2(-100.0, -100.0), vec2(-50.0, 0.0), 70.0),
        link2: Link::from(vec2(0.0, 0.0), vec2(50.0, 0.0), 160.0),
        link3: Link::from(vec2(50.0, 0.0), vec2(0.0, 0.0), 200.0),
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    egui.set_elapsed_time(update.since_start);

    let ctx = egui.begin_frame();

    egui::Window::new("Rum window").show(&ctx, |ui| {
        ui.label("res");
        ui.add(egui::Slider::new(&mut model.link1.radius, 1.0..=800.0));
        ui.add(egui::Slider::new(&mut model.link2.radius, 1.0..=800.0));
        ui.add(egui::Slider::new(&mut model.link3.radius, 1.0..=800.0));
        ui.add(egui::Checkbox::new(&mut model.show_radii, "show radii?"))
    });
    model.angle += 0.1 / PI;
    model.link1.origin.x = model.angle.sin() * 100.0;

    let point_link = model.link1.radius_intersection(&model.link3);
    

    Link::link_from_intersection(point_link, &mut model.link1.p2, &mut model.link2.origin);
    Link::link_from_intersection(point_link, &mut model.link2.p2, &mut model.link3.p2);

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

    let c1 = Circle::from(model.link1.origin, model.link1.radius);
    let c2 = Circle::from(model.link2.origin, model.link2.radius);
    let c3 = Circle::from(model.link3.origin, model.link3.radius);

    if model.show_radii {
        c1.draw(&draw);
        c2.draw(&draw);
        c3.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
