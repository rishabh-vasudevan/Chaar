unsafe extern "C" {
    pub fn write(fd: i32, buf: *const u8, count: usize) -> isize;
}

fn main() {
    let msg = "Hello from rust from a syscall!!\n";
    unsafe {
        write(1, msg.as_ptr(), msg.len());
    }
}
