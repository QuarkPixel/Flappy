use bracket_lib::prelude::{BTerm, RandomNumberGenerator, to_cp437};
use crate::{plr, POSITION_OFFSET, SCREEN_HEIGHT};
use crate::palette;

pub(crate) struct Obstacle {
    pub(crate) x: i32,
    gap_y: i32,
    size: i32,
    pub(crate) passed: bool,
}

impl Obstacle {
    pub(crate) fn new(x: i32, score: i32) -> Self {
        let gap_y = RandomNumberGenerator::new().range(10, SCREEN_HEIGHT - 10);
        Obstacle {
            x,
            gap_y,
            size: i32::max(2, 20 - score),
            passed: false,
        }
    }

    pub(crate) fn render(&self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x + POSITION_OFFSET;
        let half_size = self.size / 2;

        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, palette::OBSTACLE, palette::OBSTACLE_BG, to_cp437('P'));
        }

        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, palette::OBSTACLE, palette::OBSTACLE_BG, to_cp437('P'));
        }
    }

    pub(crate) fn hit_obstacle(&self, player: &plr::Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;

        return does_x_match && (player_above_gap || player_below_gap);
    }
}