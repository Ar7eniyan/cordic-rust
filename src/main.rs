const MAX_N: usize = 100;

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

fn sincos(theta: f64, n: usize) -> (f64, f64) {
    // x and y are coordinates on a unit circle multiplied by 10^18
    // they are stored as integers to use fast multiplication by powers of 2
    let (mut x, mut y) = (10i64.pow(18), 0i64);
    // phi is the angle of (x, y) vector to the positive x-axis
    // we rotdte the vector iteratively to match phi with theta
    let mut phi = 0f64;
    let mut steps_made = n;

    for i in 0..n {
        println!("step {}: {} {} {}", i, x, y, phi);
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

    println!("steps made: {}", steps_made);
    // normalize the resulting vector, convert to float
    let x = x as f64 * cos_products()[steps_made - 1] / 10f64.powi(18);
    let y = y as f64 * cos_products()[steps_made - 1] / 10f64.powi(18);
    (y, x)
}

fn main() {
    unsafe { precompute() };

    // now it doesn't panic on low angles
    let angle = 0.001;

    println!("{:?}", sincos(angle, 25));
    println!("{:?}", (angle.sin(), angle.cos()));
}