use macroquad::{prelude::*, rand::ChooseRandom};

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    color: Color,
    collided: bool,
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
    let mut bullets: Vec<Shape> = vec![];

    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        color: YELLOW,
        collided: false,
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

            // Conditionally create bullet.
            if is_key_pressed(KeyCode::Space) {
                bullets.push(Shape {
                    x: circle.x,
                    y: circle.y,
                    speed: circle.speed * 2.0,
                    size: 5.0,
                    color: BLACK,
                    collided: false,
                });
            }

            // Generate a new square.
            if rand::gen_range(0, 99) >= 95 {
                let size = rand::gen_range(16.0, 64.0);
                squares.push(Shape {
                    size,
                    speed: rand::gen_range(50.0, 150.0),
                    x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                    y: -size,
                    color: *colors.choose().unwrap(),
                    collided: false,
                });
            }

            // Move squares.
            for square in &mut squares {
                square.y += square.speed * delta_time;
            }

            // Move bullets.
            for bullet in &mut bullets {
                bullet.y -= bullet.speed * delta_time;
            }

            // Remove squares below bottom of screen.
            squares.retain(|square| square.y < screen_height() + square.size);

            // Remove bullets off the screen.
            bullets.retain(|bullet| bullet.y > 0.0 - bullet.size / 2.0);

            // Remove collided bullets and squares.
            squares.retain(|square| !square.collided);
            bullets.retain(|bullet| !bullet.collided);

            // Check for collisions of circle and squres.
            if squares.iter().any(|square| circle.collides_with(square)) {
                game_over = true;
            }

            // Check for collisions of bullets and squares.
            for square in squares.iter_mut() {
                for bullet in bullets.iter_mut() {
                    if bullet.collides_with(square) {
                        bullet.collided = true;
                        square.collided = true;
                    }
                }
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
            if is_key_pressed(KeyCode::Enter) {
                squares.clear();
                bullets.clear();
                circle.x = screen_width() / 2.0;
                circle.y = screen_height() / 2.0;
                game_over = false;
            }
        }

        // Draw bullets.
        for bullet in &bullets {
            draw_circle(bullet.x, bullet.y, bullet.size / 2.0, bullet.color);
        }

        // Draw shapes.
        draw_circle(circle.x, circle.y, circle.size / 2.0, circle.color);
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
