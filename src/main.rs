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

fn main() {
    unsafe { precompute() };
}