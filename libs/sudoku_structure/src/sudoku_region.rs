use super::utils::*;
use crate::sudoku_size::SudokuSizeType;

#[derive(Clone, Debug)]
pub struct SudokuRegion {
    cells: Vec<usize>,
    size_type: SudokuSizeType
}

impl SudokuRegion {
    pub fn from_indices(indices: &[usize], size_type: SudokuSizeType) -> Result<Self, Error> {
        if indices.len() < size_type.get_digit_num() {
            return Err(Error::create(
                ErrorKind::BadSudokuIndexNum,
                "Num of indices to create a region error"
            ));
        }

        return Ok(Self { cells: Vec::from(indices), size_type });
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
        if index >= self.size_type.get_digit_num() {
            panic!("Region index out of range");
        }

        return self.cells[index];
    }
}

impl<'a> IntoIterator for &'a SudokuRegion {
    type Item = usize;
    type IntoIter = SudokuRegionIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        return Self::IntoIter {cells: &self.cells, len: self.size_type.get_digit_num(), index: 0};
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
