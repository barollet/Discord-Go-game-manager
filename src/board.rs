use std::str::FromStr;

pub type Color = bool;
pub const BLACK: bool = true;
pub const WHITE: bool = false;

// Wrapper for parsing
#[derive(Clone, Copy)]
pub struct Intersection(usize);

// Inefficient board (really inefficient board)
pub struct Board {
    stones: [Option<Color>; 19 * 19],
    pub to_move: Color,
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
        for neighbor in intersection.neighbors() {
            // If the neighboring intersection is occupied by an opponent stone
            if self.stones[neighbor.0] == Some(!self.to_move) {
                // If there is no more liberties
                if !self.flowing_liberties(neighbor) {
                    // Flow to capture the stones
                    self.capture_rec(neighbor);
                }
            }
        }

        // TODO ko detection
        // If a single stone is captured we set it as a ko

        // We check that after placing the stone we still have one
        // liberty
        self.stones[intersection.0] = Some(self.to_move);
        if self.flowing_liberties(intersection) {
            // If we still have a liberty the move is legal and we change the side to play
            self.to_move = !self.to_move;
            true
        } else {
            // Otherwise we remove the stone and the move is illegal
            self.stones[intersection.0] = None;
            false
        }
    }

    // Flowing algorithm to find liberty to the given intersection
    fn flowing_liberties(&self, intersection: Intersection) -> bool {
        let mut visited = [false; 19 * 19];
        self.flowing_rec(intersection, &mut visited)
    }

    fn flowing_rec(&self, intersection: Intersection, visited: &mut [bool; 19 * 19]) -> bool {
        // We visit the current intersection
        visited[intersection.0] = true;
        // If any neighbor has a liberty then we are alive
        intersection.neighbors().iter().any(|&neighbor| {
            // If this is a liberty we can return alive
            if self.stones[neighbor.0].is_none() {
                true
            // Else we do a recursive call if the color is the same as the side to move
            // and the neighbors is not visited
            } else if self.stones[neighbor.0] == Some(self.to_move) && !visited[neighbor.0] {
                self.flowing_rec(neighbor, visited)
            } else {
                // Opponent stone color is not a liberty
                false
            }
        })
    }

    // Capture a given stone and all the neighbors (captured stones are from opponent color)
    fn capture_rec(&mut self, intersection: Intersection) {
        self.stones[intersection.0] = None;
        // For all neighbors
        for neighbor in intersection.neighbors() {
            // If the neighboring stone is of opponent color
            if self.stones[neighbor.0] == Some(!self.to_move) {
                // We also capture it
                self.capture_rec(neighbor);
            }
        }
    }
}

impl Intersection {
    // TODO smallvec ?
    fn neighbors(self) -> Vec<Intersection> {
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
        let mut chars = s.chars();
        let letter = chars.next().ok_or(())?.to_ascii_lowercase();
        let number: String = chars.collect();

        let line: usize = number.parse().map_err(|_| ())?;
        let mut column = letter as u8 - b'a';
        if letter > 'i' {
            column -= 1;
        }
        if column < 19 || line < 19 {
            Ok(Intersection(line * 19 + column as usize))
        } else {
            Err(())
        }
    }
}

impl From<Intersection> for usize {
    fn from(i: Intersection) -> Self {
        i.0
    }
}

const LETTER_CONVERTION: [&str; 19] = [
    ":one:",
    ":two:",
    ":three:",
    ":four:",
    ":five:",
    ":six:",
    ":seven:",
    ":eight:",
    ":nine:",
    ":keycap_ten:",
    ":one:",
    ":two:",
    ":three:",
    ":four:",
    ":five:",
    ":six:",
    ":seven:",
    ":eight:",
    ":nine:",
];

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Prints first line
        write!(f, ":park:")?;
        for c in b'a'..=b't' {
            if c != b'i' {
                write!(f, ":regional_indicator_{}:", c as char)?;
            }
        }

        // Prints board lines
        for (i, stone) in self.stones.iter().enumerate() {
            // If new line
            if i % 19 == 0 {
                write!(f, "\n{}", LETTER_CONVERTION[i / 19])?;
            }
            write!(
                f,
                "{}",
                match stone {
                    Some(BLACK) => ":chestnut:",
                    Some(WHITE) => ":cookie",
                    None => {
                        ":lol:"
                    }
                }
            )?;
        }

        Ok(())
    }
}
