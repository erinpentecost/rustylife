use crate::display;
use crate::game;
use crate::inputs;
use crate::scope;
use std::convert::TryInto;

#[repr(u8)] // This forces the cell to be 1 byte
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

pub struct Life {
    // world size
    width: i32,
    height: i32,
    // view port
    view_x: i32,
    view_y: i32,
    // cursor
    mouse_x: i32,
    mouse_y: i32,
    // paused sim
    paused: bool,
    /// true is alive
    cells: Vec<Cell>,
}

impl Life {
    fn get_index(&self, row: i32, column: i32) -> usize {
        let row_pos: i32 = row.rem_euclid(self.height);

        let column_pos: i32 = column.rem_euclid(self.width);

        (row_pos * self.width + column_pos) as usize
    }
    fn get_neighbor_indices(&self, row: i32, column: i32) -> [usize; 8] {
        [
            self.get_index(row + 1, column + 1),
            self.get_index(row + 1, column),
            self.get_index(row + 1, column - 1),
            self.get_index(row - 1, column + 1),
            self.get_index(row - 1, column),
            self.get_index(row - 1, column - 1),
            self.get_index(row, column + 1),
            self.get_index(row, column - 1),
        ]
    }

    fn live_neighbor_count(&self, row: i32, column: i32) -> u8 {
        let mut sum: u8 = 0;
        for n_idx in &self.get_neighbor_indices(row, column) {
            sum += self.cells[*n_idx] as u8
        }
        sum
    }

    pub fn new() -> Life {
        let width = 100 as i32;
        let height = 100 as i32;
        // Set cells to random alive v dead
        let rando: &mut [u8; 300 * 300 / 8] = &mut [0; 300 * 300 / 8];
        scope::get_scope()
            .crypto()
            .unwrap()
            .get_random_values_with_u8_array(rando)
            .unwrap();
        let cells = (0..width * height)
            .map(|i| {
                let rdr = i as usize % (rando.len() * 8 - 1);
                let rdx = (rdr % 8) as u8;
                let rdi = (rdr / 8) as usize;
                if (rando[rdi] & rdx).count_ones() > 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        Life {
            width,
            height,
            view_x: 0,
            view_y: 0,
            mouse_x: 0,
            mouse_y: 0,
            paused: false,
            cells,
        }
    }
}

impl game::Game for Life {
    fn get_target_fps(&self) -> f64 {
        60 as f64
    }

    fn handle_input(&mut self, _time: f64, map: &inputs::InputMap) {
        // Move screen
        if map.held(inputs::Key::ArrowDown) || map.held(inputs::Key::S) || map.held(inputs::Key::E)
        {
            self.view_y += 1;
        }
        if map.held(inputs::Key::ArrowLeft) || map.held(inputs::Key::A) || map.held(inputs::Key::N)
        {
            self.view_x -= 1;
        }
        if map.held(inputs::Key::ArrowUp) || map.held(inputs::Key::W) || map.held(inputs::Key::U) {
            self.view_y -= 1;
        }
        if map.held(inputs::Key::ArrowRight) || map.held(inputs::Key::D) || map.held(inputs::Key::I)
        {
            self.view_x += 1;
        }

        // Pause
        if map.hit(inputs::Key::Space) {
            self.paused = !self.paused;
        }

        // Clear all!
        if map.hit(inputs::Key::Escape) {
            self.cells = vec![Cell::Dead; self.cells.len()];
        }

        // Cursor tracking
        self.mouse_x = map.cursor_x + self.view_x;
        self.mouse_y = map.cursor_y + self.view_y;

        // Change board state
        if map.held(inputs::Key::MouseLeft) {
            // map the screen cursor position to world position
            let state_idx = self.get_index(self.mouse_y, self.mouse_x);
            self.cells[state_idx] = Cell::Alive;
        } else if map.held(inputs::Key::MouseRight) {
            // map the screen cursor position to world position
            let state_idx = self.get_index(self.mouse_y, self.mouse_x);
            self.cells[state_idx] = Cell::Dead;
        }
    }

    fn simulate(&mut self, _time: f64) {
        if self.paused {
            return;
        }

        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_state = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };
                next[idx] = next_state;
            }
        }

        self.cells = next;
    }

    // the Javascript side passes a pointer to a buffer, the size of the corresponding canvas
    // and the current timestamp.
    fn render(&mut self, time: f64, screen: &mut display::Display) {
        // We might not be able to fit in the whole world.
        let resolution = { screen.get_resolution() };

        let cursor_idx = self.get_index(self.mouse_y, self.mouse_x);

        for (i, px) in screen.get_buffer_mut().iter_mut().enumerate() {
            // get the position of current pixel on the screen
            let h: i32 = (i / resolution.width).try_into().unwrap();
            let w: i32 = (i % resolution.width).try_into().unwrap();

            // map the screen position to world position
            let x_world = w + self.view_x;
            let y_world = h + self.view_y;

            // set opacity to 1
            px.a = 255;

            let world_idx = self.get_index(y_world, x_world);

            if world_idx == cursor_idx {
                px.r = 0;
                px.g = 255;
                px.b = 0;
            } else if self.cells[world_idx] == Cell::Alive {
                px.r = 0;
                px.g = 0;
                px.b = 0;
            } else {
                px.r = 255;
                px.g = 255;
                px.b = 255;
            }
        }
    }
}
