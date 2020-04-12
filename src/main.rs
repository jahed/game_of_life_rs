use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use std::collections::HashMap;

const CELL_SIZE: i32 = 8;
const CELL_BOUNDS: graphics::Rect = graphics::Rect { x: 0.0, y: 0.0, w: CELL_SIZE as f32, h: CELL_SIZE as f32 };

fn get_coords (index: usize, columns: i32) -> (i32, i32) {
  let x = index as f32 % columns as f32;
  let y = (index as f32 / columns as f32).trunc();
  (x as i32, y as i32)
}

fn get_alive_neighbours(cells: &Vec<bool>, index: usize, columns: i32) -> i32 {
  let (x, y) = get_coords(index, columns);

  let max_x = columns - 1;
  let max_y = (cells.len() as i32 / columns) - 1;

  let neighbours = [
    ((x - 1), (y - 1)),
    (x, (y - 1)),
    ((x + 1), (y - 1)),
    ((x - 1), y),
    ((x + 1), y),
    ((x - 1), (y + 1)),
    (x, (y + 1)),
    ((x + 1), (y + 1))
  ];

  let mut result = 0;

  for i in neighbours.iter() {
    let (x, y) = *i;
    if x < 0 || x > max_x || y < 0 || y > max_y {
      continue;
    }

    let index = ((y * columns) + x) as usize;
    if cells[index] {
      result += 1;
    }
  }

  result
}

struct MainState {
  columns: i32,
  cells: Vec<bool>,
}

impl MainState {
  fn new() -> GameResult<MainState> {
    let s = MainState {
      columns: 64,
      cells: vec![
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
    ]
  };
    Ok(s)
  }
}

impl event::EventHandler for MainState {
  fn update(&mut self, _ctx: &mut Context) -> GameResult {
    let mut changes: HashMap<usize, bool> = HashMap::new();

    for index in 0..self.cells.len() {
      let alive = self.cells[index];
      let alive_neighbours = get_alive_neighbours(&self.cells, index, self.columns);

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
        let (x, y) = get_coords(index, self.columns);
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

#[cfg(test)]
mod tests {
    use super::get_alive_neighbours;
    use super::get_coords;

    #[test]
    fn test_get_coords() {
      assert_eq!(get_coords(2, 4), (2, 0));
      assert_eq!(get_coords(0, 4), (0, 0));
      assert_eq!(get_coords(5, 4), (1, 1));
      assert_eq!(get_coords(8, 4), (0, 2));
    }

    #[test]
    fn test_get_alive_neighbours() {
      let cells = vec![
        false, true, false, true,
        false, false, true, false,
        false, false, false, false
      ];

      assert_eq!(get_alive_neighbours(&cells, 2, 4), 3);
      assert_eq!(get_alive_neighbours(&cells, 0, 4), 1);
      assert_eq!(get_alive_neighbours(&cells, 5, 4), 2);
      assert_eq!(get_alive_neighbours(&cells, 8, 4), 0);
    }
}
