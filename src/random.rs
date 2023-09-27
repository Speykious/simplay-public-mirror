#![allow(dead_code)]

use std::ops::Range;
use rand::Rng;

pub fn integer(range: Range<i32>) -> i32 {
    return rand::thread_rng().gen_range(range);
}

pub fn boolean() -> bool {
    let bi = integer(0..2);

    if bi == 0 {
        return false;
    }

    else if bi == 1 {
        return true;
    }

    else {
        println!("random::boolean() got an invalid state!");
        std::process::exit(1);
    }
}
