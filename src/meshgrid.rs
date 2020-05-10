extern crate ndarray;
use ndarray::{ArrayView1, ArrayView2};

/// Generates two matrices with one input vector as the
/// columns of one and the other input vector as the rows
/// of the second matrix.
///
/// # Parameters
/// * `a1` - input array 1
/// * `a2` - input array 2
///
/// # Returns
/// * Two matrices one with row vectors the other with column vectors
pub fn meshgrid<'a, 'b, A>(
    a1: &'a ArrayView1<A>,
    a2: &'b ArrayView1<A>,
) -> (ArrayView2<'a, A>, ArrayView2<'b, A>) {
    let rows = a1.shape()[0] as usize;
    let cols = a2.shape()[0] as usize;
    return (
        a1.broadcast((cols, rows)).unwrap().reversed_axes(),
        a2.broadcast((rows, cols)).unwrap(),
    );
}
