use std::io::stdin;

///exponent constant offset (decoded exponent - BIAS)
const BIAS: i32 = 127;

///Floating point number radix for computer representation is 2
const RADIX: f32 = 2.0;


/// Parsing float number to sections - sign(1 bit), exponent(2..=9 bits), fraction(mantissa section)(10..=32 bits)
fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();

    /*Strips 31 unwanted bits
    away by shifting these
    nowhere, leaving only
    the sign bit */
    let sign = (bits >> 31) & 1;
    /*
    Filters out the top bit with a
    logical AND mask, then strips
    23 unwanted bits away
    */
    let exponent = (bits >> 23) & 0xff;
    
    //Retains only the 23 least The mantissa part significant bits via an AND mask
    let fraction = bits & 0x7fffff;

    /* 
    The mantissa part significant bits via an AND mask is called a fraction
    here as it becomes
    the mantissa once
    it’s decoded.
    */
    (sign, exponent, fraction)
}


///Decode parsed sections to valid values
fn decode(
    sign: u32,
    exponent: u32,
    fraction: u32,
) -> (f32, f32, f32) {

    /*
    Converts the sign bit to 1.0 or
    –1.0 (–1sign). Parentheses are
    required around –1.0_f32 to
    clarify operator precedence as
    method calls rank higher than
    a unary minus.
    */
    let signed_1 = (-1.0_f32).powf(sign as f32);
    /* 
    exponent must become an
    i32 in case subtracting the
    BIAS results in a negative
    number; then it needs to be
    cast as a f32 so that it can
    be used for exponentiation
    */
    let exponent = (exponent as i32) - BIAS;
    let exponent = RADIX.powf(exponent as f32);
    
    //sum of 2^-n where n - index of bit where value is 1
    let mut mantissa = 1.0;
    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = fraction & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }

    (signed_1, exponent, mantissa)

}


/// creating float number from sections
/// n = –1^sign_bit × mantissa × Radix^(exponent–Bias)
fn from_parts(
    sign: f32,
    exponent: f32,
    mantissa: f32,
) -> f32 {
    sign * mantissa * exponent
}

fn main() {
    let mut input = String::new();

    println!("Float number dissecter.");
    println!("Write float number: ");

    stdin().read_line(&mut input)
        .expect("Incorrect string");

    let n: f32 = input.trim().parse().unwrap();

    let (sign, exponent, fraction) = to_parts(n);
    let (sign_, exponent_, mantissa_) = decode(sign, exponent, fraction);
    let n_ = from_parts(sign_, exponent_, mantissa_);

    println!("{} -> {}", n, n_);
    println!("field     |   as bits                  |   as real number");
    println!("sign      | {:01b}                          | {}", sign, sign_);
    println!("exponent  | {:08b}                   | {}", exponent, exponent_);
    println!("mantissa  | {:023b}    | {}", fraction, mantissa_);
}
    



