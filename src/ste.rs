use std::collections::VecDeque;
use std::io;

use bracket_lib::prelude::{BTerm, GameState, VirtualKeyCode};

use crate::{
    obs,
    plr,
    sc,
    FRAME_DURATION,
    OBSTACLE_GAP,
    POSITION_OFFSET,
    SCORE_FILE_PATH,
    SCREEN_HEIGHT,
    SCREEN_WIDTH,
    HIGHLIGHT_DURATION,
};
use crate::palette;

enum GameMode {
    Menu,
    Playing,
    End,
}

pub(crate) struct State {
    player: plr::Player,
    frame_time: f32,
    mode: GameMode,
    obstacles: VecDeque<obs::Obstacle>,
    score: i32,
    history_score: Option<io::Result<sc::ScoreInfo>>,
    highlight_countdown: f32,
}

impl State {
    pub(crate) fn new() -> Self {
        State {
            player: plr::Player::new(POSITION_OFFSET, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
            obstacles: VecDeque::new(),
            score: 0,
            history_score: None,
            highlight_countdown: HIGHLIGHT_DURATION,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        let background_color: palette::Cor = if self.highlight_countdown > 0.0 {
            self.highlight_countdown -= ctx.frame_time_ms;
            blend_colors(
                palette::BACKGROUND,
                palette::BACKGROUND_HIGHLIGHT,
                self.highlight_countdown / HIGHLIGHT_DURATION,
            )
        } else { palette::BACKGROUND };

        ctx.cls_bg(background_color);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Space => self.player.flap(),
                VirtualKeyCode::Q => self.dying(),
                _ => {}
            }
        }

        self.player.render(ctx);


        // 判断
        if let Some(first) = self.obstacles.front_mut() {
            if self.player.x == first.x && !first.passed {
                self.score += 1;
                self.highlight_countdown = HIGHLIGHT_DURATION;
                first.passed = true;
            }

            if self.player.x > first.x + POSITION_OFFSET && first.passed {
                self.obstacles.pop_front();
                if let Some(last) = self.obstacles.back() {
                    self.obstacles.push_back(obs::Obstacle::new(last.x + OBSTACLE_GAP, self.score));
                }
            }
        }

        // 渲染
        for obstacle in &self.obstacles {
            obstacle.render(ctx, self.player.x);
        }

        // 判断
        if self.player.y > SCREEN_HEIGHT || self.obstacles.front().unwrap().hit_obstacle(&self.player) {
            self.dying();
        }

        ctx.print_color(0, 0, palette::TEXT, background_color, "Press Space to Flap");
        ctx.print_color(0, 1, palette::TEXT, background_color, &format!("Score: {}", self.score));
    }

    fn restart(&mut self) {
        self.player = plr::Player::new(POSITION_OFFSET, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.obstacles = (0..=SCREEN_WIDTH / OBSTACLE_GAP)
            .map(|i| obs::Obstacle::new(SCREEN_WIDTH + (i * OBSTACLE_GAP), 0))
            .collect();
        self.score = 0;
        self.highlight_countdown = HIGHLIGHT_DURATION;
    }

    fn dying(&mut self) {
        let highscore_manager = sc::HighScoreManager::new(SCORE_FILE_PATH);
        self.history_score = Some(highscore_manager.update_highscore(self.score));
        self.mode = GameMode::End;
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        let history_score = match &self.history_score {
            Some(Ok(score)) => {
                match score {
                    sc::ScoreInfo::NewRecord => "You set a new record!".to_string(),
                    sc::ScoreInfo::HistoryRecord(n) => format!("The highest score ever was {}", n)
                }
            }
            Some(Err(_)) => "(Unable to read the highest score in history)".to_string(),
            _ => { "(Unknown error)".to_string() }
        };
        ctx.cls_bg(palette::BACKGROUND);
        print_c(ctx, 5, "You are dead!");
        print_c(ctx, 6, &format!("You earned {} points", self.score));
        print_c(ctx, 7, &history_score);
        print_c(ctx, 9, "(P) Play Again");
        print_c(ctx, 10, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(palette::BACKGROUND);
        print_c(ctx, 5, "Welcome to Flappy Pixel");
        print_c(ctx, 8, "(P) Play Game");
        print_c(ctx, 9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx)
        };
    }
}

#[allow(dead_code)]
fn print(ctx: &mut BTerm, x: i32, y: i32, text: &str) {
    ctx.print_color(x, y, palette::TEXT, palette::TEXT_BG, text);
}

fn print_c(ctx: &mut BTerm, y: i32, text: &str) {
    ctx.print_color_centered(y, palette::TEXT, palette::TEXT_BG, text);
}

fn blend_colors(color1: (u8, u8, u8), color2: (u8, u8, u8), t: f32) -> (u8, u8, u8) {
    // 确保t在0.0到1.0之间
    let t = t.clamp(0.0, 1.0);

    let r = (color1.0 as f32 * (1.0 - t) + color2.0 as f32 * t) as u8;
    let g = (color1.1 as f32 * (1.0 - t) + color2.1 as f32 * t) as u8;
    let b = (color1.2 as f32 * (1.0 - t) + color2.2 as f32 * t) as u8;

    (r, g, b)
}