use flat_device_tree::Fdt;

fn locate_gdb_dtb() -> Option<*const u8> {
    let magic = 0xd00dfeed_u32.to_be();
    let search_start = 0x80000000_usize; // Start of DRAM
    let search_end = search_start + 0xC0000000; // 3GB search limit

    for addr in (search_start..search_end).step_by(0x100000) {
        let value = unsafe { *(addr as *const u32) };
        if value == magic {
            return Some(addr as *const u8);
        }
    }

    None
}

pub fn get_dtb() -> Fdt<'static> {
    let addr = locate_gdb_dtb().expect("GDB DTB not found");
    unsafe { Fdt::from_ptr(addr).expect("GDB DTB unparsable") }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::assert_matches::assert_matches;

    #[test_case]
    fn dtb_found() {
        assert_matches!(locate_gdb_dtb(), Some(_))
    }

    #[test_case]
    fn dtb_parsable() {
        assert_matches!(
            locate_gdb_dtb().and_then(|addr| Some(unsafe { Fdt::from_ptr(addr) })),
            Some(Ok(_))
        )
    }
}
