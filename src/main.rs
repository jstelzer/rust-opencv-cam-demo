extern crate opencv;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use opencv::{core, highgui, imgproc, prelude::*, videoio};

#[derive(FromPrimitive)]
enum KeyCodes {
    Esc = 27,
    Space = 32,
    LowerC = 99,
    LowerZ = 122,
    Plus = 43,
    Underscore = 95,
    Equals = 61,
    Minus = 45,
}

fn invert_frame(frame: &mut core::Mat) -> core::Mat {
    let mut inverted = core::Mat::default().unwrap();
    // make the masking layer empty so the whole thing is inverted
    let no_mask = core::Mat::default().unwrap();
    core::bitwise_not(frame, &mut inverted, &no_mask).unwrap();
    return inverted;
}

fn canny_frame(frame: &mut core::Mat) -> core::Mat {
    // blur it so that the edges are not excessive
    let mut blur = core::Mat::default().unwrap();
    imgproc::gaussian_blur(
        &frame.clone(),
        &mut blur,
        core::Size::new(5, 5),
        5.0,
        5.0,
        core::BORDER_DEFAULT,
    )
    .unwrap();
    let mut edges = core::Mat::default().unwrap();
    imgproc::canny(&blur, &mut edges, 30.0, 40.0, 3, true).unwrap();
    return edges;
}

fn greyscale_frame(frame: &mut core::Mat) -> core::Mat {
    let mut gray = core::Mat::default().unwrap();
    imgproc::cvt_color(&frame.clone(), &mut gray, imgproc::COLOR_BGR2GRAY, 0).unwrap();
    return gray;
}
fn run() -> opencv::Result<()> {
    let window = "Silly image transform";
    highgui::named_window(window, 1)?;
    let mut cam = videoio::VideoCapture::new(1, videoio::CAP_ANY)?; // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }
    let mut invert_flag = true;
    let mut canny_flag = true;
    let mut greyscale_flag = true;
    loop {
        let mut frame = core::Mat::default()?;
        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
            if greyscale_flag {
                frame = greyscale_frame(&mut frame);
            }
            // once we have a frame we grayscale
            if canny_flag {
                frame = canny_frame(&mut frame);
            }
            if invert_flag {
                frame = invert_frame(&mut frame);
            }
            //show it.
            highgui::imshow(window, &mut frame)?;
        }
        let key = highgui::wait_key(10)?;
        //TODO: Wire in keyboard controls to adjust edge detection thresholds as well as toggle everything else.
        // keys come back as the decimal value of the ascii(7) code set.
        if key > -1 {
            // -1 is when nothing is pressed.
            match FromPrimitive::from_i32(key) {
                Some(KeyCodes::Esc) => break, //esc key
                Some(KeyCodes::LowerC) => {
                    println!("Got key {}", key);
                    invert_flag = !invert_flag;
                }
                Some(KeyCodes::Space) => {
                    canny_flag = !canny_flag;
                }
                Some(KeyCodes::LowerZ) => {
                    greyscale_flag = !greyscale_flag;
                }
                _ => println!("Unmapped key {}", key),
            }
        }
    }
    Ok(())
}

fn main() {
    run().unwrap()
}
