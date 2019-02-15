#[derive(Debug)]
pub enum ErrorKind {
    IndexInvalid,
    BadSudokuDigit,
    BadSudokuRegionSize,
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    info: &'static str
}

impl Error {
    pub fn create(kind: ErrorKind, info: &'static str) -> Self {
        return Error {kind, info};
    }
}
