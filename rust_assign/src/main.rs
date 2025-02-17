 //declare the constant values for faren 
/*const FREEZING_WATER : f64 = 32.0;

//create the converters
//faren -> cels
fn fahrenheit_to_celsius(f: f64) -> f64{
    (f - FREEZING_WATER) * 5.0 / 9.0
} 

//cels to faren
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + FREEZING_WATER
}
fn main () {
    //made the inital temp this, but can change it if needed
    let mut temp_f: f64 = 50.0;
    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{} deg faren {:.2} deg cels", temp_f, temp_c);

    //loop 
    for i in 1..=5{
        let next_f = temp_f + i as f64;
        let next_c = fahrenheit_to_celsius(next_f as f64);
        println!("{} deg faren {:.2} deg cels", next_f, next_c);
    }
}*/

fn is_even (n: i32) -> bool {
    n % 2 == 0
}
fn main() {
//create an array of int numbers of my choice
let numbers = [1,2,3,4,5,10,34,55,78,27];

//use for loop to iterate through the array
for &num in &numbers {
    match (num % 3 == 0, num % 5 == 0){
        (true, true) => println! ("{}: FizzBuzz", num),
        (true, false) => println! ("{}: Fizz", num),
        (false, true) => println! ("{}: Buzz", num),
        //else, add the num to the string
        _ => match is_even(num){
            true => println!("{}: Even" , num),
            false => println! ("{}: Odd", num),
        },
    }
}
//now lets make a while loop
let mut sum = 0; //create sum and set it to 0
let mut x = 0; //create mutable variable x and set it to 0
while x < numbers.len(){
    sum += numbers[x];
    x += 1;
}
println!("{} Sum of numbers: ", sum);

//find the largest number
//using a loop'
let mut large = numbers[0];
for &num in &numbers {
    large = large.max(num);
}
println!("{} Largest number of all: ", large);
}

/*fn check_guess(guess: i32, secret: i32) -> i32 {
    match guess {
        _ if guess == secret => 0, 
        _ if guess > secret => 1, 
        _ => -1,
    }
}

fn main() {
    let secret = 7;
    let mut guess = 1;
    let mut attempts = 0;

    loop {
        //guess = 5;
        attempts += 1;

        match check_guess (guess, secret){
            0 => {
                println!("The correct numbers is: {}", secret);
                break;

            }
           1 => println! ("{} NO! Too high", guess),
           -1 => println! ("{} NO! Too low", guess),
           _ => (),
        }
            guess += 1;
    }
    println!("It took you this many guesses until you got it right {}", attempts);
}*/