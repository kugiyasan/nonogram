use crate::services::point::Point;

#[derive(Clone, Debug)]
pub struct Grid<T> {
    grid: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize, value: T) -> Self {
        Self {
            grid: vec![value; width * height],
            width,
            height,
        }
    }

    // pub fn width(&self) -> usize {
    //     self.width
    // }
    //
    // pub fn height(&self) -> usize {
    //     self.height
    // }
    //
    // pub fn get(&self, p: Point) -> &T {
    //     &self.grid[self.point_to_index(p)]
    // }

    pub fn set(&mut self, p: Point, value: T) {
        let i = self.point_to_index(p);
        self.grid[i] = value;
    }

    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        self.grid.chunks(self.width)
    }

    pub fn get_row(&self, row_index: usize) -> impl Iterator<Item = &T> {
        let start = row_index * self.width;
        let end = start + self.width;
        self.grid[start..end].iter()
    }

    pub fn get_column(&self, column_index: usize) -> impl Iterator<Item = &T> {
        self.grid.iter().skip(column_index).step_by(self.width)
    }

    pub fn enumerate(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        self.grid
            .iter()
            .enumerate()
            .map(|(i, cell)| ((i % self.height, i / self.height), cell))
    }

    fn point_to_index(&self, p: Point) -> usize {
        p.y * self.width + p.x
    }
}
