extern crate opencv;

use opencv::{core, highgui, imgproc, prelude::*, videoio};


fn run() -> opencv::Result<()> {
    let window = "video capture";
    highgui::named_window(window, 1)?;
    let mut cam = videoio::VideoCapture::new(1, videoio::CAP_ANY)?; // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }
    loop {
        let mut frame = core::Mat::default()?;
        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
            let mut gray = core::Mat::default()?;
            imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
            let mut edges = core::Mat::default()?;
            imgproc::canny(&gray, &mut edges, 70.0, 45.0, 3, true)?;

            let mut inverted = core::Mat::default()?;
            let no_mask = core::Mat::default()?;
            core::bitwise_not(&edges, &mut inverted, &no_mask)?;
//            imgproc::gaussian_blur(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray, ksize: core::Size, sigma_x: f64, sigma_y: f64, border_type: i32)
            highgui::imshow(window, &mut inverted)?;
        }
        let key = highgui::wait_key(10)?;
        if key > 0 && key != 255 {
            break;
        }
    }
    Ok(())
}

fn main() {
    run().unwrap()
}
