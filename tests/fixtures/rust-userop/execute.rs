use std::sync::atomic::{AtomicU32, Ordering::SeqCst};

static CALLS: AtomicU32 = AtomicU32::new(0);
static PORT: AtomicU32 = AtomicU32::new(0);
static DATA: AtomicU32 = AtomicU32::new(0);
static BREAKS: AtomicU32 = AtomicU32::new(0);
static YIELDS: AtomicU32 = AtomicU32::new(0);
static mut R1: u8 = 255;

fn r#in(port: i16) -> u32 {
    CALLS.fetch_add(1, SeqCst);
    PORT.store(port as u16 as u32, SeqCst);
    port as u16 as u32 ^ 0x12345678
}

fn out(port: i16, data: u8) {
    CALLS.fetch_add(1, SeqCst);
    PORT.store(port as u16 as u32, SeqCst);
    DATA.store(data as u32, SeqCst);
}

fn r#break() {
    BREAKS.fetch_add(1, SeqCst);
}

fn r#yield(value: u32) {
    YIELDS.fetch_add(1, SeqCst);
    DATA.store(value, SeqCst);
}

fn main() {
    unsafe {
        assert_eq!(read_port(), (0x31 ^ 0x78) & 255);
        assert_eq!(CALLS.load(SeqCst), 1);
        assert_eq!(PORT.load(SeqCst), 0x31);
        let mut calls = 1;
        for port in [0, 1, 255, 32768, 65535, 65536, u32::MAX] {
            assert_eq!(read_variable(port), (port ^ 0x78) & 255);
            calls += 1;
            assert_eq!(CALLS.load(SeqCst), calls);
            assert_eq!(PORT.load(SeqCst), port & 65535);
            for data in [0, 127, 128, 255] {
                write_port(port, data);
                calls += 1;
                assert_eq!(CALLS.load(SeqCst), calls);
                assert_eq!(PORT.load(SeqCst), port & 65535);
                assert_eq!(DATA.load(SeqCst), data as u32);
            }
        }
        avr_break();
        avr_break();
        assert_eq!(BREAKS.load(SeqCst), 2);
        let zero_register = R1;
        assert_eq!(zero_register, 0);
        for (index, value) in [0, 1, 0x80000000, u32::MAX].iter().enumerate() {
            cooperate(*value);
            assert_eq!(YIELDS.load(SeqCst), index as u32 + 1);
            assert_eq!(DATA.load(SeqCst), *value);
        }
    }
    println!("42 machine operation calls checked");
}
