extern crate alloc;

use alloc::format;
use alloc::string::String;
use log::info;
use lazy_static::lazy_static;

use crate::include::memory::tests::space as MEMORY_SPACE;

lazy_static! {
    pub static ref SPACES: [fn () -> &'static[TestDescriptor]; 1] = [MEMORY_SPACE];

}


pub struct TestManager { }

impl TestManager {
    pub fn run() {
        info!("Start");
        for space in SPACES.into_iter() {
            for test in space() {
                let result = (test.function)();
                let mut string = format!("{}: [OK]", test.name);
                if result.is_err() { string = format!("{}: [ERR] Message: [{}]", test.name, result.err().unwrap()); }
                info!("{}", string);
            }
        }
    }
}

pub struct TestDescriptor {
    pub function: fn() -> Result<(), String>,
    pub name: &'static str,
}