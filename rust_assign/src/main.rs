//problem 1
/*fn concat_strings(s1: &String, s2: &String) -> String {
    //your code here
   format!("{}{}", s1, s2)
}

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"
} */

//problem 2
/*fn clone_and_modify(s: &String) -> String {
    // Your code here
    let mut cloneing = s.clone();
    cloneing.push_str("World");
    cloneing
}

fn main() {
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
} */

//problem 3
#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) {
    // Write your code here!
    *total = (low..=high).sum();
}

fn main() {
    // create necessary variables and test your function for low 0 high 100
    let mut total = 0;
    sum(&mut total, 0, 100);
    // total should be 5050
    println!("Total sum {}", total);
} 


