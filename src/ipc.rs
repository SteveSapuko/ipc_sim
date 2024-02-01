#![allow(arithmetic_overflow)]
use crate::components::*;

pub struct Computer {
    pub acc: u8,
    pub registers: Registers,
    pub pc: u16,
    pub rom: Memory,
    pub ram: Memory,
    pub flags: Flags,
}

impl Computer{
    pub fn new(program: &[u8]) -> Self {
        let mut ready_rom: [u8; 65536] = [0; 65536];
        for byte in program.iter().enumerate() {
            ready_rom[byte.0] = *byte.1;
        }

        Computer {
            acc: 0,
            registers: Registers{r0: 0, r1: 0, r2: 0, r3: 0, sf: 0, sp: 0, marl: 0, marh: 0},
            pc: 0,
            rom: Memory::new(&ready_rom),
            ram: Memory::new(&[0; 65536]),
            flags: Flags {f_carry: false, f_zero: false, f_parity: false, f_sign: false,}
        }
    }

    pub fn execute_instruction(&mut self) -> bool{
        let opcode: u8 = self.rom.read(self.pc);
        let instruction: u8 = opcode & 0b00011111; 
        let register: u8 = (opcode & 0b11100000) >> 5;
        let mar: u16 = ((self.registers.marh as u16) << 8) | (self.registers.marl as u16);
        self.pc += 1;

        if instruction < 9 {
            let result = self.alu(instruction, register);
            self.flags.f_sign = (result >> 7) != 0;
            self.flags.f_parity = (result & 1) != 0;
            self.flags.f_zero = result == 0;

            if instruction != 8 {
                self.acc = result;
            }
            
            return true;
        }

        match instruction {
            9 => {self.flags.f_carry = true;},
            10 => {self.flags.f_carry = false;},
            11 => {self.acc = self.registers.read(register);}
            12 => {self.registers.set(register, self.acc);},

            13 => {self.pc = mar;},
            14 => {if self.flags.f_carry {self.pc = mar;}},
            15 => {if !self.flags.f_carry {self.pc = mar;}},
            16 => {if self.flags.f_zero {self.pc = mar;}},
            17 => {if !self.flags.f_zero {self.pc = mar;}},
            18 => {if self.flags.f_sign {self.pc = mar;}},
            19 => {if !self.flags.f_sign {self.pc = mar;}},
            20 => {if self.flags.f_parity {self.pc = mar;}},
            21 => {if !self.flags.f_parity {self.pc = mar;}},

            22 => {self.registers.set(register, self.rom.read(self.pc)); self.pc += 1;},
            23 => {self.registers.set(2, self.pc as u8); self.registers.set(3, (self.pc >> 8) as u8);},
            24 => {self.ram.write(mar, self.registers.read(register));},
            25 => {self.registers.set(register, self.ram.read(mar));},
            29 => {return false;}
            _ => {}
        }

        true

    }

    pub fn print_state(&self) {
        let inst = self.rom.read(self.pc) & 0b00011111;
        
        println!("
        pc: {}
        instruction: {}
        
        acc: {}
        r0: {}
        r1: {}
        r2: {}
        r3: {}
        sf: {}
        sp: {}

        marl: {}
        marh: {}
        
        carry: {}
        zero: {}
        parity: {}
        sign: {}",
        self.pc, inst, self.acc, self.registers.r0, self.registers.r1, self.registers.r2, self.registers.r3,
        self.registers.sf, self.registers.sp, self.registers.marl, self.registers.marh,
        self.flags.f_carry as u8, self.flags.f_zero as u8, self.flags.f_parity as u8, self.flags.f_sign as u8)
    }
    
    pub fn get_state(&self, x: u8) -> u16 {
        if x == 9 {return self.pc;}
        
        return match x {
            0 => self.registers.r0,
            1 => self.registers.r1,
            2 => self.registers.r2,
            3 => self.registers.r3,
            4 => self.registers.sf,
            5 => self.registers.sp,
            6 => self.registers.marl,
            7 => self.registers.marh,
            8 => self.acc,
            _ => 0
        } as u16
    }

    fn alu(&mut self, i: u8, r: u8) -> u8 {
        let a = self.acc;
        let b: u8 = self.registers.read(r);
        let a16: i16 = a as i16;
        let b16: i16 = b as i16;

        match i {
            0 => { //add
                let result = a + b;
                self.acc = result;

                if (result as i16) < a16 + b16 {self.flags.f_carry = true}
                else {self.flags.f_carry = false}
                return result
            },
            
            1 | 8 => { //sub / cmp
                let result = a - b;

                self.acc = result;
                if (result as i16) > a16 - b16 {self.flags.f_carry = true}
                else {self.flags.f_carry = true}
                return result
            },
            
            2 => { //adc
                let result  = a + b + self.flags.f_carry as u8;
                self.acc = result;

                if (result as i16) < a16 + b16 + self.flags.f_carry as i16 {self.flags.f_carry = true}
                else {self.flags.f_carry = false}
                return result
            },
            
            3 => { //sbc
                let result = a - b - self.flags.f_carry as u8;
                self.acc = result;

                if (result as i16) > a16 - b16 - self.flags.f_carry as i16 {self.flags.f_carry = true}
                else {self.flags.f_carry = true}
                return result
            },
            
            4 => { // ror
                let fell = a & 0b00000001;
                let result = (a >> 1) & ((self.flags.f_carry as u8) << 7);
                self.flags.f_carry = fell != 0;
                return result
            }

            5 => { //nor
                let result = !(a | b);
                result
            }

            6 => { // and
                let result = a & b;
                result
            }

            _ => 0

        }
    }
}