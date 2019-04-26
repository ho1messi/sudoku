use super::sudoku_size::*;


#[derive(Clone, Debug)]
pub struct Candidate {
    value: Vec<bool>,
    size_type: SudokuSizeType
}

impl Candidate {
    pub fn new() -> Self {
        let size_type = SudokuSizeType::Nine;
        return Self { value: vec![false; size_type.get_digit_num()], size_type};
    }

    pub fn with_size(size_type: SudokuSizeType) -> Self {
        return Self { value: vec![false; size_type.get_digit_num()], size_type};
    }

    pub fn create_full(size_type: SudokuSizeType) -> Self {
        return Self { value: vec![true; size_type.get_digit_num()], size_type};
    }

    pub fn get_flag(&self, index: usize) -> bool {
        if self.is_out_of_range(index) {
            panic!("candidate index out of range");
        }

        return self.value[index];
    }

    pub fn set_flag(&mut self, index: usize, flag: bool) {
        if self.is_out_of_range(index) {
            panic!("candidate index out of range");
        }

        self.value[index] = flag;
    }

    pub fn set_flag_and(&mut self, index: usize, flag: bool) {
        if self.is_out_of_range(index) {
            panic!("candidate index out of range");
        }

        self.value[index] &= flag;
    }

    pub fn set_flag_or(&mut self, index: usize, flag: bool) {
        if self.is_out_of_range(index) {
            panic!("candidate index out of range");
        }

        self.value[index] |= flag;
    }

    pub fn change_flag(&mut self, index: usize) {
        if index >= self.size_type.get_digit_num() {
            panic!("candidate index out of range");
        }

        self.value[index] = !self.value[index];
    }

    pub fn apply_mask(&mut self, mask: Candidate) {
        if self.size_type != mask.size_type {
            panic!("candidate mask must have a same size type");
        }

        for index in 0..self.size_type.get_digit_num() {
            self.set_flag_and(index, mask.get_flag(index));
        }
    }

    pub fn clear(&mut self) {
        self.value = vec![false; self.size_type.get_digit_num()];
    }

    pub fn is_out_of_range(&self, index: usize) -> bool {
        return index >= self.size_type.get_digit_num();
    }
}

impl<'a> IntoIterator for &'a Candidate {
    type Item = bool;
    type IntoIter = SudokuNoteIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        return Self::IntoIter{ value: &self.value, len: self.size_type.get_digit_num(), index: 0};
    }
}

pub struct SudokuNoteIter<'a> {
    value: &'a[bool],
    len: usize,
    index: usize
}

impl<'a> Iterator for SudokuNoteIter<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;

        if self.index == self.len {
            return None;
        }

        return Some(self.value[self.index]);
    }
}