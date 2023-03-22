use std::{sync::atomic::{AtomicU64, Ordering::Relaxed}, mem::transmute, fmt::Debug};

const PX: u64 = 1 << 32;
const PERC: u64 = 2 << 32;
const ALL: u64 = 3 << 32;

#[derive(Copy, Clone, Debug)]
pub enum CoordValue {
    Px(f32),
    Perc(f32),
    Auto
}

#[derive(Default)]
pub struct Coord(AtomicU64);
impl Coord {
    #[inline(always)]
    pub fn set_px(&self, v: f32) {
        self.0.store(unsafe { transmute::<f32, u32>(v) } as u64 | PX, Relaxed)
    }
    #[inline(always)]
    pub fn set_perc(&self, v: f32) {
        self.0.store(unsafe { transmute::<f32, u32>(v) } as u64 | PERC, Relaxed)
    }
    #[inline(always)]
    pub fn set_auto(&self) {
        self.0.store(0, Relaxed)
    }
    #[inline(always)]
    pub fn get(&self) -> CoordValue {
        let v = self.0.load(Relaxed);
        match v & ALL {
            0 => CoordValue::Auto,
            PX => CoordValue::Px(unsafe { transmute(v as u32) }),
            PERC => CoordValue::Perc(unsafe { transmute(v as u32) }),
            _ => unreachable!()
        }
    }
}
impl Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.get())
    }
}