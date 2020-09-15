const XLEN: usize = 32;

struct Cpu {
    x: [u32; XLEN],
    pc: u32,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu{x: Default::default(), pc: 0}
    }

    pub fn reset(&mut self) {

    }

    fn decode(&mut self, inst :u32) {
        let opcode = inst & 0x0000007f;
        match opcode {
            0b0010011 => {
                let imm = (inst >> 20) & 0x00000FFF;
                let rs1 = ((inst >> 15) & 0x0000001F) as usize;
                let rd  = ((inst >> 7)  & 0x0000001F) as usize;
                self.x[rd] = self.x[rs1] + imm;
            },
            _ => unimplemented!(),
        };
    }

    fn fetch(&mut self) -> u32 {
        0
    }
    pub fn step(&mut self) {
        let inst = self.fetch();
        self.decode(inst);

        self.pc += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut cpu = Cpu::new();
        cpu.reset();
        let mut inst = 0;
        inst |= 1 << 20;
        inst |= 1 << 15;
        inst |= 1 << 7;
        inst |= 0x13 << 0; 
        cpu.decode(inst);
        assert!(cpu.x[1] == 1);
    }
}
