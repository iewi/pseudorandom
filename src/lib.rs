extern crate rgsl;

mod pseudorandom;

pub fn to_binary(number: u32) -> Vec<u8> {
    let mut n = number;
    let mut binary: Vec<u8> = Vec::new();
    
    while n != 0 {
        binary.push((n % 2) as u8);
        n >>= 1;
    }
    
    binary
}

#[cfg(test)]
mod tests {
    use super::to_binary;
    use super::pseudorandom;
    use rgsl;
    
    #[test]
    fn binary_works() {
        let bin: Vec<u8> = vec![1,0,1];
        assert_eq!(bin, to_binary(5));
    }
    
    #[test]
    fn lc_number_distribution() {
        // get bit string
        let lc: Vec<u8> = {
            let mut v: Vec<u8> = Vec::new();
            for i in pseudorandom::linear_congruential(100,9213693,1000) {
                v.append(&mut to_binary(i));
            }
            v
        };
        
        // sum count
        let mut running_count: f64 = 0.0;
        let len = lc.len() as f64;
        
        for number in lc {
            running_count += {
                if number == 0 { -1.0 }
                else { 1.0 }  
            };
        }
        
        // determine test statistic
        let s = running_count.abs() / len.sqrt();
        
        // determine p value
        let p = rgsl::error::erfc(s/(2.0_f64).sqrt());
        
        assert!(p >= 0.01);
    }
    
    #[test]
    fn pm_number_distribution() {
        // get bit string
        let pm: Vec<u8> = {
            let mut v: Vec<u8> = Vec::new();
            for i in pseudorandom::pure_multiplicative(100,9213693,1000) {
                v.append(&mut to_binary(i));
            }
            v
        };
        
        // sum count
        let mut running_count: f64 = 0.0;
        let len = pm.len() as f64;
        
        for number in pm {
            running_count += {
                if number == 0 { -1.0 }
                else { 1.0 }  
            };
        }
        
        // determine test statistic
        let s = running_count.abs() / len.sqrt();
        
        // determine p value
        let p = rgsl::error::erfc(s/(2.0_f64).sqrt());
        
        assert!(p >= 0.01);
    
    }
    
    /*#[test]
    fn lc_block_number_distribution() {
    
    }
    
    #[test]
    fn pm_block_number_distribution() {
    
    }
    
    #[test]
    fn lc_longest_run() {
    
    }
    
    #[test]
    fn pm_longest_run() {
    
    }*/
}
