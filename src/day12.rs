pub fn day12a() -> String {
    let commands = read_data();
    let mut ship = Ship::default();
    commands.iter().for_each(|command| ship.sail(command));
    ship.manhattan_distance().to_string()
}

pub fn day12b() -> String {
    let commands = read_data();
    let mut ship = Ship::default();
    commands
        .iter()
        .for_each(|command| ship.sail_to_waypoint(command));
    ship.manhattan_distance().to_string()
}

fn read_data() -> Vec<Command> {
    std::fs::read_to_string("inputs/day12.txt")
        .expect("Couldn't read file")
        .lines()
        .map(Command::new)
        .collect::<Vec<Command>>()
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
    Forward,
    Right,
    Left,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::East
    }
}

#[derive(Debug)]
struct Command {
    dir: Direction,
    val: isize,
}

impl Command {
    fn new(s: &str) -> Self {
        let val: isize = s[1..].parse().unwrap();
        let dir = match &s[0..1] {
            "N" => Direction::North,
            "S" => Direction::South,
            "E" => Direction::East,
            "W" => Direction::West,
            "F" => Direction::Forward,
            "R" => Direction::Right,
            "L" => Direction::Left,
            _ => unreachable!(),
        };
        Command { dir, val }
    }
}

struct Ship {
    // postive x is the east direction
    // postive y is the north direction
    position: (isize, isize),
    waypoint: (isize, isize),
    facing: Direction,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            position: Default::default(),
            waypoint: (10, 1),
            facing: Default::default(),
        }
    }
}

impl Ship {
    fn sail(&mut self, command: &Command) {
        let val = command.val;
        match command.dir {
            Direction::North => {
                self.position.1 += val;
            }
            Direction::South => {
                self.position.1 -= val;
            }
            Direction::East => {
                self.position.0 += val;
            }
            Direction::West => {
                self.position.0 -= val;
            }
            Direction::Forward => self.sail(&Command {
                dir: self.facing,
                val,
            }),
            _ => self.rotate_ship(command),
        }
    }

    fn sail_to_waypoint(&mut self, command: &Command) {
        let val = command.val;
        match command.dir {
            Direction::North => {
                self.waypoint.1 += val;
            }
            Direction::South => {
                self.waypoint.1 -= val;
            }
            Direction::East => {
                self.waypoint.0 += val;
            }
            Direction::West => {
                self.waypoint.0 -= val;
            }
            Direction::Forward => {
                self.position = (
                    self.position.0 + self.waypoint.0 * val,
                    self.position.1 + self.waypoint.1 * val,
                );
            }
            _ => self.rotate_waypoint(command),
        }
    }

    fn rotate_waypoint(&mut self, command: &Command) {
        macro_rules! tuple_as {
            ($t: expr, $ty: ident) => {{
                let (a, b) = $t;
                let a = a as $ty;
                let b = b as $ty;
                (a, b)
            }};
        }
        let (sin, cos);
        let (x, y) = self.waypoint;
        match command.dir {
            Direction::Right => {
                (sin, cos) = tuple_as!(
                    ((-command.val / 90) as f32 * std::f32::consts::FRAC_PI_2).sin_cos(),
                    isize
                );
            }
            Direction::Left => {
                (sin, cos) = tuple_as!(
                    ((command.val / 90) as f32 * std::f32::consts::FRAC_PI_2).sin_cos(),
                    isize
                );
            }
            _ => unreachable!(),
        }
        self.waypoint = (cos * x - sin * y, sin * x + cos * y);
    }

    fn rotate_ship(&mut self, command: &Command) {
        const DIRECTIONS: [Direction; 4] = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];
        let facing_dir = self.facing;
        let current_dir_idx =
            if let Some(current_dir_idx) = DIRECTIONS.iter().position(|&dir| dir == facing_dir) {
                current_dir_idx
            } else {
                unreachable!()
            };
        let new_dir_idx;
        match command.dir {
            Direction::Right => {
                new_dir_idx = (current_dir_idx + (command.val as usize / 90)) % 4;
            }
            Direction::Left => {
                new_dir_idx = (current_dir_idx + (3 * command.val as usize / 90)) % 4;
            }
            _ => unreachable!(),
        }

        self.facing = DIRECTIONS[new_dir_idx];
    }

    fn manhattan_distance(&self) -> usize {
        self.position.0.unsigned_abs() + self.position.1.unsigned_abs()
    }
}
