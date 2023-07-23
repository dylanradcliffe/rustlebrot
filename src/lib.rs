use std::f64;
use std::num;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::Element;
use web_sys::{CanvasRenderingContext2d, ImageData};

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn draw_mb(canvas: Element, xoff: f64, yoff: f64, scale: f64, iters: i32) {
    //let document = web_sys::window().unwrap().document().unwrap();
    //let canvas = document.get_element_by_id("canvas").unwrap();

    //alert(&format!("Hello, {}!", name));

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

    draw(
        &context,
        canvas.width(),
        canvas.height(),
        xoff,
        yoff,
        scale,
        iters,
    );
}

static XOFF: f64 = -2.0 + 2.47 / 2.0;
//static MHEIGHT: f64 = 2.24;
static YOFF: f64 = -1.12 + 2.24 / 2.0;
//static YSCALE: f64 = 2.24;
//2.47
fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    xoff: f64,
    yoff: f64,
    mheight: f64,
    iters: i32,
) -> Result<(), JsValue> {
    // The real workhorse of this algorithm, generating pixel data

    let scale = mheight / (height as f64);

    let mut data = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let xf: f64 = ((x as f64) - (width as f64) / 2.0) * scale + xoff;
            let yf: f64 = ((y as f64) - (height as f64) / 2.0) * scale + yoff;
            data.append(&mut colour(escape_time(xf, yf, iters), iters))
        }
    }
    let img_data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&data), width, height)?;
    ctx.put_image_data(&img_data, 0.0, 0.0)
}

static MAX_ITERATION: i32 = 40;
//static COL_BASE: i32 = 4;
//static COL_SCALE: i32 = (256 / COL_BASE) as i32;

fn escape_time(x0: f64, y0: f64, iters: i32) -> i32 {
    let mut i = 0;
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;

    while (i < iters) && (x * x + y * y <= 4.0) {
        (x, y) = (x * x - y * y + x0, 2.0 * x * y + y0);
        i += 1;
        //   print!("{}\n", i)
    }
    i
}

fn rainbow_colour(ifrac: f64) -> Vec<u8> {
    let sec: i32 = ((ifrac * 6.0) as i32);
    let dir = 1 - 2 * (sec % 2);
    let y: f64 = ifrac - (sec as f64) / 6.0;
    let z: u8 = (-1 * dir * ((y * 256.0) as i32)) as u8;

    match sec {
        0 => vec![255, z, 0, 255],
        1 => vec![z, 255, 0, 255],
        2 => vec![0, 255, z, 255],
        3 => vec![0, z, 255, 255],
        4 => vec![z, 0, 255, 255],
        5 => vec![z, 0, 255, 255],
        _ => vec![255, 0, z, 255],
    }
}

fn colour(i: i32, iters: i32) -> Vec<u8> {
    if i >= iters {
        vec![0, 0, 0, 255]
    } else {
        rainbow_colour(1.0 - (i as f64) / (iters as f64))
    }
}
