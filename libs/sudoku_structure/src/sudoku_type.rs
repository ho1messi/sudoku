#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum SudokuType {
    Six,
    Nine
}

impl SudokuType {
    pub fn get_digit_num(&self) -> usize {
        return match self {
            SudokuType::Six => 6,
            SudokuType::Nine => 9
        };
    }

    pub fn get_region_width(&self) -> usize {
        return match self {
            SudokuType::Six => 3,
            SudokuType::Nine => 3
        };
    }

    pub fn get_region_height(&self) -> usize {
        return match self {
            SudokuType::Six => 2,
            SudokuType::Nine => 3
        };
    }

    pub fn get_region_num(&self) -> usize {
        return match self {
            SudokuType::Six => 6,
            SudokuType::Nine => 9
        }
    }

    pub fn get_grid_size(&self) -> usize {
        return match self {
            SudokuType::Six => 36,
            SudokuType::Nine => 81
        };
    }
}