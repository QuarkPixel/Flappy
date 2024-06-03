use bracket_lib::prelude::{BTerm, to_cp437};
use crate::{POSITION_OFFSET, HIGHLIGHT_DURATION};
use crate::libs::blend_colors;
use crate::palette;

pub(crate) struct Player {
    pub(crate) x: i32,
    pub(crate) y: i32,
    velocity: f32,
    time: f32,
    highlight_countdown: f32,
}

impl Player {
    pub(crate) fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
            time: 0.0,
            highlight_countdown: HIGHLIGHT_DURATION,
        }
    }

    pub(crate) fn render(&mut self, ctx: &mut BTerm) {
        let color: palette::Cor = if self.highlight_countdown > 0.0 {
            self.highlight_countdown -= ctx.frame_time_ms;
            blend_colors(
                palette::BIRD,
                palette::BIRD_HIGHLIGHT,
                self.highlight_countdown / HIGHLIGHT_DURATION,
            )
        } else { palette::BIRD };
        ctx.set(POSITION_OFFSET, self.y, color, color, to_cp437('♂'))
    }

    pub(crate) fn gravity_and_move(&mut self) {
        self.time += 0.1;

        self.velocity += 0.2; // 加速度影响速度
        self.y = (self.y as f32 + self.velocity * self.time + 0.5 * 0.2 * self.time.powi(2)) as i32;
        self.x += 1;

        if self.y < 0 {
            self.y = 0;
        }
    }

    pub(crate) fn flap(&mut self) {
        self.highlight_countdown = HIGHLIGHT_DURATION;
        self.velocity = -2.0;
        self.time = 0.0;
    }
}

