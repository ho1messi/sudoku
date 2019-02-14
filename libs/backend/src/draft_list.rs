use super::utils::*;

#[derive(Clone, Debug)]
pub struct DraftList {
    nums: [bool; 9]
}

impl DraftList {
    pub fn create(nums: &[bool; 9]) -> Self {
        return Self {nums: nums.clone()};
    }

    pub fn new() -> Self {
        return Self {nums: [false; 9]};
    }

    pub fn get_flag_at(&self, index: usize) -> Option<bool> {
        if index >= 9 {
            return None;
        }

        return Some(self.nums[index]);
    }

    pub fn set_flag_at(&mut self, index: usize, flag: bool) -> Result<(), Error> {
        if index >= 9 {
            return Err(Error::create(ErrorKind::BadSudokuDigit, "Bad digit seted to draft"));
        }

        self.nums[index] = flag;

        return Ok(());
    }

    pub fn clear(&mut self) {
        self.nums = [false; 9];
    }
}