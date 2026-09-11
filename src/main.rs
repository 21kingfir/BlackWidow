use memfd_exec::MemFdExecutable;

const rwbinary: &[u8] = include_bytes!("../bin/binary"); 

fn main() {
    let exe = MemFdExecutable::new("payload", rwbinary.to_vec()).arg("--version");
    exe.spawn().unwrap().wait().unwrap();
}