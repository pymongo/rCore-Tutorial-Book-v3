use crate::batch::{APP_BASE_ADDRESS, APP_SIZE_LIMIT, USER_STACK, USER_STACK_SIZE};

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {}
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }

    let ptr = buf as usize;
    if ptr < APP_BASE_ADDRESS {
        return -1;
    }
    if ptr + len > APP_BASE_ADDRESS + APP_SIZE_LIMIT {
        return -1;
    }
    println!("sys_write: ptr = {:x}", ptr);

    let slice = unsafe { core::slice::from_raw_parts(buf, len) };
        let str = core::str::from_utf8(slice).unwrap();
        print!("{}", str);
        len as isize
}
