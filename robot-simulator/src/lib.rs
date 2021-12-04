// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    dir: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, dir: Direction) -> Self {
        Robot {
            x,
            y,
            dir
        }
    }

    pub fn turn_right(self) -> Self {
        match self.dir {
            Direction::North => Robot::new(self.x, self. y, Direction::East),
            Direction::East => Robot::new(self.x, self.y, Direction::South),
            Direction::South => Robot::new(self.x, self.y, Direction::West),
            Direction::West => Robot::new(self.x, self.y, Direction::North),
        }
    }

    pub fn turn_left(self) -> Self {
        match self.dir {
            Direction::North => Robot::new(self.x, self.y, Direction::West),
            Direction::East => Robot::new(self.x, self.y, Direction::North),
            Direction::South => Robot::new(self.x, self.y, Direction::East),
            Direction::West => Robot::new(self.x, self.y, Direction::South),
        }
    }

    pub fn advance(self) -> Self {
        match self.dir {
            Direction::North => Robot::new(self.x, self.y + 1, self.dir),
            Direction::South => Robot::new(self.x, self.y - 1, self.dir),
            Direction::East => Robot::new(self.x + 1, self.y, self.dir),
            Direction::West => Robot::new(self.x - 1, self.y, self.dir),
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, c| {
            match c {
                'R' => robot.turn_right(),
                'A' => robot.advance(),
                'L' => robot.turn_left(),
                _ => panic!("Movement not supported!"),
            }
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }
}
