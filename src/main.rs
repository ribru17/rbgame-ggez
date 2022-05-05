use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, set_window_title, Text, DrawParam, Font, PxScale, Image, screen_coordinates};
use ggez::event::{self, EventHandler};
use ggez::input::{keyboard};
// use ggez::filesystem;
use ggez::timer::fps;
use std::path;
// use ggez::nalgebra::{Point2};
// use nalgebra::{Point2};

const PLAYER_SPEED: f32 = 10.0;
const LASER_SPEED: f32 = 50.0;

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("Cool Game", "RB Games")
        .add_resource_path(path::PathBuf::from("./resources"))
        .build()
        .expect("couldn't create ggez context");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);
    set_window_title(&ctx, "COOLGAME");
    // Run!
    event::run(ctx, event_loop, my_game);
}

struct Laser {
    pub x: f32,
    pub y: f32,
    pub right_facing: bool,
}

impl Laser {
    pub fn new(x: f32, y: f32, right_facing: bool) -> Laser {
        Laser {
            x: x,
            y: y,
            right_facing: right_facing
        }
    }
}

struct MyGame {
    pub score : i32,
    pub keydown : bool,
    pub x: f32,
    pub y: f32,
    pub player: Image,
    pub laser: Image,
    pub laser_positions: Vec<Laser>,
    // pub shot_ago: u16,
    pub right_facing: bool,
    // Your state here...
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            score: 0,
            keydown: false,
            x: 50.0,
            y: 50.0,
            player: Image::new(ctx, "/goodpixelguy.png").expect("expected player"),
            laser: Image::solid(ctx, 20, Color::RED).expect("expected laser"),
            // shot_ago: 0,
            laser_positions: Vec::new(),
            right_facing: true,
            // ...
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        self.laser_positions.retain(|laser| laser.x > 0.0 && laser.x < screen_coordinates(ctx).w);
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::F) && !self.keydown {
            self.score += 1;
            self.laser_positions.push(Laser::new(self.x, self.y + self.player.height() as f32 / 6.0, self.right_facing));
            self.keydown = true;
        };
        if !keyboard::is_key_pressed(ctx, keyboard::KeyCode::F) {
            self.keydown = false;
        }
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::D) {
            if self.x + self.player.width() as f32 / 4.0 < screen_coordinates(ctx).w {
                self.x += PLAYER_SPEED;
            }
            self.right_facing = true;
        }
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::A) {
            if self.x - self.player.width() as f32 / 4.0 > 0.0 {
                self.x -= PLAYER_SPEED;
            }
            self.right_facing = false;
        }
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::W) {
            if self.y > 0.0 {
                self.y -= PLAYER_SPEED;
            }
        }
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::S) {
            if self.y + self.player.height() as f32 / 2.0 < screen_coordinates(ctx).h {
                self.y += PLAYER_SPEED;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::new(0.5, 0.0, 0.5, 0.0));
        // Draw code here...
        render_guy(ctx, self.x, self.y, &self.player, self.right_facing);
        render_score(ctx, self.score);
        for i in 0..self.laser_positions.len() {
            render_laser(ctx, &self.laser, self.laser_positions[i].right_facing, &mut self.laser_positions, i);
        }
        graphics::present(ctx)
    }
}

fn render_score(ctx: &mut Context, score: i32) {
    let mut display = Text::new(format!(
        "Score: {} (FPS: {})",
        score,
        fps(ctx),
    ));
    display.set_font(Font::default(), PxScale::from(35.0));
    graphics::draw(
        ctx,
        &display,
        DrawParam::default().color(Color::WHITE).dest([25.0, 25.0])
    ).unwrap();
}

fn render_guy(ctx: &mut Context, x: f32, y: f32, player: &Image, right_facing: bool) {
    // let image = Image::new(ctx, "/goodpixelguy.png").expect("expected image");
    let draw_params;
    if right_facing {
        draw_params = DrawParam::new().dest([x, y]).scale([-0.5, 0.5]).offset([0.5, 0.0]);
    } else {
        draw_params = DrawParam::new().dest([x, y]).scale([0.5, 0.5]).offset([0.5, 0.0]);
    }
    graphics::draw(ctx, player, draw_params).expect("expected render");
}

fn render_laser(ctx: &mut Context, laser: &Image, right_facing: bool, positions: &mut Vec<Laser>, id: usize) {
    let draw_params;
    if right_facing {
        draw_params = DrawParam::new().dest([positions[id].x + 40.0, positions[id].y]);
        positions[id].x += LASER_SPEED;
    } else {
        draw_params = DrawParam::new().dest([positions[id].x - 40.0, positions[id].y]);
        positions[id].x -= LASER_SPEED;
    }
    graphics::draw(ctx, laser, draw_params).expect("expected laser render");
}