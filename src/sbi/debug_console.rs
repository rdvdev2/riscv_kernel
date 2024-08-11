use super::{raw_sbi_call_1_arg, raw_sbi_call_3_args};

pub const EID: usize = 0x4442434E;

pub fn sbi_debug_console_write(s: &str) -> Result<usize, usize> {
    let bytes = s.as_bytes();
    let base_addr = bytes.as_ptr() as usize;

    unsafe { raw_sbi_call_3_args::<EID, 0>(bytes.len(), base_addr, 0) }
}

pub fn sbi_debug_console_read(s: &mut str) -> Result<usize, usize> {
    let bytes = unsafe { s.as_bytes_mut() };
    let base_addr = bytes.as_mut_ptr() as usize;

    unsafe { raw_sbi_call_3_args::<EID, 1>(bytes.len(), base_addr, 0) }
}

pub fn sbi_debug_console_write_byte(b: u8) -> Result<(), usize> {
    unsafe { raw_sbi_call_1_arg::<EID, 2>(b as usize) }.map(|_| ())
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::sbi::base::sbi_probe_extension;

    #[test_case]
    fn extension_present() {
        assert_eq!(sbi_probe_extension(EID), 1);
    }
}
