extern crate image;
extern crate ndarray;
extern crate ndarray_stats;
extern crate rand;

use image::ImageBuffer;
use ndarray::{Array, Array1, Array2};
use ndarray_stats::errors::MinMaxError;
use ndarray_stats::QuantileExt;
use rand::distributions::{Distribution, Uniform};
use std::convert::TryInto;
use std::iter::FromIterator;

/// A struct representing an image
pub struct ArrayImage {
    /// An array of pixels that make up the image
    pixels: Array2<f64>,
}

impl ArrayImage {
    /// Create a new image from a vector of pixels
    ///
    /// # Parameters
    /// * `v` - vector of pixel values
    ///
    /// # Returns
    /// A new `ArrayImage`
    ///
    /// # Safety
    /// Unsafe due to reshape with unwrap
    pub fn from_vector(v: &Array1<f64>) -> ArrayImage {
        let n = (v.shape()[0] as f64).sqrt();
        ArrayImage {
            pixels: v.clone().into_shape((n as usize, n as usize)).unwrap(),
        }
    }

    /// Converts an image into a vector of pixel values
    ///
    /// # Parameters
    /// * None
    ///
    /// # Returns
    /// A vector of pixel values
    pub fn to_vector(&self) -> Array1<f64> {
        return Array::from_iter(self.pixels.iter().cloned());
    }

    /// Normalizes an input matrix of pixels
    ///
    /// # Parameters
    /// * `data` - A 2D array of pixel values
    ///
    /// # Returns
    /// The pixel values normalized between 0 and 1
    fn normalize(data: &Array2<f64>) -> Result<Array2<f64>, MinMaxError> {
        let max = data.max();
        let min = data.min();
        min.and_then(|v| max.map(|m| (data - *v) / *m))
    }

    /// Writes an image to the provided file path
    ///
    /// # Parameters
    /// * `path` - path to the destination image file
    ///
    /// # Safety
    /// Unsafe due to unwrap of image shape and casting of dimensions
    pub fn save(&self, path: &str) {
        let normalized = ArrayImage::normalize(&self.pixels).unwrap();
        let shape = normalized.raw_dim();
        let mut img = ImageBuffer::new(shape[1].try_into().unwrap(), shape[0].try_into().unwrap());
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let value = normalized[(y as usize, x as usize)];
            *pixel = image::Luma([(value * 255 as f64) as u8]);
        }
        img.save(path).unwrap();
    }
}

pub fn starfield(stars: u32, size: usize) -> ArrayImage {
    let mut result = Array2::zeros((size, size));
    let mut rng = rand::thread_rng();
    let gen = Uniform::from(0..size);
    for _ in 0..stars {
        let row = gen.sample(&mut rng);
        let col = gen.sample(&mut rng);
        result[[row, col]] = 1.0;
    }
    ArrayImage { pixels: result }
}
