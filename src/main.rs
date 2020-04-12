use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use std::collections::HashMap;

const COLUMNS: u16 = 64;
const CELL_SIZE: u16 = 4;
const CELL_BOUNDS: graphics::Rect = graphics::Rect { x: 0.0, y: 0.0, w: CELL_SIZE as f32, h: CELL_SIZE as f32 };

fn get_coords (index: usize) -> (u16, u16) {
  let x = index as f32 % COLUMNS as f32;
  let y = (index as f32 / COLUMNS as f32).trunc();
  (x as u16, y as u16)
}

fn get_alive_neighbours(cells: &Vec<bool>, index: usize) -> u16 {
  let mut result = 0;
  let max_columns = COLUMNS - 1;
  let max_rows = (cells.len() as u16 / COLUMNS) - 1;
  let (x, y) = get_coords(index);

  if y > 0 && x > 0 && cells[((y - 1) * COLUMNS + (x - 1)) as usize] {
    result += 1;
  }
  if y > 0 && cells[((y - 1) * COLUMNS + x) as usize] {
    result += 1;
  }
  if y > 0 && x < max_columns && cells[((y - 1) * COLUMNS + (x + 1)) as usize] {
    result += 1;
  }
  if x > 0 && cells[(y * COLUMNS + (x - 1)) as usize] {
    result += 1;
  }
  if x < max_columns && cells[(y * COLUMNS + (x + 1)) as usize] {
    result += 1;
  }
  if y < max_rows && x > 0 && cells[((y + 1) * COLUMNS + (x - 1)) as usize] {
    result += 1;
  }
  if y < max_rows && cells[((y + 1) * COLUMNS + x) as usize] {
    result += 1;
  }
  if y < max_rows && x < max_columns && cells[((y + 1) * COLUMNS + (x + 1)) as usize] {
    result += 1;
  }
  result
}

struct MainState {
  cells: Vec<bool>,
}

impl MainState {
  fn new() -> GameResult<MainState> {
    let s = MainState { cells: vec![
      false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, true, true, true, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false,
false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, true, false, false, false, false, true, false, false, true, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false,
false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, true, true, false, false, false, false, false, true, true, true, true, false, false, false, false, true, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false,
false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, true, false, false, false, false, false, false, false, false, true, true, true, true, false, false, false, false, false, true, true, false, false, false, true, false, false, false, false, false, false, false, false, false, true, true,
false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, true, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, true, false, false, false, true, false, false, false, false, false, false, false, false, false, true, true,
false, false, false, false, true, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false,
true, true, false, true, false, false, true, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false,
true, true, false, false, true, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    ] };
    Ok(s)
  }
}

impl event::EventHandler for MainState {
  fn update(&mut self, _ctx: &mut Context) -> GameResult {
    let mut changes: HashMap<usize, bool> = HashMap::new();

    for index in 0..self.cells.len() {
      let alive = self.cells[index];
      let alive_neighbours = get_alive_neighbours(&self.cells, index);

      if alive {
        if alive_neighbours < 2 || alive_neighbours > 3 {
          changes.insert(index, false);
        }
      } else {
        if alive_neighbours == 3 {
          changes.insert(index, true);
        }
      }
    }

    for (key, val) in changes.iter() {
      self.cells[*key] = *val;
    }

    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
    let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), CELL_BOUNDS, graphics::WHITE)?;

    for index in 0..self.cells.len() {
      let alive = self.cells[index];
      if alive {
        let (x, y) = get_coords(index);
        graphics::draw(ctx, &mesh, (na::Point2::new((x * CELL_SIZE) as f32, (y * CELL_SIZE) as f32),))?;
      }
    }

    graphics::present(ctx)?;
    Ok(())
  }
}

pub fn main() -> GameResult {
  let (ctx, event_loop) = &mut ggez::ContextBuilder::new("game_of_life", "Jahed Ahmed")
    .window_setup(ggez::conf::WindowSetup::default().title("Game of Life"))
    .build()?;
  let state = &mut MainState::new()?;
  event::run(ctx, event_loop, state)
}
