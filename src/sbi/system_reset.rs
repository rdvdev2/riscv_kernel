use super::raw_sbi_call_2_args;

pub const EID: usize = 0x53525354;

pub enum SystemResetType {
    Shutdown,
    ColdReboot,
    WarmReboot,
    Custom(usize),
}

impl From<SystemResetType> for usize {
    fn from(value: SystemResetType) -> Self {
        match value {
            SystemResetType::Shutdown => 0,
            SystemResetType::ColdReboot => 1,
            SystemResetType::WarmReboot => 2,
            SystemResetType::Custom(x) => x,
        }
    }
}

pub enum SystemResetReason {
    NoReason,
    SystemFailure,
    Custom(usize),
}

impl From<SystemResetReason> for usize {
    fn from(value: SystemResetReason) -> Self {
        match value {
            SystemResetReason::NoReason => 0,
            SystemResetReason::SystemFailure => 1,
            SystemResetReason::Custom(x) => x,
        }
    }
}

pub fn sbi_system_reset(
    reset_type: SystemResetType,
    reset_reason: SystemResetReason,
) -> Result<!, usize> {
    unsafe { raw_sbi_call_2_args::<EID, 0>(reset_type.into(), reset_reason.into()) }
        .map(|_| unreachable!("System reset success doesn't return"))
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
