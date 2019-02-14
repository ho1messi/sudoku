#[derive(Copy, Clone, Debug)]
pub struct SudokuCoord {
    pub row: usize,
    pub col: usize
}

impl SudokuCoord {
    pub fn from_index(index: usize) -> Self {
        return Self {
            row: index / 9,
            col: index % 9
        };
    }

    pub fn to_index(&self) -> usize {
        return self.row * 9 + self.col;
    }
}

#[derive(Clone, Debug)]
pub struct SudokuRegion {
    cells: [SudokuCoord; 9]
}

impl SudokuRegion {
    pub fn from_indices(indices: &[usize; 9]) -> Self {
        let mut coords = [SudokuCoord {row: 0, col: 0}; 9];

        for index in 0..9 {
            coords[index] = SudokuCoord::from_index(indices[index]);
        }

        return Self {cells: coords};
    }

    pub fn from_coords(coords: &[SudokuCoord; 9]) -> Self {
        return Self {cells: coords.clone()};
    }

    pub fn from_rect(top: usize, left: usize) -> Self {
        let mut coords = [SudokuCoord {row: 0, col: 0}; 9];

        let mut index = 0;
        for row in top..(top + 3) {
            for col in left..(left + 3) {
                coords[index] = SudokuCoord {row, col};
                index += 1;
            }
        }

        return Self {cells: coords};
    }
}