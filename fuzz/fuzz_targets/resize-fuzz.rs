#![no_main]

extern crate resize;
use libfuzzer_sys::fuzz_target;
use rgb::FromSlice;
use resize::px::RGB;
use resize::Pixel::RGB8;
use resize::Type::Lanczos3;

fuzz_target!(|data: &[u8]| {
    let imgwidth = (data.len() as f64).sqrt() as usize;
    if imgwidth > 1 {
        let (w2, h2) = (100, 100);
        let dat_vec = data.to_vec();
        let rgb_vec = dat_vec.as_rgb();
        let mut dst = vec![RGB::new(0,0,0); w2*h2];
        let mut resizer = resize::new(imgwidth, imgwidth, w2, h2, RGB8, Lanczos3);
        // Do resize without heap allocations.
        // Might be executed multiple times for different `src` or `dst`.
        match resizer {
            Ok(mut ok_resizer) => {
                ok_resizer.resize(&rgb_vec, &mut dst);
            },
            Err(..) => ()
        }
    }
});
