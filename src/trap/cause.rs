use core::arch::asm;

#[derive(Debug, Clone, Copy)]
pub enum ExceptionCause {
    InstructionAddressMisaligned,
    InstructionAccessFault,
    IllegalInstruction,
    Breakpoint,
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAddressMisaligned,
    StoreAccessFault,
    UserEnvironmentCall,
    SupervisorEnvironmentCall,
    InstructionPageFault,
    LoadPageFault,
    StorePageFault,
    StandardOther(usize),
    CustomOther(usize),
}

impl From<usize> for ExceptionCause {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::InstructionAddressMisaligned,
            1 => Self::InstructionAccessFault,
            2 => Self::IllegalInstruction,
            3 => Self::Breakpoint,
            4 => Self::LoadAddressMisaligned,
            5 => Self::LoadAccessFault,
            6 => Self::StoreAddressMisaligned,
            7 => Self::StoreAccessFault,
            8 => Self::UserEnvironmentCall,
            9 => Self::SupervisorEnvironmentCall,
            12 => Self::InstructionPageFault,
            13 => Self::LoadPageFault,
            15 => Self::StorePageFault,
            24..=31 | 48..=63 => Self::CustomOther(value),
            _ => Self::StandardOther(value),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum InterruptCause {
    UserSoftware,
    SupervisorSoftware,
    UserTimer,
    SupervisorTimer,
    UserExternal,
    SupervisorExternal,
    StandardOther(usize),
    PlatformOther(usize),
}

impl From<usize> for InterruptCause {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::UserSoftware,
            1 => Self::SupervisorSoftware,
            4 => Self::UserTimer,
            5 => Self::SupervisorTimer,
            8 => Self::UserExternal,
            9 => Self::SupervisorExternal,
            ..=15 => Self::StandardOther(value),
            _ => Self::PlatformOther(value),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TrapCause {
    Exception(ExceptionCause),
    Interrupt(InterruptCause),
}

impl From<usize> for TrapCause {
    fn from(value: usize) -> Self {
        if value >> (usize::BITS - 1) == 0 {
            Self::Exception(value.into())
        } else {
            Self::Interrupt((value & (usize::MAX - 1)).into())
        }
    }
}

impl TrapCause {
    pub fn get() -> TrapCause {
        let scause: usize;
        unsafe { asm!("csrr {val}, scause", val = out(reg) scause) };
        scause.into()
    }
}
