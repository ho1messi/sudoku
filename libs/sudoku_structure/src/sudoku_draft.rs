use super::utils::*;

#[derive(Copy, Clone, Debug)]
pub struct SudokuDigit {
    value: i32
}

impl SudokuDigit {
    pub fn create(digit: i32) -> Result<Self, Error> {
        if digit < 0 || digit >= 9 {
            return Err(Error::create(ErrorKind::BadSudokuDigit, "Bad digit to set"));
        }

        return Ok(Self { value: digit });
    }

    pub fn set(&mut self, digit: i32) -> Result<(), Error> {
        if digit < 0 || digit >= 9 {
            return Err(Error::create(ErrorKind::BadSudokuDigit, "Bad digit to set"));
        }

        self.value = digit;
        return Ok(());
    }

    pub fn get(&self) -> i32 {
        return self.value;
    }

    pub fn is_odd(&self) -> bool {
        return self.value % 2 != 0;
    }

    pub fn is_even(&self) -> bool {
        return self.value % 2 == 0;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SudokuDraft {
    flags: [bool; 9]
}

impl SudokuDraft {
    pub fn create(flags: &[bool; 9]) -> Self {
        return Self { flags: flags.clone()};
    }

    pub fn new() -> Self {
        return Self { flags: [false; 9]};
    }

    pub fn get_flag(&self, digit: SudokuDigit) -> bool {
        return self.flags[digit.get() as usize];
    }

    pub fn get_all(&self) -> [bool; 9] {
        return self.flags;
    }

    pub fn set_flag(&mut self, digit: SudokuDigit, flag: bool) {
        self.flags[digit.get() as usize] = flag;
    }

    pub fn change_flag(&mut self, digit: SudokuDigit) {
        let index = digit.get() as usize;
        self.flags[index] = !self.flags[index];
    }

    pub fn clear(&mut self) {
        self.flags = [false; 9];
    }
}

impl IntoIterator for SudokuDraft {
    type Item = bool;
    type IntoIter = SudokuDraftIter;

    fn into_iter(self) -> Self::IntoIter {
        return Self::IntoIter{flags: self.flags, index: 0};
    }
}

pub struct SudokuDraftIter {
    flags: [bool; 9],
    index: usize
}

impl Iterator for SudokuDraftIter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;

        if self.index >= 9 {
            return None;
        }

        return Some(self.flags[self.index]);
    }
}