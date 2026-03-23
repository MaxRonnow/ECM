use num_integer::gcd;
use rand::RngExt;

pub fn ecm(n: i64) -> (i64, i64) {
    let mut rng = rand::rng();

    loop {
        let cap_a: i64 = rng.random::<i64>() % n;
        let a: i64 = rng.random::<i64>() % n;
        let b: i64 = rng.random::<i64>() % n;

        let cap_p = (a, b);
        // let cap_b = (b * b - a * a * a - cap_a * a) % n;

        // E: y^2 = (x^3 + Ax + B) mod n

        let mut iter = 2;
        let mut d: i64 = 0;
        let mut div_failed = false;
        let mut prev_point = cap_p;

        while !div_failed {
            let mut temp_point = prev_point;
            let mut dividend: i64;
            let mut divisor: i64;
            for _ in 1..iter {
                let alpha: i64;
                if prev_point == temp_point {
                    dividend = 3 * temp_point.0 * temp_point.0 + cap_a;
                    divisor = 2 * temp_point.1;
                } else {
                    dividend = prev_point.1 - temp_point.1;
                    divisor = prev_point.0 - temp_point.0;
                }
                if gcd(divisor, n) == 1 {
                    alpha = (dividend * mod_inverse(divisor, n)) % n;
                    let x3 = alpha * alpha - temp_point.0 - prev_point.0;
                    temp_point = (x3, (alpha * (temp_point.0 - x3) - temp_point.1));
                } else {
                    div_failed = true;
                    d = divisor;
                }
            }
            prev_point = temp_point;
            iter += iter;
        }
        let k = gcd(d, n);
        if d == k && d == n {
        } else if 1 < k && k < n {
            return (k, n / k);
        }
    }
}

// based on pseudocode from wikipedia
fn mod_inverse(a: i64, n: i64) -> i64 {
    let mut t = 0;
    let mut newt = 1;
    let mut r = n;
    let mut newr = a;

    while newr != 0 {
        let quotient = r / newr;
        (t, newt) = (newt, t - quotient * newt);
        (r, newr) = (newr, r - quotient * newr);
    }
    if r > 1 {
        panic!();
    }
    if t < 0 {
        t = t + n
    }
    return t;
}
