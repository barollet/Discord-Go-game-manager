use std::str::FromStr;

pub type Color = bool;
pub const BLACK: bool = true;
pub const WHITE: bool = false;

// Wrapper for parsing
pub struct Intersection(usize);

// Inefficient board (really inefficient board)
pub struct Board {
    stones: [Option<Color>; 19 * 19],
    to_move: Color,
    ko: Option<usize>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            stones: [None; 19 * 19],
            to_move: BLACK,
            ko: None,
        }
    }

    // Play a move an returns if the move is legal
    pub fn play(&mut self, intersection: Intersection) -> bool {
        // If the intersection is occupied
        if self.stones[intersection.0].is_some() {
            return false;
        }
        // If the intersection was the previous ko fight
        if self.ko.map_or(false, |ko| ko == intersection.0) {
            return false;
        }
        // Capture stones (naive way)
        // For all neighboring stone we flow through neighbors until we find a liberty, if we don't
        // the stones are captured

        // If a single stone is captured we set it as a ko
        // Stone is not auto atari
        true
    }

    // Flowing algorithm to find liberty to the given intersection
    fn flowing_liberties(&self, intersection: Intersection) -> bool {
        // TODO
        true
    }
}

impl Intersection {
    // TODO smallvec ?
    fn neighbors(&self) -> Vec<Intersection> {
        let mut neighbors = vec![];
        // top
        if self.0 >= 19 {
            neighbors.push(Intersection(self.0 - 19));
        }
        // bottom
        if self.0 < 19 * 18 {
            neighbors.push(Intersection(self.0 + 19));
        }
        // left
        if self.0 % 19 > 0 {
            neighbors.push(Intersection(self.0 - 1));
        }
        // right
        if self.0 % 19 < 18 {
            neighbors.push(Intersection(self.0 + 1));
        }

        neighbors
    }
}

impl FromStr for Intersection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO
        Ok(Intersection(0))
        /*
        let coords: Vec<&str> = s
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Point {
            x: x_fromstr,
            y: y_fromstr,
        })
            */
    }
}

impl From<Intersection> for usize {
    fn from(i: Intersection) -> Self {
        i.0
    }
}
