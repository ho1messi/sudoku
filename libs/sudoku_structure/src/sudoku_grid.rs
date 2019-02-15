use super::sudoku_draft::*;
use super::sudoku_cell::SudokuCell;
use super::sudoku_region::*;

#[derive(Clone)]
pub struct SudokuGrid {
    cells: [SudokuCell; 81],
    regions: Vec<SudokuRegion>,
}

impl SudokuGrid {
    pub fn common() -> Self {
        let mut grid = Self {cells: [SudokuCell::new(); 81], regions: Vec::with_capacity(9)};

        for row in 0..3 {
            for col in 0..3 {
                grid.regions.push(SudokuRegion::from_rect(row * 3, col * 3));
            }
        }

        return grid;
    }

    pub fn diagonal() -> Self {
        let mut grid = Self::common();

        let mut diag1 = [GridCoord::zero(); 9];
        let mut diag2 = [GridCoord::zero(); 9];
        for i in 0..9 {
            diag1[i] = GridCoord { row: i, col: i };
            diag2[i] = GridCoord { row: i, col: 9 - i };
        }

        grid.regions.push(SudokuRegion::from_coords(&diag1));
        grid.regions.push(SudokuRegion::from_coords(&diag2));

        return grid;
    }

    pub fn irregular(region_map: &[RegionIndex; 81]) -> Self {
        let mut grid = Self {cells: [SudokuCell::new(); 81], regions: Vec::with_capacity(9)};

        let mut indices_list = [[GridIndex::zero(); 9]; 9];
        let mut count_list = [0; 9];
        for i in 0..81 {
            let grid_index = GridIndex::create(i).unwrap();
            let region_index = region_map[i].get();

            indices_list[region_index][count_list[region_index]] = grid_index;
            count_list[region_index] += 1;
        }

        for indices in indices_list.iter() {
            grid.regions.push(SudokuRegion::from_indices(indices));
        }

        return grid;
    }

    pub fn get_digit(&self, index: GridIndex) -> Option<SudokuDigit> {
        return self.cells[index.get()].get_digit();
    }

    pub fn set_digit(&mut self, index: GridIndex, digit: SudokuDigit) {
        self.cells[index.get()].set_digit(digit);
    }

    pub fn clear_digit(&mut self, index: GridIndex) {
        self.cells[index.get()].clear_digit();
    }

    pub fn get_drafts(&self, index: GridIndex) -> [bool; 9] {
        return self.cells[index.get()].get_drafts();
    }

    pub fn set_draft(&mut self, index: GridIndex, digit: SudokuDigit) {
        self.cells[index.get()].set_draft(digit, true);
    }

    pub fn remove_draft(&mut self, index: GridIndex, digit: SudokuDigit) {
        self.cells[index.get()].set_draft(digit, false);
    }

    pub fn change_draft(&mut self, index: GridIndex, digit: SudokuDigit) {
        self.cells[index.get()].change_draft(digit);
    }

    pub fn clear_drafts(&mut self, index: GridIndex) {
        self.cells[index.get()].clear_drafts();
    }

    pub fn fill_drafts(&mut self, index: GridIndex) {
        self.cells[index.get()].fill_drafts();
    }

    pub fn get_region_num(&self) -> usize {
        return self.regions.len();
    }

    pub fn region_include(&self, i: usize, index: GridIndex) -> bool {
        return self.regions[i].include(index);
    }

    pub fn get_cell(&self, index: GridIndex) -> SudokuCell {
        return self.cells[index.get()];
    }

    pub fn get_region_cell(&self, i: usize, index: RegionIndex) -> SudokuCell {
        let grid_index = self.regions[i].get_index(index);
        return self.cells[grid_index.get()];
    }
}
