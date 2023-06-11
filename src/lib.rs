// maximum number of iterations can be only up to 64,
// since ue use bit shifts on 64-bit integers
const MAX_N: usize = 64;

// angles, tangents of which are powers of 2,
// namely such that tan(TAN_ANGLES[i]) = 2^-i
static mut TAN_ANGLES: [f64; MAX_N] = [0.0; MAX_N];
// accumulated product of cos(TAN_ANGLES[i]), multiplied by 10^-18,
// where cos(TAN_ANGLES[i]) is equal to 1 / sqrt(2 ^ -2i + 1)
// 10^-18 is to convert fixed point i64 to floating point, where 1.0 = 10^18
static mut COS_PRODUCTS: [f64; MAX_N] = [0.0; MAX_N];

/// precompute all the needed values, should only be called once
pub fn precompute() {  unsafe {
    TAN_ANGLES[0] =  1f64.atan();
    COS_PRODUCTS[0] = 1f64 / 2f64.sqrt() * 10f64.powi(-18);

    for i in 1..MAX_N {
        TAN_ANGLES[i] =  2f64.powi(-(i as i32)).atan();
        COS_PRODUCTS[i] = COS_PRODUCTS[i - 1] / (2f64.powi(-2 * i as i32) + 1f64).sqrt();
    }
}}

// rust is really hard with static mut variables
#[inline(always)]
fn tan_angles() -> &'static [f64] {
    unsafe { &TAN_ANGLES }
}
#[inline(always)]
fn cos_products() -> &'static [f64] {
    unsafe { &COS_PRODUCTS }
}

pub fn sincos(theta: f64, n: usize) -> (f64, f64, usize) {
    const SINCOS_10E18_I: i64 = 10i64.pow(18);

    // x and y are coordinates on a unit circle multiplied by 10^18
    // they are stored as fixed point integers to use fast multiplication by powers of 2
    let (mut x, mut y) = (SINCOS_10E18_I, 0i64);
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
    let x = x as f64 * cos_products()[steps_made - 1];
    let y = y as f64 * cos_products()[steps_made - 1];
    (y, x, steps_made)
}

// a tiny bit faster version that uses unsafe and doesn't return number of steps made
// assumes that n is in [1; 64]
pub fn sincos_faster(theta: f64, n: usize) -> (f64, f64) {
    const SINCOS_10E18_I: i64 = 10i64.pow(18);

    // x and y are coordinates on a unit circle multiplied by 10^18
    // they are stored as fixed point integers to use fast multiplication by powers of 2
    let (mut x, mut y) = (SINCOS_10E18_I, 0i64);
    // phi is the angle of (x, y) vector to the positive x-axis
    // we rotdte the vector iteratively to match phi with theta
    let mut phi = 0f64;
    let mut steps_made = n;

    for i in 0..n {
        if phi < theta {
            // rotate clockwise
            phi += unsafe { tan_angles().get_unchecked(i) };
            (x, y) = (x - (y >> i), y + (x >> i));
        } else if phi > theta {
            // rotate counter-clockwise
            phi -= unsafe { tan_angles().get_unchecked(i) };
            (x, y) = (x + (y >> i), y - (x >> i));
        } else {
            // if phi == theta, we are done
            steps_made = i;
            break;
        }
    }

    // normalize the resulting vector, convert to float
    let cos_prod = unsafe { cos_products().get_unchecked(steps_made - 1) };
    let x = x as f64 * cos_prod;
    let y = y as f64 * cos_prod;
    (y, x)
}