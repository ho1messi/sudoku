#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum SudokuSizeType {
    Six,
    Nine
}

impl SudokuSizeType {
    pub fn get_digit_num(&self) -> usize {
        return match self {
            SudokuSizeType::Six => 6,
            SudokuSizeType::Nine => 9
        }
    }

    pub fn get_subgrid_width(&self) -> usize {
        return match self {
            SudokuSizeType::Six => 3,
            SudokuSizeType::Nine => 3
        }
    }

    pub fn get_subgrid_height(&self) -> usize {
        return match self {
            SudokuSizeType::Six => 2,
            SudokuSizeType::Nine => 3
        }
    }

    pub fn get_gird_length_cell(&self) -> usize {
        return match self {
            SudokuSizeType::Six => 6,
            SudokuSizeType::Nine => 9
        }
    }

    pub fn get_gird_width_subgrid(&self) -> usize {
        return match self {
            SudokuSizeType::Six => 3,
            SudokuSizeType::Nine => 3
        }
    }

    pub fn get_gird_height_subgrid(&self) -> usize {
        return match self {
            SudokuSizeType::Six => 2,
            SudokuSizeType::Nine => 3
        }
    }

    pub fn get_subgrid_num(&self) -> usize {
        return match self {
            SudokuSizeType::Six => 6,
            SudokuSizeType::Nine => 9
        }
    }

    pub fn get_cell_num(&self) -> usize {
        return match self {
            SudokuSizeType::Six => 36,
            SudokuSizeType::Nine => 81
        }
    }
}
