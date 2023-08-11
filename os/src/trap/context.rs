use riscv::register::sstatus::{Sstatus, self};

#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) { self.x[2] = sp; }
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let sstatus = sstatus::read();
        let mut bits: usize = unsafe { core::mem::transmute(sstatus) };
        bits &= !(1<<8);
        let mut cx = Self {
            x: [0; 32],
            sstatus: unsafe { core::mem::transmute(bits) },
            sepc: entry,
        };
        cx.set_sp(sp);
        cx
    }
}
