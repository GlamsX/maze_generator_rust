use crate::cell::Cell;
use rand::seq::SliceRandom;
pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub start: (usize, usize),
    pub end: (usize, usize),
    pub cells: Vec<Vec<Cell>>,
}

impl Maze {

    fn set_start(&mut self, x: usize, y: usize) {
        self.cells[y][x].set_is_start(true);
    }

    fn set_end(&mut self, x: usize, y: usize) {
        self.cells[y][x].set_is_end(true);
    }

    fn fill_walls(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.cells[y][x].set_is_wall(true);
            }
        }
    }

    pub fn new(width: usize, height: usize, start: (usize, usize)) -> Maze {
        let mut cells = Vec::new();
        for y in 0..height {
            let mut row = Vec::new();
            for x in 0..width {
                row.push(Cell::new(x, y));
            }
            cells.push(row);
        }


        let mut maze = Maze {
            width,
            height,
            cells,
            start,
            end : (width - 1, height - 2),
        };

        maze.fill_walls();
        maze.set_start(start.0, start.1);
        maze.set_end(width - 1 , height - 2);

        maze
    }

    pub fn draw_maze_with_walls(&self) {
        for y in 0..self.height {
            // print!("X");
            for x in 0..self.width {
                if self.cells[y][x].is_wall() {
                    print!("X");
                } else if self.cells[y][x].is_start() {
                    print!("S");
                } else if self.cells[y][x].is_end() {
                    print!("E");
                } else {
                    print!(" ");
                }
            }
            print!("X");
            println!();
        }
        for _ in 0..self.width + 1 {
            print!("X");
        }
        println!();
    }

    pub fn write_maze_on_file(&self) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::prelude::*;
        use uuid::Uuid;
        let uuid:String = Uuid::new_v4().to_string();
        let filename: String = format!("{}maze_{}_w{}_h{}.txt", "./mazes/",uuid, self.width, self.height);
        let mut file: File = File::create(filename)?;

        for y in 0..self.height {
            for x in 0..self.width {
                if self.cells[y][x].is_wall() {
                    file.write_all(b"X")?;
                } else if self.cells[y][x].is_start() {
                    file.write_all(b"S")?;
                } else if self.cells[y][x].is_end() {
                    file.write_all(b"E")?;
                } else {
                    file.write_all(b" ")?;
                }
            }
            file.write_all(b"X\n")?;
        }
        for _ in 0..self.width + 1 {
            file.write_all(b"X")?;
        }
        Ok(())
    }

    pub fn dfs(&mut self, x: usize, y: usize) {
        let directions = [(0, 2), (2, 0), (0, -2), (-2, 0)];
        let mut rng = rand::thread_rng();
        let mut directions: Vec<(i32, i32)> = directions.to_vec();
        
        self.cells[y][x].set_visited(true);
        self.cells[y][x].set_is_wall(false);
        directions.shuffle(&mut rng);
        for &(dx, dy) in &directions {
            let nx = (x as i32) + dx;
            let ny = (y as i32) + dy;

            if nx >= 0 && ny >= 0 && nx < (self.width as i32) && ny < (self.height as i32) {
                let nx = nx as usize;
                let ny = ny as usize;

                if !self.cells[ny][nx].is_visited() {
                    self.cells[(y as i32 + dy / 2) as usize][(x as i32 + dx / 2) as usize].set_is_wall(false);
                    self.dfs(nx, ny);
                }
            }
        }

    }
}