
// Date
// Random

use time::{ 
    Tm, Timespec, TmFmt, Duration,
    at, at_utc, now, now_utc,
};


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Date {
    
}


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Crypto {

}

impl Crypto {
    pub fn getRandomValues(input: Vec<u8>) -> Vec<u8> {
        unimplemented!()
    }
}
