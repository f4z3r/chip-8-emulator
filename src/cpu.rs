//! CPU module

use rand::random;

use prelude::*;
use interconnect::Interconnect;


/// A CHIP-8 CPU.
pub struct Cpu<T, U> where T: Input, U: Graphics {
    // interconnect allowing access to peripherals
    interconnect: Interconnect<T, U>,
    // program counter
    pc: u16,
    // function call stack
    stack: [u16; 16],
    // stack pointer
    sp: u8,
    // general purpose registers
    v: [u8; 16],
    // address register
    i: u16,
    // timer registers
    dt: u8,
}

impl<T, U> Cpu<T, U> where T: Input, U: Graphics {
    /// Constructor.
    ///
    /// # Arguments
    /// - `interconnect`: the interconnect that the CPU will use to communicate with memory and peripherals.
    pub fn new(interconnect: Interconnect<T, U>) -> Cpu<T, U> {
        Cpu {
            interconnect,
            pc: 0,
            stack: [0; 16],
            sp: 0,
            v: [0; 16],
            i: 0,
            dt: 0
        }
    }

    /// Execute instructions from memory.
    pub fn run(&mut self) {
        loop {
            if self.interconnect.input.close_requested() {
                break
            }
            self.execute_cycle();
            self.interconnect.input.handle_inputs();
        }
    }

    /// Execute a single cycle of the program.
    fn execute_cycle(&mut self) {
        self.handle_timers();
        let opcode = self.interconnect.memory.read_word(self.pc as usize);
        self.process_opcode(opcode);
    }

    /// Handle timers
    fn handle_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
    }

    /// Process an opcode.
    fn process_opcode(&mut self, opcode: u16) {
        // get potential register values and parameters
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let vx = self.v[x];
        let vy = self.v[y];
        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;

        // beak opcode into nibbles
        let op_1 = (opcode & 0xF000) >> 12;
        let op_2 = (opcode & 0x0F00) >> 8;
        let op_3 = (opcode & 0x00F0) >> 4;
        let op_4 = opcode & 0x000F;

        // increment program counter
        self.pc += 2;

        match (op_1, op_2, op_3, op_4) {
            // CLS
            (0, 0, 0xE, 0) => self.interconnect.graphics.cls(),
            // RET
            (0, 0, 0xE, 0xE) => {
                self.sp = self.sp - 1;
                self.pc = self.stack[self.sp as usize];
            },
            // JP
            (0x1, _, _, _) => self.pc = nnn,
            // CALL
            (0x2, _, _, _) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp = self.sp + 1;
                self.pc = nnn;
            },
            // SE Vx KK
            (0x3, _, _, _) => self.pc += if vx == kk { 2 } else { 0 },
            // SNE Vx KK
            (0x4, _, _, _) => self.pc += if vx != kk { 2 } else { 0 },
            // SE Vx Vy
            (0x5, _, _, _) => self.pc += if vx == vy { 2 } else { 0 },
            // LD Vx
            (0x6, _, _, _) => self.v[x] = kk,
            // ADD Vx, byte
            (0x7, _, _, _) => self.v[x] = vx.wrapping_add(kk),
            // LD Vx, Vy
            (0x8, _, _, 0x0) => self.v[x] = self.v[y],
            // OR Vx, Vy
            (0x8, _, _, 0x1) => self.v[x] = self.v[x] | self.v[y],
            // AND Vx, Vy
            (0x8, _, _, 0x2) => self.v[x] = self.v[x] & self.v[y],
            // XOR Vx, Vy
            (0x8, _, _, 0x3) => self.v[x] = self.v[x] ^ self.v[y],
            // ADD Vx, Vy
            (0x8, _, _, 0x4) => {
                let res = self.v[x] as u16 + self.v[y] as u16;
                self.v[0xF] = if res > 0xFF { 1 } else { 0 };
                self.v[x] = (res & 0xFF) as u8;
            }
            // SUB Vx, Vy
            (0x8, _, _, 0x5) => {
                let res = self.v[x] as i8 - self.v[y] as i8;
                self.v[x] = res as u8;
                self.v[0xF] = if res < 0 { 1 } else { 0 };
            }
            // SHR Vx
            (0x8, _, _, 0x6) => {
                self.v[0xF] = self.v[x] & 0x1;
                self.v[x] >>= 1;
            }
            // SUBN Vx, Vy
            (0x8, _, _, 0x7) => {
                let res = self.v[y] as i8 - self.v[x] as i8;
                self.v[x] = res as u8;
                self.v[0xF] = if res < 0 { 1 } else { 0 };
            },
            // SHL Vx
            (0x8, _, _, 0xE) => {
                self.v[0xF] = self.v[x] & 0x80;
                self.v[x] <<= 1;
            }
            // SNE Vx Vy
            (0x9, _, _, _) => self.pc += if vx != vy { 2 } else { 0 },
            // LD I
            (0xA, _, _, _) => self.i = nnn,
            // JP V0
            (0xB, _, _, _) => self.pc = nnn + self.v[0] as u16,
            // RND
            (0xC, _, _, _) => self.v[x] = random::<u8>() & kk,
            // DRW
            (0xD, _, _, _) => {
                let sprite = self.interconnect.memory.get_slice(self.i as usize, n);
                let collision = self.interconnect.graphics.draw(vx as usize, vy as usize, sprite);
                self.v[0xF] = if collision { 1 } else { 0 };
            }
            // SKP Vx
            (0xE, _, 0x9, 0xE) => self.pc += if self.interconnect.input.is_key_down(vx) { 2 } else { 0 },
            // SKNP Vx
            (0xE, _, 0xA, 0x1) => self.pc += if self.interconnect.input.is_key_down(vx) { 0 } else { 2 },
            // LD Vx, DT
            (0xF, _, 0x0, 0x7) => self.v[x] = self.dt,
            // LD Vx, K
            (0xF, _, 0x0, 0xA) => {
                let key = self.interconnect.input.wait_input();
                self.v[x] = key;
            },
            // LD DT, Vx
            (0xF, _, 0x1, 0x5) => self.dt = self.v[x],
            // ADD I, Vx
            (0xF, _, 0x1, 0xE) => self.i = self.i + self.v[x] as u16,
            // LD F, Vx
            (0xF, _, 0x2, 0x9) => self.i = vx as u16 * 5,
            // LD B, Vx
            (0xF, _, 0x3, 0x3) => {
                self.interconnect.memory.write(self.i as usize, vx / 100);
                self.interconnect.memory.write(self.i as usize + 1, (vx / 10) % 10);
                self.interconnect.memory.write(self.i as usize + 2, (vx % 100) % 10);
            },
            // LD [I], Vx
            (0xF, _, 0x5, 0x5) => self.interconnect.memory.get_slice_mut(self.i as usize, x as u8 + 1)
                        .copy_from_slice(&self.v[0..(x as usize + 1)]),
            // LD Vx, [I]
            (0xF, _, 0x6, 0x5) =>  self.v[0..(x as usize + 1)]
                        .copy_from_slice(&self.interconnect.memory.get_slice_mut(self.i as usize, x as u8 + 1)),
            (_, _, _, _) => ()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use input::TestKeyboard;
    use graphics::TestDisplay;

    fn get_cpu() -> Cpu<TestKeyboard, TestDisplay> {
        let rom = vec![0_u8; 10];
        let interconnect = Interconnect::new_test(rom);
        let cpu = Cpu::new(interconnect);
        cpu
    }

    #[test]
    fn opcode_jp() {
        let mut cpu = get_cpu();
        cpu.process_opcode(0x1A2A);
        assert_eq!(cpu.pc, 0x0A2A, "the program counter is updated");
    }

    #[test]
    fn opcode_call() {
        let mut cpu = get_cpu();
        let addr = 0x23;
        cpu.pc = addr;

        cpu.process_opcode(0x2ABC);

        assert_eq!(cpu.pc, 0x0ABC, "the program counter is updated to the new address");
        assert_eq!(cpu.sp, 1, "the stack pointer is incremented");
        assert_eq!(cpu.stack[0], addr + 2, "the stack stores the previous address");
    }

    #[test]
    fn opcode_se_vx_byte() {
        let mut cpu = get_cpu();
        cpu.v[1] = 0xFE;

        // vx == kk
        cpu.process_opcode(0x31FE);
        assert_eq!(cpu.pc, 4, "the stack pointer skips");

        // vx != kk
        cpu.process_opcode(0x31FA);
        assert_eq!(cpu.pc, 6, "the stack pointer is incremented");
    }

    #[test]
    fn opcode_sne_vx_byte() {
        let mut cpu = get_cpu();
        cpu.v[1] = 0xFE;

        // vx == kk
        cpu.process_opcode(0x41FE);
        assert_eq!(cpu.pc, 2, "the stack pointer is incremented");

        // vx != kk
        cpu.process_opcode(0x41FA);
        assert_eq!(cpu.pc, 6, "the stack pointer skips");
    }

    #[test]
    fn opcode_se_vx_vy() {
        let mut cpu = get_cpu();
        cpu.v[1] = 1;
        cpu.v[2] = 3;
        cpu.v[3] = 3;

        // vx == vy
        cpu.process_opcode(0x5230);
        assert_eq!(cpu.pc, 4, "the stack pointer skips");

        // vx != vy
        cpu.process_opcode(0x5130);
        assert_eq!(cpu.pc, 6, "the stack pointer is incremented");
    }

    #[test]
    fn opcode_sne_vx_vy() {
        let mut cpu = get_cpu();
        cpu.v[1] = 1;
        cpu.v[2] = 3;
        cpu.v[3] = 3;

        // vx == vy
        cpu.process_opcode(0x9230);
        assert_eq!(cpu.pc, 2, "the stack pointer is incremented");

        // vx != vy
        cpu.process_opcode(0x9130);
        assert_eq!(cpu.pc, 6, "the stack pointer skips");
    }

    #[test]
    fn opcode_add_vx_kkk() {
        let mut cpu = get_cpu();
        cpu.v[1] = 3;

        cpu.process_opcode(0x7101);
        assert_eq!(cpu.v[1], 4, "Vx was incremented by one");
    }

    #[test]
    fn opcode_ld_vx_vy() {
        let mut cpu = get_cpu();
        cpu.v[1] = 3;
        cpu.v[0] = 0;

        cpu.process_opcode(0x8010);
        assert_eq!(cpu.v[0], 3, "Vx was loaded with vy");
    }

    #[test]
    fn opcode_or_vx_vy() {
        let mut cpu = get_cpu();
        cpu.v[2] = 0b01101100;
        cpu.v[3] = 0b11001110;

        cpu.process_opcode(0x8231);
        assert_eq!(cpu.v[2], 0b11101110, "Vx was loaded with vx OR vy");
    }

    #[test]
    fn opcode_and_vx_vy() {
        let mut cpu = get_cpu();
        cpu.v[2] = 0b01101100;
        cpu.v[3] = 0b11001110;

        cpu.process_opcode(0x8232);
        assert_eq!(cpu.v[2], 0b01001100, "Vx was loaded with vx AND vy");
    }

    #[test]
    fn opcode_xor_vx_vy() {
        let mut cpu = get_cpu();
        cpu.v[2] = 0b01101100;
        cpu.v[3] = 0b11001110;

        cpu.process_opcode(0x8233);
        assert_eq!(cpu.v[2], 0b10100010, "Vx was loaded with vx XOR vy");
    }

    #[test]
    fn opcode_add_vx_vy() {
        let mut cpu = get_cpu();
        cpu.v[1] = 10;
        cpu.v[2] = 100;
        cpu.v[3] = 250;

        cpu.process_opcode(0x8124);
        assert_eq!(cpu.v[1], 110, "Vx was loaded with vx + vy");
        assert_eq!(cpu.v[0xF], 0, "no overflow occured");

        cpu.process_opcode(0x8134);
        assert_eq!(cpu.v[1], 0x68, "Vx was loaded with vx + vy");
        assert_eq!(cpu.v[0xF], 1, "overflow occured");
    }

    #[test]
    fn opcode_ld_i_vx() {
        let mut cpu = get_cpu();
        cpu.v[0] = 5;
        cpu.v[1] = 4;
        cpu.v[2] = 3;
        cpu.v[3] = 2;
        cpu.i = 0x300;

        // load v0 - v2 into memory at i
        cpu.process_opcode(0xF255);
        assert_eq!(cpu.interconnect.memory.read(cpu.i as usize), 5, "V0 was loaded into memory at i");
        assert_eq!(cpu.interconnect.memory.read(cpu.i as usize + 1), 4, "V1 was loaded into memory at i + 1");
        assert_eq!(cpu.interconnect.memory.read(cpu.i as usize + 2), 3, "V2 was loaded into memory at i + 2");
        assert_eq!(cpu.interconnect.memory.read(cpu.i as usize + 3), 0, "i + 3 was not loaded");
    }

    #[test]
    fn opcode_ld_b_vx() {
        let mut cpu = get_cpu();
        cpu.i = 0x300;
        cpu.v[2] = 234;

        // load v0 - v2 from memory at i
        cpu.process_opcode(0xF233);
        assert_eq!(cpu.interconnect.memory.read(cpu.i as usize), 2, "hundreds");
        assert_eq!(cpu.interconnect.memory.read(cpu.i as usize + 1), 3, "tens");
        assert_eq!(cpu.interconnect.memory.read(cpu.i as usize + 2), 4, "digits");
    }

    #[test]
    fn opcode_ld_vx_i() {
        let mut cpu = get_cpu();
        cpu.i = 0x300;
        cpu.interconnect.memory.write(cpu.i as usize, 5);
        cpu.interconnect.memory.write(cpu.i as usize + 1, 4);
        cpu.interconnect.memory.write(cpu.i as usize + 2, 3);
        cpu.interconnect.memory.write(cpu.i as usize + 3, 2);


        // load v0 - v2 from memory at i
        cpu.process_opcode(0xF265);
        assert_eq!(cpu.v[0], 5, "V0 was loaded from memory at i");
        assert_eq!(cpu.v[1], 4, "V1 was loaded from memory at i + 1");
        assert_eq!(cpu.v[2], 3, "V2 was loaded from memory at i + 2");
        assert_eq!(cpu.v[3], 0, "i + 3 was not loaded");
    }

    #[test]
    fn opcode_ret() {
        let mut cpu = get_cpu();
        let addr = 0x23;
        cpu.pc = addr;

        // jump to 0x0ABC
        cpu.process_opcode(0x2ABC);
        // return
        cpu.process_opcode(0x00EE);

        assert_eq!(cpu.pc, 0x25, "the program counter is updated to the new address");
        assert_eq!(cpu.sp, 0, "the stack pointer is decremented");
    }


    #[test]
    fn opcode_ld_i_addr() {
        let mut cpu = get_cpu();

        cpu.process_opcode(0x61AA);
        assert_eq!(cpu.v[1], 0xAA, "V1 is set");
        assert_eq!(cpu.pc, 2, "the program counter is advanced two bytes");

        cpu.process_opcode(0x621A);
        assert_eq!(cpu.v[2], 0x1A, "V2 is set");
        assert_eq!(cpu.pc, 4, "the program counter is advanced two bytes");

        cpu.process_opcode(0x6A15);
        assert_eq!(cpu.v[10], 0x15, "V10 is set");
        assert_eq!(cpu.pc, 6, "the program counter is advanced two bytes");
    }

    #[test]
    fn opcode_axxx() {
        let mut cpu = get_cpu();
        cpu.process_opcode(0xAFAF);

        assert_eq!(cpu.i, 0x0FAF, "the 'i' register is updated");
        assert_eq!(cpu.pc, 2, "the program counter is advanced two bytes");
    }
}
