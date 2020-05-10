extern crate ndarray;
extern crate num;

use crate::meshgrid::meshgrid;
use ndarray::{stack, Array, Array2, Axis};
use num::complex::Complex;
use std::iter::FromIterator;

/// A set of samples representing coordinates in the (l,m) plane
pub struct Beam {
    /// Stacked row vectors of (l,m) coordinates
    pub samples: Array2<f64>,
}

impl Beam {
    /// Creates a new beam from a given beamwidth and dish size.
    ///
    /// # Parameters
    /// * `beamwidth` - antenna beamwidth in radians
    /// * `n` - number of samples per side of square beam
    ///
    /// # Returns
    /// `Beam` - the initialized `Beam` struct
    pub fn from_beamwidth(beamwidth: f64, n: usize) -> Beam {
        let l = beamwidth * Array::range(-1., 1., 2. / (n as f64));
        let lview = l.view();
        let (L, M) = meshgrid(&lview, &lview);
        let l_vec = Array::from_iter(L.iter().cloned()).insert_axis(Axis(1));
        let m_vec = Array::from_iter(M.iter().cloned()).insert_axis(Axis(1));
        let lm = stack![Axis(1), l_vec, m_vec];
        Beam { samples: lm }
    }

    /// Calculates the phase offset of each (l,m) point in the beam
    /// for the provided array of (u,v) coordinates
    ///
    /// # Parameters
    /// * `z` - matrix of stacked row (u,v) coordinates
    ///
    /// # Returns
    /// Matrix of phases with rows for each antenna and columns
    /// for each (l,m) beam point
    pub fn phases(&self, z: &Array2<f64>) -> Array2<Complex<f64>> {
        let zdots = z.dot(&self.samples.t());
        zdots.mapv(|x| Complex::new(0., 2. * x).exp())
    }
}
