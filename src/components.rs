
pub struct Registers {
    pub r0: u8,
    pub r1: u8,
    pub r2: u8,
    pub r3: u8,
    pub sf: u8,
    pub sp: u8,
    pub marl: u8,
    pub marh: u8,
}

impl Registers {
    pub fn read(&self, r: u8) -> u8 {
        match r {
            0 => return self.r0,
            1 => return self.r1,
            2 => return self.r2,
            3 => return self.r3,
            4 => return self.sf,
            5 => return self.sp,
            6 => return self.marl,
            7 => return self.marh,
            _ => return 0,
        }
    }

    pub fn set(&mut self, r: u8, value: u8) {
        match r {
            0 => self.r0 = value,
            1 => self.r1 = value,
            2 => self.r2 = value,
            3 => self.r3 = value,
            4 => self.sf = value,
            5 => self.sp = value,
            6 => self.marl = value,
            7 => self.marh = value,
            _ => ()
        }
    }
}


pub struct Memory {
    pub data: Box<[u8; 65536]>
}

impl Memory {
    pub fn new(_data: &[u8; 65536]) -> Self{
        Memory {data: Box::new(*_data)}
    }
    
    pub fn read(&self, ptr: u16) -> u8 {
        return self.data[ptr as usize];
    }

    pub fn write(&mut self, ptr: u16, value: u8) {
        self.data[ptr as usize] = value;
    }
}


pub struct Flags {
    pub f_carry: bool,
    pub f_zero: bool,
    pub f_parity: bool,
    pub f_sign: bool,
}