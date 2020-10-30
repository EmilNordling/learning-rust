fn fizz_buzz(iterations: i32) -> (i32, i32, i32) {
    let mut fizz_count: i32 = 0;
    let mut buzz_count: i32 = 0;
    let mut fizz_buzz_count: i32 = 0;

    for x in 1..iterations + 1 {
        if x % 15 == 0 {
            fizz_buzz_count += 1;
        } else if x % 5 == 0 {
            buzz_count += 1;
        } else if x % 3 == 0 {
            fizz_count += 1;
        }
    }

    return (fizz_count, buzz_count, fizz_buzz_count);
}

fn main() {
    let fizz_buzz_result = fizz_buzz(1000);

    match fizz_buzz_result {
        (x, y, z) => println!("Fizz {:?}. Buzz {:?}. FizzBuzz {:?} :)", x, y, z),
    }
}