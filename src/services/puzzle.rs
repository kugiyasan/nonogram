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

    pub fn cols_hints(&self) -> &Hints {
        &self.cols_hints
    }

    pub fn rows_hints(&self) -> &Hints {
        &self.rows_hints
    }

    pub fn map_rows<'a, F, R>(&'a self, f: F) -> impl Iterator<Item = R> + 'a
    where
        F: FnMut((usize, &[Pixel])) -> R + 'a,
    {
        self.grid.rows().enumerate().map(f)
    }

    pub fn is_column_done(&self, col_index: usize) -> bool {
        let column = Self::consecutive_filled_squares(self.grid.get_column(col_index));
        column == self.cols_hints[col_index]
    }

    pub fn is_row_done(&self, row_index: usize) -> bool {
        let row = Self::consecutive_filled_squares(self.grid.get_row(row_index));
        row == self.rows_hints[row_index]
    }

    fn consecutive_filled_squares<'a, I>(iterator: I) -> Vec<u32>
    where
        I: Iterator<Item = &'a Pixel>,
    {
        let mut result = vec![];
        let mut count = 0;

        for pixel in iterator {
            if pixel == &Pixel::Filled {
                count += 1;
            } else {
                if count > 0 {
                    result.push(count);
                }
                count = 0;
            }
        }

        if count > 0 {
            result.push(count);
        }

        result
    }
}
