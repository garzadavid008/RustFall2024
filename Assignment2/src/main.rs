
fn is_even(n: &i32) -> bool {
    n % 2 == 0 
}

fn main() {
    let nums = [93, 42, 32, 15, 11, 56, 61, 10, 2, 4];

    for num in nums.iter() {
        let even_odd = if is_even(num) { "even" } else { "odd" };
        println!("The number {} is: {}", num, even_odd);

        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 5 == 0 {
            println!("Buzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        }
    }

    let sum: i32 = nums.iter().sum();
    println!("The sum is: {}", sum);

    let largest = nums.iter().max().unwrap();
    println!("The largest number in the array is: {}", largest);
}
