use nix::unistd::Uid;
use std::process::Command;
fn assert_root() -> bool {
    Uid::effective().is_root()
}

pub fn assert_modprobe() {
    if assert_root() {
        // TODO: check if it's already enabled
        // In that case just return true
        let _ = Command::new("modprobe")
            .args(["ec_sys", "write_support=1"])
            .status();
    }
}
