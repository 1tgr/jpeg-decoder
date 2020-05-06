extern crate jpeg_decoder as jpeg;
extern crate png;
extern crate walkdir;

#[cfg(not(feature = "std"))]
extern crate core;

#[cfg(feature = "std")]
mod common;

#[cfg(feature = "std")]
mod crashtest;

#[cfg(feature = "std")]
mod reftest;

#[test]
fn read_info() {
    let data = include_bytes!("reftest/images/mozilla/jpg-progressive.jpg");

    let mut decoder = jpeg::Decoder::new(&data[..]);
    let ref_data = decoder.decode().unwrap();
    let ref_info = decoder.info().unwrap();

    decoder = jpeg::Decoder::new(&data[..]);
    decoder.read_info().unwrap();
    let info = decoder.info().unwrap();
    let data = decoder.decode().unwrap();

    assert_eq!(info, decoder.info().unwrap());
    assert_eq!(info, ref_info);
    assert_eq!(data, ref_data);
}
