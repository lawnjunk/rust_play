extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{MouseCursorEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

#[derive(Clone, Copy)]
pub struct MouseCursor {
    x: f64,
    y: f64,
}

pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
}

impl Color {
    fn to_array(&self) -> [f32; 4] {
        [
            self.red.clamp(0.0, 1.0),
            self.green.clamp(0.0, 1.0),
            self.blue.clamp(0.0, 1.0),
            self.alpha.clamp(0.0, 1.0),
        ]
    }
}

pub struct App {
    gl: GlGraphics,
    mouse_cursor: Option<MouseCursor>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let [width, height] = args.viewport().window_size;

        let mouse_cursor = self
            .mouse_cursor
            .or(Some(MouseCursor { x: 1.0, y: 1.0 }))
            .unwrap();

        let color_scale_x: f32 = (mouse_cursor.x / width) as f32;
        let color_scale_y: f32 = (mouse_cursor.y / width) as f32;

        let background_color = Color {
            red: 1.0 - color_scale_x,
            green: 1.0,
            blue: color_scale_y,
            alpha: 1.0,
        };

        let rectangle_color = Color {
            red: 0.0,
            green: 1.0 - color_scale_x,
            blue: 1.0 - color_scale_x,
            alpha: (color_scale_y / 2.0) + 0.5,
        };

        let inner_rectangle =
            rectangle::rectangle_by_corners(100.0, 100.0, width - 100.0, height - 100.0);

        self.gl.draw(args.viewport(), |ctx, gl| {
            clear(background_color.to_array(), gl);
            rectangle(
                rectangle_color.to_array(),
                inner_rectangle,
                ctx.transform,
                gl,
            );
        });
    }

    fn handle_mouse(&mut self, mouse_cursor: MouseCursor) {
        self.mouse_cursor = Some(mouse_cursor);
    }

    fn handle_update(&mut self, arg: &UpdateArgs) {
        println!("dt: {}", arg.dt);
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("generative design", [400, 400])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut event_source = Events::new(EventSettings::new());

    let mut app = App {
        gl: GlGraphics::new(opengl),
        mouse_cursor: None,
    };

    while let Some(event) = event_source.next(&mut window) {
        if let Some(args) = event.render_args() {
            app.render(&args);
        }

        if let Some(args) = event.mouse_cursor_args() {
            app.handle_mouse(MouseCursor {
                x: args[0],
                y: args[1],
            });
        }

        if let Some(args) = event.update_args() {
            app.handle_update(&args);
        }
    }
    println!("Hello, world!");
}
