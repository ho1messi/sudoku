use super::utils::*;
use super::sudoku_type::*;
use super::sudoku_cell::SudokuCell;
use super::sudoku_region::*;

#[derive(Clone)]
pub struct SudokuGrid {
    cells: Vec<SudokuCell>,
    regions: Vec<SudokuRegion>,
}

impl SudokuGrid {
    pub fn common(t: SudokuType) -> Self {
        let mut grid = Self {
            cells: vec![SudokuCell::new(); t.get_grid_size()],
            regions: Vec::new()
        };

        let width = t.get_digit_num() / t.get_digit_num();
        let height = t.get_digit_num() / t.get_region_height();
        for row in 0..height {
            for col in 0..width {
                grid.regions.push(SudokuRegion::from_rect(
                    row * t.get_region_width(),
                    col * t.get_region_height(),
                    t
                ));
            }
        }

        return grid;
    }

    pub fn diagonal(t: SudokuType) -> Result<Self, Error> {
        let mut grid = Self::common(t);

        let digit_num = t.get_digit_num();

        let mut diag1 = Vec::new();
        let mut diag2 = Vec::new();
        for i in 0..digit_num {
            diag1.push(i * digit_num + i);
            diag2.push(i * digit_num + digit_num - i);
        }

        grid.regions.push(SudokuRegion::from_indices(&diag1, t)?);
        grid.regions.push(SudokuRegion::from_indices(&diag2, t)?);

        return Ok(grid);
    }

    pub fn irregular(region_map: &[usize], t: SudokuType) -> Result<Self, Error> {
        if region_map.len() != t.get_grid_size() {
            return Err(Error::create(
                ErrorKind::BadSudokuIndexNum,
                "Num of indices to create a grid error"
            ));
        }

        let mut grid = Self {cells: Vec::new(), regions: Vec::new()};

        let mut indices_list = Vec::new();
        for index in 0..t.get_grid_size() {
            let region = region_map[index];
            while region >= indices_list.len() {
                indices_list.push(Vec::new());
            }

            grid.cells.push(SudokuCell::new());
            indices_list[region].push(index);
        }

        for region in indices_list {
            grid.regions.push(SudokuRegion::from_indices(&region, t)?);
        }

        return Ok(grid);
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
