use super::utils::*;
use super::sudoku_cell::SudokuCell;
use super::sudoku_region::*;
use crate::sudoku_size::SudokuSizeType;

static STANDARD_SIX_INDICES: [[usize; 6]; 18] = [
    // sub grid
    [ 0,  1,  2,  6,  7,  8],
    [ 3,  4,  5,  9, 10, 11],
    [12, 13, 14, 18, 19, 20],
    [15, 16, 17, 21, 22, 23],
    [24, 25, 26, 30, 31, 32],
    [27, 28, 29, 33, 34, 35],

    // row
    [ 0,  1,  2,  3,  4,  5],
    [ 6,  7,  8,  9, 10, 11],
    [12, 13, 14, 15, 16, 17],
    [18, 19, 20, 21, 22, 23],
    [24, 25, 26, 27, 28, 29],
    [30, 31, 32, 33, 34, 35],

    // column
    [ 0,  6, 12, 18, 24, 30],
    [ 1,  7, 13, 19, 25, 31],
    [ 2,  8, 14, 20, 26, 32],
    [ 3,  9, 15, 21, 27, 33],
    [ 4, 10, 16, 22, 28, 34],
    [ 5, 11, 17, 23, 29, 35],
];

static STANDARD_NINE_INDICES: [[usize; 9]; 27] = [
    // sub grid
    [ 0,  1,  2,  9, 10, 11, 18, 19, 20],
    [ 3,  4,  5, 12, 13, 14, 21, 22, 23],
    [ 6,  7,  8, 15, 16, 17, 24, 25, 26],
    [27, 28, 29, 36, 37, 38, 45, 46, 47],
    [30, 31, 32, 39, 40, 41, 48, 49, 50],
    [33, 34, 35, 42, 43, 44, 51, 52, 53],
    [54, 55, 56, 63, 64, 65, 72, 73, 74],
    [57, 58, 59, 66, 67, 68, 75, 76, 77],
    [60, 61, 62, 69, 70, 71, 78, 79, 80],

    // row
    [ 0,  1,  2,  3,  4,  5,  6,  7,  8],
    [ 9, 10, 11, 12, 13, 14, 15, 16, 17],
    [18, 19, 20, 21, 22, 23, 24, 25, 26],
    [27, 28, 29, 30, 31, 32, 33, 34, 35],
    [36, 37, 38, 39, 40, 41, 42, 43, 44],
    [45, 46, 47, 48, 49, 50, 51, 52, 53],
    [54, 55, 56, 57, 58, 59, 60, 61, 62],
    [63, 64, 65, 66, 67, 68, 69, 70, 71],
    [72, 73, 74, 75, 76, 77, 78, 79, 80],

    // column
    [ 0,  9, 18, 27, 36, 45, 54, 63, 72],
    [ 1, 10, 19, 28, 37, 46, 55, 64, 73],
    [ 2, 11, 20, 29, 38, 47, 56, 65, 74],
    [ 3, 12, 21, 30, 39, 48, 57, 66, 75],
    [ 4, 13, 22, 31, 40, 49, 58, 67, 76],
    [ 5, 14, 23, 32, 41, 50, 59, 68, 77],
    [ 6, 15, 24, 33, 42, 51, 60, 69, 78],
    [ 7, 16, 25, 34, 43, 52, 61, 70, 79],
    [ 8, 17, 26, 35, 44, 53, 62, 71, 80],
];

static DIAGONAL_SIX_INDICES: [[usize; 6]; 2] = [
    [ 0,  7, 14, 21, 28, 35],
    [ 5, 10, 15, 20, 25, 30]
];

static DIAGONAL_NINE_INDICES: [[usize; 9]; 2] = [
    [ 0, 10, 20, 30, 40, 50, 60, 70, 80],
    [ 8, 16, 24, 32, 40, 48, 56, 64, 72]
];

#[derive(Clone)]
pub struct SudokuGrid {
    cells: Vec<SudokuCell>,
    regions: Vec<SudokuRegion>,
}

impl SudokuGrid {
    pub fn standard_six() -> Self {
        let size_type = SudokuSizeType::Six;

        let cells = vec![SudokuCell::new(); size_type.get_cell_num()];

        let mut regions = Vec::new();
        for indices in STANDARD_SIX_INDICES.iter() {
            regions.push(SudokuRegion::from_indices(indices, size_type).unwrap());
        }

        return Self {cells, regions};
    }

    pub fn standard_nine() -> Self {
        let size_type = SudokuSizeType::Nine;

        let cells = vec![SudokuCell::new(); size_type.get_cell_num()];

        let mut regions = Vec::new();
        for indices in STANDARD_NINE_INDICES.iter() {
            regions.push(SudokuRegion::from_indices(indices, size_type).unwrap());
        }

        return Self {cells, regions};
    }

    pub fn diagonal_six() -> Self {
        let size_type = SudokuSizeType::Six;

        let cells = vec![SudokuCell::new(); size_type.get_cell_num()];

        let mut regions = Vec::new();
        for indices in STANDARD_SIX_INDICES.iter() {
            regions.push(SudokuRegion::from_indices(indices, size_type).unwrap());
        }
        for indices in DIAGONAL_SIX_INDICES.iter() {
            regions.push(SudokuRegion::from_indices(indices, size_type).unwrap());
        }

        return Self {cells, regions};
    }

    pub fn diagonal_nine() -> Self {
        let size_type = SudokuSizeType::Nine;

        let cells = vec![SudokuCell::new(); size_type.get_cell_num()];

        let mut regions = Vec::new();
        for indices in STANDARD_NINE_INDICES.iter() {
            regions.push(SudokuRegion::from_indices(indices, size_type).unwrap());
        }
        for indices in DIAGONAL_NINE_INDICES.iter() {
            regions.push(SudokuRegion::from_indices(indices, size_type).unwrap());
        }

        return Self {cells, regions};
    }

    pub fn get_digit(&self, index: usize) -> Option<usize> {
        return self.cells[index].get_digit();
    }

    pub fn set_digit(&mut self, index: usize, digit: usize) {
        self.cells[index].set_digit(digit);
    }

    pub fn clear_digit(&mut self, index: usize) {
        self.cells[index].clear_digit();
    }

    pub fn set_note(&mut self, index: usize, digit: usize) {
        self.cells[index].set_note(digit, true);
    }

    pub fn remove_note(&mut self, index: usize, digit: usize) {
        self.cells[index].set_note(digit, false);
    }

    pub fn change_note(&mut self, index: usize, digit: usize) {
        self.cells[index].change_note(digit);
    }

    pub fn clear_notes(&mut self, index: usize) {
        self.cells[index].clear_notes();
    }

    pub fn fill_notes(&mut self, index: usize) {
        self.cells[index].fill_notes();
    }

    pub fn get_region_num(&self) -> usize {
        return self.regions.len();
    }

    pub fn region_include(&self, region_index: usize, index: usize) -> bool {
        return self.regions[region_index].include(index);
    }

    pub fn get_cell(&self, index: usize) -> SudokuCell {
        return self.cells[index].clone();
    }

    pub fn get_cell_by_region(&self, region_index: usize, index: usize) -> SudokuCell {
        return self.cells[self.regions[region_index].get_grid_index(index)].clone();
    }
}
