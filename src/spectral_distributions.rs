use std::collections::hash_map::Values;

use crate::math::util::{clamp, lerp};

const LAMBDA_MIN: i32 = 360;
const LAMBDA_MAX: i32 = 830;

/// A spectral distribution.
pub trait Spectrum {
    /// Returns the value of the spectrum evaluated at the given wavelength.
    fn operator(&self, lambda: f64) -> f64;
    /// Returns the maximum value the distribution attains.
    fn max_value(&self) -> f64;
}

/// A spectrum with a constant distribution.
pub struct ConstantSpectrum {
    /// The value of the distribution at any point in its domain.
    value: f64,
}

impl Spectrum for ConstantSpectrum {
    fn operator(&self, _lambda: f64) -> f64 {
        self.value
    }

    fn max_value(&self) -> f64 {
        self.value
    }
}

/// A spectral distribution that is explicitly sampled at every (integer) point in its domain.
///
/// Needless to say, this uses a lot of memory (about 2kB for the visible spectrum).
pub struct DenselySampledSpectrum {
    /// The lower bound of the distribution.
    lambda_min: i32,
    /// The upper bound of the distribution.
    lambda_max: i32,
    /// The values of the distribution evaluated at each integer (1nm) wavelength in its domain.
    values: Vec<f64>,
}

impl DenselySampledSpectrum {
    /// Create a densely sampled spectrum by sampling another spectrum at every wavelength in the domain.
    fn from(spectrum: impl Spectrum, lambda_min: i32, lambda_max: i32) -> DenselySampledSpectrum {
        let mut values: Vec<f64> = Vec::with_capacity((lambda_max - lambda_min) as usize + 1);
        for lambda in lambda_min..=lambda_max {
            values[(lambda - lambda_min) as usize] = spectrum.operator(lambda.into());
        }
        DenselySampledSpectrum {
            lambda_min,
            lambda_max,
            values,
        }
    }
}

impl Spectrum for DenselySampledSpectrum {
    fn operator(&self, lambda: f64) -> f64 {
        let offset = lambda.round() as i32 - self.lambda_min;
        if offset < 0 || offset >= self.values.len().try_into().unwrap() {
            return 0.0;
        }
        self.values[offset as usize]
    }

    fn max_value(&self) -> f64 {
        *self
            .values
            .iter()
            // stupid floats dont have total order because of NaN >:(
            .reduce(|acc, elem| if elem > acc { elem } else { acc })
            .expect("NAN found in densely sampled spectrum")
    }
}

// A spectral distribution that linearly interpolates between points.
pub struct PiecewiseLinearSpectrum {
    lambdas: Vec<f64>,
    values: Vec<f64>,
}

impl PiecewiseLinearSpectrum {
    /// Create a new piecewise linear spectral distribution. The lists of wavelengths (`lambdas`) and values for those wavelengths (`values`) must be the same length, and must be sorted by wavelength.
    fn new(lambdas: Vec<f64>, values: Vec<f64>) -> PiecewiseLinearSpectrum {
        debug_assert_eq!(lambdas.len(), values.len());
        if !lambdas.iter().is_sorted() {
            panic!("Unsorted values (wavelengths) passed to a PiecewiseLinearSpectrum");
        }
        PiecewiseLinearSpectrum { lambdas, values }
    }
}

impl Spectrum for PiecewiseLinearSpectrum {
    fn operator(&self, lambda: f64) -> f64 {
        if self.lambdas.is_empty()
            || lambda
                < *self.lambdas.first().expect(
                    "PiecewiseLinearSpectrum must have wavelengths defined (cannot be empty)",
                )
            || lambda
                > *self.lambdas.last().expect(
                    "PiecewiseLinearSpectrum must have wavelengths defined (cannot be empty)",
                )
        {
            return 0.0;
        }
        // Find the interval in which `lambda` lies
        let o = find_interval(&self.lambdas, lambda);
        let t = (lambda - self.lambdas[o]) / (self.lambdas[o + 1] - self.lambdas[o]);

        lerp(t, self.values[o], self.values[o + 1])
    }

    fn max_value(&self) -> f64 {
        if self.values.is_empty() {
            return 0.0;
        }
        *self
            .values
            .iter()
            .reduce(|acc, e| if e > acc { e } else { acc })
            .unwrap()
    }
}

/// Returns the index of the larges element of `values` that is less than or equal to `value`.
///
/// TODO: Note that pbrt does this differently (using a function object) so that might be worth doing later.
fn find_interval<T: PartialOrd>(values: &[T], value: T) -> usize {
    let sz = values.len();
    let mut size = sz - 2;
    let mut first = 1;
    while size > 0 {
        let half = size >> 1;
        let middle = first + half;
        if values[middle] <= value {
            first = middle + 1;
            size -= half + 1;
        } else {
            size = half;
        }
    }
    clamp(first - 1, 0, sz - 2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_interval_basic() {
        let xs = vec![0, 2, 4];
        let x = 1;
        assert_eq!(find_interval(&xs, x), 0)
    }
    #[test]
    fn test_find_interval_basic_2() {
        let xs = vec![0, 2, 4];
        let x = 2;
        assert_eq!(find_interval(&xs, x), 1)
    }
    #[test]
    fn test_find_interval_floats_out_of_range_below() {
        let xs = vec![-0.0, 2.1, 4.5];
        let x = 1.5;
        assert_eq!(find_interval(&xs, x), 0)
    }
    #[test]
    fn test_find_interval_floats_out_of_range_above() {
        let xs = vec![-0.0, 2.1, 4.5];
        let x = 10.5;
        assert_eq!(find_interval(&xs, x), 1)
    }
}
