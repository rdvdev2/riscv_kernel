use super::{raw_sbi_call_0_args, raw_sbi_call_1_arg};

use derive_more::Display;

#[derive(Clone, Copy, Debug, Display)]
#[display("{major}.{minor}")]
pub struct SbiSpecVersion {
    major: usize,
    minor: usize,
}

pub fn sbi_get_spec_version() -> SbiSpecVersion {
    let result = unsafe { raw_sbi_call_0_args::<0x10, 0>() }.expect("This call is infallible");

    SbiSpecVersion {
        major: (result >> 24) & 0x7F,
        minor: (result >> 0) & 0xFFFFFF,
    }
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, Display)]
pub enum SbiImplementationId {
    BerkeleyBootLoader = 0,
    OpenSBI = 1,
    Xvisor = 2,
    KVM = 3,
    RustSBI = 4,
    Diosix = 5,
    Coffer = 6,
    XenProject = 7,
    PolarFire = 8,
    Coreboot = 9,
    Oreboot = 10,
    Other(usize),
}

impl From<usize> for SbiImplementationId {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::BerkeleyBootLoader,
            1 => Self::OpenSBI,
            2 => Self::Xvisor,
            3 => Self::KVM,
            4 => Self::RustSBI,
            5 => Self::Diosix,
            6 => Self::Coffer,
            7 => Self::XenProject,
            8 => Self::PolarFire,
            9 => Self::Coreboot,
            10 => Self::Oreboot,
            x => Self::Other(x),
        }
    }
}

pub fn sbi_get_impl_id() -> SbiImplementationId {
    unsafe { raw_sbi_call_0_args::<0x10, 1>() }
        .expect("This call is infallible")
        .into()
}

pub fn sbi_get_impl_version() -> usize {
    unsafe { raw_sbi_call_0_args::<0x10, 2>() }.expect("This call is infallible")
}

pub fn sbi_probe_extension(extension_id: usize) -> usize {
    unsafe { raw_sbi_call_1_arg::<0x10, 3>(extension_id) }.expect("This call is infallible")
}

pub fn sbi_get_mvendorid() -> usize {
    unsafe { raw_sbi_call_0_args::<0x10, 4>() }.expect("This call is infallible")
}

pub fn sbi_get_marchid() -> usize {
    unsafe { raw_sbi_call_0_args::<0x10, 5>() }.expect("This call is infallible")
}

pub fn sbi_get_mimpid() -> usize {
    unsafe { raw_sbi_call_0_args::<0x10, 6>() }.expect("This call is infallible")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test_case]
    fn all_calls_succeed() {
        sbi_get_spec_version();
        sbi_get_impl_id();
        sbi_get_impl_version();
        sbi_probe_extension(0);
        sbi_get_mvendorid();
        sbi_get_marchid();
        sbi_get_mimpid();
    }

    #[test_case]
    fn base_extension_present() {
        assert_eq!(sbi_probe_extension(0x10), 1);
    }
}
