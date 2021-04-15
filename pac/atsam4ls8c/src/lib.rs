#![doc = "Peripheral access API for ATSAM4LS8C microcontrollers (generated using svd2rust v0.17.0 (2bbb605 2020-05-16))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {
    fn HFLASHC();
    fn PDCA_0();
    fn PDCA_1();
    fn PDCA_2();
    fn PDCA_3();
    fn PDCA_4();
    fn PDCA_5();
    fn PDCA_6();
    fn PDCA_7();
    fn PDCA_8();
    fn PDCA_9();
    fn PDCA_10();
    fn PDCA_11();
    fn PDCA_12();
    fn PDCA_13();
    fn PDCA_14();
    fn PDCA_15();
    fn CRCCU();
    fn USBC();
    fn PEVC_TR();
    fn PEVC_OV();
    fn PM();
    fn SCIF();
    fn FREQM();
    fn GPIO_0();
    fn GPIO_1();
    fn GPIO_2();
    fn GPIO_3();
    fn GPIO_4();
    fn GPIO_5();
    fn GPIO_6();
    fn GPIO_7();
    fn GPIO_8();
    fn GPIO_9();
    fn GPIO_10();
    fn GPIO_11();
    fn BPM();
    fn BSCIF();
    fn AST_ALARM();
    fn AST_PER();
    fn AST_OVF();
    fn AST_READY();
    fn AST_CLKREADY();
    fn WDT();
    fn EIC_1();
    fn EIC_2();
    fn EIC_3();
    fn EIC_4();
    fn EIC_5();
    fn EIC_6();
    fn EIC_7();
    fn EIC_8();
    fn IISC();
    fn SPI();
    fn TC00();
    fn TC01();
    fn TC02();
    fn TC10();
    fn TC11();
    fn TC12();
    fn TWIM0();
    fn TWIS0();
    fn TWIM1();
    fn TWIS1();
    fn USART0();
    fn USART1();
    fn USART2();
    fn USART3();
    fn ADCIFE();
    fn DACC();
    fn ACIFC();
    fn ABDACB();
    fn TRNG();
    fn PARC();
    fn CATB();
    fn TWIM2();
    fn TWIM3();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 79] = [
    Vector { _handler: HFLASHC },
    Vector { _handler: PDCA_0 },
    Vector { _handler: PDCA_1 },
    Vector { _handler: PDCA_2 },
    Vector { _handler: PDCA_3 },
    Vector { _handler: PDCA_4 },
    Vector { _handler: PDCA_5 },
    Vector { _handler: PDCA_6 },
    Vector { _handler: PDCA_7 },
    Vector { _handler: PDCA_8 },
    Vector { _handler: PDCA_9 },
    Vector { _handler: PDCA_10 },
    Vector { _handler: PDCA_11 },
    Vector { _handler: PDCA_12 },
    Vector { _handler: PDCA_13 },
    Vector { _handler: PDCA_14 },
    Vector { _handler: PDCA_15 },
    Vector { _handler: CRCCU },
    Vector { _handler: USBC },
    Vector { _handler: PEVC_TR },
    Vector { _handler: PEVC_OV },
    Vector { _reserved: 0 },
    Vector { _handler: PM },
    Vector { _handler: SCIF },
    Vector { _handler: FREQM },
    Vector { _handler: GPIO_0 },
    Vector { _handler: GPIO_1 },
    Vector { _handler: GPIO_2 },
    Vector { _handler: GPIO_3 },
    Vector { _handler: GPIO_4 },
    Vector { _handler: GPIO_5 },
    Vector { _handler: GPIO_6 },
    Vector { _handler: GPIO_7 },
    Vector { _handler: GPIO_8 },
    Vector { _handler: GPIO_9 },
    Vector { _handler: GPIO_10 },
    Vector { _handler: GPIO_11 },
    Vector { _handler: BPM },
    Vector { _handler: BSCIF },
    Vector {
        _handler: AST_ALARM,
    },
    Vector { _handler: AST_PER },
    Vector { _handler: AST_OVF },
    Vector {
        _handler: AST_READY,
    },
    Vector {
        _handler: AST_CLKREADY,
    },
    Vector { _handler: WDT },
    Vector { _handler: EIC_1 },
    Vector { _handler: EIC_2 },
    Vector { _handler: EIC_3 },
    Vector { _handler: EIC_4 },
    Vector { _handler: EIC_5 },
    Vector { _handler: EIC_6 },
    Vector { _handler: EIC_7 },
    Vector { _handler: EIC_8 },
    Vector { _handler: IISC },
    Vector { _handler: SPI },
    Vector { _handler: TC00 },
    Vector { _handler: TC01 },
    Vector { _handler: TC02 },
    Vector { _handler: TC10 },
    Vector { _handler: TC11 },
    Vector { _handler: TC12 },
    Vector { _handler: TWIM0 },
    Vector { _handler: TWIS0 },
    Vector { _handler: TWIM1 },
    Vector { _handler: TWIS1 },
    Vector { _handler: USART0 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector { _handler: USART3 },
    Vector { _handler: ADCIFE },
    Vector { _handler: DACC },
    Vector { _handler: ACIFC },
    Vector { _handler: ABDACB },
    Vector { _handler: TRNG },
    Vector { _handler: PARC },
    Vector { _handler: CATB },
    Vector { _reserved: 0 },
    Vector { _handler: TWIM2 },
    Vector { _handler: TWIM3 },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "0 - HFLASHC"]
    HFLASHC = 0,
    #[doc = "1 - PDCA_0"]
    PDCA_0 = 1,
    #[doc = "2 - PDCA_1"]
    PDCA_1 = 2,
    #[doc = "3 - PDCA_2"]
    PDCA_2 = 3,
    #[doc = "4 - PDCA_3"]
    PDCA_3 = 4,
    #[doc = "5 - PDCA_4"]
    PDCA_4 = 5,
    #[doc = "6 - PDCA_5"]
    PDCA_5 = 6,
    #[doc = "7 - PDCA_6"]
    PDCA_6 = 7,
    #[doc = "8 - PDCA_7"]
    PDCA_7 = 8,
    #[doc = "9 - PDCA_8"]
    PDCA_8 = 9,
    #[doc = "10 - PDCA_9"]
    PDCA_9 = 10,
    #[doc = "11 - PDCA_10"]
    PDCA_10 = 11,
    #[doc = "12 - PDCA_11"]
    PDCA_11 = 12,
    #[doc = "13 - PDCA_12"]
    PDCA_12 = 13,
    #[doc = "14 - PDCA_13"]
    PDCA_13 = 14,
    #[doc = "15 - PDCA_14"]
    PDCA_14 = 15,
    #[doc = "16 - PDCA_15"]
    PDCA_15 = 16,
    #[doc = "17 - CRCCU"]
    CRCCU = 17,
    #[doc = "18 - USBC"]
    USBC = 18,
    #[doc = "19 - PEVC_TR"]
    PEVC_TR = 19,
    #[doc = "20 - PEVC_OV"]
    PEVC_OV = 20,
    #[doc = "22 - PM"]
    PM = 22,
    #[doc = "23 - SCIF"]
    SCIF = 23,
    #[doc = "24 - FREQM"]
    FREQM = 24,
    #[doc = "25 - GPIO_0"]
    GPIO_0 = 25,
    #[doc = "26 - GPIO_1"]
    GPIO_1 = 26,
    #[doc = "27 - GPIO_2"]
    GPIO_2 = 27,
    #[doc = "28 - GPIO_3"]
    GPIO_3 = 28,
    #[doc = "29 - GPIO_4"]
    GPIO_4 = 29,
    #[doc = "30 - GPIO_5"]
    GPIO_5 = 30,
    #[doc = "31 - GPIO_6"]
    GPIO_6 = 31,
    #[doc = "32 - GPIO_7"]
    GPIO_7 = 32,
    #[doc = "33 - GPIO_8"]
    GPIO_8 = 33,
    #[doc = "34 - GPIO_9"]
    GPIO_9 = 34,
    #[doc = "35 - GPIO_10"]
    GPIO_10 = 35,
    #[doc = "36 - GPIO_11"]
    GPIO_11 = 36,
    #[doc = "37 - BPM"]
    BPM = 37,
    #[doc = "38 - BSCIF"]
    BSCIF = 38,
    #[doc = "39 - AST_ALARM"]
    AST_ALARM = 39,
    #[doc = "40 - AST_PER"]
    AST_PER = 40,
    #[doc = "41 - AST_OVF"]
    AST_OVF = 41,
    #[doc = "42 - AST_READY"]
    AST_READY = 42,
    #[doc = "43 - AST_CLKREADY"]
    AST_CLKREADY = 43,
    #[doc = "44 - WDT"]
    WDT = 44,
    #[doc = "45 - EIC_1"]
    EIC_1 = 45,
    #[doc = "46 - EIC_2"]
    EIC_2 = 46,
    #[doc = "47 - EIC_3"]
    EIC_3 = 47,
    #[doc = "48 - EIC_4"]
    EIC_4 = 48,
    #[doc = "49 - EIC_5"]
    EIC_5 = 49,
    #[doc = "50 - EIC_6"]
    EIC_6 = 50,
    #[doc = "51 - EIC_7"]
    EIC_7 = 51,
    #[doc = "52 - EIC_8"]
    EIC_8 = 52,
    #[doc = "53 - IISC"]
    IISC = 53,
    #[doc = "54 - SPI"]
    SPI = 54,
    #[doc = "55 - TC00"]
    TC00 = 55,
    #[doc = "56 - TC01"]
    TC01 = 56,
    #[doc = "57 - TC02"]
    TC02 = 57,
    #[doc = "58 - TC10"]
    TC10 = 58,
    #[doc = "59 - TC11"]
    TC11 = 59,
    #[doc = "60 - TC12"]
    TC12 = 60,
    #[doc = "61 - TWIM0"]
    TWIM0 = 61,
    #[doc = "62 - TWIS0"]
    TWIS0 = 62,
    #[doc = "63 - TWIM1"]
    TWIM1 = 63,
    #[doc = "64 - TWIS1"]
    TWIS1 = 64,
    #[doc = "65 - USART0"]
    USART0 = 65,
    #[doc = "66 - USART1"]
    USART1 = 66,
    #[doc = "67 - USART2"]
    USART2 = 67,
    #[doc = "68 - USART3"]
    USART3 = 68,
    #[doc = "69 - ADCIFE"]
    ADCIFE = 69,
    #[doc = "70 - DACC"]
    DACC = 70,
    #[doc = "71 - ACIFC"]
    ACIFC = 71,
    #[doc = "72 - ABDACB"]
    ABDACB = 72,
    #[doc = "73 - TRNG"]
    TRNG = 73,
    #[doc = "74 - PARC"]
    PARC = 74,
    #[doc = "75 - CATB"]
    CATB = 75,
    #[doc = "77 - TWIM2"]
    TWIM2 = 77,
    #[doc = "78 - TWIM3"]
    TWIM3 = 78,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Audio Bitstream DAC"]
pub struct ABDACB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ABDACB {}
impl ABDACB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const abdacb::RegisterBlock {
        0x4006_4000 as *const _
    }
}
impl Deref for ABDACB {
    type Target = abdacb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ABDACB::ptr() }
    }
}
#[doc = "Audio Bitstream DAC"]
pub mod abdacb;
#[doc = "Analog Comparator Interface"]
pub struct ACIFC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ACIFC {}
impl ACIFC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const acifc::RegisterBlock {
        0x4004_0000 as *const _
    }
}
impl Deref for ACIFC {
    type Target = acifc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ACIFC::ptr() }
    }
}
#[doc = "Analog Comparator Interface"]
pub mod acifc;
#[doc = "ADC controller interface"]
pub struct ADCIFE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADCIFE {}
impl ADCIFE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adcife::RegisterBlock {
        0x4003_8000 as *const _
    }
}
impl Deref for ADCIFE {
    type Target = adcife::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADCIFE::ptr() }
    }
}
#[doc = "ADC controller interface"]
pub mod adcife;
#[doc = "Asynchronous Timer"]
pub struct AST {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AST {}
impl AST {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ast::RegisterBlock {
        0x400f_0800 as *const _
    }
}
impl Deref for AST {
    type Target = ast::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AST::ptr() }
    }
}
#[doc = "Asynchronous Timer"]
pub mod ast;
#[doc = "Backup Power Manager"]
pub struct BPM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BPM {}
impl BPM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const bpm::RegisterBlock {
        0x400f_0000 as *const _
    }
}
impl Deref for BPM {
    type Target = bpm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*BPM::ptr() }
    }
}
#[doc = "Backup Power Manager"]
pub mod bpm;
#[doc = "Backup System Control Interface"]
pub struct BSCIF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BSCIF {}
impl BSCIF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const bscif::RegisterBlock {
        0x400f_0400 as *const _
    }
}
impl Deref for BSCIF {
    type Target = bscif::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*BSCIF::ptr() }
    }
}
#[doc = "Backup System Control Interface"]
pub mod bscif;
#[doc = "Capacitive Touch Module B"]
pub struct CATB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CATB {}
impl CATB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const catb::RegisterBlock {
        0x4007_0000 as *const _
    }
}
impl Deref for CATB {
    type Target = catb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CATB::ptr() }
    }
}
#[doc = "Capacitive Touch Module B"]
pub mod catb;
#[doc = "Chip ID Registers"]
pub struct CHIPID {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CHIPID {}
impl CHIPID {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const chipid::RegisterBlock {
        0x400e_0400 as *const _
    }
}
impl Deref for CHIPID {
    type Target = chipid::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CHIPID::ptr() }
    }
}
#[doc = "Chip ID Registers"]
pub mod chipid;
#[doc = "CRC Calculation Unit"]
pub struct CRCCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRCCU {}
impl CRCCU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crccu::RegisterBlock {
        0x400a_4000 as *const _
    }
}
impl Deref for CRCCU {
    type Target = crccu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRCCU::ptr() }
    }
}
#[doc = "CRC Calculation Unit"]
pub mod crccu;
#[doc = "DAC Controller"]
pub struct DACC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DACC {}
impl DACC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dacc::RegisterBlock {
        0x4003_c000 as *const _
    }
}
impl Deref for DACC {
    type Target = dacc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DACC::ptr() }
    }
}
#[doc = "DAC Controller"]
pub mod dacc;
#[doc = "External Interrupt Controller"]
pub struct EIC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EIC {}
impl EIC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const eic::RegisterBlock {
        0x400f_1000 as *const _
    }
}
impl Deref for EIC {
    type Target = eic::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EIC::ptr() }
    }
}
#[doc = "External Interrupt Controller"]
pub mod eic;
#[doc = "Flash Controller"]
pub struct HFLASHC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HFLASHC {}
impl HFLASHC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hflashc::RegisterBlock {
        0x400a_0000 as *const _
    }
}
impl Deref for HFLASHC {
    type Target = hflashc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HFLASHC::ptr() }
    }
}
#[doc = "Flash Controller"]
pub mod hflashc;
#[doc = "Frequency Meter"]
pub struct FREQM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FREQM {}
impl FREQM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const freqm::RegisterBlock {
        0x400e_0c00 as *const _
    }
}
impl Deref for FREQM {
    type Target = freqm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FREQM::ptr() }
    }
}
#[doc = "Frequency Meter"]
pub mod freqm;
#[doc = "Glue Logic Controller"]
pub struct GLOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GLOC {}
impl GLOC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gloc::RegisterBlock {
        0x4006_0000 as *const _
    }
}
impl Deref for GLOC {
    type Target = gloc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GLOC::ptr() }
    }
}
#[doc = "Glue Logic Controller"]
pub mod gloc;
#[doc = "General-Purpose Input/Output Controller"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio::RegisterBlock {
        0x400e_1000 as *const _
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "General-Purpose Input/Output Controller"]
pub mod gpio;
#[doc = "Cortex M I&D Cache Controller"]
pub struct HCACHE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HCACHE {}
impl HCACHE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hcache::RegisterBlock {
        0x400a_0400 as *const _
    }
}
impl Deref for HCACHE {
    type Target = hcache::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HCACHE::ptr() }
    }
}
#[doc = "Cortex M I&D Cache Controller"]
pub mod hcache;
#[doc = "HSB Matrix"]
pub struct HMATRIX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HMATRIX {}
impl HMATRIX {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hmatrix::RegisterBlock {
        0x400a_1000 as *const _
    }
}
impl Deref for HMATRIX {
    type Target = hmatrix::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HMATRIX::ptr() }
    }
}
#[doc = "HSB Matrix"]
pub mod hmatrix;
#[doc = "Inter-IC Sound (I2S) Controller"]
pub struct IISC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IISC {}
impl IISC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const iisc::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for IISC {
    type Target = iisc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*IISC::ptr() }
    }
}
#[doc = "Inter-IC Sound (I2S) Controller"]
pub mod iisc;
#[doc = "Parallel Capture"]
pub struct PARC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PARC {}
impl PARC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const parc::RegisterBlock {
        0x4006_c000 as *const _
    }
}
impl Deref for PARC {
    type Target = parc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PARC::ptr() }
    }
}
#[doc = "Parallel Capture"]
pub mod parc;
#[doc = "Peripheral DMA Controller"]
pub struct PDCA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDCA {}
impl PDCA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pdca::RegisterBlock {
        0x400a_2000 as *const _
    }
}
impl Deref for PDCA {
    type Target = pdca::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PDCA::ptr() }
    }
}
#[doc = "Peripheral DMA Controller"]
pub mod pdca;
#[doc = "Peripheral Event Controller"]
pub struct PEVC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PEVC {}
impl PEVC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pevc::RegisterBlock {
        0x400a_6000 as *const _
    }
}
impl Deref for PEVC {
    type Target = pevc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PEVC::ptr() }
    }
}
#[doc = "Peripheral Event Controller"]
pub mod pevc;
#[doc = "Pico UART"]
pub struct PICOUART {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PICOUART {}
impl PICOUART {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const picouart::RegisterBlock {
        0x400f_1400 as *const _
    }
}
impl Deref for PICOUART {
    type Target = picouart::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PICOUART::ptr() }
    }
}
#[doc = "Pico UART"]
pub mod picouart;
#[doc = "Power Manager"]
pub struct PM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PM {}
impl PM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pm::RegisterBlock {
        0x400e_0000 as *const _
    }
}
impl Deref for PM {
    type Target = pm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PM::ptr() }
    }
}
#[doc = "Power Manager"]
pub mod pm;
#[doc = "System Control Interface"]
pub struct SCIF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCIF {}
impl SCIF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scif::RegisterBlock {
        0x400e_0800 as *const _
    }
}
impl Deref for SCIF {
    type Target = scif::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCIF::ptr() }
    }
}
#[doc = "System Control Interface"]
pub mod scif;
#[doc = "System Manager Access Port"]
pub struct SMAP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMAP {}
impl SMAP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const smap::RegisterBlock {
        0x400a_3000 as *const _
    }
}
impl Deref for SMAP {
    type Target = smap::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SMAP::ptr() }
    }
}
#[doc = "System Manager Access Port"]
pub mod smap;
#[doc = "Serial Peripheral Interface"]
pub struct SPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI {}
impl SPI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for SPI {
    type Target = spi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI::ptr() }
    }
}
#[doc = "Serial Peripheral Interface"]
pub mod spi;
#[doc = "Timer/Counter 0"]
pub struct TC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC0 {}
impl TC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for TC0 {
    type Target = tc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC0::ptr() }
    }
}
#[doc = "Timer/Counter 0"]
pub mod tc0;
#[doc = "Timer/Counter 1"]
pub struct TC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC1 {}
impl TC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4001_4000 as *const _
    }
}
impl Deref for TC1 {
    type Target = tc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC1::ptr() }
    }
}
#[doc = "True Random Number Generator"]
pub struct TRNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TRNG {}
impl TRNG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const trng::RegisterBlock {
        0x4006_8000 as *const _
    }
}
impl Deref for TRNG {
    type Target = trng::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TRNG::ptr() }
    }
}
#[doc = "True Random Number Generator"]
pub mod trng;
#[doc = "Two-wire Master Interface 0"]
pub struct TWIM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIM0 {}
impl TWIM0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twim0::RegisterBlock {
        0x4001_8000 as *const _
    }
}
impl Deref for TWIM0 {
    type Target = twim0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWIM0::ptr() }
    }
}
#[doc = "Two-wire Master Interface 0"]
pub mod twim0;
#[doc = "Two-wire Master Interface 1"]
pub struct TWIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIM1 {}
impl TWIM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twim0::RegisterBlock {
        0x4001_c000 as *const _
    }
}
impl Deref for TWIM1 {
    type Target = twim0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWIM1::ptr() }
    }
}
#[doc = "Two-wire Master Interface 2"]
pub struct TWIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIM2 {}
impl TWIM2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twim0::RegisterBlock {
        0x4007_8000 as *const _
    }
}
impl Deref for TWIM2 {
    type Target = twim0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWIM2::ptr() }
    }
}
#[doc = "Two-wire Master Interface 3"]
pub struct TWIM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIM3 {}
impl TWIM3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twim0::RegisterBlock {
        0x4007_c000 as *const _
    }
}
impl Deref for TWIM3 {
    type Target = twim0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWIM3::ptr() }
    }
}
#[doc = "Two-wire Slave Interface 0"]
pub struct TWIS0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIS0 {}
impl TWIS0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twis0::RegisterBlock {
        0x4001_8400 as *const _
    }
}
impl Deref for TWIS0 {
    type Target = twis0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWIS0::ptr() }
    }
}
#[doc = "Two-wire Slave Interface 0"]
pub mod twis0;
#[doc = "Two-wire Slave Interface 1"]
pub struct TWIS1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIS1 {}
impl TWIS1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twis0::RegisterBlock {
        0x4001_c400 as *const _
    }
}
impl Deref for TWIS1 {
    type Target = twis0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWIS1::ptr() }
    }
}
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 0"]
pub struct USART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART0 {}
impl USART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4002_4000 as *const _
    }
}
impl Deref for USART0 {
    type Target = usart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART0::ptr() }
    }
}
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 0"]
pub mod usart0;
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 1"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4002_8000 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 2"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4002_c000 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART2::ptr() }
    }
}
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 3"]
pub struct USART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART3 {}
impl USART3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4003_0000 as *const _
    }
}
impl Deref for USART3 {
    type Target = usart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART3::ptr() }
    }
}
#[doc = "USB 2.0 Interface"]
pub struct USBC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBC {}
impl USBC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbc::RegisterBlock {
        0x400a_5000 as *const _
    }
}
impl Deref for USBC {
    type Target = usbc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USBC::ptr() }
    }
}
#[doc = "USB 2.0 Interface"]
pub mod usbc;
#[doc = "Watchdog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt::RegisterBlock {
        0x400f_0c00 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod wdt;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "ABDACB"]
    pub ABDACB: ABDACB,
    #[doc = "ACIFC"]
    pub ACIFC: ACIFC,
    #[doc = "ADCIFE"]
    pub ADCIFE: ADCIFE,
    #[doc = "AST"]
    pub AST: AST,
    #[doc = "BPM"]
    pub BPM: BPM,
    #[doc = "BSCIF"]
    pub BSCIF: BSCIF,
    #[doc = "CATB"]
    pub CATB: CATB,
    #[doc = "CHIPID"]
    pub CHIPID: CHIPID,
    #[doc = "CRCCU"]
    pub CRCCU: CRCCU,
    #[doc = "DACC"]
    pub DACC: DACC,
    #[doc = "EIC"]
    pub EIC: EIC,
    #[doc = "HFLASHC"]
    pub HFLASHC: HFLASHC,
    #[doc = "FREQM"]
    pub FREQM: FREQM,
    #[doc = "GLOC"]
    pub GLOC: GLOC,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "HCACHE"]
    pub HCACHE: HCACHE,
    #[doc = "HMATRIX"]
    pub HMATRIX: HMATRIX,
    #[doc = "IISC"]
    pub IISC: IISC,
    #[doc = "PARC"]
    pub PARC: PARC,
    #[doc = "PDCA"]
    pub PDCA: PDCA,
    #[doc = "PEVC"]
    pub PEVC: PEVC,
    #[doc = "PICOUART"]
    pub PICOUART: PICOUART,
    #[doc = "PM"]
    pub PM: PM,
    #[doc = "SCIF"]
    pub SCIF: SCIF,
    #[doc = "SMAP"]
    pub SMAP: SMAP,
    #[doc = "SPI"]
    pub SPI: SPI,
    #[doc = "TC0"]
    pub TC0: TC0,
    #[doc = "TC1"]
    pub TC1: TC1,
    #[doc = "TRNG"]
    pub TRNG: TRNG,
    #[doc = "TWIM0"]
    pub TWIM0: TWIM0,
    #[doc = "TWIM1"]
    pub TWIM1: TWIM1,
    #[doc = "TWIM2"]
    pub TWIM2: TWIM2,
    #[doc = "TWIM3"]
    pub TWIM3: TWIM3,
    #[doc = "TWIS0"]
    pub TWIS0: TWIS0,
    #[doc = "TWIS1"]
    pub TWIS1: TWIS1,
    #[doc = "USART0"]
    pub USART0: USART0,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "USART3"]
    pub USART3: USART3,
    #[doc = "USBC"]
    pub USBC: USBC,
    #[doc = "WDT"]
    pub WDT: WDT,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            ABDACB: ABDACB {
                _marker: PhantomData,
            },
            ACIFC: ACIFC {
                _marker: PhantomData,
            },
            ADCIFE: ADCIFE {
                _marker: PhantomData,
            },
            AST: AST {
                _marker: PhantomData,
            },
            BPM: BPM {
                _marker: PhantomData,
            },
            BSCIF: BSCIF {
                _marker: PhantomData,
            },
            CATB: CATB {
                _marker: PhantomData,
            },
            CHIPID: CHIPID {
                _marker: PhantomData,
            },
            CRCCU: CRCCU {
                _marker: PhantomData,
            },
            DACC: DACC {
                _marker: PhantomData,
            },
            EIC: EIC {
                _marker: PhantomData,
            },
            HFLASHC: HFLASHC {
                _marker: PhantomData,
            },
            FREQM: FREQM {
                _marker: PhantomData,
            },
            GLOC: GLOC {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            HCACHE: HCACHE {
                _marker: PhantomData,
            },
            HMATRIX: HMATRIX {
                _marker: PhantomData,
            },
            IISC: IISC {
                _marker: PhantomData,
            },
            PARC: PARC {
                _marker: PhantomData,
            },
            PDCA: PDCA {
                _marker: PhantomData,
            },
            PEVC: PEVC {
                _marker: PhantomData,
            },
            PICOUART: PICOUART {
                _marker: PhantomData,
            },
            PM: PM {
                _marker: PhantomData,
            },
            SCIF: SCIF {
                _marker: PhantomData,
            },
            SMAP: SMAP {
                _marker: PhantomData,
            },
            SPI: SPI {
                _marker: PhantomData,
            },
            TC0: TC0 {
                _marker: PhantomData,
            },
            TC1: TC1 {
                _marker: PhantomData,
            },
            TRNG: TRNG {
                _marker: PhantomData,
            },
            TWIM0: TWIM0 {
                _marker: PhantomData,
            },
            TWIM1: TWIM1 {
                _marker: PhantomData,
            },
            TWIM2: TWIM2 {
                _marker: PhantomData,
            },
            TWIM3: TWIM3 {
                _marker: PhantomData,
            },
            TWIS0: TWIS0 {
                _marker: PhantomData,
            },
            TWIS1: TWIS1 {
                _marker: PhantomData,
            },
            USART0: USART0 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            USART3: USART3 {
                _marker: PhantomData,
            },
            USBC: USBC {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
        }
    }
}
