mod utils;

use utils::set_panic_hook;
use wasm_bindgen::{prelude::*, Clamped, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

const CHAR_HEIGHT: usize = 8;
const CHAR_WIDTH: usize = 8;
const ROWS: u32 = 25;
const COLS: u32 = 40;

fn display_char(x: usize, y: usize, pixels: &mut Vec<u8>, char_pixels: &Vec<u8>) {
    for r in 0..=7 {
        for c in 0..=7 {
            let char_index = r * CHAR_WIDTH + c;
            let pixel_index = ((y * CHAR_HEIGHT + r) * 40 * CHAR_WIDTH + (x * CHAR_WIDTH + c)) * 4;
            pixels[pixel_index + 0] = char_pixels[char_index] * 0x33;
            pixels[pixel_index + 1] = char_pixels[char_index] * 0xff;
            pixels[pixel_index + 2] = char_pixels[char_index] * 0x66;
            pixels[pixel_index + 3] = 0xff;
        }
    }
}

fn init_context_canvas() -> (CanvasRenderingContext2d, HtmlCanvasElement) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    canvas.set_width(COLS * CHAR_WIDTH as u32);
    canvas.set_height(ROWS * CHAR_HEIGHT as u32);
    (context, canvas)
}

#[wasm_bindgen(start)]
pub fn start() {
    set_panic_hook();

    let (context, canvas) = init_context_canvas();

    let sample: Vec<u8> = vec![
        1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0,
        0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1,
        1, 1, 1, 1,
    ];

    let mut pixels: Vec<u8> = vec![0; 40 * 8 * 25 * 8 * 4];
    let mut offset = 0u8;

    for y in 0..ROWS as usize {
        for x in 0..COLS as usize {
            display_char(x, y, &mut pixels, &sample);
            let image = ImageData::new_with_u8_clamped_array_and_sh(
                Clamped(&pixels),
                COLS * CHAR_WIDTH as u32,
                ROWS * CHAR_HEIGHT as u32,
            )
            .unwrap();
            context.put_image_data(&image, 0., 0.).unwrap();
        }
    }
}
