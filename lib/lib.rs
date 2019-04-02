use std::fmt;


mod opcode_table;
use opcode_table::AddressMode;

pub use opcode_table::Opcode;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Operand {
    Absolute(u16),
    AbsoluteX(u16),
    AbsoluteY(u16),
    Accumulator,
    Immediate(u8),
    Implied,
    IndexedIndirect(u8),
    Indirect(u16),
    IndirectIndexed(u8),
    Relative(i8),
    ZeroPage(u8),
    ZeroPageX(u8),
    ZeroPageY(u8)
}


impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operand::Absolute(v) => write!(f, "${:04X}", v),
            Operand::AbsoluteX(v) => write!(f, "${:04X}", v),
            Operand::AbsoluteY(v) => write!(f, "${:04X}", v),
            Operand::Accumulator => write!(f, "A"),
            Operand::Immediate(v) => write!(f, "#${:02X}", v),
            Operand::Implied => write!(f, ""),
            Operand::Indirect(v) => write!(f, "(${:04X})", v),
            Operand::IndexedIndirect(v) => write!(f, "(${:02X},X)", v),
            Operand::IndirectIndexed(v) => write!(f, "(${:02X}),Y", v),
            Operand::Relative(v) => write!(f, "${:02X}", v),
            Operand::ZeroPage(v) => write!(f, "${:02X}", v),
            Operand::ZeroPageX(v) => write!(f, "${:02X}", v),
            Operand::ZeroPageY(v) => write!(f, "${:02X}", v),
        }
    }
}



#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
    operand: Operand,
    size: usize
}


impl Instruction {
    fn new(opcode: Opcode, operand: Operand, size: usize) -> Instruction {
        Instruction {
            opcode: opcode,
            operand: operand,
            size: size
        }
    }

    pub fn opcode(&self) -> &Opcode {
        &self.opcode
    }

    pub fn operand(&self) -> &Operand {
        &self.operand
    }

    pub fn size(&self) -> usize {
        self.size
    }
}


impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.operand {
            Operand::Implied => self.opcode.fmt(f),
            _ => write!(f, "{} {}", self.opcode, self.operand)
        }
    }
}



pub fn decode(buf: &[u8]) -> Option<Instruction> {
    fn op16(buf: &[u8]) -> Option<u16> {
        Some(((buf.get(1)?.clone() as u16) << 8) | (buf.get(2)?.clone() as u16))
    }

    fn op8(buf: &[u8]) -> Option<u8> {
        Some(buf.get(1)?.clone())
    }

    let opcode = buf.get(0)?.clone();
    let &(ref opcode, ref address_mode) =
        opcode_table::OPCODE_TABLE[opcode as usize].as_ref()?;


    let (operand, size) = match *address_mode {
        AddressMode::Absolute => (Operand::Absolute(op16(buf)?), 3),
        AddressMode::AbsoluteX => (Operand::AbsoluteX(op16(buf)?), 3),
        AddressMode::AbsoluteY => (Operand::AbsoluteY(op16(buf)?), 3),
        AddressMode::Accumulator => (Operand::Accumulator, 1),
        AddressMode::Immediate => (Operand::Immediate(op8(buf)?), 2),
        AddressMode::Implied => (Operand::Implied, 1),
        AddressMode::IndexedIndirect => (Operand::IndexedIndirect(op8(buf)?), 2),
        AddressMode::Indirect => (Operand::Indirect(op16(buf)?), 3),
        AddressMode::IndirectIndexed => (Operand::IndirectIndexed(op8(buf)?), 2),
        AddressMode::Relative => (Operand::Relative(op8(buf)? as i8), 2),
        AddressMode::ZeroPage => (Operand::ZeroPage(op8(buf)?), 2),
        AddressMode::ZeroPageX => (Operand::ZeroPageX(op8(buf)?), 2),
        AddressMode::ZeroPageY => (Operand::ZeroPageY(op8(buf)?), 2),
    };

    Some(Instruction::new(opcode.clone(), operand, size))
}