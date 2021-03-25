fn main() {
    let ac = autocfg::new();
    // Version 1.36.0 where `extern crate alloc` was stabilized
    ac.emit_sysroot_crate("alloc");
    // Version where rules for implementing traits on foreign types were relaxed
    ac.emit_rustc_version(1, 41);
    // Version where doc(alias) was stabilized
    ac.emit_rustc_version(1, 48);
}
