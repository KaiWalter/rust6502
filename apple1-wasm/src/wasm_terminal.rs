use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
};
use wasm_bindgen::{prelude::*, Clamped, JsCast};
use web_sys::*;

const CHAR_HEIGHT: usize = 8;
const CHAR_WIDTH: usize = 8;
const ROWS: u32 = 25;
const COLS: u32 = 40;

pub struct WasmTerminal {
    rx_input: Receiver<u8>,
    rx_output: Receiver<u8>,
    context: CanvasRenderingContext2d,
    cursor_x: usize,
    cursor_y: usize,
    char_buffer: Vec<u8>,
    pixel_buffer: Vec<u8>,
}

impl WasmTerminal {
    pub fn new(rx_output: Receiver<u8>) -> WasmTerminal {
        // init canvas
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

        let (tx_input, rx_input) = mpsc::channel();

        // thread::spawn(move || loop {
        //     tx_input.send(getch() as u8).unwrap();
        // });

        WasmTerminal {
            rx_input,
            rx_output,
            context,
            cursor_x: 0,
            cursor_y: 0,
            char_buffer: vec![0; COLS as usize * ROWS as usize],
            pixel_buffer: vec![0; COLS as usize * CHAR_WIDTH * ROWS as usize * CHAR_HEIGHT * 4],
        }
    }

    // pub fn check_input(&self) -> Result<u8, TryRecvError> {
    //     self.rx_input.try_recv()
    // }

    pub fn event_loop(&mut self) {
        let mut screen_changed = false;

        loop {
            match self.rx_output.try_recv() {
                Ok(b) => {
                    let c = (b & !0x80).to_ascii_uppercase() as u8;
                    match c {
                        0x0A | 0x0D => {
                            self.cursor_x = 0;
                            self.cursor_y += 1;
                        }
                        _ => {
                            self.char_buffer[self.cursor_x + self.cursor_y * COLS as usize];
                            let sample: Vec<u8> = vec![
                                1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0,
                                0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0,
                                0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                            ];
                            self.display_char(self.cursor_x, self.cursor_y, &sample);
                            screen_changed = true;
                            self.cursor_x += 1;
                        }
                    }
                }
                Err(_) => break,
            }
        }

        if screen_changed {
            self.render();
        }
    }

    pub fn display_char(&mut self, x: usize, y: usize, char_pixels: &Vec<u8>) {
        for r in 0..=7 {
            for c in 0..=7 {
                let char_index = r * CHAR_WIDTH + c;
                let pixel_index =
                    ((y * CHAR_HEIGHT + r) * 40 * CHAR_WIDTH + (x * CHAR_WIDTH + c)) * 4;
                self.pixel_buffer[pixel_index + 0] = char_pixels[char_index] * 0x33;
                self.pixel_buffer[pixel_index + 1] = char_pixels[char_index] * 0xff;
                self.pixel_buffer[pixel_index + 2] = char_pixels[char_index] * 0x66;
                self.pixel_buffer[pixel_index + 3] = 0xff;
            }
        }
    }

    pub fn render(&self) {
        let image = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.pixel_buffer),
            COLS * CHAR_WIDTH as u32,
            ROWS * CHAR_HEIGHT as u32,
        )
        .unwrap();
        self.context.put_image_data(&image, 0., 0.).unwrap();
    }
}
