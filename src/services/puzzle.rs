use super::grid::Grid;


type Hints = Vec<Vec<u32>>;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Pixel {
    Unknown,
    Filled,
    NonFilled,
}

#[derive(Clone, Debug)]
pub struct Puzzle {
    pub grid: Grid<Pixel>,
    cols_hints: Hints,
    rows_hints: Hints,
}

impl Puzzle {
    pub fn new(cols_hints: Hints, rows_hints: Hints) -> Self {
        let width = cols_hints.len();
        let height = rows_hints.len();
        let grid = Grid::new(width, height, Pixel::Unknown);

        Self {
            grid,
            cols_hints,
            rows_hints,
        }
    }
}
