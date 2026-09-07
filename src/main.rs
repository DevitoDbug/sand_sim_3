use macroquad::prelude::*;

use crate::engine::{
    consts::{BLOCK_SIZE, COLS, ROWS},
    game::Game,
};

mod engine;
#[macroquad::main(config())]
async fn main() {
    let mut game = Game::new();
    game.render().await;
}

fn config() -> Conf {
    Conf {
        window_width: COLS * BLOCK_SIZE as i32,
        window_height: ROWS * BLOCK_SIZE as i32,
        window_resizable: false,

        ..Default::default()
    }
}
