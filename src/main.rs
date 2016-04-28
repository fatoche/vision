extern crate camera_capture;
extern crate piston_window;
extern crate image;
extern crate texture;
extern crate imageproc;

use piston_window::{PistonWindow, Texture, WindowSettings, TextureSettings, clear};
use image::ConvertBuffer;
use std::sync::mpsc::*;
use imageproc::edges::canny;

fn main() {
    let window: PistonWindow =
        WindowSettings::new("piston: image", [300, 300])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut tex: Option<Texture<_>> = None;
    let (sender, receiver) = sync_channel(1);
    let imgthread = std::thread::spawn(move || {
        let cam = camera_capture::create(0).unwrap()
                                                    .fps(30.0)
                                                    .unwrap()
                                                    .start()
                                                    .unwrap();
        for frame in cam {
            match sender.try_send(frame.convert()) {
                Err(TrySendError::Disconnected(_)) => break,
                Err(_) => {},
                Ok(_) => {},

            }
        }
    });

    let (img_proc_sender, img_proc_receiver) = channel();
    let img_proc_thread = std::thread::spawn(move || {
        for frame in receiver {
            let mut mirror_frame = frame.clone();
            let width = frame.width();

            for (x, y, pixel) in frame.enumerate_pixels() {
                mirror_frame.put_pixel(width - x - 1, y, *pixel);
            }
            let new_image = canny(&mirror_frame, 100.0, 120.0);

            if let Err(_) = img_proc_sender.send(new_image.convert()) {
                break;
            }
        }
    });

    for e in window {
        if let Ok(frame) = img_proc_receiver.try_recv() {
            if let Some(mut t) = tex {
                t.update(&mut *e.encoder.borrow_mut(), &frame).unwrap();
                tex = Some(t);
            } else {
                tex = Texture::from_image(&mut *e.factory.borrow_mut(), &frame, &TextureSettings::new()).ok();
            }
        }
        e.draw_2d(|c, g| {
            clear([1.0; 4], g);
            if let Some(ref t) = tex {
                piston_window::image(t, c.transform, g);
            }
        });
    }
    drop(img_proc_receiver);
    imgthread.join().unwrap();
    img_proc_thread.join().unwrap();
}
