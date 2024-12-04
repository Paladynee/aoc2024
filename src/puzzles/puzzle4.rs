use crate::solver::SolverSentinel;

#[derive(Clone, PartialEq, Eq, Hash)]
struct String2D {
    lines: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl String2D {
    #[inline]
    pub fn from(s: &[u8]) -> Option<Self> {
        let lines: Vec<Vec<u8>> = s.split(|n| *n == b'\n' || *n == b'\r').map(|l| l.to_vec()).collect();
        let width = lines.first().map(|l| l.len())?;
        let height = lines.len();
        Some(Self { lines, width, height })
    }

    #[inline]
    pub fn get_2d(&self, x: usize, y: usize) -> Option<u8> {
        self.lines.get(y)?.get(x).copied()
    }

    #[inline]
    pub fn check_xmas_successively_for_one_direction(&self, x: usize, y: usize, direction: Direction) -> Option<()> {
        if self.get_2d(x, y) != Some(b'X') {
            return None;
        }

        let (x, y) = direction.offset(x, y)?;
        if self.get_2d(x, y) != Some(b'M') {
            return None;
        }

        let (x, y) = direction.offset(x, y)?;
        if self.get_2d(x, y) != Some(b'A') {
            return None;
        }

        let (x, y) = direction.offset(x, y)?;
        if self.get_2d(x, y) != Some(b'S') {
            return None;
        }

        Some(())
    }

    #[inline]
    pub fn check_xmas_successively_for_all_directions(&self, x: usize, y: usize) -> usize {
        Direction::ALL
            .iter()
            .filter(|&&d| self.check_xmas_successively_for_one_direction(x, y, d).is_some())
            .count()
    }

    #[inline]
    pub fn check_xmas_all_locations_for_all_directions(&self) -> usize {
        let mut count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                count += self.check_xmas_successively_for_all_directions(x, y);
            }
        }
        count
    }

    #[inline]
    pub fn check_mas(&self, ax: usize, ay: usize) -> Option<()> {
        if self.get_2d(ax, ay) != Some(b'A') {
            return None;
        }

        let (x, y) = Direction::TopRight.offset(ax, ay)?;

        match self.get_2d(x, y)? {
            b'S' => {
                let (x, y) = Direction::BottomLeft.offset(ax, ay)?;
                if self.get_2d(x, y) != Some(b'M') {
                    return None;
                }
            }
            b'M' => {
                let (x, y) = Direction::BottomLeft.offset(ax, ay)?;
                if self.get_2d(x, y) != Some(b'S') {
                    return None;
                }
            }

            _ => return None,
        };

        let (x, y) = Direction::TopLeft.offset(ax, ay)?;

        match self.get_2d(x, y)? {
            b'S' => {
                let (x, y) = Direction::BottomRight.offset(ax, ay)?;
                if self.get_2d(x, y) != Some(b'M') {
                    return None;
                }
            }

            b'M' => {
                let (x, y) = Direction::BottomRight.offset(ax, ay)?;
                if self.get_2d(x, y) != Some(b'S') {
                    return None;
                }
            }

            _ => return None,
        }

        Some(())
    }

    #[inline]
    pub fn check_mas_all_locations(&self) -> usize {
        let mut count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.check_mas(x, y).is_some() {
                    count += 1;
                }
            }
        }
        count
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Direction {
    TopLeft,
    Top,
    TopRight,
    BottomLeft,
    Bottom,
    BottomRight,
    Left,
    Right,
}

impl Direction {
    pub const ALL: [Self; 8] = [
        Self::TopLeft,
        Self::Top,
        Self::TopRight,
        Self::Left,
        Self::Right,
        Self::BottomLeft,
        Self::Bottom,
        Self::BottomRight,
    ];

    #[inline]
    pub fn offset(self, x: usize, y: usize) -> Option<(usize, usize)> {
        Some(match self {
            Self::TopLeft => (x.checked_sub(1)?, y.checked_sub(1)?),
            Self::Top => (x, y.checked_sub(1)?),
            Self::TopRight => (x.checked_add(1)?, y.checked_sub(1)?),
            Self::Left => (x.checked_sub(1)?, y),
            Self::Right => (x.checked_add(1)?, y),
            Self::BottomLeft => (x.checked_sub(1)?, y.checked_add(1)?),
            Self::Bottom => (x, y.checked_add(1)?),
            Self::BottomRight => (x.checked_add(1)?, y.checked_add(1)?),
        })
    }
}

#[inline]
pub fn solve_part_1(input: &str, _sentinel: &mut SolverSentinel) -> usize {
    let input = String2D::from(input.as_bytes()).unwrap();
    input.check_xmas_all_locations_for_all_directions()
}

#[inline]
pub fn solve_part_2(input: &str, _sentinel: &mut SolverSentinel) -> usize {
    let input = String2D::from(input.as_bytes()).unwrap();
    input.check_mas_all_locations()
}
