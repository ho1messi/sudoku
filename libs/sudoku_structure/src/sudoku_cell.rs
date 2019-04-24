use super::candidate::*;
use super::sudoku_size::*;

use self::CellValue::*;

#[derive(Clone, Debug)]
pub enum CellValue {
    CVDigit(usize),
    CVCandidate(Candidate),
    CVEmpty
}

#[derive(Clone, Debug)]
pub struct SudokuCell {
    value: CellValue,
    size_type: SudokuSizeType,
    region_index: Option<usize>
}

impl SudokuCell {
    pub fn new() -> Self {
        return Self { value: CVEmpty, size_type: SudokuSizeType::Nine, region_index: None};
    }

    pub fn with_type(size_type: SudokuSizeType) -> Self {
        return Self {value: CVEmpty, size_type, region_index: None};
    }

    pub fn create(digit: usize, size_type: SudokuSizeType) -> Self {
        return Self { value: CVDigit(digit), size_type, region_index: None};
    }

    pub fn get_digit(&self) -> Option<usize> {
        match self.value {
            CVDigit(digit) => return Some(digit),
            _ => return None
        }
    }

    pub fn set_digit(&mut self, digit: usize) {
        self.value = CVDigit(digit);
    }

    pub fn clear_digit(&mut self) {
        self.value = CVEmpty;
    }

    pub fn set_note(&mut self, digit: usize, flag: bool) {
        match &mut self.value {
            CVDigit(_) => return,
            CVCandidate(note) => {
                note.set_flag(digit, flag);
            },
            CVEmpty => {
                let mut candidate = Candidate::with_size(self.size_type);
                candidate.set_flag(digit, true);
                self.value = CVCandidate(candidate);
            },
        }
    }

    pub fn change_note(&mut self, digit: usize) {
        match &mut self.value {
            CVDigit(_) => return,
            CVCandidate(note) => {
                note.change_flag(digit);
            },
            CVEmpty => {
                let mut candidate = Candidate::with_size(self.size_type);
                candidate.set_flag(digit, true);
                self.value = CVCandidate(candidate);
            },
        }
    }

    pub fn clear_notes(&mut self) {
        match self.value {
            CVDigit(_) => {},
            _ => self.value = CVEmpty
        }
    }

    pub fn fill_notes(&mut self) {
        match self.value {
            CVDigit(_) => {},
            _ => self.value = CVCandidate(Candidate::create_full(self.size_type)),
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