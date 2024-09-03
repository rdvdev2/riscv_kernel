use flat_device_tree::Fdt;

// Safety: dtb_addr must be valid
pub unsafe fn get_dtb(dtb_addr: *const u8) -> Fdt<'static> {
    unsafe { Fdt::from_ptr(dtb_addr).expect("GDB DTB unparsable") }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::assert_matches::assert_matches;

    // TODO: Fix to use SBI provided DTB
    #[test_case]
    fn dtb_parsable() {
        // assert_matches!(
        //     locate_gdb_dtb().and_then(|addr| Some(unsafe { Fdt::from_ptr(addr) })),
        //     Some(Ok(_))
        // )
    }
}
