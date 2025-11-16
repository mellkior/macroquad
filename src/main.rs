use macroquad::{prelude::*, rand::ChooseRandom};

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    color: Color,
}

impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }
}

#[macroquad::main("learning macroquad")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 200.0;
    let colors: Vec<Color> = vec![RED, ORANGE, YELLOW, GREEN, BLUE];
    let mut game_over = false;

    rand::srand(miniquad::date::now() as u64);
    let mut squares = vec![];
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        color: YELLOW,
    };

    loop {
        clear_background(DARKPURPLE);

        if !game_over {
            let delta_time = get_frame_time();

            // Check for user input.
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

            // Clamp X and Y to be within the screen.
            circle.x = clamp(circle.x, 0.0, screen_width());
            circle.y = clamp(circle.y, 0.0, screen_height());

            // Generate a new square.
            if rand::gen_range(0, 99) >= 95 {
                let size = rand::gen_range(16.0, 64.0);
                squares.push(Shape {
                    size,
                    speed: rand::gen_range(50.0, 150.0),
                    x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                    y: -size,
                    color: *colors.choose().unwrap(),
                });
            }

            // Move squares.
            for square in &mut squares {
                square.y += square.speed * delta_time;
            }

            // Remove squares below bottom of screen.
            squares.retain(|square| square.y < screen_height() + square.size);

            // Check for collisions.
            if squares.iter().any(|square| circle.collides_with(square)) {
                game_over = true;
            }
        } else {
            // Display game over message.
            let text = "GAME OVER!";
            let text_dimensions = measure_text(text, None, 50, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dimensions.width / 2.0,
                screen_height() / 2.0,
                50.0,
                RED,
            );

            // Restart game.
            if is_key_pressed(KeyCode::Space) {
                squares.clear();
                circle.x = screen_width() / 2.0;
                circle.y = screen_height() / 2.0;
                game_over = false;
            }
        }

        // Draw shapes.
        draw_circle(circle.x, circle.y, circle.size / 2.0, YELLOW);
        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                square.color,
            );
        }

        next_frame().await
    }
}
