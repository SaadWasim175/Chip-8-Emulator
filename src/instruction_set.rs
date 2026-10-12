use std::sync::{Arc, atomic::AtomicU8};

use minifb::{Key, Window};

use crate::{HEIGHT, WIDTH};

#[allow(non_snake_case)]
pub struct Registers{
    pub V0: u8, 
    pub V1: u8, 
    pub V2: u8, 
    pub V3: u8, 
    pub V4: u8, 
    pub V5: u8, 
    pub V6: u8, 
    pub V7: u8, 
    pub V8: u8, 
    pub V9: u8,
    pub VA: u8, 
    pub VB: u8, 
    pub VC: u8, 
    pub VD: u8, 
    pub VE: u8, 
    pub VF: u8,
    pub I: u16, 
    pub PC: u16,
    pub SP: u8, 
    pub stack: [u16; 16],
    pub DT: u8,
    pub ST: u8,
    pub ram: [u8; 4*1024]
}

pub struct KeyMap{
    key: Key,
    value: u8
}

impl KeyMap {
    pub const fn new(key: Key, value: u8) -> KeyMap {
        KeyMap { key, value }
    }
}

impl Registers {
    pub fn new() -> Registers {
        Registers { V0: 0, V1: 0, V2: 0, V3: 0, V4: 0, V5: 0, V6: 0, V7: 0, V8: 0, V9: 0, VA: 0, VB: 0, VC: 0, VD: 0, VE: 0, VF: 0, I: 0, PC: 0x200, SP: 0, stack: [0; 16], DT: 0, ST: 0, ram: [0; 4*1024] }
    }

    pub fn get_register_val(&self, register_num: u16) -> u8 {
        if register_num == 0 {
            return self.V0
        }
        if register_num == 1 {
            return self.V1
        }if register_num == 2 {
            return self.V2
        }if register_num == 3 {
            return self.V3
        }if register_num == 4 {
            return self.V4
        }if register_num == 5 {
            return self.V5
        }if register_num == 6 {
            return self.V6
        }if register_num == 7 {
            return self.V7
        }if register_num == 8 {
            return self.V8
        }if register_num == 9 {
            return self.V9
        }if register_num == 10 {
            return self.VA
        }if register_num == 11 {
            return self.VB
        }if register_num == 12 {
            return self.VC
        }if register_num == 13 {
            return self.VD
        }if register_num == 14 {
            return self.VE
        }if register_num == 15 {
            return self.VF
        }
        else {
            panic!("Invalid register number passed")
        }
    }
    pub fn set_register_val(&mut self, register_num: u16, register_val: u8) {
        match register_num {
            0 => self.V0 = register_val,
            1 => self.V1 = register_val,
            2 => self.V2 = register_val,
            3 => self.V3 = register_val,
            4 => self.V4 = register_val,
            5 => self.V5 = register_val,
            6 => self.V6 = register_val,
            7 => self.V7 = register_val,
            8 => self.V8 = register_val,
            9 => self.V9 = register_val,
            10 => self.VA = register_val,
            11 => self.VB = register_val,
            12 => self.VC = register_val,
            13 => self.VD = register_val,
            14 => self.VE = register_val,
            15 => self.VF = register_val,
            _ => panic!("Invalid register number passed"),
        }
    }
    pub fn add_to_register(&mut self, register_num: u16, register_val: u8) {
        match register_num {
            0 => self.V0 += register_val,
            1 => self.V1 += register_val,
            2 => self.V2 += register_val,
            3 => self.V3 += register_val,
            4 => self.V4 += register_val,
            5 => self.V5 += register_val,
            6 => self.V6 += register_val,
            7 => self.V7 += register_val,
            8 => self.V8 += register_val,
            9 => self.V9 += register_val,
            10 => self.VA += register_val,
            11 => self.VB += register_val,
            12 => self.VC += register_val,
            13 => self.VD += register_val,
            14 => self.VE += register_val,
            15 => self.VF += register_val,
            _ => panic!("Invalid register number passed"),
        }
    }
}

//by default, skip next instruction should be equal to false or bad things happen
pub fn parse_instruction(window: &mut Window, instruction: u16, registers: &mut Registers, skip_next_instruction: &mut bool, screen_buf: &mut Vec<u32>, dt: &mut Arc<AtomicU8>) {
    if instruction == 0x00E0 {
        for val in screen_buf {
            *val = 0;
        }
    }
    else if instruction == 0x00EE {
        registers.SP -= 1;
        registers.PC = registers.stack[registers.SP as usize];

    }
    //0x1MMM instruciton 
    else if (instruction & 0xF000) == 0x1000 {
        registers.PC = instruction & 0x0FFF
    }
    //0x2MMM instruction
    else if (instruction & 0xF000) == 0x2000 {
        registers.stack[registers.SP as usize] = registers.PC;
        registers.SP+=1;
        let lower_digits = instruction & 0x0FFF;
        registers.PC = lower_digits;

    }
    //0x3XKK instruction
    else if (instruction & 0xF000) == 0x3000 {
        let register_num = (instruction >> 8) & 0x0F;
        let val = registers.get_register_val(register_num);
        let kk_vals = instruction & 0xFF;
        if kk_vals == val as u16 {
            *skip_next_instruction = true;            
        }
    }
    //0x4XKK instructin
    else if (instruction & 0xF000) == 0x4000 {
        let register_num = (instruction >> 8) & 0x0F;
        let val = registers.get_register_val(register_num);
        let kk_vals = instruction & 0xFF;
        if kk_vals != val as u16 {
            *skip_next_instruction = true;            
        }
    }
    //0x5XY0 instruction
    else if (instruction & 0xF000) == 0x5000 {
        let x_val = registers.get_register_val((instruction >> 8) & 0x0F);
        let y_val = registers.get_register_val((instruction & 0x00F0) >> 4);

        if x_val == y_val {
            *skip_next_instruction = true;
        }
    }
    //0x6XKK instruction
    else if (instruction & 0xF000) == 0x6000 {
        let x_val = (instruction >> 8) & 0x0F;

        registers.set_register_val(x_val, (instruction & 0xFF) as u8);
    }

    //0x7XKK instruction
    else if (instruction & 0xF000) == 0x7000 {
        let x_val = (instruction >> 8) & 0x0F;
        registers.add_to_register(x_val, (instruction & 0xFF) as u8);
    }

    
    //0x8XY1 instruction
    else if (instruction & 0xF00F) == 0x8001 {
        let x_val = (instruction >> 8) & 0x0F;
        let y_val = (instruction >> 4) & 0x0F;
        
        registers.set_register_val(x_val, registers.get_register_val(x_val) | registers.get_register_val(y_val));
    }
    
    //0x8XY2 instruction
    else if (instruction & 0xF00F) == 0x8002 {
        let x_val = (instruction >> 8) & 0x0F;
        let y_val = (instruction >> 4) & 0x0F;

        registers.set_register_val(x_val, registers.get_register_val(x_val) & registers.get_register_val(y_val));
    }

    //0x8XY3 instruction
    else if (instruction & 0xF00F) == 0x8003 {
        let x_val = (instruction >> 8) & 0x0F;
        let y_val = (instruction >> 4) & 0x0F;

        registers.set_register_val(x_val, registers.get_register_val(x_val) ^ registers.get_register_val(y_val));
    }

    // 0x8XY4 instruction
    else if (instruction & 0xF00F) == 0x8004 {
        let x_val = (instruction >> 8) & 0x0F;
        let y_val = (instruction >> 4) & 0x0F;

        let val_x = registers.get_register_val(x_val);
        let val_y = registers.get_register_val(y_val);

        let total_sum = val_x as u16 + val_y as u16;
        
        registers.set_register_val(x_val, total_sum as u8);
        
        if total_sum > 255 {
            registers.VF = 1;
        } else {
            registers.VF = 0;
        }
    }
    
    //0x8XY5 instruction
    else if (instruction & 0xF00F) == 0x8005 {
        let x_val = (instruction >> 8) & 0x0F;
        let y_val = (instruction >> 4) & 0x0F;

        let val_x = registers.get_register_val(x_val);
        let val_y = registers.get_register_val(y_val);

        let carry = if val_x >= val_y { 1 } else { 0 };

        registers.set_register_val(x_val, val_x.wrapping_sub(val_y));
        
        registers.VF = carry;
    }
    
    //0x8XY0 instruction
    else if (instruction & 0xF00F) == 0x8000 {
        let x_val = (instruction >> 8) & 0x0F;
        let y_val = (instruction >> 4) & 0x0F;
        
        registers.set_register_val(x_val, registers.get_register_val(y_val));
    }
    //0x9XY0 instruction
    else if (instruction & 0xF000) == 0x9000 {
        let x_val = (instruction >> 8) & 0x0F;
        let y_val = (instruction >> 4) & 0x0F;
        if registers.get_register_val(x_val) != registers.get_register_val(y_val) {
            *skip_next_instruction = true;
        }
    }

    //0xAMMM instructionn
    else if (instruction & 0xF000) == 0xA000 {
        let mmm_val = instruction & 0x0FFF;
        registers.I = mmm_val;
    }

    //0xBMMM instruction
    else if (instruction & 0xF000) == 0xB000 {
        let mmm_val = instruction & 0x0FFF;
        registers.PC = mmm_val + registers.get_register_val(0) as u16;
    }

    //0xCXKK instruction
    else if (instruction & 0xF000) == 0xC000 {
        let x_val = (instruction >> 8) & 0x0F;
        let kk = instruction & 0x00FF;
        let num: u8 = rand::random();
        registers.set_register_val(x_val, kk as u8 & num);
    }

    //0xDXYN instruction
    else if (instruction & 0xF000) == 0xD000 {
        let x_val = (instruction >> 8) & 0x0F;
        let y_val = (instruction >> 4) & 0x0F;
        let n_val = instruction & 0xF;
        
        let start_x = registers.get_register_val(x_val);
        let start_y = registers.get_register_val(y_val);
        
        registers.VF = 0;
        
        for row in 0..n_val {
            let sprite_byte = registers.ram[(registers.I + row) as usize];

            for pixel_col in 0..8 {
                if (sprite_byte >> (7 - pixel_col)) & 1 == 1 {
                    let screen_x = (start_x as usize + pixel_col) % WIDTH;
                    let screen_y = (start_y as usize + row as usize) % HEIGHT;
                    let ind = screen_y * WIDTH + screen_x;
                    
                    if screen_buf[ind] == 0xFFFFFFFF {
                        registers.VF = 1;
                    }
                    
                    screen_buf[ind] ^= 0xFFFFFFFF;
                }
            }
        }
    }


    //EX9E instruction
    else if (instruction & 0xF0FF) == 0xE09E {
        let x_val = (instruction >> 8) & 0x0F;
        let val = registers.get_register_val(x_val);
        let target_key = match val {
            0x1 => Some(Key::Key1),
            0x2 => Some(Key::Key2),
            0x3 => Some(Key::Key3),
            0xC => Some(Key::Key4),
            0x4 => Some(Key::Q),
            0x5 => Some(Key::W),
            0x6 => Some(Key::E),
            0xD => Some(Key::R),
            0x7 => Some(Key::A),
            0x8 => Some(Key::S),
            0x9 => Some(Key::D),
            0xE => Some(Key::F),
            0xA => Some(Key::Z),
            0x0 => Some(Key::X),
            0xB => Some(Key::C),
            0xF => Some(Key::V),
            _ => None,
        };
        if let Some(key) = target_key {
            if window.is_key_down(key) {
                *skip_next_instruction = true
            }
        }
    }
    // 0xEXA1 instruction
    else if (instruction & 0xF0FF) == 0xE0A1 {
        let x_val = (instruction >> 8) & 0x0F;
        let val = registers.get_register_val(x_val);
        let target_key = match val {
            0x1 => Some(Key::Key1),
            0x2 => Some(Key::Key2),
            0x3 => Some(Key::Key3),
            0xC => Some(Key::Key4),
            0x4 => Some(Key::Q),
            0x5 => Some(Key::W),
            0x6 => Some(Key::E),
            0xD => Some(Key::R),
            0x7 => Some(Key::A),
            0x8 => Some(Key::S),
            0x9 => Some(Key::D),
            0xE => Some(Key::F),
            0xA => Some(Key::Z),
            0x0 => Some(Key::X),
            0xB => Some(Key::C),
            0xF => Some(Key::V),
            _ => None,
        };
        if let Some(key) = target_key {
            if !window.is_key_down(key) {
                *skip_next_instruction = true
            }
        }
    }

    //0xFX07 instruction
    else if (instruction & 0xF0FF) == 0xF007 {
        let x_val = (instruction >> 8) & 0x0F;
        registers.set_register_val(x_val, dt.clone().load(std::sync::atomic::Ordering::Relaxed));
    }

    // 0xFX0A instruction
    else if (instruction & 0xF0FF) == 0xF00A {
        let x_val = (instruction >> 8) & 0x0F;
        let key_pressed = window.get_keys_pressed(minifb::KeyRepeat::No);
        
        if !key_pressed.is_empty() {
            let keymaps = vec![
                (Key::Key1, 0x1),
                (Key::Key2, 0x2),
                (Key::Key3, 0x3),
                (Key::Key4, 0xC),
                (Key::Q, 0x4),
                (Key::W, 0x5),
                (Key::E, 0x6),
                (Key::R, 0xD),
                (Key::A, 0x7),
                (Key::S, 0x8),
                (Key::D, 0x9),
                (Key::F, 0xE),
                (Key::Z, 0xA),
                (Key::X, 0x0),
                (Key::C, 0xB),
                (Key::V, 0xF)
            ];

            for key in keymaps {
                if key_pressed[0] == key.0 {
                    registers.set_register_val(x_val, key.1);
                    return;
                }
            }
        }
        
        registers.PC -= 2;
    }


    //0xFX15 instruction
    else if (instruction & 0xF0FF) == 0xF015 {
        let x_val = (instruction >> 8) & 0x0F;
        let val = registers.get_register_val(x_val);
    
        let dt_clone = Arc::clone(dt);
        dt_clone.store(val, std::sync::atomic::Ordering::Relaxed);
    }
    //0xFX17 instruction
    else if (instruction & 0xF0FF) == 0xF017 {
        //il come back to it later   
    }
    //0xFX1E instruction
    else if (instruction & 0xF0FF) == 0xF01E {
        let x_val = (instruction >> 8) & 0x0F;

        let x = registers.get_register_val(x_val);

        registers.I = x as u16 + registers.I

    }

    //0xFX29 instruction
    else if (instruction & 0xF0FF) == 0xF029 {
        let x_val = (instruction >> 8) & 0x0F;
        registers.I = registers.get_register_val(x_val) as u16 * 5;
    }

    //0xFX33 instruction
    else if (instruction & 0xF0FF) == 0xF033 {
        let x_reg = (instruction >> 8) & 0x0F;
        let val = registers.get_register_val(x_reg);

        registers.ram[registers.I as usize] = val / 100;
        registers.ram[registers.I as usize + 1] = (val / 10) % 10;
        registers.ram[registers.I as usize + 2] = val % 10;
    }

    //0xFX55 instruction
    else if (instruction & 0xF0FF) == 0xF055 {
        let x = (instruction >> 8) & 0x0F;
        let I = registers.I;
        for i in 0..=x {
            registers.ram[I as usize + i as usize] = registers.get_register_val(i as u16);
        }
    }

    //0xFX65 instruction 
    else if (instruction & 0xF0FF) == 0xF065 {
        let x = (instruction >> 8) & 0x0F;

        for i in 0..=x {
            registers.set_register_val(i, registers.ram[registers.I as usize + i as usize as usize]);
        }
    }

    // 0x8XY6 instruction
    else if (instruction & 0xF00F) == 0x8006 {
        let x_val = (instruction >> 8) & 0x0F;
        let y_val = (instruction >> 4) & 0x0F;
        
        let val_y = registers.get_register_val(y_val);
        
        let lsb = val_y & 0x01;
        
        registers.set_register_val(x_val, val_y >> 1);
        registers.VF = lsb;
    }

    // 0x8XYE instruction
    else if (instruction & 0xF00F) == 0x800E {
        let x_val = (instruction >> 8) & 0x0F;
        let y_val = (instruction >> 4) & 0x0F;
        
        let val_y = registers.get_register_val(y_val);
        
        let msb = (val_y >> 7) & 0x01;
        
        registers.set_register_val(x_val, val_y << 1);
        registers.VF = msb;
    }

    // 0xFX18 instruction
    else if (instruction & 0xF0FF) == 0xF018 {
        let x_val = (instruction >> 8) & 0x0F;
        let val = registers.get_register_val(x_val);
        
        registers.ST = val; 
    }



}