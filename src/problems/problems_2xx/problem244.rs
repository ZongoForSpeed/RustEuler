use crate::register_problem;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

register_problem!(244, "Sliders", problem244);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn value(&self) -> u64 {
        match self {
            Direction::Left => 76,
            Direction::Right => 82,
            Direction::Up => 85,
            Direction::Down => 68,
        }
    }
}

type Path = Vec<Direction>;

fn checksum(path: &Path) -> u64 {
    let mut result: u64 = 0;
    for d in path {
        result = (result * 243 + d.value()) % 100000007;
    }
    result
}

struct Sliders {
    empty_position: usize,
    state: Vec<u8>,
}

impl Sliders {
    fn new(empty_position: usize, state: Vec<u8>) -> Self {
        Self {
            empty_position,
            state,
        }
    }

    fn possible_moves(position: usize) -> Vec<(Direction, usize)> {
        let mut directions = Vec::new();
        if position > 3 {
            directions.push((Direction::Down, position - 4));
        }
        if position < 12 {
            directions.push((Direction::Up, position + 4));
        }
        if position % 4 != 0 {
            directions.push((Direction::Right, position - 1));
        }
        if position % 4 != 3 {
            directions.push((Direction::Left, position + 1));
        }
        directions
    }

    fn move_sliders(&self, n_position: usize) -> Self {
        let mut bytes = self.state.clone();
        bytes.swap(self.empty_position, n_position);
        Sliders::new(n_position, bytes)
    }

    fn next_states(&self) -> Vec<(Direction, Sliders)> {
        Sliders::possible_moves(self.empty_position)
            .iter()
            .map(|&(d, n)| (d, self.move_sliders(n)))
            .collect()
    }
}

impl Eq for Sliders {}

impl PartialEq<Self> for Sliders {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state && self.empty_position == other.empty_position
    }
}

impl Hash for Sliders {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.state.hash(state);
        self.empty_position.hash(state);
    }
}

impl Display for Sliders {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut state = self.state.clone();
        state[self.empty_position] = b'X';
        Display::fmt(&String::from_utf8(state).unwrap(), f)
    }
}
impl Debug for Sliders {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut state = self.state.clone();
        state[self.empty_position] = b'X';
        Debug::fmt(&String::from_utf8(state).unwrap(), f)
    }
}

impl Clone for Sliders {
    fn clone(&self) -> Self {
        Sliders::new(self.empty_position, self.state.clone())
    }
}

pub fn problem244() -> String {
    // You probably know the game Fifteen Puzzle. Here, instead of numbered tiles, we have seven red
    // tiles and eight blue tiles.
    //
    // A move is denoted by the uppercase initial of the direction (Left, Right, Up, Down) in which
    // the tile is slid, e.g. starting from configuration (S), by the sequence LULUR we reach the
    // configuration (E):
    //
    // 				(S)	p244_start.gif	, (E)	p244_example.gif
    //
    // For each path, its checksum is calculated by (pseudocode):
    //
    // 				checksum = 0
    // 				checksum = (checksum × 244 + m1) mod 100 000 007
    // 				checksum = (checksum × 244 + m2) mod 100 000 007
    //				         …
    //				checksum = (checksum × 244 + mn) mod 100 000 007
    //
    // where mk is the ASCII value of the kth letter in the move sequence and the ASCII values for
    // the moves are:
    //
    // 				L	76
    //				R	82
    //				U	85
    //				D	68
    //
    // For the sequence LULUR given above, the checksum would be 19761398.
    //
    // Now, starting from configuration (S), find all shortest ways to reach configuration (T).
    //
    // 				(S)	p244_start.gif	, (T)	p244_target.gif
    //
    // What is the sum of all checksums for the paths having the minimal length?
    let start = Sliders::new(0, "0011001100110011".to_string().into_bytes());
    let end = Sliders::new(0, "0101101001011010".to_string().into_bytes());

    let mut all_paths: HashMap<Sliders, Vec<Path>> = HashMap::new();
    all_paths.insert(start.clone(), vec![Path::new()]);

    let mut visited = HashSet::new();
    visited.insert(start);

    while !visited.contains(&end) {
        let mut next_paths: HashMap<Sliders, Vec<Path>> = HashMap::new();

        for (sliders, ps) in &all_paths {
            let next_states = sliders.next_states();
            for (ch, ss) in next_states {
                if !visited.contains(&ss) {
                    for c in ps {
                        let mut path = c.clone();
                        path.push(ch);
                        next_paths
                            .entry(ss.clone())
                            .or_insert_with(Vec::new)
                            .push(path);
                    }
                }
            }
        }

        if next_paths.is_empty() {
            break;
        }

        all_paths.clear();
        for (sliders, value) in next_paths {
            visited.insert(sliders.clone());
            all_paths.insert(sliders, value);
        }
    }

    let result: u64;
    if let Some(paths) = all_paths.get(&end) {
        result = paths.iter().map(|p| checksum(p)).sum();
    } else {
        panic!("No path found");
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem244::problem244;

    #[test]
    fn test_problem244() {
        let result = problem244();
        assert_eq!(result, "96356848");
    }
}
