use crate::utils;
use crate::sudoku_app::SudokuApp;
use conrod::widget::*;
use conrod::color::Colorable;
use conrod::position::Positionable;

pub fn gui(ui: &mut conrod::UiCell, ids: &utils::ids::Ids, app: &mut SudokuApp) {
    conrod::widget::Canvas::new()
        .pad(0.0)
        .color(conrod::color::BLUE)
        .set(ids.sudoku_canvas, ui);
    conrod::widget::Line::new([0.0, 0.0], [20.0, 20.0])
        .top_left_of(ids.sudoku_canvas)
        .color(conrod::color::YELLOW)
        .set(ids.line1, ui);
    conrod::widget::Line::new([0.0, 0.0], [50.0, 20.0])
        .top_left_of(ids.sudoku_canvas)
        .color(conrod::color::YELLOW)
        .set(ids.line2, ui);
}
