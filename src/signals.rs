extern crate ndarray;
extern crate num;

use ndarray::{Array, Array1, Axis};
use num::complex::Complex;
use std::iter::FromIterator;

/// A struct to store an array of complex antenna signals
pub struct AntennaSignals {
    /// Array of complex antenna signals
    pub signals: Array1<Complex<f64>>,
}

impl AntennaSignals {
    /// Computes the cross correlation of all the signals
    ///
    /// # Parameters
    /// * None
    ///
    /// # Returns
    /// * Complex vector of signal correlations
    pub fn xcorr(&self) -> Array1<Complex<f64>> {
        let sigscol = self.signals.clone().insert_axis(Axis(1));
        let sigsrow = self.signals.clone().insert_axis(Axis(0)).mapv(|x| x.conj());
        let result = sigscol.dot(&sigsrow);
        Array::from_iter(result.iter().cloned())
    }
}
