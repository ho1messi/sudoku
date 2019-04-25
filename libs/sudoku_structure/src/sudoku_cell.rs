use super::candidate::*;
use super::sudoku_size::*;

use self::CellValue::*;

#[derive(Clone, Debug)]
pub enum CellValue {
    CVDigit(usize),
    CVCandidate(Candidate)
}

#[derive(Clone, Debug)]
pub struct SudokuCell {
    value: CellValue,
    size_type: SudokuSizeType
}

impl SudokuCell {
    pub fn new() -> Self {
        let size_type = SudokuSizeType::Nine;
        return Self { value: CVCandidate(Candidate::with_size(size_type)), size_type};
    }

    pub fn with_type(size_type: SudokuSizeType) -> Self {
        return Self {value: CVCandidate(Candidate::with_size(size_type)), size_type};
    }

    pub fn with_digit(digit: usize, size_type: SudokuSizeType) -> Self {
        return Self { value: CVDigit(digit), size_type};
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
        self.value = CVCandidate(Candidate::with_size(self.size_type));
    }

    pub fn set_candidate(&mut self, digit: usize, flag: bool) {
        match &mut self.value {
            CVDigit(_) => return,
            CVCandidate(candidate) => {
                candidate.set_flag(digit, flag);
            }
        }
    }

    pub fn change_candidate(&mut self, digit: usize) {
        match &mut self.value {
            CVDigit(_) => return,
            CVCandidate(candidate) => {
                candidate.change_flag(digit);
            }
        }
    }

    pub fn clear_candidates(&mut self) {
        match self.value {
            CVDigit(_) => {},
            _ => self.value = CVCandidate(Candidate::with_size(self.size_type))
        }
    }

    pub fn fill_candidates(&mut self) {
        match self.value {
            CVDigit(_) => {},
            _ => self.value = CVCandidate(Candidate::create_full(self.size_type))
        }
    }

    pub fn apply_candidate_mask(&mut self, mask: Candidate) {
        match &mut self.value {
            CVDigit(_) => {},
            CVCandidate(candidate) => {
                candidate.apply_mask(mask);
            }
        }
    }
}