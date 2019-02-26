use super::utils::*;
use super::sudoku_type::*;

#[derive(Clone, Debug)]
pub struct SudokuRegion {
    cells: Vec<usize>,
    t: SudokuType
}

impl SudokuRegion {
    pub fn from_indices(indices: &[usize], t: SudokuType) -> Result<Self, Error> {
        if indices.len() < t.get_digit_num() {
            return Err(Error::create(
                ErrorKind::BadSudokuIndexNum,
                "Num of indices to create a region error"
            ));
        }

        return Ok(Self { cells: Vec::from(indices), t });
    }

    pub fn from_rect(top: usize, left: usize, t: SudokuType) -> Self {
        let mut region = SudokuRegion{cells: Vec::new(), t};

        for row in top..(top + t.get_region_height()) {
            for col in left..(left + t.get_region_width()) {
                region.cells.push(row * t.get_digit_num() + col);
            }
        }

        return region;
    }

    pub fn include(&self, index: usize) -> bool {
        for index_ref in self.cells.iter() {
            if *index_ref == index {
                return true;
            }
        }

        return false;
    }

    pub fn get_grid_index(&self, index: usize) -> usize {
        if index >= self.t.get_digit_num() {
            panic!("Region index out of range");
        }

        return self.cells[index];
    }
}

impl<'a> IntoIterator for &'a SudokuRegion {
    type Item = usize;
    type IntoIter = SudokuRegionIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        return Self::IntoIter {cells: &self.cells, len: self.t.get_digit_num(), index: 0};
    }
}

pub struct SudokuRegionIter<'a> {
    cells: &'a [usize],
    len: usize,
    index: usize
}

impl<'a> Iterator for SudokuRegionIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;

        if self.index == self.len {
            return None;
        }

        return Some(self.cells[self.index]);
    }
}
