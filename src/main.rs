use nannou::image;
use nannou::image::{DynamicImage, GenericImageView};
use nannou::prelude::*;
use nannou::rand::{thread_rng, Rng};
use std::env;
use std::process::exit;

struct Model {
    image: DynamicImage,
    dvd_rect: Rect,
    dvd_vel: Vec2,
    m_pos: Option<Vec2>,
}

fn main() {
    let flag = env::args().nth(1).unwrap_or_default();
    if flag.starts_with("/c") || flag.starts_with("/p") {
        println!("Configuration menu and in-window preview are not implemented");
        exit(0);
    }
    nannou::app(model).update(update).run();
}

fn change_color(image: &DynamicImage) -> DynamicImage {
    image.huerotate(thread_rng().gen_range(120..240))
}

fn model(app: &App) -> Model {
    let primary_window_id = app
        .new_window()
        .event(window_event)
        .view(view)
        .fullscreen()
        .build()
        .unwrap();

    let primary_window = app.window(primary_window_id).unwrap();
    primary_window.set_cursor_visible(false);

    let img_data = include_bytes!("../assets/dvd_logo.png");
    let image = change_color(&image::load_from_memory(img_data).unwrap().thumbnail(
        app.window_rect().w() as u32 / 7,
        app.window_rect().h() as u32 / 7,
    ));
    let rect = Rect::from_x_y_w_h(
        0.0,
        0.0,
        image.dimensions().0 as f32,
        image.dimensions().1 as f32,
    );

    Model {
        image,
        dvd_rect: rect,
        dvd_vel: Vec2::new(300.0, 300.0),
        m_pos: None,
    }
}

fn window_event(app: &App, model: &mut Model, event: WindowEvent) {
    if app.time > 0.1 {
        match event {
            WindowEvent::MouseMoved(pos) => {
                if model.m_pos.is_none() {
                    model.m_pos = Some(pos);
                }
                if model.m_pos.unwrap() != pos {
                    app.quit();
                }
            }
            WindowEvent::MousePressed(..)
            | WindowEvent::KeyPressed(..)
            | WindowEvent::MouseWheel(..) => app.quit(),
            _ => (),
        }
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let win = app.window_rect();
    let delta_time = app.duration.since_prev_update.secs() as f32;
    let dvd_vel = &mut model.dvd_vel;

    // Cap movement per frame to prevent overshooting
    let max_step = 10.0;
    let step_x = (dvd_vel.x * delta_time).clamp(-max_step, max_step);
    let step_y = (dvd_vel.y * delta_time).clamp(-max_step, max_step);

    model.dvd_rect = model.dvd_rect.shift_x(step_x).shift_y(step_y);

    // Collision detection and manual repositioning
    if model.dvd_rect.left() <= win.left() {
        dvd_vel.x = dvd_vel.x.abs(); // Move right
        model.dvd_rect = model.dvd_rect.shift_x(win.left() - model.dvd_rect.left());
        model.image = change_color(&model.image);
    } else if model.dvd_rect.right() >= win.right() {
        dvd_vel.x = -dvd_vel.x.abs(); // Move left
        model.dvd_rect = model.dvd_rect.shift_x(win.right() - model.dvd_rect.right());
        model.image = change_color(&model.image);
    }

    if model.dvd_rect.bottom() <= win.bottom() {
        dvd_vel.y = dvd_vel.y.abs(); // Move up
        model.dvd_rect = model.dvd_rect.shift_y(win.bottom() - model.dvd_rect.bottom());
        model.image = change_color(&model.image);
    } else if model.dvd_rect.top() >= win.top() {
        dvd_vel.y = -dvd_vel.y.abs(); // Move down
        model.dvd_rect = model.dvd_rect.shift_y(win.top() - model.dvd_rect.top());
        model.image = change_color(&model.image);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let texture = wgpu::Texture::from_image(app, &model.image);

    draw.texture(&texture)
        .xy(model.dvd_rect.xy())
        .wh(model.dvd_rect.wh());

    frame.clear(BLACK);
    draw.to_frame(app, &frame).unwrap();
}
