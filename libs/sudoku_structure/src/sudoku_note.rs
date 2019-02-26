use super::utils::*;
use super::sudoku_type::*;


#[derive(Clone, Debug)]
pub struct SudokuNote {
    v: Vec<bool>,
    t: SudokuType
}

impl SudokuNote {
    pub fn new() -> Self {
        let t = SudokuType::Nine;
        return Self { v: vec![false; t.get_digit_num()], t};
    }

    pub fn with_type(t: SudokuType) -> Self {
        return Self { v: vec![false; t.get_digit_num()], t};
    }

    pub fn with_digit(digit: usize, t: SudokuType) -> Result<Self, Error> {
        if digit >= t.get_digit_num() {
            return Err(Error::create(
                ErrorKind::BadSudokuDigit,
                "Wrong sudoku digit use when creating note"
            ));
        }

        let mut note = Self::with_type(t);
        note.set_flag(digit, true);
        return Ok(note);
    }

    pub fn create(flags: &[bool], t: SudokuType) -> Result<Self, Error> {
        if flags.len() < t.get_digit_num() {
            return Err(Error::create(
                ErrorKind::BadSudokuNoteNum,
                "Num of flags to create a note error"
            ));
        }

        return Ok(Self { v: Vec::from(flags), t});
    }

    pub fn get_flag(&self, index: usize) -> bool {
        if index >= self.t.get_digit_num() {
            panic!("index out of range");
        }

        return self.v[index];
    }

    pub fn set_flag(&mut self, index: usize, flag: bool) {
        if index >= self.t.get_digit_num() {
            panic!("index out of range");
        }

        self.v[index] = flag;
    }

    pub fn change_flag(&mut self, index: usize) {
        if index >= self.t.get_digit_num() {
            panic!("index out of range");
        }

        self.v[index] = !self.v[index];
    }

    pub fn clear(&mut self) {
        self.v = vec![false; self.t.get_digit_num()];
    }
}

impl<'a> IntoIterator for &'a SudokuNote {
    type Item = bool;
    type IntoIter = SudokuNoteIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        return Self::IntoIter{ v: &self.v, len: self.t.get_digit_num(), index: 0};
    }
}

pub struct SudokuNoteIter<'a> {
    v: &'a[bool],
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

        return Some(self.v[self.index]);
    }
}