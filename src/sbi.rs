use core::arch::asm;

pub mod base;

pub unsafe fn raw_sbi_call_no_args<const EID: usize, const FID: usize>() -> Result<usize, usize> {
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

pub unsafe fn raw_sbi_call_1_arg<const EID: usize, const FID: usize>(
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
