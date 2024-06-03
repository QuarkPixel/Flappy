mod sc;
mod plr;
mod obs;
mod ste;
mod palette;

use bracket_lib::prelude::*;

const SCORE_FILE_PATH: &str = "flappy_info.json";


const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 50.0;
const POSITION_OFFSET: i32 = 5;
const OBSTACLE_GAP: i32 = 30;
const HIGHLIGHT_DURATION: f32 = 500.0;


fn main() -> BError {
    let context = BTermBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT)
        .unwrap()
        .with_title("Flappy Pixel")
        .build()?;

    main_loop(context, ste::State::new())
}

