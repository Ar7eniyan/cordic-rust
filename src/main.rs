use std::f64::consts::PI;

use rand::prelude::*;

// maximum number of iterations can be only up to 64,
// since ue use bit shifts on 64-bit integers
const MAX_N: usize = 64;

// angles, tangents of which are powers of 2,
// namely such that tan(TAN_ANGLES[i]) = 2^-i
static mut TAN_ANGLES: [f64; MAX_N] = [0.0; MAX_N];
// accumulated product of cos(TAN_ANGLES[i]),
// equal to product 1 / sqrt(2 ^ -2i + 1)
static mut COS_PRODUCTS: [f64; MAX_N] = [0.0; MAX_N];

unsafe fn precompute() {
    TAN_ANGLES[0] =  1f64.atan();
    COS_PRODUCTS[0] = 1f64 / 2f64.sqrt();

    for i in 1..MAX_N {
        TAN_ANGLES[i] =  2f64.powi(-(i as i32)).atan();
        COS_PRODUCTS[i] = COS_PRODUCTS[i - 1] / (2f64.powi(-2 * i as i32) + 1f64).sqrt();
    }
}

// rust is really hard with static mut variables
#[inline(always)]
fn tan_angles() -> &'static [f64] {
    unsafe { &TAN_ANGLES }
}
#[inline(always)]
fn cos_products() -> &'static [f64] {
    unsafe { &COS_PRODUCTS }
}

fn sincos(theta: f64, n: usize) -> (f64, f64, usize) {
    // x and y are coordinates on a unit circle multiplied by 10^18
    // they are stored as integers to use fast multiplication by powers of 2
    let (mut x, mut y) = (10i64.pow(18), 0i64);
    // phi is the angle of (x, y) vector to the positive x-axis
    // we rotdte the vector iteratively to match phi with theta
    let mut phi = 0f64;
    let mut steps_made = n;

    for i in 0..n {
        // println!("step {}: {} {} {}", i, x, y, phi);
        if phi < theta {
            // rotate clockwise
            phi += tan_angles()[i];
            (x, y) = (x - (y >> i), y + (x >> i));
        } else if phi > theta {
            // rotate counter-clockwise
            phi -= tan_angles()[i];
            (x, y) = (x + (y >> i), y - (x >> i));
        } else {
            // if phi == theta, we are done
            steps_made = i;
            break;
        }
    }

    // println!("steps made: {}", steps_made);
    // normalize the resulting vector, convert to float
    let x = x as f64 * cos_products()[steps_made - 1] / 10f64.powi(18);
    let y = y as f64 * cos_products()[steps_made - 1] / 10f64.powi(18);
    (y, x, steps_made)
}

fn test_errors(iterations: usize, n: usize) {
    println!("----- error test, iterations: {}, max steps: {} -----", iterations, n);

    let (mut sin_err_sum, mut sin_err_max) = (0f64, 0f64);
    let (mut cos_err_sum, mut cos_err_max) = (0f64, 0f64);

    for _ in 0..iterations {
        let angle = random::<f64>() * PI / 2f64;
        let (sin, cos, _) = sincos(angle, n);
        
        let sin_err = (sin - angle.sin()).abs();
        sin_err_sum += sin_err;
        sin_err_max = sin_err_max.max(sin_err);
        
        let cos_err = (cos - angle.cos()).abs();
        cos_err_sum += cos_err;
        cos_err_max = cos_err_max.max(cos_err);
    }

    println!("mean/max absolute sin error: {}/{}", sin_err_sum / iterations as f64, sin_err_max);
    println!("mean/max absolute cos error: {}/{}", cos_err_sum / iterations as f64, cos_err_max);

    println!("----- end of error test -----\n");
}

fn test_steps(iterations: usize, n: usize) {
    println!("----- steps test, iterations: {}, max steps: {} -----", iterations, n);

    let mut steps_total = 0;
    let mut steps_distr_total = vec![0u64; n + 1];
    let mut steps_distr = vec![0f64; n + 1];

    for _ in 0..iterations {
        let angle = random::<f64>() * PI / 2f64;
        let (_, _, steps) = sincos(angle, n);

        steps_total += steps;
        steps_distr_total[steps] += 1;
    }

    for i in 0..=n {
        steps_distr[i] = steps_distr_total[i] as f64 / iterations as f64;
    }

    println!("mean steps: {}", steps_total as f64 / iterations as f64);
    println!("steps distribution:");
    for i in 0..=n {
        println!("\t{}: {}", i, steps_distr[i]);
    }

    println!("----- end of steps test -----\n");
}

fn main() {
    unsafe { precompute() };
 
    test_errors(1_000_000, 54);
    test_steps(1_000_000, 64);
}