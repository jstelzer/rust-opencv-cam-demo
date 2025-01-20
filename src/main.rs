use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

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

#[derive(Serialize, Deserialize)]
struct CammyOpts {
    threshold_1: f64,
    threshold_2: f64,
}

#[derive(Serialize, Deserialize)]
struct FrameFlags {
    invert: bool,
    canny: bool,
    greyscale: bool,
    blur: bool,
}

#[derive(Serialize, Deserialize)]
struct AppConfig {
    edge_thresholds: CammyOpts,
    flags: FrameFlags,
}

// on mac: 0 phone, 1 logitech, 2 is apple. When the logitech isn't plugged in 1 is builtin apple camera.
//         on mac, I prefer the logitech then the builtin camera generally so just default to 1 and use the best available.
// on linux: when cam is plugged in it's zero. or if its not plugged in this program will crash.
#[cfg(target_os = "linux")]
fn get_camera_offset() -> i32 {
    return 0;
}

#[cfg(target_os = "macos")]
fn get_camera_offset() -> i32 {
    return 1;
}

fn display_help_overlay(window: &str) -> opencv::Result<()> {
    let help_text = "Key bindings\nEsc: quit.\nSpace: toggle canny edges.\nc: toggle color invert.\nb: toggle blur.\nz: toggle greyscale.\nd: reset thresholds.\n+: increase t1.\n=: decrease t1.\n._: increase t2.\n-: decrease t2.\n";
    highgui::display_overlay(window, help_text, 5000)?;
    Ok(())
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
        core::AlgorithmHint::ALGO_HINT_DEFAULT,
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
    imgproc::cvt_color(
        &frame.clone(),
        &mut gray,
        imgproc::COLOR_BGR2GRAY,
        0,
        core::AlgorithmHint::ALGO_HINT_DEFAULT,
    )
    .unwrap();
    gray
}

fn process_frame(frame: &mut core::Mat, opts: &CammyOpts, flags: &FrameFlags) -> core::Mat {
    let mut result = frame.clone();
    if flags.greyscale {
        result = greyscale_frame(&mut result);
    }
    if flags.canny {
        result = canny_frame(&mut result, opts);
    }
    if flags.blur {
        result = blur_frame(&mut result);
    }
    if flags.invert {
        result = invert_frame(&mut result);
    }
    result
}

fn handle_key_press(
    key: i32,
    edge_thresholds: &mut CammyOpts,
    default_thresholds: &CammyOpts,
    flags: &mut FrameFlags,
) -> bool {
    match FromPrimitive::from_i32(key) {
        Some(KeyCodes::Esc) => return false,
        Some(KeyCodes::LowerD) => {
            edge_thresholds.threshold_1 = default_thresholds.threshold_1;
            edge_thresholds.threshold_2 = default_thresholds.threshold_2;
        }
        Some(KeyCodes::LowerB) => {
            flags.blur = !flags.blur;
        }
        Some(KeyCodes::LowerC) => {
            flags.invert = !flags.invert;
        }
        Some(KeyCodes::Space) => {
            flags.canny = !flags.canny;
        }
        Some(KeyCodes::LowerZ) => {
            flags.greyscale = !flags.greyscale;
        }
        Some(KeyCodes::Plus) => {
            edge_thresholds.threshold_1 += 1.0;
        }
        Some(KeyCodes::Equals) => {
            edge_thresholds.threshold_1 -= 1.0;
        }
        Some(KeyCodes::Underscore) => {
            edge_thresholds.threshold_2 += 1.0;
        }
        Some(KeyCodes::Minus) => {
            edge_thresholds.threshold_2 -= 1.0;
        }
        Some(KeyCodes::LowerH) => {
            if let Err(e) = display_help_overlay("Silly image transform") {
                println!("Error displaying help: {}", e);
            }
        }
        _ => println!("Unmapped key {}", key),
    }
    true
}

fn run() -> opencv::Result<()> {
    let window = "Silly image transform";
    // Replace this with WINDOW_NORMAL if you want the fancy new toolbar.
    highgui::named_window(window, highgui::WINDOW_GUI_NORMAL)?;
    let camera_offset = get_camera_offset();
    let mut cam = videoio::VideoCapture::new(camera_offset, videoio::CAP_ANY)?; // 0 is my phone, 1 is logitech, 2 is apple cameracd
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        return Err(opencv::Error::new(
            opencv::core::StsError,
            "Unable to open default camera!",
        ));
    }
    let default_thresholds = CammyOpts {
        threshold_1: 30.0,
        threshold_2: 40.0,
    };
    let mut edge_thresholds = CammyOpts {
        threshold_1: default_thresholds.threshold_1,
        threshold_2: default_thresholds.threshold_2,
    };
    let mut flags = FrameFlags {
        invert: true,
        canny: true,
        greyscale: true,
        blur: true,
    };
    let config_dir = format!("{}/.config/cannycam", std::env::var("HOME").unwrap());
    let config_file = format!("{}/settings.json", config_dir);
    if Path::new(&config_file).exists() {
        if let Ok(contents) = fs::read_to_string(&config_file) {
            if let Ok(saved) = serde_json::from_str::<AppConfig>(&contents) {
                edge_thresholds = saved.edge_thresholds;
                flags = saved.flags;
            }
        }
    }
    loop {
        let mut frame = core::Mat::default();
        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
            let processed = process_frame(&mut frame, &edge_thresholds, &flags);
            highgui::imshow(window, &processed)?;
        }
        let key = highgui::wait_key(10)?;
        if key > -1 {
            if !handle_key_press(key, &mut edge_thresholds, &default_thresholds, &mut flags) {
                break;
            }
        }
    }
    let new_config = AppConfig {
        edge_thresholds,
        flags,
    };
    fs::create_dir_all(&config_dir).ok();
    match serde_json::to_string_pretty(&new_config) {
        Ok(json_str) => {
            if let Err(e) = fs::write(&config_file, json_str) {
                eprintln!("Warning: Could not save settings: {}", e);
            }
        }
        Err(e) => eprintln!("Warning: Could not serialize settings: {}", e),
    }
    Ok(())
}

fn main() -> opencv::Result<()> {
    run()
}
