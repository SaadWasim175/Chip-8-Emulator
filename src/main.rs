mod instruction_set;

use std::{fs::File, io::Read, sync::{Arc, atomic::AtomicU8}, thread::{self, sleep}, time::{Duration, Instant}};

use minifb::{Window, WindowOptions, Key};

use crate::instruction_set::{Registers, parse_instruction};

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
fn main() {
    const DISPLAY_WIDTH: usize = 640;
    const DISPLAY_HEIGHT: usize = 320;
    
    //timer part
    let mut dt = Arc::new(AtomicU8::new(60));
    let bg_dt = Arc::clone(&dt);

    thread::spawn(move|| {
        loop {
            thread::sleep(Duration::from_micros(16670));
            let a = bg_dt.load(std::sync::atomic::Ordering::Relaxed);
            if a > 0 {
                bg_dt.store(a - 1, std::sync::atomic::Ordering::Relaxed);
            }
        }
    });

    let mut display_buffer = vec![0; DISPLAY_HEIGHT*DISPLAY_WIDTH];
    let mut buffer: Vec<u32> = vec![0; HEIGHT*WIDTH];
    let mut window = Window::new("Chip-8 Emulator", DISPLAY_WIDTH, DISPLAY_HEIGHT, WindowOptions::default()).unwrap();

    window.set_target_fps(60);

    let mut registers = Registers::new();

    let mut skip_next_instruction = false;

    let mut file = File::open("logo.ch8").unwrap();
    let chip8_fontset: [u8; 80] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xE0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80  // F
    ];
    let mut i = 0;
    for font in chip8_fontset{
        registers.ram[i] = font;
        i += 1;
    }

    let bytes_read = file.read(&mut registers.ram[0x200..]).unwrap();

    println!("{bytes_read}");
    
    const TARGET_FPS: u64 = 60;
    const TICKS_PER_FRAME: usize = 9;

    window.set_target_fps(TARGET_FPS as usize);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for _ in 0..TICKS_PER_FRAME {
            let instruction = ((registers.ram[registers.PC as usize] as u16) << 8) 
                | (registers.ram[(registers.PC + 1) as usize] as u16);
            
            parse_instruction(
                &mut window, 
                instruction, 
                &mut registers, 
                &mut skip_next_instruction, 
                &mut buffer, 
                &mut dt
            );

            if skip_next_instruction {
                registers.PC += 4;
                skip_next_instruction = false;
            } else {
                registers.PC += 2;
            }
        }

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let src_index = y * WIDTH + x;
                let color = if buffer[src_index] != 0 { 0xFFFFFF } else { 0 };
                
                for dy in 0..10 {
                    for dx in 0..10 {
                        let display_x = x * 10 + dx;
                        let display_y = y * 10 + dy;
                        let display_index = display_y * DISPLAY_WIDTH + display_x;
                        display_buffer[display_index] = color;
                    }
                }
            }
        }

        window.update_with_buffer(&display_buffer, DISPLAY_WIDTH, DISPLAY_HEIGHT).unwrap();
    }


}
