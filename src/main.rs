extern crate num_bigint;
#[macro_use]
extern crate rocket;

use num_bigint::BigUint;
use rocket::response::content::RawHtml;
use rocket::{Build, Rocket};
use std::mem::replace;

use num_traits::{One, Zero};

fn fib(n: u128) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for _ in 0..n {
        let f2 = f0 + &f1;
        f0 = replace(&mut f1, f2);
    }
    f0
}

fn prime(i: u128) -> u128 {
    let mut primes: Vec<u128> = vec![2];
    while (primes.len() as u128) <= i {
        let new_prime = (*primes.last().unwrap() + 1..)
            .find(|j| {
                primes
                    .iter()
                    .take_while(|prime| **prime * **prime <= *j)
                    .all(|prime| j % *prime != 0)
            })
            .expect("End of primes reached, math broke");
        primes.push(new_prime);
    }
    *primes.last().expect("Somehow the vector became empty")
}

fn ack(m: u128, n: u128) -> u128 {
    match (m, n) {
        (0, _) => n + 1,
        (_, 0) => ack(m - 1, 1),
        (_, _) => ack(m - 1, ack(m, n - 1)),
    }
}

#[get("/fib/<i>")]
fn fib_get(i: u128) -> String {
    fib(i).to_string()
}

#[get("/prime/<i>")]
fn prime_get(i: u128) -> String {
    prime(i).to_string()
}

#[get("/ack/<m>/<n>")]
fn ack_get(m: u128, n: u128) -> String {
    ack(m, n).to_string()
}

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml {
        0: "<ul><li><a href='/fib/0'>fib</a></li><li><a href='/prime/0'>prime</a></li><li><a href='/ack/0/0'>ack</a></li></ul>",
    }
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index, prime_get, fib_get, ack_get])
}
