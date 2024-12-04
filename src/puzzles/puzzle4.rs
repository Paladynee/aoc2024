use crate::solver::SolverSentinel;

#[derive(Clone, PartialEq, Eq, Hash)]
struct String2D<'a> {
    lines: Vec<&'a [u8]>,
    width: usize,
    height: usize,
}

impl<'a> String2D<'a> {
    #[inline]
    pub fn from(s: &'a [u8]) -> Option<Self> {
        let lines: Vec<_> = s.split(|n| *n == b'\n' || *n == b'\r').collect();
        let width = lines.first().map(|l| l.len())?;
        let height = lines.len();
        Some(Self { lines, width, height })
    }

    #[inline]
    pub fn get_2d(&self, x: usize, y: usize) -> Option<u8> {
        self.lines.get(y)?.get(x).copied()
    }

    #[inline]
    pub fn check_xmas_successively_for_one_direction(&self, x: usize, y: usize, direction: Direction) -> bool {
        let Some((nx, ny)) = direction.offset_times(x, y, 3) else { return false };
        if self.get_2d(nx, ny) != Some(b'S') {
            return false;
        }

        let Some((nx, ny)) = direction.offset_times(x, y, 2) else { return false };
        if self.get_2d(nx, ny) != Some(b'A') {
            return false;
        }

        let Some((nx, ny)) = direction.offset_times(x, y, 1) else { return false };
        if self.get_2d(nx, ny) != Some(b'M') {
            return false;
        }

        true
    }

    #[inline]
    pub fn check_xmas_successively_for_all_directions(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for d in Direction::ALL {
            if self.check_xmas_successively_for_one_direction(x, y, d) {
                count += 1;
            }
        }
        count
    }

    #[inline]
    pub fn check_xmas_all_locations_for_all_directions(&self) -> usize {
        self.lines.iter().enumerate().fold(0_usize, |count, (y, line)| {
            count
                + line
                    .iter()
                    .enumerate()
                    .filter(|&(_, &c)| c == b'X')
                    .map(|(x, _)| self.check_xmas_successively_for_all_directions(x, y))
                    .sum::<usize>()
        })
    }

    #[inline]
    pub fn check_mas(&self, ax: usize, ay: usize) -> bool {
        let Some((x, y)) = Direction::TopRight.offset(ax, ay) else {
            return false;
        };

        match self.get_2d(x, y) {
            Some(b'S') => {
                let Some((x, y)) = Direction::BottomLeft.offset(ax, ay) else {
                    return false;
                };
                if self.get_2d(x, y) != Some(b'M') {
                    return false;
                }
            }
            Some(b'M') => {
                let Some((x, y)) = Direction::BottomLeft.offset(ax, ay) else {
                    return false;
                };
                if self.get_2d(x, y) != Some(b'S') {
                    return false;
                }
            }
            _ => return false,
        };

        let Some((x, y)) = Direction::TopLeft.offset(ax, ay) else { return false };

        match self.get_2d(x, y) {
            Some(b'S') => {
                let Some((x, y)) = Direction::BottomRight.offset(ax, ay) else {
                    return false;
                };
                if self.get_2d(x, y) != Some(b'M') {
                    return false;
                }
            }
            Some(b'M') => {
                let Some((x, y)) = Direction::BottomRight.offset(ax, ay) else {
                    return false;
                };
                if self.get_2d(x, y) != Some(b'S') {
                    return false;
                }
            }
            _ => return false,
        }

        true
    }

    #[inline]
    pub fn check_mas_all_locations(&self) -> usize {
        self.lines.iter().enumerate().fold(0_usize, |count, (y, line)| {
            count
                + line
                    .iter()
                    .enumerate()
                    .filter(|&(x, &c)| if c == b'A' { self.check_mas(x, y) } else { false })
                    .count()
        })
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

    #[inline]
    pub fn offset_times(self, x: usize, y: usize, factor: usize) -> Option<(usize, usize)> {
        Some(match self {
            Self::TopLeft => (x.checked_sub(factor)?, y.checked_sub(factor)?),
            Self::Top => (x, y.checked_sub(factor)?),
            Self::TopRight => (x.checked_add(factor)?, y.checked_sub(factor)?),
            Self::Left => (x.checked_sub(factor)?, y),
            Self::Right => (x.checked_add(factor)?, y),
            Self::BottomLeft => (x.checked_sub(factor)?, y.checked_add(factor)?),
            Self::Bottom => (x, y.checked_add(factor)?),
            Self::BottomRight => (x.checked_add(factor)?, y.checked_add(factor)?),
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
