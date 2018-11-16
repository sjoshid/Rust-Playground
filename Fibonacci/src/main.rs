static FINAL_NUMBER:u32 = 10;

fn main() {
	fibonacciRecurse(0, 1, 0);
    println!("Fibonacci loop");
    fibonacciLoop(0, 1, 0);
}

fn fibonacciRecurse(first: u32, second: u32, mut number: u32) {
	if number != FINAL_NUMBER {
		let addition = first + second;
		println!("{}", addition);
		number += 1;
		fibonacciRecurse(second, addition, number)
	}
}

fn fibonacciLoop(mut first: u32, mut second: u32, mut number: u32) {
	while number != FINAL_NUMBER {
		let addition = first + second;
		println!("{}", addition);
		number += 1;

		first = second;
		second = addition;
	}
}
