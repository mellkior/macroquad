use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "learning macroquad".to_owned(),
        window_height: 600,
        window_width: 800,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        // clear_background(GREEN);
        next_frame().await
    }
}
