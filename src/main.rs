use std::{fs::File, io::Read, time::Duration};
use crossterm::event::{read, Event, KeyCode};
use ipc::Computer;

mod ipc;
mod components;

fn main() {
    let mut file = match File::open("test.bin") {
        Ok(f) => f,
        Err(_) => {
            print!("No ROM Found.\n");
            std::process::exit(0);
        }
    };

    let mut rom: [u8; 65536] = [0; 65536];
    file.read(&mut rom).expect("Couldn't Read ROM");

    let mut ipc = Computer::new(&rom);

    let mut now = std::time::Instant::now();
    let mut n: u32 = 0;

    loop {
        if now.elapsed() >= Duration::from_millis(1000) {
            now = std::time::Instant::now();
            
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!("Cycles Since Last Print: {}", n);
            ipc.print_state();
        
            n = 0;
        }
        ipc.execute_instruction();
        n += 1;
        
    }

    /* 
    loop {
        if let Ok(e) = read() {
            if e == Event::Key(KeyCode::Enter.into()) {
                ipc.print_state();
                let run = ipc.execute_instruction();
                
                if !run {break;}
            }
        }
    }*/
}
