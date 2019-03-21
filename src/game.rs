use std::collections::HashMap;
use std::mem;
use web_dom::*;
const WIDTH: f32 = 400.0;
const HEIGHT: f32 = 600.0;

pub struct GameState {
    window: Element,
    ctx: DOMReference,
    request_animation_frame_listener: EventListener,
    key_down_listener: EventListener,
    key_up_listener: EventListener,
    keys: HashMap<i32, bool>,
    player: Player,
    computer: Computer,
    ball: Ball,
}

#[derive(Default)]
struct Paddle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    vx: f32,
    vy: f32,
}

impl Paddle {
    fn render(&self, ctx: DOMReference) {
        drawing::set_fill_style(ctx, "#0000FF");
        drawing::fill_rect(ctx, self.x, self.y, self.width, self.height);
    }
    fn move_paddle(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
        self.vx = x;
        self.vy = y;
        if self.x < 0.0 {
            self.x = 0.0;
            self.vx = 0.0;
        } else if self.x + self.width > 400.0 {
            self.x = 400.0 - self.width;
            self.vx = 0.0;
        }
    }
}

#[derive(Default)]
struct Player {
    paddle: Paddle,
}

impl Player {
    fn render(&self, ctx: DOMReference) {
        self.paddle.render(ctx);
    }
    fn update(&mut self, game: &GameState) {
        if *game.keys.get(&37).unwrap_or(&false) {
            self.paddle.move_paddle(-4.0, 0.0);
        } else if *game.keys.get(&39).unwrap_or(&false) {
            self.paddle.move_paddle(4.0, 0.0);
        } else {
            self.paddle.move_paddle(0.0, 0.0);
        }
    }
}

#[derive(Default)]
struct Computer {
    paddle: Paddle,
}

impl Computer {
    fn render(&self, ctx: DOMReference) {
        self.paddle.render(ctx);
    }

    fn update(&mut self, game: &GameState) {
        let x_pos = game.ball.x;
        let mut diff = -((self.paddle.x + (self.paddle.width / 2.0)) - x_pos);
        if diff < 0.0 && diff < -4.0 {
            diff = -5.0;
        } else if diff > 0.0 && diff > 4.0 {
            diff = 5.0;
        }
        self.paddle.move_paddle(diff, 0.0);
        if self.paddle.x < 0.0 {
            self.paddle.x = 0.0;
        } else if self.paddle.x + self.paddle.width > 400.0 {
            self.paddle.x = 400.0 - self.paddle.width;
        }
    }
}

#[derive(Default)]
struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

impl Ball {
    fn render(&self, ctx: DOMReference) {
        drawing::begin_path(ctx);
        drawing::arc(ctx, self.x, self.y, 5.0, 2.0 * std::f32::consts::PI, 0.0, 0);
        drawing::set_fill_style(ctx, "#FF0000");
        drawing::fill(ctx);
    }

    fn update(&mut self, game: &GameState) {
        self.x += self.vx;
        self.y += self.vy;
        let top_x = self.x - 5.0;
        let top_y = self.y - 5.0;
        let bottom_x = self.x + 5.0;
        let bottom_y = self.y + 5.0;

        if self.x - 5.0 < 0.0 {
            self.x = 5.0;
            self.vx = -self.vx;
        } else if self.x + 5.0 > 400.0 {
            self.x = 395.0;
            self.vx = -self.vx;
        }

        if self.y < 0.0 || self.y > 600.0 {
            self.vx = 0.0;
            self.vy = 3.0;
            self.x = 200.0;
            self.y = 300.0;
        }

        let paddle1 = &game.player.paddle;
        let paddle2 = &game.computer.paddle;
        if top_y > 300.0 {
            if top_y < (paddle1.y + paddle1.height)
                && bottom_y > paddle1.y
                && top_x < (paddle1.x + paddle1.width)
                && bottom_x > paddle1.x
            {
                self.vy = -3.0;
                self.vx += paddle1.vx / 2.0;
                self.y += self.vy;
            }
        } else {
            if top_y < (paddle2.y + paddle2.height)
                && bottom_y > paddle2.y
                && top_x < (paddle2.x + paddle2.width)
                && bottom_x > paddle2.x
            {
                self.vy = 3.0;
                self.vx += paddle2.vx / 2.0;
                self.y += self.vy;
            }
        }
    }
}

impl GameState {
    pub fn new() -> GameState {
        let win = window();
        let doc = window::get_document(win);
        let screen = element::query_selector(doc, "#screen");
        return GameState {
            window: win,
            ctx: htmlcanvas::get_context(screen, "2d"),
            request_animation_frame_listener: create_event_listener(),
            key_down_listener: create_event_listener(),
            key_up_listener: create_event_listener(),
            keys: HashMap::new(),
            player: Player {
                paddle: Paddle {
                    x: 175.0,
                    y: 580.0,
                    width: 50.0,
                    height: 10.0,
                    vx: 0.0,
                    vy: 0.0,
                },
            },
            computer: Computer {
                paddle: Paddle {
                    x: 175.0,
                    y: 10.0,
                    width: 50.0,
                    height: 10.0,
                    vx: 0.0,
                    vy: 0.0,
                },
            },
            ball: Ball {
                x: 200.0,
                y: 300.0,
                vx: 0.0,
                vy: 3.0,
            },
        };
    }

    pub fn init(&mut self) {
        window::request_animation_frame(self.window, self.request_animation_frame_listener);
        eventtarget::add_event_listener(self.window, "keydown", self.key_down_listener);
        eventtarget::add_event_listener(self.window, "keyup", self.key_up_listener);
    }

    pub fn route_event(&mut self, listener: EventListener, event: Event) {
        if listener == self.request_animation_frame_listener {
            self.run();
            window::request_animation_frame(self.window, self.request_animation_frame_listener);
        } else if listener == self.key_up_listener {
            self.keys.insert(keyboardevent::get_key_code(event), false);
        } else if listener == self.key_down_listener {
            self.keys.insert(keyboardevent::get_key_code(event), true);
        }
    }

    pub fn run(&mut self) {
        self.clear();
        self.update();
        self.render();
    }

    fn update(&mut self) {
        let mut player = Player::default();
        mem::swap(&mut self.player, &mut player);
        player.update(self);
        mem::swap(&mut self.player, &mut player);
        let mut computer = Computer::default();
        mem::swap(&mut self.computer, &mut computer);
        computer.update(self);
        mem::swap(&mut self.computer, &mut computer);
        let mut ball = Ball::default();
        mem::swap(&mut self.ball, &mut ball);
        ball.update(self);
        mem::swap(&mut self.ball, &mut ball);
    }

    fn render(&mut self) {
        self.player.render(self.ctx);
        self.computer.render(self.ctx);
        self.ball.render(self.ctx);
    }

    pub fn clear(&self) {
        drawing::clear_rect(self.ctx, 0.0, 0.0, WIDTH, HEIGHT);
    }
}
