use num_integer::gcd;
use rand::RngExt;

pub fn ecm(n: i128) -> (i128, i128) {
    // initializes the random number generator
    let mut rng = rand::rng();

    // loops until it gets random values that works
    loop {
        // generates random values needed for algorithm
        let cap_a: i128 = rng.random::<i128>() % n;
        let a: i128 = rng.random::<i128>() % n;
        let b: i128 = rng.random::<i128>() % n;

        let cap_p = (a, b);

        // let cap_b = (b * b - a * a * a - cap_a * a) % n;
        // The resulting elliptic curve:
        // E: y^2 = (x^3 + Ax + B) mod n

        // variables needed to run the loop
        let mut iter = 2;
        let mut d: i128 = 0; // initializing the divisor variable
        let mut div_failed = false;
        let mut prev_point = cap_p;

        // loop runs until the division in the addition stage fails
        while !div_failed {
            // initialize variables for calculations
            let mut temp_point = prev_point;
            let mut dividend: i128; // dividend for addition step
            let mut divisor: i128; // divisor for addition step

            // loop to add together the points, adds +1 time for each iteration
            for _ in 1..iter {
                let alpha: i128; // init alpha for the addition step

                // addition of points
                if prev_point == temp_point {
                    dividend = 3 * temp_point.0 * temp_point.0 + cap_a;
                    divisor = 2 * temp_point.1;
                } else {
                    dividend = prev_point.1 - temp_point.1;
                    divisor = prev_point.0 - temp_point.0;
                }

                // checks if modular inverse of divisor exists
                if gcd(divisor, n) == 1 {
                    alpha = (dividend * mod_inverse(divisor, n)) % n;
                    let x3 = alpha * alpha - temp_point.0 - prev_point.0;
                    temp_point = (x3, (alpha * (temp_point.0 - x3) - temp_point.1));
                } else {
                    // if no inverse exists division fails and we have found a factor if 1 < k < N
                    div_failed = true;
                    d = divisor;
                }
            }
            // setting up for next iteration
            prev_point = temp_point;
            iter += iter;
        }

        // checks if we have found an answer, if not the loop starts again
        // If answer found the function returns the factors of the number
        let k = gcd(d, n);
        if 1 < k && k < n {
            return (k, n / k);
        }
    }
}

// Function to calculate the modular inverse using the Extended Euclidean Algorithm
// based on pseudocode from the wikipedia page "Extended Euclidean algorithm "
fn mod_inverse(a: i128, n: i128) -> i128 {
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
