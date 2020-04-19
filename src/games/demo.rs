use crate::display;
use crate::game;
use crate::inputs;
extern crate image;
use std::convert::TryInto;
use strum::IntoEnumIterator;

static BOOT: &'static [u8] = include_bytes!("../../assets/boot.png");

pub struct Demo {
    paused: bool,
    cursor_x: i32,
    cursor_y: i32,
    some_asset: Option<image::RgbaImage>,
}

impl Demo {
    pub fn new() -> Demo {
        Demo {
            paused: true,
            cursor_x: 0,
            cursor_y: 0,
            some_asset: None,
        }
    }
}

impl game::Game for Demo {
    fn get_target_fps(&self) -> f64 {
        60 as f64
    }

    fn handle_input(&mut self, _time: f64, map: &inputs::InputMap) {
        // Pause or unpause?
        if map.hit(inputs::Key::Space) {
            self.paused = !self.paused;
        }

        // Remember cursor.
        self.cursor_x = map.cursor_x;
        self.cursor_y = map.cursor_y;

        // Print everything we see.
        for key in inputs::Key::iter() {
            if map.hit(key) {
                info!("Hit: {}", key.as_ref());
            }
            if map.held(key) {
                info!("Held: {}", key.as_ref());
            }
        }
    }

    fn simulate(&mut self, _time: f64) {
        if self.paused {
            return;
        }
    }

    // the Javascript side passes a pointer to a buffer, the size of the corresponding canvas
    // and the current timestamp.
    fn render(&mut self, time: f64, screen: &mut display::Display) {
        let resolution = { screen.get_resolution() };

        // Do asset loading..
        if self.some_asset.is_none() {
            //let boot: Vec<u8> = block_on(scope::get_asset("boot.png")).unwrap();
            self.some_asset
                .replace(image::load_from_memory(BOOT).unwrap().into_rgba());
        }

        // Don't draw if paused.
        if self.paused {
            return;
        }

        // Draw a boot.
        if self.some_asset.is_some() {
            let boot: image::RgbaImage = self.some_asset.clone().unwrap();
            for (i, px) in screen.get_buffer_mut().iter_mut().enumerate() {
                // get the position of current pixel
                let h: u32 = (i / resolution.width).try_into().unwrap();
                let w: u32 = (i % resolution.width).try_into().unwrap();

                if h < boot.dimensions().1 && w < boot.dimensions().0 {
                    let bootpix = (*boot.get_pixel(w, h)).0;

                    if bootpix[3] > 10 {
                        px.r = bootpix[0];
                        px.g = bootpix[1];
                        px.b = bootpix[2];
                    }
                }
            }
        }
    }
}
