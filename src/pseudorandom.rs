extern crate rand;

use self::rand::Rng;

// linear congruential method of generating pseudorandom numbers
pub fn linear_congruential(repeat: usize, seed: u64, m: u64) -> Vec<u64> {
    let a: u64 = 4294967296;
    let c: u64 = m - 1;
    
    let mut results: Vec<u64> = vec![0; repeat];
    {
        let mut xn = seed;
        for i in 0..repeat {
            xn = (xn * a + c) % m;
            results[i] = xn;
        }
    }
        
    results
}

// pure multiplicative method of generating pseudorandom numbers
pub fn pure_multiplicative(repeat: usize, seed: u64, m: u64) -> Vec<u64> {
    let a: u64 = 4294967296;
      
    let mut results: Vec<u64> = vec![0; repeat];
    {
        let mut xn = seed;
        for i in 0..repeat {
            xn = (xn * a) % m;
            results[i] = xn;
        }
    }
       
    results
}

pub fn multiply_with_carry(repeat: usize, seed: u64, m: u64) -> Vec<u64> {
    let a: u64 = 4294967295;
    
    let mut results: Vec<u64> = vec![0; repeat];
    {
        let mut rng = rand::thread_rng();
        
        // seed initial values
        for k in 0..repeat {
            results[k] = rng.gen::<u8>() as u64;
        }
        
        let mut xn = results[0];
        let mut x_temp = results[0];
        let mut cn: u64 = seed;
        
        for i in 0..repeat {
            xn = (xn * a + cn) % m;
            cn = (x_temp * a + cn) / m;
            
            x_temp = xn;
            
            results[i] = xn;
        }
        
    }
    
    results

}

pub fn problem_lc(repeat: usize) -> Vec<u64> {
    let a: u64 = 4;
    let c: u64 = 1;
    let m: u64 = 7;
    let seed: u64 = 3;
    
    let mut results: Vec<u64> = vec![0; repeat];
    {
        let mut xn = seed;
        for i in 0..repeat {
            xn = (xn * a + c) % m;
            
            results[i] = xn;
        }
    }
    
    results
}
