mod context;

use riscv::register::{
    mtvec::TrapMode,
    stvec,
    scause::{
        self,
        Trap,
        Exception,
    },
    stval,
};
use crate::{syscall::syscall, batch::USER_STACK_SIZE};
use crate::batch::run_next_app;
use core::arch::global_asm;

global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" { fn __alltraps(); }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        // 目前教学演示的代码为了链接执行方便内存地址用的都是绝对地址
        Trap::Exception(Exception::UserEnvCall) => {
            // check user stack sp?
            // dbg!(cx.sstatus);
            // check buf ptr address
            let a1 = cx.x[11];
            // println!("if is sys_write ptr_arg={:x}", a1);
            // check sp
            let sp = cx.x[2];
            // println!("sp from ctx={:x}", sp);
            cx.sepc += 4;
            // only stack and .data is valid range
            
            if a1 > sp || a1 < sp - USER_STACK_SIZE {
                println!("invalid memory access, a1 = {:x}", a1);
            } else {
                cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
            }
        }
        Trap::Exception(Exception::StoreFault) |
        Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, core dumped.");
            run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, core dumped.");
            run_next_app();
        }
        _ => {
            panic!("Unsupported trap {:?}, stval = {:#x}!", scause.cause(), stval);
        }
    }
    cx
}

pub use context::TrapContext;