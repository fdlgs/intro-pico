#![allow(unused)]

use rp_pico as bsp;

use bsp::hal::{
    // alias for the Hardware Abstraction Layer, which provides higher-level drivers.
    self,
    // Pull in any important traits
    prelude::*,
    // alias for the Peripheral Access Crate, which provides low-level register access
    pac,
};



#[allow(non_snake_case)]
pub(crate) struct PiPico {
    XOSC: Option<pac::XOSC>,
    CLOCKS: Option<pac::CLOCKS>,
    PLL_SYS: Option<pac::PLL_SYS>,
    PLL_USB: Option<pac::PLL_USB>,
    TIMER: Option<pac::TIMER>,
    IO_BANK0: Option<pac::IO_BANK0>,
    PADS_BANK0: Option<pac::PADS_BANK0>,
    SYST: Option<pac::SYST>,
    
    RESETS: pac::RESETS,

    watchdog: hal::Watchdog,
    clocks: Option<hal::clocks::ClocksManager>,
    sio: Option<hal::sio::Sio>,

}

impl PiPico {
    pub fn take() -> Option<Self> {
        Some(Self::new(
            pac::Peripherals::take()?,
            pac::CorePeripherals::take()?,
        ))
    }
    pub fn new(p: pac::Peripherals, cp: pac::CorePeripherals) -> Self {

        Self {
            XOSC: Some(p.XOSC),
            CLOCKS: Some(p.CLOCKS),
            PLL_SYS: Some(p.PLL_SYS),
            PLL_USB: Some(p.PLL_USB),
            TIMER: Some(p.TIMER),
            IO_BANK0: Some(p.IO_BANK0),
            PADS_BANK0: Some(p.PADS_BANK0),
            SYST: Some(cp.SYST),
            RESETS: p.RESETS,
            watchdog: hal::Watchdog::new(p.WATCHDOG),
            clocks:None,
            // The single-cycle I/O block controls our GPIO pins
            sio: Some(hal::Sio::new(p.SIO)),

        }
    } 
    pub fn init_pins(&mut self) -> Option<rp_pico::Pins> {
        Some(bsp::Pins::new(
            self.IO_BANK0.take()?,
            self.PADS_BANK0.take()?,
            self.sio.take()?.gpio_bank0,
            &mut self.RESETS,
        ))

    }

    pub fn init_timer(&mut self) -> Option<hal::Timer> {
    // pub fn init_timer(&mut self) -> Option<cortex_m::delay::Delay> {
        self.clocks = hal::clocks::init_clocks_and_plls(
            bsp::XOSC_CRYSTAL_FREQ,
            self.XOSC.take()?,
            self.CLOCKS.take()?,
            self.PLL_SYS.take()?,
            self.PLL_USB.take()?,
            &mut self.RESETS,
            &mut self.watchdog,
        )
        .ok();
        match self.clocks {
            Some(ref ref_clock) => Some(
                hal::Timer::new(self.TIMER.take()?, &mut self.RESETS, ref_clock)),
            // Some(ref clocks) => Some(
            //     cortex_m::delay::Delay::new(self.SYST.take()?, clocks.system_clock.freq().to_Hz())),
            None => None
        }
    }
}

