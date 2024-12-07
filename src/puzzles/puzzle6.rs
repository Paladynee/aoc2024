use crate::solver::SolverSentinel;
use core::fmt;
use core::fmt::Debug;
use core::mem;
#[derive(Debug, Clone, PartialEq, Eq)]
struct NavigatableMap {
    tiles: Vec2D<Tile>,
    visited_tiles: Vec2D<VisitedDirections>,
    patroller: Patroller,
    w: usize,
    h: usize,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Patroller {
    pos: (usize, usize),
    dir: Direction,
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum Ternary {
    False,
    True,
    Unknown,
}
impl From<bool> for Ternary {
    fn from(value: bool) -> Self {
        if value {
            Self::True
        } else {
            Self::False
        }
    }
}
impl Ternary {
    #[inline]
    pub const fn as_bool(&self) -> Option<bool> {
        match *self {
            Self::True => Some(true),
            Self::False => Some(false),
            Self::Unknown => None,
        }
    }
}
impl NavigatableMap {
    #[inline]
    pub fn new(input: &str) -> Self {
        let w = input.lines().next().unwrap().len();
        let h = input.lines().count();
        let tiles: Vec<Tile> = input
            .lines()
            .map(|l| l.trim())
            .collect::<String>()
            .bytes()
            .map(|a| Tile::try_from(a).unwrap())
            .collect();
        let patroller_pos = {
            // find the index of the first `Patroller` tile in `tiles`
            // return the 2D coordinates
            let (index, _) = input
                .lines()
                .map(|l| l.trim())
                .collect::<String>()
                .bytes()
                .enumerate()
                .find(|&(_, byte)| byte == b'^')
                .unwrap();
            (index % w, index / w)
        };
        let visited_tiles = vec![VisitedDirections::new(); w * h];
        // todo fix fucking logic
        // visited_tiles[patroller_pos.1 * w + patroller_pos.0].set_direction(Direction::Up);
        Self {
            tiles: Vec2D::new(w, h, tiles),
            visited_tiles: Vec2D::new(w, h, visited_tiles),
            patroller: Patroller {
                pos: patroller_pos,
                dir: Direction::Up,
            },
            w,
            h,
        }
    }
    #[inline]
    pub fn step(&mut self) -> Option<HasFinished> {
        // eprintln!("step");
        if self.check_whether_patroller_visited_current_tile_with_same_direction_before() {
            // eprintln!("step: patroller visited current tile with same direction before");
            return Some(HasFinished::LoopEntered);
        }
        match self.is_tile_ahead_an_obstacle()?.as_bool() {
            Some(true) => {
                // eprintln!("step: tile ahead is an obstacle");
                self.rotate_patroller();
            }
            Some(false) => {
                // eprintln!("step: tile ahead is not an obstacle");
                self.move_patroller();
            }

            None => {
                return Some(HasFinished::OutOfBounds);
            }
        }
        // eprintln!("step: unfinished");
        Some(HasFinished::Unfinished)
    }
    #[inline]
    pub fn is_tile_ahead_an_obstacle(&self) -> Option<Ternary> {
        // eprintln!("checking whether tile ahead is an obstacle");
        let (x, y) = self.patroller.pos;
        // eprintln!("{} {}, {} {}", x, y, self.w, self.h);
        match self.patroller.dir.offset_bounded(x, y, self.w, self.h) {
            None => Some(Ternary::Unknown),
            Some((x, y)) => Some((self.tiles.get(x, y)? == &Tile::Obstacle).into()),
        }
    }
    #[inline]
    pub fn rotate_patroller(&mut self) {
        let old_dir = self.patroller.dir;
        self.patroller.dir.rotate_clockwise_inplace();
        self.get_visited_directions_at_patroller_position_mut().set_direction(old_dir);
    }
    #[inline]
    pub fn get_visited_directions_at_patroller_position_mut(&mut self) -> &mut VisitedDirections {
        let (x, y) = self.patroller.pos;
        self.visited_tiles.get_mut(x, y).unwrap_or_else(
            #[cold]
            || panic!("Patroller out of bounds"),
        )
    }
    #[inline]
    pub fn get_visited_directions_at_position(&mut self, x: usize, y: usize) -> &mut VisitedDirections {
        self.visited_tiles.get_mut(x, y).unwrap_or_else(
            #[cold]
            || panic!("Position out of bounds"),
        )
    }
    #[inline]
    pub fn get_visited_directions_at_patroller_position(&self) -> &VisitedDirections {
        let (x, y) = self.patroller.pos;
        self.visited_tiles.get(x, y).unwrap_or_else(
            #[cold]
            || panic!("Patroller out of bounds"),
        )
    }
    #[inline]
    pub fn move_patroller(&mut self) -> Option<()> {
        // eprintln!(
        //     "moving patroller from {}, {} to {}, {}",
        //     self.patroller.pos.0,
        //     self.patroller.pos.1,
        //     self.patroller.dir.offset(self.patroller.pos.0, self.patroller.pos.1)?.0,
        //     self.patroller.dir.offset(self.patroller.pos.0, self.patroller.pos.1)?.1
        // );
        let (x, y) = self.patroller.pos;
        let (nx, ny) = self.patroller.dir.offset_bounded(x, y, self.w, self.h)?;
        self.patroller.pos = (nx, ny);
        let dir = self.patroller.dir;
        // eprintln!("Add direction {:?} to visited directions at {}, {}", dir, x, y);
        self.get_visited_directions_at_position(x, y).set_direction(dir);
        Some(())
    }
    #[inline]
    pub fn check_whether_patroller_visited_current_tile_with_same_direction_before(&self) -> bool {
        // eprintln!("checking whether patroller visited current tile with same direction before");
        let visited = self.get_visited_directions_at_patroller_position();
        visited.is_direction_visited(self.patroller.dir)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum HasFinished {
    LoopEntered,
    OutOfBounds,
    Unfinished,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Vec2D<T> {
    data: Vec<T>,
    w: usize,
    h: usize,
}
impl<T> Vec2D<T> {
    #[inline]
    pub fn new(w: usize, h: usize, data: Vec<T>) -> Self {
        assert!(w * h == data.len(), "Invalid dimensions");
        Self { data, w, h }
    }
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        assert!(!(x >= self.w || y >= self.h), "Out of bounds");
        self.data.get(y * self.w + x)
    }
    #[inline]
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        assert!(!(x >= self.w || y >= self.h), "Out of bounds");
        self.data.get_mut(y * self.w + x)
    }
    #[inline]
    pub fn str<F, R>(&self, f: F) -> String
    where
        R: AsRef<str>,
        F: Fn(&T) -> R,
    {
        let mut s = String::with_capacity(self.w * self.h);
        for y in 0..self.h {
            for x in 0..self.w {
                s.push_str(f(self.get(x, y).unwrap()).as_ref());
            }
            s.push('\n');
        }
        s
    }
}
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    #[inline]
    pub fn offset(self, x: usize, y: usize) -> Option<(usize, usize)> {
        Some(match self {
            Self::Up => (x, y.checked_sub(1)?),
            Self::Left => (x.checked_sub(1)?, y),
            Self::Right => (x.checked_add(1)?, y),
            Self::Down => (x, y.checked_add(1)?),
        })
    }
    #[inline]
    pub fn offset_bounded(self, x: usize, y: usize, bound_x: usize, bound_y: usize) -> Option<(usize, usize)> {
        let r = match self {
            Self::Up => (x, y.checked_sub(1)?),
            Self::Left => (x.checked_sub(1)?, y),
            Self::Right => (x.checked_add(1)?, y),
            Self::Down => (x, y.checked_add(1)?),
        };
        if r.0 >= bound_x || r.1 >= bound_y {
            None
        } else {
            Some(r)
        }
    }
    #[inline]
    pub fn offset_times(self, x: usize, y: usize, factor: usize) -> Option<(usize, usize)> {
        Some(match self {
            Self::Up => (x, y.checked_sub(factor)?),
            Self::Left => (x.checked_sub(factor)?, y),
            Self::Right => (x.checked_add(factor)?, y),
            Self::Down => (x, y.checked_add(factor)?),
        })
    }
    #[inline]
    pub const fn rotate_clockwise(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
    #[inline]
    pub const fn rotate_counter_clockwise(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }
    #[inline]
    pub const fn rotate_clockwise_inplace(&mut self) {
        *self = self.rotate_clockwise();
    }
    #[inline]
    pub const fn rotate_counter_clockwise_inplace(&mut self) {
        *self = self.rotate_counter_clockwise();
    }
}
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Tile {
    Obstacle,
    Air,
}
#[allow(clippy::struct_excessive_bools)] // cry about it
#[derive(Clone, Hash, Eq, PartialEq)]
struct VisitedDirections {
    up: bool,
    right: bool,
    down: bool,
    left: bool,
}
impl Debug for VisitedDirections {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            if self.up { '^' } else { '_' },
            if self.right { '>' } else { '_' },
            if self.down { 'v' } else { '_' },
            if self.left { '<' } else { '_' }
        )
    }
}
impl VisitedDirections {
    #[inline]
    pub const fn new() -> Self {
        Self {
            up: false,
            right: false,
            down: false,
            left: false,
        }
    }
    #[inline]
    pub const fn set_up(&mut self) -> &mut Self {
        self.up = true;
        self
    }
    #[inline]
    pub const fn set_right(&mut self) -> &mut Self {
        self.right = true;
        self
    }
    #[inline]
    pub const fn set_down(&mut self) -> &mut Self {
        self.down = true;
        self
    }
    #[inline]
    pub const fn set_left(&mut self) -> &mut Self {
        self.left = true;
        self
    }
    #[inline]
    pub const fn is_direction_visited(&self, dir: Direction) -> bool {
        match dir {
            Direction::Up => self.up,
            Direction::Right => self.right,
            Direction::Down => self.down,
            Direction::Left => self.left,
        }
    }
    #[inline]
    pub const fn is_all_visited(&self) -> bool {
        self.up && self.right && self.down && self.left
    }
    #[inline]
    pub const fn is_any_visited(&self) -> bool {
        self.up || self.right || self.down || self.left
    }
    #[inline]
    pub const fn set_direction(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.set_up(),
            Direction::Right => self.set_right(),
            Direction::Down => self.set_down(),
            Direction::Left => self.set_left(),
        };
    }
}
impl TryFrom<u8> for Tile {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'#' => Self::Obstacle,
            b'.' | b'^' => Self::Air,
            _ => return Err(format!("Invalid tile: {}", value as char)),
        })
    }
}
#[inline]
pub fn solve_part_1(input: &str, _sentinel: &mut SolverSentinel) -> i32 {
    let mut map = NavigatableMap::new(input);
    loop {
        let res = map.step();
        match res {
            Some(res) => match res {
                HasFinished::OutOfBounds | HasFinished::LoopEntered => break,
                HasFinished::Unfinished => {
                    // eprintln!("{}", map.visited_tiles.str(|visiteddir| { format!("{:?}", visiteddir) }));
                    continue;
                }
            },
            None => {
                panic!("Some error");
            }
        }
    }
    let visited_tiles = map.visited_tiles.data.iter().filter(|&visited| visited.is_any_visited()).count();
    visited_tiles as i32 + 1 // account for goddamn first tile, has a chance to be wrong if the patroller somehow passes through the starting square with a different direction.
}
#[inline]
pub fn solve_part_2(input: &str, _sentinel: &mut SolverSentinel) -> i32 {
    let mut map = NavigatableMap::new(input);
    let mut total = 0;
    let original_patroller = map.patroller.clone();
    let original_visited_tiles = map.visited_tiles.clone();
    for (index1d, tile) in map.tiles.data.clone().into_iter().enumerate() {
        if index1d == map.patroller.pos.1 * map.w + map.patroller.pos.0 {
            continue;
        }
        match tile {
            Tile::Air => {
                let x = index1d % map.w;
                let y = index1d / map.w;
                // set the tile to an obstacle in new map
                let mut previous_tile = Tile::Obstacle;
                mem::swap(map.tiles.get_mut(x, y).unwrap(), &mut previous_tile);
                'a: loop {
                    let res = map.step();
                    match res.unwrap() {
                        HasFinished::LoopEntered => {
                            total += 1;
                            break 'a;
                        }
                        HasFinished::OutOfBounds => {
                            break 'a;
                        }
                        HasFinished::Unfinished => {
                            continue;
                        }
                    }
                }
                mem::swap(map.tiles.get_mut(x, y).unwrap(), &mut previous_tile);
                map.patroller.clone_from(&original_patroller);
                map.visited_tiles.data.clone_from_slice(&original_visited_tiles.data);
            }
            Tile::Obstacle => {
                continue;
            }
        }
    }
    total
}
