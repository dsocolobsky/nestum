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
    memory: Vec<u8>,
}

impl CPU {

    fn new() -> CPU {
        CPU{a: 0, x: 0, y: 0, p: 0x34, s: 0xFD, pc: 0x10, cycle: 0, memory: Vec::new()}
    }

    fn fetch(&mut self) {
        println!("op: {:#02X?}", self.memory[self.pc as usize]);
        self.pc += 1
    }

    fn load_rom(&mut self, filename: &str) {
        self.memory = fs::read(Path::new(filename)).expect("Could not load ROM");
        println!("{:#?}", self.memory.len())
    }

    fn log_memory(&self) {
        println!("{:#02X?}", self.memory);
    }

    fn log_status(&self) {
        println!("A:{:02X?} X:{:02X?} Y:{:02X?} P:{:04X?} SP:{:04X?} CYC: {} SL:0", 
            self.a, self.x, self.y, self.p, self.s, self.cycle);
    }
}

fn main() -> Result<(), std::io::Error> {
    println!("nestum v0.01");

    //let content = fs::read(Path::new("nestest/nestest.nes"))?;
    //println!("{:#02X?}", &content[16..64]);

    let mut cpu = CPU::new();
    cpu.log_status();
    cpu.load_rom("nestest/nestest.nes");
    cpu.fetch();
    cpu.fetch();
    cpu.fetch();
    //cpu.log_memory();

    Ok(())
}
