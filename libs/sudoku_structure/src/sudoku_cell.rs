use super::sudoku_draft::*;

use self::CellValue::*;

#[derive(Copy, Clone, Debug)]
pub enum CellValue {
    Digit(SudokuDigit),
    Draft(SudokuDraft),
    Empty
}

#[derive(Copy, Clone, Debug)]
pub struct SudokuCell {
    value: CellValue,
    region_index: Option<usize>
}

impl SudokuCell {
    pub fn new() -> Self {
        return Self {value: Empty, region_index: None};
    }

    pub fn create(digit: SudokuDigit) -> Self {
        return Self {value: Digit(digit), region_index: None};
    }

    pub fn get_digit(&self) -> Option<SudokuDigit> {
        match self.value {
            Digit(digit) => return Some(digit),
            _ => return None
        }
    }

    pub fn set_digit(&mut self, digit: SudokuDigit) {
        self.value = Digit(digit);
    }

    pub fn clear_digit(&mut self) {
        self.value = Empty;
    }

    pub fn get_drafts(&self) -> [bool; 9] {
        match self.value {
            Draft(draft) => return draft.get_all(),
            _ => return [false; 9],
        }
    }

    pub fn set_draft(&mut self, digit: SudokuDigit, flag: bool) {
        let mut draft = SudokuDraft::new();
        match self.value {
            Digit(_) => return,
            Draft(draft_t) => {
                draft = draft_t;
                draft.set_flag(digit, flag);
            },
            Empty => draft.set_flag(digit, flag),
        }

        self.value = Draft(draft);
    }

    pub fn change_draft(&mut self, digit: SudokuDigit) {
        let mut draft = SudokuDraft::new();
        match self.value {
            Digit(_) => return,
            Draft(draft_t) => {
                draft = draft_t;
                draft.change_flag(digit);
            },
            Empty => draft.change_flag(digit),
        }

        self.value = Draft(draft);
    }

    pub fn clear_drafts(&mut self) {
        match self.value {
            Digit(_) => {},
            _ => self.value = Empty
        }
    }

    pub fn fill_drafts(&mut self) {
        match self.value {
            Digit(_) => {},
            _ => self.value = Draft(SudokuDraft::create(&[true; 9]))
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