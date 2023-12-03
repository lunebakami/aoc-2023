fn main() {
    let file = std::fs::read_to_string("./input").unwrap();

    let mut numbers = Vec::new();

    file.lines().
        for_each(|line| {
            let str: Vec<char> = line.chars().collect();

            let line_numbers = str.into_iter().filter(|&c| c.is_digit(10)).collect::<Vec<_>>();

            let first = line_numbers.first().unwrap();
            let last = line_numbers.last().unwrap();

            let number = format!("{}{}", first, last);

            numbers.push(number.parse::<u32>().unwrap());
        });

    let sum = numbers.iter().fold(0, |acc, &x| acc + x);

    println!("Sum: {}", sum);
}
