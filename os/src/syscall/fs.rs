use crate::config::{APP_BASE_ADDRESS, APP_SIZE_LIMIT};

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            if buf.is_null() {
                return -1;
            }
            let ptr = buf as usize;
            if ptr < APP_BASE_ADDRESS {
                return -1;
            }
            if ptr + len > APP_BASE_ADDRESS + APP_SIZE_LIMIT {
                return -1;
            }
            // csrr t2, sscratch
            // let mut t3: usize;
            // unsafe { core::arch::asm!("csrr {}, sscratch", out(reg) t3) };
            // let buf_addr = buf as usize;
            // if t3 - 4096 < buf_addr || buf_addr > t3 + 4096 {
            //     return -1;
            // }

            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        },
        fd => {
            println!("Unsupported {} in sys_write!", fd);
            -1
        }
    }
}