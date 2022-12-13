// UNTESTED
use crate::data::Matrix;

pub struct Stat {}

impl Stat {
    pub fn sum(x: &Vec<f32>) -> f32 {
        x.iter().sum()
    }

    pub fn mean(x: &Vec<f32>) -> f32 {
        Stat::sum(x) / (x.len() as f32)
    }

    pub fn mean_weighted(x: &Vec<f32>, y: &Vec<f32>) -> Result<f32, &'static str> {
        if x.len() != y.len() {
            return Err("values & weights need same length");
        }

        let mut sum = 0.0;
        let mut sumweight = 0.0;

        for i in 0..x.len() {
            sum += x[i] * y[i];
            sumweight += y[i];
        }

        Ok(sum / sumweight)
    }

    pub fn variance(x: &Vec<f32>) -> f32 {
        let n = x.len();
        let avr = Stat::mean(x);

        let mut sum = 0.0;
        for i in 0..n {
            sum += (x[i] - avr).powi(2);
        }

        sum / ((n - 1) as f32) // losing one degree of freedom
    }

    pub fn deviation(arr: &Vec<f32>) -> f32 {
        Stat::variance(&arr).powf(0.5)
    }

    pub fn covariance(x: &Vec<f32>, y: &Vec<f32>) -> Result<f32, &'static str> {
        if x.len() != y.len() {
            return Err("values & weights need same length");
        }

        let n = x.len();
        let x_avr = Stat::mean(x);
        let y_avr = Stat::mean(y);

        let mut sum = 0.0;
        for i in 0..n {
            sum += (x[i] - x_avr) * (y[i] - y_avr);
        }
        Ok(sum / ((n - 1) as f32)) // losing one degree of freedom
    }

    // variance / covariance matrix
    pub fn cov(m: Matrix) -> Matrix {
        let size = m.width;
        let mut cov = Matrix::new(size, size);

        // matrix is symmetrical, so only run through one half
        for i in 0..size {
            for j in i..size {
                let value = Stat::covariance(&m.get_column(i), &m.get_column(j)).unwrap();
                cov.set(i, j, value);
                cov.set(j, i, value);
            }
        }
        return cov;
    }

    fn eig() {
        // mathru
    }

    /**
     * Thin Single Value Decomposition.
     * Can be used for Eigen Value Decomposition
     * from G. H. Golub and C. Reinsch, Numer. Math. 14, 403-420 (1970).
     * Taken from numeric.js. not yet cleaned & optimized.
     * https://en.wikipedia.org/wiki/Singular_value_decomposition
     * @param  {FloatMatrix} A matrix to decompose, such as a covariance matrix
     * @returns [U, ∑, V]
     * U -> during EVD, these are the eigen vectors of A transposed, if im not mistaken
     * ∑ -> during EVD, these are the eigen values
     * V -> during EVD, the columns are eigen vectors. NOT TRANSPOSED !!!!!
     */
    fn singular_value_decomposition(m: Matrix) {
        // NOTE: this should be outsources
        // mathru
    }

    /**
     * Calculate the pseudo inverse of a matrix:
     * `M = UΣV†` -> `M† = V Σ† U`
     * https://en.wikipedia.org/wiki/Singular_value_decomposition
     * @param A
     */
    fn pseudo_inverse(m: Matrix) {
        // TODO
        // mathru
    }
}
