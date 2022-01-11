use glutin_window::GlutinWindow as Window;

use opengl_graphics::{GlGraphics, OpenGL};

use piston::{MouseCursorEvent, ButtonEvent};
use piston::event_loop::{EventSettings, Events};
use piston::window::WindowSettings;
use piston::input::*;

use picross_rs::game::Game;

pub fn show(game: Game) {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("Picross - Rust", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        mouse_coords: Vec2f{ x: 0.0, y: 0.0 },
        game
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(_args) = e.update_args() {
            println!("update_args()")
            // app.update(&args);
        }

        if let Some(args) = e.mouse_cursor_args() {
            app.on_mouse_move(&args);
        }

        if let Some(a) = e.button_args() {
            println!("mouse_cursor_args {:?}", a);
        }

        if let Some(button) = e.press_args() {
            app.on_button_press(&button);
        }
    }
}

struct Vec2f {
    x: f64,
    y: f64
}
pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    mouse_coords: Vec2f,
    game: Game
}

const BG_COLOR: [f32; 4] = [0.9, 0.9, 0.9, 1.0];
const LINE_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let width = args.window_size[0];
        let height = args.window_size[1];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BG_COLOR, gl);

            let transform = c.transform;

            let line = line::Line::new(LINE_COLOR, 1.0);
            let w = width / self.game.board.width() as f64;
            for n_col in 1..self.game.board.width() {
                let x = n_col as f64 * w;
                line.draw([x, 0.0, x, height], &Default::default(), transform, gl)
            }

            let h = height / self.game.board.height() as f64;
            for n_row in 1..self.game.board.height() {
                let y = n_row as f64 * h;
                line.draw([0.0, y, width, y], &Default::default(), transform, gl)
            }

            Rectangle::new_border(LINE_COLOR, 0.0)
                .draw([0.0, 0.0, width-1.0, height-1.0], &Default::default(), transform, gl);
        });
    }

    fn on_mouse_click(&mut self, button: &MouseButton) {
        if let &MouseButton::Left = button {
            // self.selected_cell = Some(field::Coords{
            //     x: (self.mouse_coords.x / self.settings.cell_size.x) as u8,
            //     y: (self.mouse_coords.y / self.settings.cell_size.y) as u8 });
        }
    }

    pub fn on_mouse_move(&mut self, args: &[f64; 2]) {
        self.mouse_coords.x = args[0];
        self.mouse_coords.y = args[1];
    }

    pub fn on_button_press(&mut self, button: &Button) {
        match button {
            &Button::Keyboard(_key) => {
                // self.on_key_down(&key);
            },
            &Button::Mouse(button) => {
                self.on_mouse_click(&button);
            }
            _ => {}
        }
    }
}