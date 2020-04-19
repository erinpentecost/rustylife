use std::convert::TryFrom;
use wasm_bindgen::prelude::*;
use web_sys;

#[wasm_bindgen]
#[derive(Clone, PartialEq, Eq)]
pub struct Resolution {
    pub width: usize,
    pub height: usize,
}

pub struct Display {
    width: usize,
    height: usize,
    buffer: Vec<u8>,
    context: web_sys::CanvasRenderingContext2d,
    dirty: bool,
}

impl Display {
    pub fn new(context: web_sys::CanvasRenderingContext2d) -> Display {
        let canvas = context.canvas().unwrap();
        let buff_size = canvas.width() * canvas.height() * 4;
        Display {
            width: canvas.width() as usize,
            height: canvas.height() as usize,
            buffer: vec![255; usize::try_from(buff_size as usize).unwrap()],
            context,
            dirty: true,
        }
    }

    // The resolution could change between frames.
    // This will return the current resolution.
    pub fn get_resolution(&mut self) -> Resolution {
        Resolution {
            width: self.width,
            height: self.height,
        }
    }

    // Exposes a mutable slice on the buffer as pixels.
    // Edit the contents of this slice for something to be rendered.
    // This is how you render to the canvas.
    pub fn get_buffer_mut(&mut self) -> &mut [rgb::RGBA8] {
        self.dirty = true;
        let num_pixels = self.width * self.height;
        unsafe {
            std::slice::from_raw_parts_mut(self.buffer.as_mut_ptr() as *mut rgb::RGBA8, num_pixels)
        }
    }

    // Renders the buffer to the attached drawing context
    pub fn render(&mut self) {
        if self.dirty {
            // Cast graphics buffer to JS memory object
            let clamped: wasm_bindgen::Clamped<&mut [u8]> = wasm_bindgen::Clamped::<&mut [u8]>(
                &mut self.buffer[0..self.width * self.height * 4],
            );
            // Wrap JS memory object into an Image
            let image_bin = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
                clamped,
                self.width as u32,
                self.height as u32,
            )
            .unwrap();
            // Write the Image to the canvas
            self.context
                .put_image_data(&image_bin, 0 as f64, 0 as f64)
                .unwrap();
        }
        self.dirty = false;
    }
}
