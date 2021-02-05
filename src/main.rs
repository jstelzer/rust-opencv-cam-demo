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
            // once we have a frame we grayscale
            let mut gray = core::Mat::default()?;
            imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
            // blur it so that the edges are not excessive
            let mut blur = core::Mat::default()?;
            imgproc::gaussian_blur(&gray, &mut blur, core::Size::new(5, 5), 5.0 , 5.0, core::BORDER_DEFAULT)?;
            // run edge detection
            let mut edges = core::Mat::default()?;
            imgproc::canny(&blur, &mut edges, 30.0, 40.0, 3, true)?;
            // invert the image so we have a white background instead of a black one.
            let mut inverted = core::Mat::default()?;
            // make the masking layer empty so the whole thing is inverted
            let no_mask = core::Mat::default()?;
            core::bitwise_not(&edges, &mut inverted, &no_mask)?;
            //show it.
            highgui::imshow(window, &mut inverted)?;
        }
        let key = highgui::wait_key(10)?;
        //TODO: Wire in keyboard controls to adjust edge detection thresholds as well as toggle everything else.
        if key > 0 && key != 255 {
            break;
        }
    }
    Ok(())
}

fn main() {
    run().unwrap()
}
