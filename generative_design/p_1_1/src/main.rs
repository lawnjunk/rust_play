extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::WindowSettings;
use piston::{ButtonEvent, RenderEvent};

fn main() {
    let mut settings = WindowSettings::new("color pallet", [400.0, 400.0]);
    settings.set_exit_on_esc(true);
    settings.set_resizable(false);

    let mut window: GlutinWindow = settings.build().unwrap();

    let opengl = OpenGL::V3_3;

    let mut gl = GlGraphics::new(opengl);
    let mut event_settings = EventSettings::new();
    event_settings.set_max_fps(10);

    let mut event_source = Events::new(event_settings);
    event_source.set_max_fps(1);

    println!("COLOR_PALLET with OPEN_GL{:?}", opengl.get_major_minor());

    // TODO @slugbyte use HSL
    // TODO @slugbyte use click to swap algorithms
    while let Some(event) = event_source.next(&mut window) {
        if let Some(_event) = event.button_args() {
            break;
        };

        if let Some(event) = event.render_args() {
            let viewport = event.viewport();
            let [width, height] = viewport.window_size;

            gl.draw(viewport, |ctx, gl| {
                graphics::clear([0.0, 0.0, 0.0, 1.0], gl);
                let mut x = 0.0;
                let x_step = width / 10.0;
                while x < width {
                    let color_step_x = (x / width) as f32;
                    let mut y = 0.0;
                    let y_step = height / 10.0;
                    while y < height {
                        let color_step = (y / width) as f32;
                        let green = [color_step, color_step_x, 1.0 - color_step, 1.0];
                        let rect =
                            graphics::rectangle::rectangle_by_corners(x, y, x + x_step, y + y_step);
                        graphics::rectangle(green, rect, ctx.transform, gl);
                        y += y_step;
                    }
                    x += x_step;
                }
            })
        };
    }

    println!("bye bye");
}
