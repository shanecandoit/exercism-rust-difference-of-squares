pub fn square_of_sum(n: u32) -> u32 {
    println!("square_of_sum({})", n);

    let mut sum = 0;
    for x in 1..n + 1 {
        sum += x;
        println!("x {} sum {}", x, sum);
    }

    let square = sum * sum;

    println!("square_of_sum({}) = {}", n, square);
    return square;
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut sum = 0;
    for x in 1..n + 1 {
        let square = x * x;
        sum += square;
        println!("x {} sum {} square {}", x, sum, square);
    }
    return sum;
}

pub fn difference(n: u32) -> u32 {
    let diff = square_of_sum(n) - sum_of_squares(n);
    return diff;
}
