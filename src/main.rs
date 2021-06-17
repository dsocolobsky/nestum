//use std::fs::File;
use std::fs;
use std::path::Path;
//const OPCODE_TABLE: &[(u8, &str, fn)] = &[(0x4C, "JMP", fn(&CPU)), (2, "chrome")];

#[derive(Debug)]
struct CPU {
    a: u8,   // Accumulator
    x: u8,   // Index register X
    y: u8,   // Index register Y
    s: u8,   // Stack pointer
    p: u8,   // Status register (lower 6 bits)
    pc: u16, // Program counter
    cycle: u32,
    memory: Vec<u8>, // Should be 64Kb for memory space
}

impl CPU {

    fn new() -> CPU {
        CPU{a: 0, x: 0, y: 0, p: 0x34, s: 0xFD, pc: 0xC000, cycle: 0, memory: vec![0; 64*1024]}
    }

    fn fetch(&mut self) {
        let pc = self.pc as usize;
        let op = self.memory[pc];
        println!("op: {:#02X?}", op);
        
        match op {
            0x4c => {
                let imm = ((self.memory[pc+2] as u16) << 8) | (self.memory[pc+1] as u16);
                self.op_jmp_imm16(imm);
            },
            _ => {println!("Unhandled operation");},
        }
    }

    fn load_rom(&mut self, filename: &str) {
        let rom = fs::read(Path::new(filename)).expect("Could not load ROM");
        // Skip first 16 bytes because it's the iNES header
        let mut how_many = 0x4000;
        for (dst, src) in self.memory[0x8000..].iter_mut().zip(&rom[16..]) {
            *dst = *src;
            how_many -= 1;
            if how_many == 0 {
                break;
            }
        }
        how_many = 0x4000;
        for (dst, src) in self.memory[0xC000..].iter_mut().zip(&rom[16..]) {
            *dst = *src;
            how_many -= 1;
            if how_many == 0 {
                break;
            }
        }
    }

    fn log_memory(&self) {
        println!("{:#02X?}", self.memory);
    }

    fn log_memory_range(&self, start: usize, end: usize) {
        println!("{:#02X?}", &self.memory[start..end]);
    }

    fn log_status(&self) {
        println!("A:{:02X?} X:{:02X?} Y:{:02X?} P:{:04X?} SP:{:04X?} CYC: {} SL:0", 
            self.a, self.x, self.y, self.p, self.s, self.cycle);
    }

    fn op_jmp_imm16(&mut self, addr: u16) {
        println!("JMP ${:04X?}", addr);
        self.pc = addr;
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
    cpu.fetch();
    //cpu.log_memory_range(0xC000, 0xC000+20*2);

    Ok(())
}
