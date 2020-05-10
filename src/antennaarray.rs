extern crate ndarray;
extern crate ndarray_rand;
extern crate rand;

use crate::meshgrid::meshgrid;
use ndarray::{arr2, stack, Array, Array2, Axis};
use ndarray_rand::RandomExt;
use rand::distributions::Uniform;
use std::f64::consts::PI;
use std::iter::FromIterator;

/// A struct storing information about an antenna array
pub struct AntennaArray {
    /// Antenna positions as stacked row vectors (in meters)
    pub positions: Array2<f64>,
    /// Parabolic dish size in meters
    pub dish_size: f64,
}

impl AntennaArray {
    /// Converts the antenna positions to (u,v) coordinates
    ///
    /// # Parameters
    /// * `wavelength` - the wavelength of the observation in meters
    ///
    /// # Returns
    /// * Matrix of stacked row vectors of (u,v) coordinates
    pub fn as_uv(&self, wavelength: f64) -> Array2<f64> {
        self.positions.mapv(|x| x / wavelength)
    }

    /// Returns the (u,v) coordinates for all the baselines
    /// in the array.
    ///
    /// # Parameters
    /// * `wavelength` - wavlength of the observation in meters
    ///
    /// # Returns
    /// * Matrix of stacked row vectors of (u,v) coordinates
    pub fn baselines(&self, wavelength: f64) -> Array2<f64> {
        let xcol = self.positions.column(0);
        let ycol = self.positions.column(1);
        let (X1, X2) = meshgrid(&xcol, &xcol);
        let (Y1, Y2) = meshgrid(&ycol, &ycol);
        let x = Array::from_iter(X1.iter().cloned()) - Array::from_iter(X2.iter().cloned());
        let y = Array::from_iter(Y1.iter().cloned()) - Array::from_iter(Y2.iter().cloned());
        stack![Axis(1), x.insert_axis(Axis(1)), y.insert_axis(Axis(1))] / wavelength
    }

    /// Propagates the antenna positions for an elapsed time.
    ///
    /// # Parameters
    /// * `hours` - number of hours to propagate
    ///
    /// # Returns
    /// Nothing. `AntennaArray` is modified in place.
    pub fn propagate(&mut self, hours: f64) {
        let angle = hours / 24. * 2. * PI;
        let s = angle.sin();
        let c = angle.cos();
        let rotmat = arr2(&[[c, -s], [s, c]]);
        self.positions = self.positions.dot(&rotmat);
    }

    /// Generates an array of random antennas
    ///
    /// # Parameters
    /// * `num_antennas` - number of antennas in the array
    /// * `scale` - the max width/height of the antenna array in meters
    /// * `size` - the parabolic dish size in meters
    pub fn random(num_antennas: usize, scale: f64, size: f64) -> AntennaArray {
        AntennaArray {
            positions: Array::random((num_antennas, 2), Uniform::from(0.0..scale)) - scale / 2.0,
            dish_size: size,
        }
    }
}
