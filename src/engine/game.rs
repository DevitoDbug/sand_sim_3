use crate::engine::consts::{BLOCK_SIZE, COLS, ROWS};
use macroquad::prelude::*;
use macroquad::{color, rand};

const NUM_OF_CELLS: usize = (ROWS * COLS) as usize;

#[derive(Debug)]
struct Particle {
    val: i32,
    color: color::Color,
}

impl Particle {
    fn new(val: i32, color: color::Color) -> Self {
        Self { val, color }
    }
}

pub struct Game {
    board: Vec<Particle>,
    color: color::Color,
}

impl Game {
    pub fn new() -> Self {
        let board: Vec<Particle> = (0..NUM_OF_CELLS).map(|_| Particle::new(0, WHITE)).collect();
        Self { board, color: RED }
    }

    pub async fn render(&mut self) {
        let mut time = 0.;
        let colors = [GOLD, RED, VIOLET, ORANGE, BLUE, PURPLE];
        let mut color_index = 0;
        loop {
            clear_background(WHITE);
            self.render_board();

            if is_mouse_button_down(MouseButton::Left) {
                self.spawn_particle();
            }
            self.move_particles();

            if time >= 5. {
                time = 0.;
                color_index += 1;
                if color_index >= colors.len() {
                    color_index = 0
                }
            }
            time += get_frame_time();
            self.color = colors[color_index];
            next_frame().await;
        }
    }

    fn render_board(&self) {
        for (i, particle) in self.board.iter().rev().enumerate() {
            if particle.val == 0 {
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
                particle.color,
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
        for i in -3..3 {
            let index_val = index + i;
            if index_val < 0 || index_val as usize >= self.board.len() {
                continue;
            }
            self.board[index_val as usize].val = 1;
            self.board[index_val as usize].color = self.color;
        }
    }

    fn move_particles(&mut self) {
        for i in (0..self.board.len()).rev() {
            if self.board[i].val == 0 {
                continue;
            }

            let target_spot = i + (COLS as usize);
            let is_in_bound = target_spot < self.board.len();

            // Moving down
            if is_in_bound && self.board[i + COLS as usize].val == 0 {
                self.board[target_spot].val = 1;
                self.board[target_spot].color = self.board[i].color;

                self.board[i].val = 0;
                self.board[i].color = WHITE;
                continue;
            }

            // Moving down sideways
            let dx: i32 = if rand::gen_range(0, 2) == 0 { -1 } else { 1 };
            let is_in_bound = (target_spot as i32 + dx) < self.board.len() as i32;
            if is_in_bound && self.board[(target_spot as i32 + dx) as usize].val == 0 {
                self.board[(target_spot as i32 + dx) as usize].val = 1;
                self.board[(target_spot as i32 + dx) as usize].color = self.board[i].color;

                self.board[i].val = 0;
                self.board[i].color = WHITE;
            }
        }
    }
}
