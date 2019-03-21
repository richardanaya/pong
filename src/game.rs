use std::collections::HashMap;
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

struct Player {
    paddle: Paddle,
}

struct Computer {
    paddle: Paddle,
}

struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
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
        self.player_update();
        self.computer_update();
        self.ball_update();
    }

    fn render(&mut self) {
        self.player_render();
        self.computer_render();
        self.ball_render();
    }

    pub fn clear(&self) {
        drawing::clear_rect(self.ctx, 0.0, 0.0, WIDTH, HEIGHT);
    }

    fn player_render(&self) {
        self.player.paddle.render(self.ctx);
    }
    fn player_update(&mut self) {
        if *self.keys.get(&37).unwrap_or(&false) {
            self.player.paddle.move_paddle(-4.0, 0.0);
        } else if *self.keys.get(&39).unwrap_or(&false) {
            self.player.paddle.move_paddle(4.0, 0.0);
        } else {
            self.player.paddle.move_paddle(0.0, 0.0);
        }
    }

    fn computer_render(&self) {
        self.computer.paddle.render(self.ctx);
    }

    fn computer_update(&mut self) {
        let computer = &mut self.computer;
        let x_pos = self.ball.x;
        let mut diff = -((computer.paddle.x + (computer.paddle.width / 2.0)) - x_pos);
        if diff < 0.0 && diff < -4.0 {
            diff = -5.0;
        } else if diff > 0.0 && diff > 4.0 {
            diff = 5.0;
        }
        computer.paddle.move_paddle(diff, 0.0);
        if computer.paddle.x < 0.0 {
            computer.paddle.x = 0.0;
        } else if computer.paddle.x + computer.paddle.width > 400.0 {
            computer.paddle.x = 400.0 - computer.paddle.width;
        }
    }

    fn ball_render(&self) {
        drawing::begin_path(self.ctx);
        drawing::arc(self.ctx, self.ball.x, self.ball.y, 5.0, 2.0 * std::f32::consts::PI, 0.0, 0);
        drawing::set_fill_style(self.ctx, "#FF0000");
        drawing::fill(self.ctx);
    }

    fn ball_update(&mut self) {
        let ball = &mut self.ball;
        ball.x += ball.vx;
        ball.y += ball.vy;
        let top_x = ball.x - 5.0;
        let top_y = ball.y - 5.0;
        let bottom_x = ball.x + 5.0;
        let bottom_y = ball.y + 5.0;

        if ball.x - 5.0 < 0.0 {
            ball.x = 5.0;
            ball.vx = -ball.vx;
        } else if ball.x + 5.0 > 400.0 {
            ball.x = 395.0;
            ball.vx = -ball.vx;
        }

        if ball.y < 0.0 || ball.y > 600.0 {
            ball.vx = 0.0;
            ball.vy = 3.0;
            ball.x = 200.0;
            ball.y = 300.0;
        }

        let paddle1 = &self.player.paddle;
        let paddle2 = &self.computer.paddle;
        if top_y > 300.0 {
            if top_y < (paddle1.y + paddle1.height)
                && bottom_y > paddle1.y
                && top_x < (paddle1.x + paddle1.width)
                && bottom_x > paddle1.x
            {
                ball.vy = -3.0;
                ball.vx += paddle1.vx / 2.0;
                ball.y += ball.vy;
            }
        } else {
            if top_y < (paddle2.y + paddle2.height)
                && bottom_y > paddle2.y
                && top_x < (paddle2.x + paddle2.width)
                && bottom_x > paddle2.x
            {
                ball.vy = 3.0;
                ball.vx += paddle2.vx / 2.0;
                ball.y += ball.vy;
            }
        }
    }
}
