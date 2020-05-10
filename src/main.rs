extern crate argparse;
extern crate blas_src;
extern crate ndarray;
extern crate num;
mod antennaarray;
mod arrayimage;
mod beam;
mod meshgrid;
mod signals;

use antennaarray::AntennaArray;
use argparse::{ArgumentParser, Store, StoreTrue};
use arrayimage::ArrayImage;
use beam::Beam;
use ndarray::{stack, Array, Array1, Array2, Axis};
use num::complex::Complex;
use signals::AntennaSignals;
use std::iter::FromIterator;

fn main() {
    let mut dish_size = 1.0;
    let mut wavelength = 0.21;
    let mut duration = 8.0;
    let time_samples = 50;
    let mut num_dishes = 10;
    let mut image_size = 128;
    let mut sky = "stars".to_string();
    let mut save_sky = false;
    let mut array_type = "random".to_string();
    let mut scale = 50.0;
    let mut out_image_path = "out.jpg".to_string();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Simulation radio inteferometeric observations.");
        ap.refer(&mut sky)
            .add_option(&["--sky"], Store, "Sky source (`stars`)")
            .required();
        ap.refer(&mut num_dishes)
            .add_option(&["--count"], Store, "Number of dishes in the array")
            .required();
        ap.refer(&mut duration)
            .add_option(&["--duration"], Store, "Duration of observation in hours")
            .required();
        ap.refer(&mut wavelength)
            .add_option(&["--wavelength"], Store, "Wavelength (meters)")
            .required();
        ap.refer(&mut dish_size)
            .add_option(&["--size"], Store, "Dish diameter (meters)")
            .required();
        ap.refer(&mut image_size)
            .add_option(&["--image-size"], Store, "Height/width of image in pixels")
            .required();
        ap.refer(&mut save_sky)
            .add_option(&["--save-sky"], StoreTrue, "Flag to save sky as sky.jpg")
            .required();
        ap.refer(&mut array_type)
            .add_option(&["--type"], Store, "Array type (`random` | `spiral` | `y`)")
            .required();
        ap.refer(&mut scale)
            .add_option(&["--scale"], Store, "Dimension of array in meters")
            .required();
        ap.refer(&mut out_image_path)
            .add_option(&["--out"], Store, "Path to output image")
            .required();
        ap.parse_args_or_exit();
    }

    let src = if sky == "stars" {
        arrayimage::starfield(5, image_size)
    } else {
        arrayimage::starfield(5, image_size)
    };

    if save_sky {
        let filename = out_image_path
            .get(..out_image_path.rfind(".").unwrap())
            .unwrap();
        src.save(&format!("{}-sky.jpg", filename));
    }

    let pixels = src
        .to_vector()
        .mapv(|x| Complex::new(x, 0.))
        .insert_axis(Axis(1));

    let mut array = if array_type == "random" {
        AntennaArray::random(num_dishes, scale, dish_size)
    } else {
        AntennaArray::random(num_dishes, scale, dish_size)
    };
    let beamwidth = wavelength / dish_size;
    let beam = Beam::from_beamwidth(beamwidth, image_size);

    let mut all_baselines: Vec<Array2<f64>> = Vec::new();
    let mut all_xcorr: Vec<Array1<Complex<f64>>> = Vec::new();
    let step = duration / time_samples as f64;
    for _ in 0..time_samples {
        array.propagate(step);
        let uv = -array.as_uv(wavelength);
        let phases = beam.phases(&uv);
        let rx = AntennaSignals {
            signals: Array::from_iter(phases.dot(&pixels).iter().cloned()),
        };
        let xcorr = rx.xcorr();
        let baselines = array.baselines(wavelength);
        all_baselines.push(baselines);
        all_xcorr.push(xcorr);
    }
    // Convert to views, then stack
    let baselines_views: Vec<_> = all_baselines.iter().map(|x| x.view()).collect();
    let xcorr_views: Vec<_> = all_xcorr.iter().map(|x| x.view()).collect();
    let baselines = stack(Axis(0), &baselines_views).unwrap();
    let xcorr = stack(Axis(0), &xcorr_views).unwrap();
    let phases_bl = beam.phases(&baselines);
    let dirty_pixels = Array::from_iter(xcorr.insert_axis(Axis(0)).dot(&phases_bl).iter().cloned())
        .mapv(|x| x.norm());
    let dirty_image = ArrayImage::from_vector(&dirty_pixels);
    dirty_image.save(&out_image_path);
}
