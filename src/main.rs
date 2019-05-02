#[macro_use]
extern crate conrod;

use conrod::backend::glium::glium::{self, Surface};
use crate::sudoku_app::SudokuApp;

mod utils;
mod sudoku_app;

mod interfaces;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window_builder = glium::glutin::WindowBuilder::new()
        .with_title("Sudoku")
        .with_dimensions(WIDTH, HEIGHT);
    let context_builder = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window_builder, context_builder, &events_loop).unwrap();

    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64])
        .theme(utils::theme::theme())
        .build();

    let ids = utils::ids::Ids::new(ui.widget_id_generator());

    let mut app = SudokuApp::new();

    let mut image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    let mut event_loop = utils::event_loop::EventLoop::new();

    'main: loop {
        for event in event_loop.next(&mut events_loop) {
            if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }

            match event {
                glium::glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        glium::glutin::WindowEvent::Closed |
                        glium::glutin::WindowEvent::KeyboardInput {
                            input: glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape), ..
                            },
                            ..
                        } => break 'main,
                        _ => (),
                    }
                }
                _ => (),
            }
        }

        interfaces::main_menu::gui(&mut ui.set_widgets(), &ids, &mut app);

        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}
