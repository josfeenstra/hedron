// TODO rewrite this shit, or find the rust equivalents :)
// I prefer rewriting, it will teach me things about rust

// // name:    statistics.ts
// // author:  Jos Feenstra
// // purpose: functionality and documentation of variance,
// //          covariance, eigen vectors, least squares, and other
// //          statistical operations.

// // source:  been a while since I did this,
// //          https://datascienceplus.com/understanding-the-covariance-matrix/
// //          https://wiki.pathmind.com/eigenvector
// //          used to make sure the basics are correct :).
// // notes:   Whats the difference between a Principal Component and an Eigen vector?
// //          "Because eigenvectors trace the principal lines of force, and the axes of greatest variance and covariance illustrate where the data is most susceptible to change."

// import { FloatMatrix } from "../data/FloatMatrix";
// import { Matrix4, MultiVector2, MultiVector3 } from "../lib";

// export namespace Stat {
//     // calculate sum
//     export function sum(x: ArrayLike<number>) {
//         let sum = 0;
//         for (let i = 0; i < x.length; i++) {
//             sum += x[i];
//         }
//         return sum;
//     }

//     // calculate average
//     export function mean(x: ArrayLike<number>) {
//         return sum(x) / x.length;
//     }

//     // calculate weighted mean
//     export function meanWeighted(values: number[] | Float32Array, weights: number[] | Float32Array) {
//         if (values.length != weights.length) {
//             throw new Error("values & weights need same length");
//         }

//         var sum = 0.0;
//         var sumweight = 0.0;

//         for (let i = 0; i < values.length; i++) {
//             sum += values[i] * weights[i];
//             sumweight += weights[i];
//         }

//         return sum / sumweight;
//     }

//     // calculate variance
//     export function variance(x: number[] | Float32Array) {
//         //σ^2x = (1/n−1) * n∑i=1 (x[i] – xAvr)^2
//         let n = x.length;
//         let avr = mean(x);

//         let sum = 0;
//         for (let i = 0; i < n; i++) {
//             sum += (x[i] - avr) ** 2;
//         }
//         return sum / (n - 1);
//     }

//     // calculate the standard deviation
//     export function deviation(x: number[] | Float32Array) {
//         return variance(x) ** 0.5;
//     }

//     // calculate covariance
//     export function covariance(x: number[] | Float32Array, y: number[] | Float32Array) {
//         if (x.length != y.length) throw "this is not how covariance works...";
//         let n = x.length;
//         let xAvr = mean(x);
//         let yAvr = mean(y);

//         let sum = 0;
//         for (let i = 0; i < n; i++) {
//             sum += (x[i] - xAvr) * (y[i] - yAvr);
//         }
//         return sum / (n - 1); // losing one degree of freedom
//     }

//     // calculate variance / covariance matrix
//     export function cov(matrix: FloatMatrix) {
//         let size = matrix.width;
//         let cov = new FloatMatrix(size, size);

//         let columns = Array<Float32Array>(size);
//         for (let i = 0; i < size; i++) {
//             columns[i] = matrix.getColumn(i);
//         }

//         // matrix is symmertical, so only run through one half
//         for (let i = 0; i < size; i++) {
//             for (let j = i; j < size; j++) {
//                 let value = covariance(columns[i], columns[j]);
//                 cov.set(i, j, value);
//                 cov.set(j, i, value);
//             }
//         }
//         return cov;
//     }

//     export function eig(A: FloatMatrix) : [Float32Array, FloatMatrix] {
//         let results = svd(A);
//         return [results[1], results[2]];
//     }

//     /**
//      * Thin Single Value Decomposition.
//      * Can be used for Eigen Value Decomposition
//      * from G. H. Golub and C. Reinsch, Numer. Math. 14, 403-420 (1970).
//      * Taken from numeric.js. not yet cleaned & optimized.
//      * https://en.wikipedia.org/wiki/Singular_value_decomposition
//      * @param  {FloatMatrix} A matrix to decompose, such as a covariance matrix
//      * @returns [U, ∑, V]
//      * U -> during EVD, these are the eigen vectors of A transposed, if im not mistaken
//      * ∑ -> during EVD, these are the eigen values
//      * V -> during EVD, the columns are eigen vectors. NOT TRANSPOSED !!!!!
//      */
//     export function svd(A: FloatMatrix): [FloatMatrix, Float32Array, FloatMatrix] {
//         var prec = Math.pow(2, -52); // assumes double prec
//         var tolerance = 1e-64 / prec;
//         var itmax = 50;
//         var c = 0;
//         var i = 0;
//         var j = 0;
//         var k = 0;
//         var l = 0;

//         var u = A.clone().toNative();
//         var m = u.length;

//         var n = u[0].length;

//         if (m < n) throw "Need more rows than columns";

//         var e = new Array(n);
//         var q = new Array(n);
//         for (i = 0; i < n; i++) e[i] = q[i] = 0.0;
//         var v = rep([n, n], 0);

//         function pythag(a: number, b: number) {
//             a = Math.abs(a);
//             b = Math.abs(b);
//             if (a > b) return a * Math.sqrt(1.0 + (b * b) / a / a);
//             else if (b == 0.0) return a;
//             return b * Math.sqrt(1.0 + (a * a) / b / b);
//         }

//         //rep function, [JF] : dont know what this does exactly...
//         function rep(s: Array<number>, v: number, k = 0) {
//             let n = s[k];
//             let ret = Array(n);
//             let i;
//             if (k === s.length - 1) {
//                 for (i = n - 2; i >= 0; i -= 2) {
//                     ret[i + 1] = v;
//                     ret[i] = v;
//                 }
//                 if (i === -1) {
//                     ret[0] = v;
//                 }
//                 return ret;
//             }
//             for (i = n - 1; i >= 0; i--) {
//                 ret[i] = rep(s, v, k + 1);
//             }
//             return ret;
//         }

//         //Householder's reduction to bidiagonal form

//         var f = 0.0;
//         var g = 0.0;
//         var h = 0.0;
//         var x = 0.0;
//         var y = 0.0;
//         var z = 0.0;
//         var s = 0.0;

//         for (i = 0; i < n; i++) {
//             e[i] = g;
//             s = 0.0;
//             l = i + 1;
//             for (j = i; j < m; j++) s += u[j][i] * u[j][i];
//             if (s <= tolerance) g = 0.0;
//             else {
//                 f = u[i][i];
//                 g = Math.sqrt(s);
//                 if (f >= 0.0) g = -g;
//                 h = f * g - s;
//                 u[i][i] = f - g;
//                 for (j = l; j < n; j++) {
//                     s = 0.0;
//                     for (k = i; k < m; k++) s += u[k][i] * u[k][j];
//                     f = s / h;
//                     for (k = i; k < m; k++) u[k][j] += f * u[k][i];
//                 }
//             }
//             q[i] = g;
//             s = 0.0;
//             for (j = l; j < n; j++) s = s + u[i][j] * u[i][j];
//             if (s <= tolerance) g = 0.0;
//             else {
//                 f = u[i][i + 1];
//                 g = Math.sqrt(s);
//                 if (f >= 0.0) g = -g;
//                 h = f * g - s;
//                 u[i][i + 1] = f - g;
//                 for (j = l; j < n; j++) e[j] = u[i][j] / h;
//                 for (j = l; j < m; j++) {
//                     s = 0.0;
//                     for (k = l; k < n; k++) s += u[j][k] * u[i][k];
//                     for (k = l; k < n; k++) u[j][k] += s * e[k];
//                 }
//             }
//             y = Math.abs(q[i]) + Math.abs(e[i]);
//             if (y > x) x = y;
//         }

//         // accumulation of right hand gtransformations
//         for (i = n - 1; i != -1; i += -1) {
//             if (g != 0.0) {
//                 h = g * u[i][i + 1];
//                 for (j = l; j < n; j++) v[j][i] = u[i][j] / h;
//                 for (j = l; j < n; j++) {
//                     s = 0.0;
//                     for (k = l; k < n; k++) s += u[i][k] * v[k][j];
//                     for (k = l; k < n; k++) v[k][j] += s * v[k][i];
//                 }
//             }
//             for (j = l; j < n; j++) {
//                 v[i][j] = 0;
//                 v[j][i] = 0;
//             }
//             v[i][i] = 1;
//             g = e[i];
//             l = i;
//         }

//         // accumulation of left hand transformations
//         for (i = n - 1; i != -1; i += -1) {
//             l = i + 1;
//             g = q[i];
//             for (j = l; j < n; j++) u[i][j] = 0;
//             if (g != 0.0) {
//                 h = u[i][i] * g;
//                 for (j = l; j < n; j++) {
//                     s = 0.0;
//                     for (k = l; k < m; k++) s += u[k][i] * u[k][j];
//                     f = s / h;
//                     for (k = i; k < m; k++) u[k][j] += f * u[k][i];
//                 }
//                 for (j = i; j < m; j++) u[j][i] = u[j][i] / g;
//             } else for (j = i; j < m; j++) u[j][i] = 0;
//             u[i][i] += 1;
//         }

//         // diagonalization of the bidiagonal form
//         prec = prec * x;
//         for (k = n - 1; k != -1; k += -1) {
//             for (var iteration = 0; iteration < itmax; iteration++) {
//                 // test f splitting
//                 var test_convergence = false;
//                 for (l = k; l != -1; l += -1) {
//                     if (Math.abs(e[l]) <= prec) {
//                         test_convergence = true;
//                         break;
//                     }
//                     if (Math.abs(q[l - 1]) <= prec) break;
//                 }
//                 if (!test_convergence) {
//                     // cancellation of e[l] if l>0
//                     c = 0.0;
//                     s = 1.0;
//                     var l1 = l - 1;
//                     for (i = l; i < k + 1; i++) {
//                         f = s * e[i];
//                         e[i] = c * e[i];
//                         if (Math.abs(f) <= prec) break;
//                         g = q[i];
//                         h = pythag(f, g);
//                         q[i] = h;
//                         c = g / h;
//                         s = -f / h;
//                         for (j = 0; j < m; j++) {
//                             y = u[j][l1];
//                             z = u[j][i];
//                             u[j][l1] = y * c + z * s;
//                             u[j][i] = -y * s + z * c;
//                         }
//                     }
//                 }
//                 // test f convergence
//                 z = q[k];
//                 if (l == k) {
//                     //convergence
//                     if (z < 0.0) {
//                         //q[k] is made non-negative
//                         q[k] = -z;
//                         for (j = 0; j < n; j++) v[j][k] = -v[j][k];
//                     }
//                     break; //break out of iteration loop and move on to next k value
//                 }
//                 if (iteration >= itmax - 1) throw "Error: no convergence.";
//                 // shift from bottom 2x2 minor
//                 x = q[l];
//                 y = q[k - 1];
//                 g = e[k - 1];
//                 h = e[k];
//                 f = ((y - z) * (y + z) + (g - h) * (g + h)) / (2.0 * h * y);
//                 g = pythag(f, 1.0);
//                 if (f < 0.0) f = ((x - z) * (x + z) + h * (y / (f - g) - h)) / x;
//                 else f = ((x - z) * (x + z) + h * (y / (f + g) - h)) / x;
//                 // next QR transformation
//                 c = 1.0;
//                 s = 1.0;
//                 for (i = l + 1; i < k + 1; i++) {
//                     g = e[i];
//                     y = q[i];
//                     h = s * g;
//                     g = c * g;
//                     z = pythag(f, h);
//                     e[i - 1] = z;
//                     c = f / z;
//                     s = h / z;
//                     f = x * c + g * s;
//                     g = -x * s + g * c;
//                     h = y * s;
//                     y = y * c;
//                     for (j = 0; j < n; j++) {
//                         x = v[j][i - 1];
//                         z = v[j][i];
//                         v[j][i - 1] = x * c + z * s;
//                         v[j][i] = -x * s + z * c;
//                     }
//                     z = pythag(f, h);
//                     q[i - 1] = z;
//                     c = f / z;
//                     s = h / z;
//                     f = c * g + s * y;
//                     x = -s * g + c * y;
//                     for (j = 0; j < m; j++) {
//                         y = u[j][i - 1];
//                         z = u[j][i];
//                         u[j][i - 1] = y * c + z * s;
//                         u[j][i] = -y * s + z * c;
//                     }
//                 }
//                 e[l] = 0.0;
//                 e[k] = f;
//                 q[k] = x;
//             }
//         }

//         for (i = 0; i < q.length; i++) if (q[i] < prec) q[i] = 0;

//         //sort eigenvalues
//         var temp;
//         for (i = 0; i < n; i++) {
//             for (j = i - 1; j >= 0; j--) {
//                 if (q[j] < q[i]) {
//                     c = q[j];
//                     q[j] = q[i];
//                     q[i] = c;
//                     for (k = 0; k < u.length; k++) {
//                         temp = u[k][i];
//                         u[k][i] = u[k][j];
//                         u[k][j] = temp;
//                     }
//                     for (k = 0; k < v.length; k++) {
//                         temp = v[k][i];
//                         v[k][i] = v[k][j];
//                         v[k][j] = temp;
//                     }
//                     i = j;
//                 }
//             }
//         }

//         // let transposeS = (s: Float32Array, A: FloatMatrix) => {
//         //     let size = Math.min(A.width, A.height);
//         //     let St = FloatMatrix.zeros(size, size);
//         //     for (let i = 0; i < size; i++) {
//         //         St.set(i, i, 1 / s[i]);
//         //     }
//         //     return St;
//         // }
//         // let S = transposeS(sum, A);

//         let sum = new Float32Array(q);

//         return [FloatMatrix.fromNative(u), sum, FloatMatrix.fromNative(v)];
//     }

//     export function diagonalize(sum: Float32Array, size: number) {

//         let St = FloatMatrix.zeros(size, size);
//         for (let i = 0; i < size; i++) {
//             St.set(i, i, sum[i]);
//         }
//         return St;
//     }

//     export function diagonalizeInverse(sum: Float32Array, size: number) {

//         let St = FloatMatrix.zeros(size, size);
//         for (let i = 0; i < size; i++) {
//             St.set(i, i, 1 / sum[i]);
//         }
//         return St;
//     }

//     /**
//      * Calculate the pseudo inverse of a matrix:
//      * `M = UΣV†` -> `M† = V Σ† U`
//      * https://en.wikipedia.org/wiki/Singular_value_decomposition
//      * @param A
//      */
//     export function pinv(A: FloatMatrix) {

//         let [U, s, V] = svd(A);
//         let St = diagonalizeInverse(s, Math.min(A.width, A.height));

//         let mul = FloatMatrix.mulBtoA;
//         let Mt = mul(U.tp(), mul(St, V));
//         return Mt;
//     }

//     export function pinv2(A: FloatMatrix) {

//         console.log("PSEUDO INVERSE 2")

//         A.print();

//         let [U, s, Vt] = svd(A);
//         let S = diagonalizeInverse(s, Math.min(A.width, A.height));

//         console.log("[ PRINTING SVD: ]");
//         let mul = FloatMatrix.mulAtoB;

//         U.print();
//         S.print();
//         Vt.print();

//         let something = mul(U, mul(S, Vt));
//         something.print()

//         // console.log(V.width, V.height);
//         // Ut.print();

//         // let Mt = S.mul(Ut).mul(V);

//         return something;
//     }
// }

// function test() {

//     // console.log("[ CHECK SUM INVERSE ]")
//     // let result = Stat.svd(A);
//     // let [U, s, V] = result;
//     // let S = Stat.diagonalize(s, Math.min(A.width, A.height));

//     // U.print();
//     // S.print();
//     // V.print();

//     // A.print();
//     // U.mul(S.mul(V.tp())).print();

// }
