use super::utils::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct RegionIndex {
    value: usize
}

impl RegionIndex {
    pub fn zero() -> Self {
        return Self { value: 0 };
    }

    pub fn create(index: usize) -> Result<Self, Error> {
        if index >= 9 {
            return Err(Error::create(ErrorKind::IndexInvalid, "Bad index seted to region"));
        }

        return Ok(Self { value: index });
    }

    pub fn set(&mut self, index: usize) -> Result<(), Error> {
        if index >= 9 {
            return Err(Error::create(ErrorKind::IndexInvalid, "Bad index seted to region"));
        }

        self.value = index;
        return Ok(());
    }

    pub fn get(&self) -> usize {
        return self.value;
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct GridIndex {
    value: usize
}

impl GridIndex {
    pub fn zero() -> Self {
        return Self { value: 0 };
    }

    pub fn create(index: usize) -> Result<Self, Error> {
        if index >= 9 {
            return Err(Error::create(ErrorKind::IndexInvalid, "Bad index setted to grid"));
        }

        return Ok(Self { value: index });
    }

    pub fn set(&mut self, index: usize) -> Result<(), Error> {
        if index >= 9 {
            return Err(Error::create(ErrorKind::IndexInvalid, "Bad index setted to grid"));
        }

        self.value = index;
        return Ok(());
    }

    pub fn get(&self) -> usize {
        return self.value;
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct GridCoord {
    pub row: usize,
    pub col: usize
}

impl GridCoord {
    pub fn zero() -> Self {
        return Self {row: 0, col: 0};
    }

    pub fn from_index(index: GridIndex) -> Self {
        return Self {
            row: index.get() / 9,
            col: index.get() % 9
        };
    }

    pub fn to_index(&self) -> GridIndex {
        return GridIndex::create(self.row * 9 + self.col).unwrap();
    }
}

#[derive(Clone, Debug)]
pub struct SudokuRegion {
    cells: [GridIndex; 9]
}

impl SudokuRegion {
    pub fn from_indices(indices: &[GridIndex; 9]) -> Self {
        return Self { cells: indices.clone() };
    }

    pub fn from_coords(coords: &[GridCoord; 9]) -> Self {
        let mut indices = [GridIndex::zero(); 9];

        for i in 0..9 {
            indices[i] = coords[i].to_index();
        }

        return Self { cells: indices };
    }

    pub fn from_rect(top: usize, left: usize) -> Self {
        let mut indices = [GridIndex::zero(); 9];

        let mut i = 0;
        for row in top..(top + 3) {
            for col in left..(left + 3) {
                indices[i] = GridCoord { row, col }.to_index();
                i += 1;
            }
        }

        return Self { cells: indices };
    }

    pub fn include(&self, index: GridIndex) -> bool {
        for cell in self.cells.iter() {
            if *cell == index {
                return true;
            }
        }

        return false;
    }

    pub fn get_index(&self, index: RegionIndex) -> GridIndex {
        return self.cells[index.get()];
    }
}

impl IntoIterator for SudokuRegion {
    type Item = GridIndex;
    type IntoIter = SudokuRegionIter;

    fn into_iter(self) -> Self::IntoIter {
        return Self::IntoIter {cells: self.cells, index: 0};
    }
}

pub struct SudokuRegionIter {
    cells: [GridIndex; 9],
    index: usize
}

impl Iterator for SudokuRegionIter {
    type Item = GridIndex;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;

        if self.index == 9 {
            return None;
        }

        return Some(self.cells[self.index]);
    }
}
