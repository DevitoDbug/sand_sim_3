use crate::engine::consts::{BLOCK_SIZE, COLS, ROWS};
use macroquad::prelude::*;
use macroquad::rand;

const NUM_OF_CELLS: usize = (ROWS * COLS) as usize;

pub struct Game {
    board: [i32; NUM_OF_CELLS],
}

impl Game {
    pub fn new() -> Self {
        let board = [0; NUM_OF_CELLS];
        Self { board }
    }

    pub async fn render(&mut self) {
        loop {
            clear_background(WHITE);
            self.render_board();

            if is_mouse_button_down(MouseButton::Left) {
                self.spawn_particle();
            }
            self.move_particles();
            next_frame().await;
        }
    }

    fn render_board(&self) {
        for (i, val) in self.board.iter().rev().enumerate() {
            if *val == 0 {
                continue;
            }

            let x = (self.board.len() - i) as i32 % COLS;
            let y = (self.board.len() - i) as i32 / COLS;
            //  0,  1,  2,  3,  4,  5,  6,  7,  8,  9
            // 10, 11, 12, 13, 14, 15, 16, 17, 18, 19
            // ....
            // ....
            // 90, 99, 92, 93, 94, 95, 96, 97, 98, 99

            draw_rectangle(
                x as f32 * BLOCK_SIZE,
                y as f32 * BLOCK_SIZE,
                BLOCK_SIZE,
                BLOCK_SIZE,
                RED,
            );
        }
    }

    fn spawn_particle(&mut self) {
        // case. BLOCK_SIZE = 32, COLS = 10, ROWS = 10
        let (x, y) = mouse_position();
        // 289., 0.
        let (x, y) = (x as i32, y as i32);
        // 289, 0
        let col = x / BLOCK_SIZE as i32;
        // 9
        let row = y / BLOCK_SIZE as i32;
        // 0
        let index = col + (COLS * row);
        // 9 + (0 * 10)
        self.board[index as usize] = 1;
    }

    fn move_particles(&mut self) {
        for i in (0..self.board.len()).rev() {
            if self.board[i] == 0 {
                continue;
            }

            let target_spot = i + (COLS as usize);
            let is_in_bound = target_spot < self.board.len();

            // Moving down
            if is_in_bound && self.board[i + COLS as usize] == 0 {
                self.board[target_spot] = 1;
                self.board[i] = 0;
                continue;
            }

            // Moving down sideways
            let dx: i32 = if rand::gen_range(0, 2) == 0 { -1 } else { 1 };
            let is_in_bound = (target_spot as i32 + dx) < self.board.len() as i32;
            if is_in_bound && self.board[(target_spot as i32 + dx) as usize] == 0 {
                self.board[(target_spot as i32 + dx) as usize] = 1;
                self.board[i] = 0;
            }
        }
    }
}
