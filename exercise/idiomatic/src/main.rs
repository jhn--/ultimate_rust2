// 1. This code looks terrible. Let's start cleaning this up by running `cargo fmt`. If you
// configured your editor or IDE to run `cargo fmt` automatically upon save, you can just save!

// 2. `cargo fmt` is great, but it doesn't add blank lines where there are none. Go ahead and add
// some blank lines in places you think it would make sense.

// 3. Time to clean up! Run `cargo clippy`. Fix up all the warnings so `cargo clippy` is silent.

// Challenge: Clippy doesn't find *everything*. What else would you change to make this code better?

// const pi:f32=3.14159265358979323846;
// use std::f64::consts::PI;

#[allow(clippy::blacklisted_name)]
fn count_to_5() -> i32 {
    let mut count = 0;
    loop {
        // println!("{}", count);
        // if count > PI as i32 && count > 5 {
        if count >= 5 {
            break;
        } else {
            count += 1;
        }
    }
    count
}

fn main() {
    println!("I can count to {}", count_to_5());
}

#[cfg(test)]
mod test {
    // run `cargo test` to execute the unit test here.
    use super::*;
    #[test]
    fn test_counting() {
        assert_eq!(count_to_5() == 5, true);
    }
}
