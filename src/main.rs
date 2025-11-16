use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "learning macroquad".to_owned(),
        window_height: 1200,
        window_width: 1800,
        fullscreen: false,
        ..Default::default()
    }
}

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
}

#[macroquad::main(window_conf)]
async fn main() {
    const MOVEMENT_SPEED: f32 = 200.0;
    rand::srand(miniquad::date::now() as u64);

    let mut squares: Vec<Shape> = vec![];
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };

    loop {
        let delta_time = get_frame_time();
        clear_background(DARKPURPLE);

        if is_key_down(KeyCode::Right) {
            circle.x += circle.speed * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            circle.x -= circle.speed * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            circle.y += circle.speed * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            circle.y -= circle.speed * delta_time;
        }

        circle.x = clamp(circle.x, 0.0, screen_width());
        circle.y = clamp(circle.y, 0.0, screen_height());

        draw_circle(circle.x, circle.y, circle.size, YELLOW);

        next_frame().await
    }
}
