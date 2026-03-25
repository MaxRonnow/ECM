use num_bigint::{BigInt, RandBigInt};
use num_integer::{Integer, gcd};

pub fn ecm(n: BigInt) -> (BigInt, BigInt) {
    // initializes the random number generator
    let mut rng = rand::thread_rng();

    // loops until it gets random values that works
    loop {
        // generates random values needed for algorithm
        let cap_a = rng.gen_bigint_range(&BigInt::from(1), &n).mod_floor(&n);
        let a = rng.gen_bigint_range(&BigInt::from(1), &n).mod_floor(&n);
        let b = rng.gen_bigint_range(&BigInt::from(1), &n).mod_floor(&n);

        let cap_p = (a, b);

        // let cap_b = (b * b - a * a * a - cap_a * a) % n;
        // The resulting elliptic curve:
        // E: y^2 = (x^3 + Ax + B) mod n

        // variables needed to run the loop
        let mut iter = 3;
        let mut d = BigInt::ZERO; // initializing the divisor variable
        let mut div_failed = false;
        let mut prev_point = cap_p;

        // loop runs until the division in the addition stage fails
        //while !div_failed
        //for _ in 1..10000
        while !div_failed {
            // initialize variables for calculations
            let mut temp_point = prev_point.clone();
            let mut dividend: BigInt; // dividend for addition step
            let mut divisor: BigInt; // divisor for addition step

            // loop to add together the points, adds +1 time for each iteration
            for _ in 1..iter {
                let alpha: BigInt; // init alpha for the addition step

                // addition of points
                if prev_point == temp_point {
                    dividend = BigInt::from(3) * &temp_point.0 * &temp_point.0 + &cap_a;
                    divisor = BigInt::from(2) * &temp_point.1;
                } else {
                    dividend = (&prev_point.1 - &temp_point.1).mod_floor(&n);
                    divisor = &prev_point.0 - &temp_point.0;
                }

                // checks if modular inverse of divisor exists
                if gcd(divisor.clone(), n.clone()) == BigInt::from(1) {
                    alpha = (&dividend * &mod_inverse(&divisor, &n)).mod_floor(&n);
                    let x3 = (&alpha * &alpha - &temp_point.0 - &prev_point.0).mod_floor(&n);
                    temp_point = (
                        x3.clone(),
                        (&alpha * (&temp_point.0 - &x3) - &temp_point.1).mod_floor(&n),
                    );
                    //print!("point: {:?}", temp_point)
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
        let k = gcd(d, n.clone());
        if k > BigInt::from(1) && k < n {
            return (k.clone(), n / k);
        }
    }
}

// Function to calculate the modular inverse using the Extended Euclidean Algorithm
// based on pseudocode from the wikipedia page "Extended Euclidean algorithm "
fn mod_inverse(a: &BigInt, n: &BigInt) -> BigInt {
    let mut t = BigInt::ZERO;
    let mut newt = BigInt::from(1);
    let mut r = n.clone();
    let mut newr = a.clone();

    while newr != BigInt::ZERO {
        let quotient = &r / &newr;
        (t, newt) = (newt.clone(), &t - &quotient * &newt);
        (r, newr) = (newr.clone(), &r - &quotient * &newr);
    }
    if r > BigInt::from(1) {
        panic!();
    }
    if t < BigInt::from(1) {
        t = &t + n
    }
    return t;
}
