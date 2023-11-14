fn main() {
    print_fizz_buzz(20);
}

fn print_fizz_buzz(num: i32) {
    for i in 1..num {
        println!("{}", fizz_buzz(i));
    }
}

fn fizz_buzz(num: i32) -> String {
    let fizz = if is_divisible_by(num, 3) { "Fizz" } else { "" };
    let buzz = if is_divisible_by(num, 5) { "Buzz" } else { "" };
    if fizz.is_empty() && buzz.is_empty() {
        return format!("{num}");
    }

    return format!("{}{}", fizz, buzz);
}

fn is_divisible_by(dividend: i32, divisor: i32) -> bool {
    dividend % divisor == 0
}
