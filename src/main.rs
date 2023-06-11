use std::f64::consts::PI;

use cordic::{sincos, precompute};
use rand::prelude::*;

// angle in [-pi / 2, pi / 2)
fn get_random_angle90() -> f64 {
    (random::<f64>() - 0.5f64) * PI
}

fn test_errors(iterations: usize, n: usize) {
    println!("----- error test, iterations: {}, max steps: {} -----", iterations, n);

    let (mut sin_err_sum, mut sin_err_max) = (0f64, 0f64);
    let (mut cos_err_sum, mut cos_err_max) = (0f64, 0f64);

    for _ in 0..iterations {
        let angle = get_random_angle90();
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
        let angle = get_random_angle90();
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
 
    // 54 iterations are enough for full f64 angle matching (phi == theta)
    // in ~78% of cases (but not full accuracy, beacuse of fixed point)
    test_errors(100_000, 54);
    test_steps(100_000, 54);
}