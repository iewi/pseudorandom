mod pseudorandom;

// viewing the results of our function
fn main() {
    let vector_lc = pseudorandom::linear_congruential(10,13,1000);
    let vector_pm = pseudorandom::pure_multiplicative(10,13,1000);
    let vector_mwc = pseudorandom::multiply_with_carry(20,13,1000);
    let vector_p = pseudorandom::problem_lc(10);
    
    println!("Linear congruential:");
    for i in &vector_lc {
        println!("\t{}",i);
    }
    
    println!("Pure multiplicative:");
    for i in &vector_pm {
        println!("\t{}",i);
    }
    
    println!("Multiply with carry:");
    for i in &vector_mwc {
        println!("\t{}",i);
    }
    
    println!("");
    println!("\nProblem 4.5 #6:");
    println!("xn = (4xn + 1) mod 7:");
    for i in &vector_p {
        println!("\t{}",i);
    }
    
    loop { print!(""); }
}
