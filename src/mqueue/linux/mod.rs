pub use self::arch::{mqd_t};
pub use self::arch::{mq_attr};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;
