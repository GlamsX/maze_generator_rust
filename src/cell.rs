pub struct Cell {
    is_start: bool,
    is_end: bool,
    is_wall: bool,
    is_visited: bool,
    pub x: usize,
    pub y: usize,
}

impl Cell {
    pub fn new(x: usize, y: usize) -> Cell {
        Cell {
            is_start: false,
            is_end: false,
            is_wall: false,
            is_visited: false,
            x,
            y,
        }
    }

    pub fn set_visited(&mut self, visited: bool) {
        self.is_visited = visited;
    }

    pub fn is_visited(&self) -> bool {
        self.is_visited
    }

    pub fn set_is_wall(&mut self, is_wall: bool) {
        if is_wall {
            self.is_start = false;
            self.is_end = false;
        }
        self.is_wall = is_wall;
    }

    pub fn is_wall(&self) -> bool {
        self.is_wall
    }

    // pub fn get_x(&self) -> usize {
    //     self.x
    // }

    // pub fn get_y(&self) -> usize {
    //     self.y
    // }

    pub fn set_is_start(&mut self, is_start: bool) {
        if is_start{
            self.is_end = false;
            self.is_wall = false;
        }
        self.is_start = is_start;
    }

    pub fn is_start(&self) -> bool {
        self.is_start
    }

    pub fn set_is_end(&mut self, is_end: bool) {
        if is_end{
            self.is_start = false;
            self.is_wall = false;
        }
        self.is_end = is_end;
    }

    pub fn is_end(&self) -> bool {
        self.is_end
    }
}