use bracket_lib::prelude::*;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};

const SCORE_FILE_PATH: &str = "high_score.json";

enum GameMode {
    Menu,
    Playing,
    End,
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 50.0;
const POSITION_OFFSET: i32 = 5;
const OBSTACLE_GAP: i32 = 5;


struct Player {
    x: i32,
    y: i32,
    velocity: f32,
    time: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
            time: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(POSITION_OFFSET, self.y, YELLOW, BLACK, to_cp437('@'))
    }

    fn gravity_and_move(&mut self) {
        self.time += 0.1;

        self.velocity += 0.2; // 加速度影响速度
        self.y = (self.y as f32 + self.velocity * self.time + 0.5 * 0.2 * self.time.powi(2)) as i32;
        self.x += 1;

        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
        self.time = 0.0;
    }
}


struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
    obstacles: VecDeque<Obstacle>,
    score: i32,
    history_score: Option<io::Result<Score>>,
}

impl State {
    fn new() -> Self {
        // println!("{}", SCREEN_WIDTH / OBSTACLE_GAP + 1);
        // let obstacles: VecDeque<Obstacle> = (0..=SCREEN_WIDTH / OBSTACLE_GAP)
        //     .map(|i| Obstacle::new(SCREEN_WIDTH + (i * OBSTACLE_GAP), 0))
        //     .collect();
        State {
            player: Player::new(POSITION_OFFSET, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
            obstacles: VecDeque::new(),
            score: 0,
            history_score: None,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Space => self.player.flap(),
                VirtualKeyCode::Q => self.mode = GameMode::End,
                _ => {}
            }
        }

        self.player.render(ctx);


        let pass_obstacle = self.player.x > self.obstacles.front().unwrap().x + POSITION_OFFSET;
        // 判断
        if pass_obstacle {
            self.score += 1;
            self.obstacles.pop_front();
        }
        if pass_obstacle {
            if let Some(last) = self.obstacles.back() {
                self.obstacles.push_back(Obstacle::new(last.x + OBSTACLE_GAP, self.score));
            }
        }

        // 渲染 & 判断
        for obstacle in &self.obstacles {
            obstacle.render(ctx, self.player.x);

            if self.player.y > SCREEN_HEIGHT || obstacle.hit_obstacle(&self.player) {
                let highscore_manager = HighScoreManager::new(SCORE_FILE_PATH);
                self.history_score = Some(highscore_manager.update_highscore(self.score));
                self.mode = GameMode::End;
            }
        }

        ctx.print(0, 0, "Press Space to Flap");
        ctx.print(0, 1, &format!("Score: {}", self.score));
        ctx.print(0, 2, &format!("{} {}", self.player.x, self.obstacles.back().unwrap().x));
    }

    fn restart(&mut self) {
        self.player = Player::new(POSITION_OFFSET, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.obstacles = (0..=SCREEN_WIDTH / OBSTACLE_GAP)
            .map(|i| Obstacle::new(SCREEN_WIDTH + (i * OBSTACLE_GAP), 0))
            .collect();
        self.score = 0;
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        let history_score = match &self.history_score {
            Some(Ok(score)) => {
                match score {
                    Score::NewRecord => "You set a new record!".to_string(),
                    Score::HistoryRecord(n) => format!("The highest score ever was {}", n)
                }
            }
            Some(Err(_)) => "(Unable to read the highest score in history)".to_string(),
            _ => { String::new() }
        };
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(6, &format!("You earned {} points", self.score));
        ctx.print_centered(7, history_score);
        ctx.print_centered(9, "(P) Play Again");
        ctx.print_centered(10, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

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

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }

    fn render(&self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x + POSITION_OFFSET;
        let half_size = self.size / 2;

        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }

    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;

        return does_x_match && (player_above_gap || player_below_gap);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50().
        with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}

enum Score {
    NewRecord,
    HistoryRecord(i32),
}

#[derive(Serialize, Deserialize, Debug)]
struct HighScore {
    score: i32,
    timestamp: DateTime<Utc>,
}

struct HighScoreManager {
    file_path: String,
}

impl HighScoreManager {
    fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
        }
    }

    fn read_highscore(&self) -> io::Result<HighScore> {
        if !Path::new(&self.file_path).exists() {
            return Ok(HighScore {
                score: 0,
                timestamp: Utc::now(),
            });
        }

        let mut file = File::open(&self.file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let highscore: HighScore = serde_json::from_str(&contents)
            .unwrap_or(HighScore {
                score: 0,
                timestamp: Utc::now(),
            });

        Ok(highscore)
    }

    fn write_highscore(&self, highscore: &HighScore) -> io::Result<()> {
        let serialized = serde_json::to_string(highscore)?;
        let mut file = File::create(&self.file_path)?;
        file.write_all(serialized.as_bytes())
    }

    fn update_highscore(&self, new_score: i32) -> io::Result<Score> {
        let current_highscore = self.read_highscore()?;
        if new_score > current_highscore.score {
            let new_highscore = HighScore {
                score: new_score,
                timestamp: Utc::now(),
            };
            self.write_highscore(&new_highscore)?;
            Ok(Score::NewRecord)
        } else {
            Ok(Score::HistoryRecord(current_highscore.score))
        }
    }
}