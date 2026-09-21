#[allow(non_snake_case)]
pub struct Registers{
    V0: u8, 
    V1: u8, 
    V2: u8, 
    V3: u8, 
    V4: u8, 
    V5: u8, 
    V6: u8, 
    V7: u8, 
    V8: u8, 
    V9: u8,
    VA: u8, 
    VB: u8, 
    VC: u8, 
    VD: u8, 
    VE: u8, 
    VF: u8,
    I: u16, 
    PC: u16,
    SP: u8, 
    stack: [u16; 16]
}

impl Registers {
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
        pub fn set_register_val(&mut self, register_num: u16, register_val: u8)  {
        if register_num == 0 {
            self.V0 = register_val
        }
        if register_num == 1 {
            self.V1 = register_val
        }if register_num == 2 {
            self.V2 = register_val
        }if register_num == 3 {
            self.V3 = register_val
        }if register_num == 4 {
            self.V4 = register_val
        }if register_num == 5 {
            self.V5 = register_val
        }if register_num == 6 {
            self.V6 = register_val
        }if register_num == 7 {
            self.V7 = register_val
        }if register_num == 8 {
            self.V8 = register_val
        }if register_num == 9 {
            self.V9 = register_val
        }if register_num == 10 {
            self.VA = register_val
        }if register_num == 11 {
            self.VB = register_val
        }if register_num == 12 {
            self.VC = register_val
        }if register_num == 13 {
            self.VD = register_val
        }if register_num == 14 {
            self.VE = register_val
        }if register_num == 15 {
            self.VF = register_val
        }
        else {
            panic!("Invalid register number passed")
        }
    }
    pub fn add_to_register(&mut self, register_num: u16, register_val: u8)  {
        if register_num == 0 {
            self.V0 += register_val
        }
        if register_num == 1 {
            self.V1 += register_val
        }if register_num == 2 {
            self.V2 += register_val
        }if register_num == 3 {
            self.V3 += register_val
        }if register_num == 4 {
            self.V4 += register_val
        }if register_num == 5 {
            self.V5 += register_val
        }if register_num == 6 {
            self.V6 += register_val
        }if register_num == 7 {
            self.V7 += register_val
        }if register_num == 8 {
            self.V8 += register_val
        }if register_num == 9 {
            self.V9 += register_val
        }if register_num == 10 {
            self.VA += register_val
        }if register_num == 11 {
            self.VB += register_val
        }if register_num == 12 {
            self.VC += register_val
        }if register_num == 13 {
            self.VD += register_val
        }if register_num == 14 {
            self.VE += register_val
        }if register_num == 15 {
            self.VF += register_val
        }
        else {
            panic!("Invalid register number passed")
        }
    }
}

//by default, skip next instruction should be equal to false or bad things happen
pub fn parse_instruction(instruction: u16, registers: &mut Registers, skip_next_instruction: &mut bool) {
    if instruction == 0x00E0 {
        //clear screen

    }
    else if instruction == 0x00EE {
        //return from subroutine
        registers.SP -= 1;
        registers.PC = registers.stack[registers.SP as usize];

    }
    //0x1MMM instruciton 
    else if (instruction & 0xF000) == 0x1000 {
        //set PC to lower 3 digits
        registers.PC = instruction & 0x0FFF
    }
    //0x2MMM instruction
    else if (instruction & 0xF000) == 0x2000 {
        //call subroutine
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

    //0x8XY0 instruction
    else if (instruction & 0xF000) == 0x8000 {
        let x_val = (instruction >> 8) & 0x0F;
        let y_val = (instruction >> 4) & 0x0F;
        
        registers.set_register_val(x_val, registers.get_register_val(y_val));
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
    // 0x8XY4 instr4uction
    else if (instruction & 0xF00F) == 0x8004 {
        let x_val = (instruction >> 8) & 0x0F;
        let y_val = (instruction >> 4) & 0x0F;

        registers.set_register_val(x_val, registers.get_register_val(x_val) + registers.get_register_val(y_val));
        if registers.get_register_val(x_val) as u16 + registers.get_register_val(y_val) as u16 > 255 {
           registers.VF = 1 
        }
    }

    //0x8XY5 instruction
    else if (instruction & 0xF00F) == 0x8005 {
        let x_val = (instruction >> 8) & 0x0F;
        let y_val = (instruction >> 4) & 0x0F;

        if registers.get_register_val(x_val) < registers.get_register_val(y_val) {
            registers.VF = 0;
        }

        registers.set_register_val(x_val, registers.get_register_val(x_val) - registers.get_register_val(y_val));
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

}