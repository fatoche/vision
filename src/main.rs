extern crate rscam;
extern crate image;

use rscam::{Camera, Config};

fn main() {
    let mut camera = Camera::new("/dev/video0").unwrap();

    camera.start(&Config {
        interval: (1, 30),      // 30 fps.
        resolution: (640, 480),
        format: b"RGB3",
        ..Default::default()
    }).unwrap();

    for i in 0..10 {
        let frame = camera.capture().unwrap();
        let buf: image::ImageBuffer<image::Rgb<u8>, _> = image::ImageBuffer::from_raw(frame.resolution.0, frame.resolution.1, frame).unwrap();
        buf.save(&format!("frame-{}.jpg", i)).unwrap();
    }
}
