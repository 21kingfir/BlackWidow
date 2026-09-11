use memexec::memexec_exe;

fn main() {
    let RWBINARY: &[u8] = include_bytes!("../bin/binary"); 

    unsafe {
        memexec_exe(RWBINARY).unwrap();
    }
}