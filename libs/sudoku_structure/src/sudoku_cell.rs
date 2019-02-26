use super::sudoku_type::*;
use super::sudoku_note::*;

use self::CellValue::*;

#[derive(Clone, Debug)]
pub enum CellValue {
    Digit(usize),
    Note(SudokuNote),
    Empty
}

#[derive(Clone, Debug)]
pub struct SudokuCell {
    v: CellValue,
    t: SudokuType,
    region_index: Option<usize>
}

impl SudokuCell {
    pub fn new() -> Self {
        return Self { v: Empty, t: SudokuType::Nine, region_index: None};
    }

    pub fn with_type(t: SudokuType) -> Self {
        return Self {v: Empty, t, region_index: None};
    }

    pub fn create(digit: usize, t: SudokuType) -> Self {
        return Self { v: Digit(digit), t, region_index: None};
    }

    pub fn get_digit(&self) -> Option<usize> {
        match self.v {
            Digit(digit) => return Some(digit),
            _ => return None
        }
    }

    pub fn set_digit(&mut self, digit: usize) {
        self.v = Digit(digit);
    }

    pub fn clear_digit(&mut self) {
        self.v = Empty;
    }

    pub fn set_note(&mut self, digit: usize, flag: bool) {
        match &mut self.v {
            Digit(_) => return,
            Note(note) => {
                note.set_flag(digit, flag);
            },
            Empty => {
                self.v = Note(SudokuNote::with_digit(digit, self.t).unwrap());
            },
        }
    }

    pub fn change_note(&mut self, digit: usize) {
        match &mut self.v {
            Digit(_) => return,
            Note(note) => {
                note.change_flag(digit);
            },
            Empty => {
                self.v = Note(SudokuNote::with_digit(digit, self.t).unwrap());
            },
        }
    }

    pub fn clear_notes(&mut self) {
        match self.v {
            Digit(_) => {},
            _ => self.v = Empty
        }
    }

    pub fn fill_notes(&mut self) {
        match self.v {
            Digit(_) => {},
            _ => self.v = Note(SudokuNote::create(
                &vec![true; self.t.get_digit_num()],
                self.t).unwrap()
            )
        }
    }

    pub fn get_region_index(&self) -> Option<usize> {
        return self.region_index;
    }

    pub fn set_region_index(&mut self, index: usize) {
        self.region_index = Some(index);
    }

    pub fn clear_region_index(&mut self) {
        self.region_index = None;
    }
}