use memfd_exec::MemFdExecutable;

const RWBINARY: &[u8] = include_bytes!("../bin/binary"); 

fn main() {
    let mut binding = MemFdExecutable::new("payload", RWBINARY);
    let exe = binding.arg("--version");
    exe.spawn().unwrap().wait().unwrap();
}