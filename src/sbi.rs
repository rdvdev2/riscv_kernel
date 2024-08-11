use core::arch::asm;

pub mod base;
pub mod debug_console;
pub mod system_reset;

unsafe fn raw_sbi_call_0_args<const EID: usize, const FID: usize>() -> Result<usize, usize> {
    let (error, value);

    unsafe {
        asm!("ecall", in("a7") EID, in("a6") FID, lateout("a0") error, lateout("a1") value);
    }

    if error == 0 {
        Ok(value)
    } else {
        Err(error)
    }
}

unsafe fn raw_sbi_call_1_arg<const EID: usize, const FID: usize>(
    arg0: usize,
) -> Result<usize, usize> {
    let (error, value);

    unsafe {
        asm!("ecall", in("a7") EID, in("a6") FID, in("a0") arg0, lateout("a0") error, lateout("a1") value);
    }

    if error == 0 {
        Ok(value)
    } else {
        Err(error)
    }
}

unsafe fn raw_sbi_call_2_args<const EID: usize, const FID: usize>(
    arg0: usize,
    arg1: usize,
) -> Result<usize, usize> {
    let (error, value);

    unsafe {
        asm!("ecall", in("a7") EID, in("a6") FID, in("a0") arg0, in("a1") arg1, lateout("a0") error, lateout("a1") value);
    }

    if error == 0 {
        Ok(value)
    } else {
        Err(error)
    }
}

unsafe fn raw_sbi_call_3_args<const EID: usize, const FID: usize>(
    arg0: usize,
    arg1: usize,
    arg2: usize,
) -> Result<usize, usize> {
    let (error, value);

    unsafe {
        asm!("ecall", in("a7") EID, in("a6") FID, in("a0") arg0, in("a1") arg1, in("a2") arg2, lateout("a0") error, lateout("a1") value);
    }

    if error == 0 {
        Ok(value)
    } else {
        Err(error)
    }
}
