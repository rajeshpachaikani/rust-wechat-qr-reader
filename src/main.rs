use opencv::prelude::*;
use opencv::{highgui, videoio, wechat_qrcode::WeChatQRCode};

fn main() {
    let mut scanner = WeChatQRCode::new(
        "./model/detect.prototxt",
        "./model/detect.caffemodel",
        "./model/sr.prototxt",
        "./model/sr.caffemodel",
    )
    .unwrap();
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap();
    let mut points = opencv::core::Vector::<opencv::core::Mat>::new();
    let mut frame = Mat::default();
    loop {
        cam.read(&mut frame).unwrap();
        let decoded = scanner.detect_and_decode(&frame, &mut points).unwrap();
        if points.len() > 0 {
            println!("{:?}", decoded);
        }

        highgui::imshow("winname", &frame).unwrap();
        highgui::wait_key(1).unwrap();
    }
}
