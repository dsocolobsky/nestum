//use std::fs::File;
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct CPU {
    a: u8,   // Accumulator
    x: u8,   // Index register X
    y: u8,   // Index register Y
    s: u8,   // Stack pointer
    p: u8,   // Status register (lower 6 bits)
    pc: u16, // Program counter
    cycle: u32,
}

impl CPU {

    fn new() -> CPU {
        CPU{a: 0, x: 0, y: 0, p: 0x34, s: 0xFD, pc: 0x00, cycle: 0}
    }

    fn fetch() {

    }

    fn log_status(&self) {
        println!("A:{:02X?} X:{:02X?} Y:{:02X?} P:{:04X?} SP:{:04X?} CYC: {} SL:0", 
            self.a, self.x, self.y, self.p, self.s, self.cycle);
    }
}

fn main() -> Result<(), std::io::Error> {
    println!("nestum v0.01");

    let path = Path::new("nestest/nestest.nes");
    
    /*let mut file = match File::open(&path) {
        Err(why) => panic!("Could not open {}: {}", path.display(), why),
        Ok(file) => file,
    };*/

    let content = fs::read(path)?;
    println!("{:#02X?}", &content[16..64]);

    let mut cpu = CPU::new();
    cpu.log_status();

    Ok(())
}
