#[macro_use] extern crate decimal;
use std::io;
use decimal::d128;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Add;

fn main() {
    println!("Give me a temp to convert...");
    let mut isDone = false;

    while !isDone {
    	let mut temperature = String::new();
    	io::stdin().read_line(&mut temperature)
        		   .expect("Failed to read line");

	    let temperature: u32 = match temperature.trim().parse() {
	    	Ok(num) => num,
	    	Err(_) => {println!("Invalid number {}. Try again.", temperature); continue}
	    };

    	println!("You entered: {}", temperature);

    	convertFtoC(temperature);
    	convertCtoF(temperature);

	    isDone = true;
	}

}

fn convertFtoC(temperature: u32) {
	//println!("Converting {} from Farenheit to Celcius..", temperature);

	let ftoc = d128::from(temperature).sub(d128!(32)).mul(d128!(0.5556));

	println!("{} Farenheit is {} Celcius.", temperature, ftoc);
}

fn convertCtoF(temperature: u32) {
	//println!("Converting {} from Celcius to Farenheit..", temperature);

	let ctof = d128::from(temperature).mul(d128!(1.8)).add(d128!(32));

	println!("{} Celcius is {} Farenheit.", temperature, ctof);
}
