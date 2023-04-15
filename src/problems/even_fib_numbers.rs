fn main() {
    let mut previous = 1 as u64;
    let mut current = 0;
    let mut next = 0;

    let limit = 4_000_000;

    let mut sum: u64 = 0;
    loop {
        next = previous + current;
        if next >= limit {
            break;
        }
        if next % 2 == 0 {
            sum += next;
        }

        previous = current;
        current = next;
    }

    println!("The sum is {}", sum);
}
