use std::fmt::{Debug, Display, Error, Formatter};
use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Grid<T: Debug> {
    grid: Vec<T>,
    grid_size: (usize, usize),
    offset: usize,
}

impl<T: Display + Debug> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut output = String::from("");
        let (x, y) = self.grid_size;

        for col in (0..y).into_iter() {
            for row in (0..x).into_iter() {
                let item = self.grid.get(col * x + row).unwrap();
                output = output.add(format!("{}", item).as_str());
            }
            output = output.add("\n");
        }

        write!(f, "\n{}", output)
    }
}

impl<T: Default + Debug> Grid<T> {
    pub fn new(width: usize, height: usize, x_offset: usize, y_offset: usize) -> Grid<T> {
        let mut data = Vec::with_capacity(width * height);

        for i in 0..width * height {
            data.insert(i, T::default())
        }

        Grid {
            grid: data,
            grid_size: (width, height),
            offset: y_offset * height + x_offset,
        }
    }
}

impl<T: Debug> Grid<T> {
    fn coords_to_index(&self, x: isize, y: isize) -> usize {
        ((y * self.grid_size.0 as isize + x) + self.offset as isize) as usize
    }

    pub fn get_row(&self, row_idx: usize) -> &[T] {
        let (width, height) = self.grid_size;
        let offset = row_idx * width;
        &self.grid[offset..offset + width]
    }
}

pub struct GridIntoIterator<T: Debug> {
    grid: Grid<T>,
    x: isize,
    y: isize,
}

#[derive(Debug)]
pub struct GridIteratorItem<T> {
    pub element: T,
    pub x: isize,
    pub y: isize,
}

impl<T: Debug + Clone> IntoIterator for Grid<T> {
    type Item = GridIteratorItem<T>;
    type IntoIter = GridIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        GridIntoIterator {
            grid: self,
            x: 0,
            y: 0,
        }
    }
}

impl<T: Debug + Clone> Iterator for GridIntoIterator<T> {
    type Item = GridIteratorItem<T>;
    fn next(&mut self) -> Option<GridIteratorItem<T>> {
        if self.x == self.grid.grid_size.0 as isize {
            self.x = 0;
            self.y += 1;
            if self.y == self.grid.grid_size.1 as isize {
                return None;
            }
        }

        let index = self.grid.coords_to_index(self.x, self.y);
        let cell = self.grid.grid.get(index).map_or(None, |x| {
            Some(GridIteratorItem {
                x: self.x,
                y: self.y,
                element: x.clone(),
            })
        });

        self.x += 1;

        cell
    }
}

impl<T: Debug + Clone> Grid<T> {
    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        if !self.check_bounds(x, y) {
            return None;
        }
        let index = self.coords_to_index(x, y) as usize;
        self.grid.get(index)
    }

    fn check_bounds(&self, x: isize, y: isize) -> bool {
        let (width, height) = self.grid_size;
        if self.offset == 0 {
            x < width as isize && y < height as isize && x >= 0 && y >= 0
        } else {
            true
        }
    }

    pub fn set(&mut self, x: isize, y: isize, item: T) -> Result<(), ()> {
        if !self.check_bounds(x, y) {
            return Err(());
        }
        let index = self.coords_to_index(x, y) as usize;
        match self.grid.get_mut(index) {
            Some(old_item) => {
                *old_item = item;
                Ok(())
            }
            None => Err(()),
        }
    }

    pub fn iter(&self) -> GridIntoIterator<T> {
        GridIntoIterator {
            grid: self.clone(),
            x: 0,
            y: 0,
        }
    }

    pub fn grid(&self) -> Vec<T> {
        self.grid.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::Grid;

    #[test]
    fn test_simple_grid() {
        let mut grid: Grid<bool> = Grid::new(5, 5, 0, 0);
        assert_eq!(&false, grid.get(2, 3).unwrap());
        assert_eq!(&false, grid.get(0, 0).unwrap());
        assert_eq!(&false, grid.get(4, 4).unwrap());

        grid.set(2, 2, true).unwrap();
        assert_eq!(&true, grid.get(2, 2).unwrap());
    }

    #[test]
    fn test_offset_grid() {
        let mut grid: Grid<bool> = Grid::new(5, 5, 2, 2);
        assert_eq!(&false, grid.get(-1, -1).unwrap());
        assert_eq!(&false, grid.get(-2, -2).unwrap());

        grid.set(-2, -2, true).unwrap();
        assert_eq!(&true, grid.get(-2, -2).unwrap());
    }

    #[test]
    fn test_iterator_length() {
        let grid: Grid<bool> = Grid::new(5, 5, 0, 0);
        assert_eq!(grid.iter().map(|_| 1).count(), 25);
        assert_eq!(grid.into_iter().count(), 25);
    }
}
