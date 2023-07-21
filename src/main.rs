mod cell;
mod maze;

use rand::Rng;
fn main() {

    let mut rnd: rand::rngs::ThreadRng = rand::thread_rng();

    // for _ in 0..1000 {
    let rnd_width = rnd.gen_range(10..=100);
    let rnd_height = rnd.gen_range(10..=100);
    let mut maze: maze::Maze = maze::Maze::new(rnd_width, rnd_height, (1, 1));
    maze.dfs(maze.start.0, maze.start.1);
    maze.draw_maze_with_walls();
        // maze.write_maze_on_file().unwrap();
    // }
}
