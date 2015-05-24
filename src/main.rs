fn prob1() {
    let mut sum = 0;
    for i in 0..1000 {
        if (i % 3 == 0) || (i % 5 == 0) {
            sum += i;
        }
    }
    println!("sum is {}", sum);
}

fn prob2() {
    let mut a = 1;
    let mut b = 1;
    let mut sum = 0;
    while b <= 4000000 {
        let c = a + b;
        a = b;
        b = c;
        if (a % 2 == 0) {
            sum += a;
        }
    }
    println!("sum is {}", sum);
}

fn prob3() {
    let mut bignum : u64 = 600851475143;
    let mut i = 2;
    let mut last_prime = -1;
    while bignum > 1 {
        if (bignum % i == 0) {
            bignum /= i;
            last_prime = i;
        } else {
            i += 1;
        }
    }
    println!("largest prime is {}", last_prime);
}


fn main() {
    prob3();
}
