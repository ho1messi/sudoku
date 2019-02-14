use super::utils::*;

use super::draft_list::DraftList;

#[derive(Clone, Debug)]
pub struct SudokuCell {
    digit: i32,
    drafts: DraftList,
    region_index: Option<usize>
}

impl SudokuCell {
    pub fn create(digit: i32) -> Self {
        return Self {digit, drafts: DraftList::new(), region_index: None};
    }

    pub fn get_digit(&self) -> i32 {
        return self.digit;
    }

    pub fn set_digit(&mut self, digit: i32) -> Result<(), Error> {
        if digit < 0 || digit >= 9 {
            return Err(Error::create(ErrorKind::BadSudokuDigit, "Bad digit to set"));
        }

        self.digit = digit;

        return Ok(());
    }

    pub fn set_draft(&mut self, num: i32, flag: bool) -> Result<(), Error> {
        return self.drafts.set_flag_at(num as usize, flag);
    }

    pub fn get_draft(&self, num: i32) -> Option<bool> {
        return self.drafts.get_flag_at(num as usize);
    }

    pub fn clear_drafts(&mut self) {
        self.drafts.clear();
    }
}