fn to_decimal(snafu: &str) -> i64 {
    snafu.chars().fold(0, |decimal, snafu_digit| {
        // Example: if char is '-', position is 1. Subtract 2 to get -1
        let decimal_digit = ['=', '-', '0', '1', '2']
            .into_iter()
            .position(|c| c == snafu_digit)
            .unwrap() as i64
            - 2;
        // cur total * 5 (^5 power) + or - the digit in this slot
        decimal * 5 + decimal_digit
    })
}

fn to_snafu(decimal: i64) -> String {
    if decimal == 0 {
        // println!("Decimal was 0, returning ''");
        return String::new();
    }

    let decimal_remainder = decimal % 5;
    // println!("(rem) {decimal} % 5 = {decimal_remainder}");
    let snafu_digit = ['0', '1', '2', '=', '-'][decimal_remainder as usize];
    // println!("['0', '1', '2', '=', '-'][{decimal_remainder}] is '{snafu_digit}'");

    // snafu digits start at -2, so add 2 to compensate
    // then divide by 5 to get next
    let new_decimal = (decimal + 2) / 5;
    // println!("(new_decimal) ({decimal} + 2) / 5 = {new_decimal}");
    let mut snafu = to_snafu(new_decimal);
    snafu.push(snafu_digit);
    // println!("Pushed snafu_digit. new snafu: '{snafu}'");

    snafu
}

pub fn run() {
    println!("\n=== Day 25 ===");
    let input = include_str!("input.txt");
    let part1 = input.lines().map(to_decimal).sum::<i64>();
    println!("Part 1: {}", to_snafu(part1));
    println!("Part 2: Merry Christmas!!");
}
