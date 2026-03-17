use num_integer::gcd;
use rand::{Rng, RngExt};

fn main() {
    println!("Hello, world!");
}

fn lenstra(n: i64) -> i64 {
    let mut rng = rand::rng();

    let cap_a: i64 = rng.random::<i64>() % n;
    let a: i64 = rng.random::<i64>() % n;
    let b: i64 = rng.random::<i64>() % n;

    let p = (a, b);

    let cap_b = ((b * b) - (a * a * a) - cap_a * a) % n;

    let cap_e = elliptic_curve::new(cap_a, cap_b, n);

    let q1 = p;
    let mut iter = 0;
    let mut done = false;
    while !done {
        for _ in 0..iter {
            
        }
    }

    return 2;
}

struct elliptic_curve {
    a: i64,
    b: i64,
    n: i64,
}

impl elliptic_curve {
    fn new(a: i64, b: i64, n: i64) -> elliptic_curve {
        elliptic_curve { a, b, n }
    }

    fn calculate(&self, x: i64) -> i64 {
        let y = ((x * x * x) + (self.a * x) + self.b) % self.n;
        return y;
    }
}

fn eliptic_curve(x: i64, a: i64, b: i64, n: i64) -> i64 {
    let y = ((x * x * x) + (a * x) + b) % n;
    return y;
}

fn add_points(p: (i64, i64), q: (i64, i64), a: i64, n: i64) -> Result<(i64, i64), i64> {
    let mut alpha: i64;
    if p == q {
        let nämnare = (2 * p.1) % n;
        let täljare = (3 * p.0 * p.0 + a) % n;
        if gcd(nämnare, n) != 1 {
            return Err(64);
        } else {
            alpha = (täljare * mod_inverse(nämnare, n)) % n;
        }
        alpha = ((3 * p.0 * p.0 + a) / (2 * p.1)) % n;
    } else {
        alpha = ((q.1 - p.1) / (q.1 - p.0)) % n;
    }
    let x3 = (alpha * alpha * alpha - p.0 - q.0) % n;
    let y3 = (alpha * (p.0 - x3) - p.1) % n;
    return (x3, y3);
}

fn mod_inverse(a: i64, n: i64) -> Result<i64, String> {
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
        return Err("Icke inverterbar".to_string());
    }
    if t < 0 {
        t = t + n
    }
    return Ok(t);
}
