extern crate rscam;
extern crate image;

use rscam::{Camera, Config};

fn main() {
    let mut camera = Camera::new("/dev/video0").unwrap();

    // find and print all possible camera configurations
    for format in camera.formats() {
        println!("{:?}", format); // Output format
        let blub = &format.unwrap().format;
        for resolution in camera.resolutions(blub) {
            println!("{:?}", resolution); // Camera resolution
            match resolution {
                rscam::ResolutionInfo::Discretes(v) => {
                    for (w, h) in v {
                        for interval in camera.intervals(blub, (w, h)) {
                            println!("{:?}", interval); // fps
                        }
                    }
                }
                rscam::ResolutionInfo::Stepwise{..} => {}
            }
        }
    }

    camera.start(&Config {
        interval: (1, 30),      // 30 fps.
        resolution: (640, 480),
        format: b"RGB3",
        ..Default::default()
    }).unwrap();

    // capture 10 images
    for i in 0..10 {
        let frame = camera.capture().unwrap();
        let buf: image::ImageBuffer<image::Rgb<u8>, _> = image::ImageBuffer::from_raw(frame.resolution.0, frame.resolution.1, frame).unwrap();
        buf.save(&format!("frame-{}.jpg", i)).unwrap();
    }
}
