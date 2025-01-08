extern crate opencv;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use opencv::{core, highgui, imgproc, prelude::*, videoio};

//TODO: https://crates.io/crates/keyboard-types way more refined way to do this.
#[derive(FromPrimitive)]
enum KeyCodes {
    Esc = 27,     //quit
    Space = 32,   // toggle canny edges
    LowerC = 99,  // toggle color invert
    LowerB = 98,  // toggle gausian blur
    LowerZ = 122, // toggle greyscale
    LowerD = 100, // reset thresholds to defaults
    //T1
    Plus = 43,   // increase edges (threshold 1)
    Equals = 61, // decrease edges (threshold 1)
    //T2
    Underscore = 95, // increase (threshold 2)
    Minus = 45,      // decrease (threshold 2)
    LowerH = 104,    //help
}

struct CammyOpts {
    threshold_1: f64,
    threshold_2: f64,
}

fn show_help() -> () {
    println!("Key bindings\nEsc: quit.\nSpace: toggle canny edges.\nc: toggle color invert.\nb: toggle blur.\nz: toggle greyscale.\nd: reset thresholds to defaults.\n+: increase t1.\n=: decrease t1.\n._: increase t2.\n-: decrease t2.\n");
    ()
}
fn invert_frame(frame: &mut core::Mat) -> core::Mat {
    let mut inverted = core::Mat::default();
    // make the masking layer empty so the whole thing is inverted
    let no_mask = core::Mat::default();
    core::bitwise_not(frame, &mut inverted, &no_mask).unwrap();
    inverted
}

fn blur_frame(frame: &mut core::Mat) -> core::Mat {
    let mut blur = core::Mat::default();
    imgproc::gaussian_blur(
        &frame.clone(),
        &mut blur,
        core::Size::new(5, 5),
        5.0,
        5.0,
        core::BORDER_DEFAULT,
    )
    .unwrap();
    blur
}

fn canny_frame(frame: &mut core::Mat, edge_opts: &CammyOpts) -> core::Mat {
    let mut edges = core::Mat::default();
    imgproc::canny(
        frame,
        &mut edges,
        edge_opts.threshold_1,
        edge_opts.threshold_2,
        3,
        true,
    )
    .unwrap();
    edges
}

fn greyscale_frame(frame: &mut core::Mat) -> core::Mat {
    let mut gray = core::Mat::default();
    imgproc::cvt_color(&frame.clone(), &mut gray, imgproc::COLOR_BGR2GRAY, 0).unwrap();
    gray
}

fn run() -> opencv::Result<()> {
    let window = "Silly image transform";
    highgui::named_window(window, 1)?;
    let mut cam = videoio::VideoCapture::new(1, videoio::CAP_ANY)?; // 0 is my phone, 1 is logitech, 2 is apple camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    let default_thresholds = CammyOpts {
        threshold_1: 30.0,
        threshold_2: 40.0,
    };
    let mut edge_thresholds = CammyOpts {
        threshold_1: default_thresholds.threshold_1,
        threshold_2: default_thresholds.threshold_2,
    };
    if !opened {
        panic!("Unable to open default camera!");
    }
    let mut invert_flag = true;
    let mut canny_flag = true;
    let mut greyscale_flag = true;
    let mut blur_flag = true;
    loop {
        let mut frame = core::Mat::default();
        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
            if greyscale_flag {
                frame = greyscale_frame(&mut frame);
            }
            if canny_flag {
                frame = canny_frame(&mut frame, &edge_thresholds);
            }
            if blur_flag {
                frame = blur_frame(&mut frame);
            }
            if invert_flag {
                frame = invert_frame(&mut frame);
            }
            //show it.
            highgui::imshow(window, &mut frame)?;
        }
        let key = highgui::wait_key(10)?;
        if key > -1 {
            // -1 is when nothing is pressed.
            match FromPrimitive::from_i32(key) {
                Some(KeyCodes::Esc) => break, //esc key
                Some(KeyCodes::LowerD) => {
                    edge_thresholds.threshold_1 = default_thresholds.threshold_1;
                    edge_thresholds.threshold_2 = default_thresholds.threshold_2;
                }
                Some(KeyCodes::LowerB) => {
                    blur_flag = !blur_flag;
                }
                Some(KeyCodes::LowerC) => {
                    invert_flag = !invert_flag;
                }
                Some(KeyCodes::Space) => {
                    canny_flag = !canny_flag;
                }
                Some(KeyCodes::LowerZ) => {
                    greyscale_flag = !greyscale_flag;
                }
                Some(KeyCodes::Plus) => {
                    edge_thresholds.threshold_1 = edge_thresholds.threshold_1 + 1.0;
                }
                Some(KeyCodes::Equals) => {
                    edge_thresholds.threshold_1 = edge_thresholds.threshold_1 - 1.0;
                }
                Some(KeyCodes::Underscore) => {
                    edge_thresholds.threshold_2 = edge_thresholds.threshold_2 + 1.0;
                }
                Some(KeyCodes::Minus) => {
                    edge_thresholds.threshold_2 = edge_thresholds.threshold_2 - 1.0;
                }
                Some(KeyCodes::LowerH) => show_help(),
                _ => println!("Unmapped key {}", key),
            }
        }
    }
    Ok(())
}

fn main() {
    run().unwrap()
}
