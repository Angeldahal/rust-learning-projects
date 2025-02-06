use opencv::{videoio, highgui, imgproc, prelude::*};

fn main() -> opencv::Result<()> {
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // Open default webcam

    if !videoio::VideoCapture::is_opened(&cam)? {
        panic!("Cannot open camera");
    }

    loop {
        let mut frame = Mat::default();
        cam.read(&mut frame)?;

        if frame.empty() {
            break;
        }

        // Convert to grayscale
        let mut gray = Mat::default();
        imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

        highgui::imshow("Live", &gray)?;
        if highgui::wait_key(10)? == 27 {
            break; // Exit if 'Esc' is pressed
        }
    }

    Ok(())
}
