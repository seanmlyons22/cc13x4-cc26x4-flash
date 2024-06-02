#![no_std]
#![no_main]

use cc13x4_cc26x4_pac as cc13x4_cc26x4;
use flash_algorithm::*;
use rtt_target::{rprintln, rtt_init_print};

extern "C" {
    fn NOROM_FlashSectorErase(ui32SectorAddress: u32) -> u32;
    fn NOROM_FlashProgram(pui8DataBuffer: *const u8, ui32Address: u32, ui32Count: u32) -> u32;
    fn NOROM_VIMSModeSafeSet(ui32Base: u32, ui32NewMode: u32, blocking: bool);
}

struct Algorithm;

algorithm!(Algorithm, {
    flash_address: 0x0,
    flash_size: 1048576,
    page_size: 2048,
    empty_value: 0xFF,
    sectors: [{
        size: 2048,
        address: 0x0,
    }]
});

const FAPI_STATUS_SUCCESS: u32 = 0x00000000; // Function completed successfully
const VIMS_BASE: u32 = 0x40034000; // VIMS
const VIMS_MODE_DISABLED: u32 = 0x00000000; // VIMS disabled
const VIMS_MODE_ENABLED: u32 = 0x00000001; // VIMS enabled

impl FlashAlgorithm for Algorithm {
    fn new(_address: u32, _clock: u32, _function: Function) -> Result<Self, ErrorCode> {
        rtt_init_print!();
        let p: Option<_> = cc13x4_cc26x4::Peripherals::take();

        match p {
            Some(p) => {
                rprintln!("Init: Initializing peripherals");
                // Setup PRCM, power the peripheral and serial domains
                p.prcm.pdctl0periph().write(|w| w.on().set_bit());
                p.prcm.pdctl0serial().write(|w| w.on().set_bit());
                p.prcm.gpioclkgr().write(|w| w.clk_en().set_bit());
                p.prcm.clkloadctl().write(|w| w.load().set_bit());
            }
            None => {
                rprintln!("Init: Peripherals already initialized, do nothing");
            }
        }

        // Disable interrupts so we can safely program the flash
        cortex_m::interrupt::disable();
        // We need to disable the flash while doing flash operations
        unsafe { NOROM_VIMSModeSafeSet(VIMS_BASE, VIMS_MODE_DISABLED, true) };
        Ok(Self)
    }

    fn erase_all(&mut self) -> Result<(), ErrorCode> {
        rprintln!("Erase All");
        let flash_size = FlashDevice.device_size;
        let page_size = FlashDevice.page_size;
        let num_pages = flash_size / page_size;

        // Do the erase
        for page in 0..num_pages {
            let addr = page * page_size;
            self.erase_sector(addr)?;
        }
        Ok(())
    }

    fn erase_sector(&mut self, addr: u32) -> Result<(), ErrorCode> {
        rprintln!("Erase sector addr:{}", addr);

        let status: u32 = unsafe { NOROM_FlashSectorErase(addr) };

        match status {
            FAPI_STATUS_SUCCESS => Ok(()),
            _ => Err(ErrorCode::new(status).unwrap()),
        }
    }

    fn program_page(&mut self, addr: u32, data: &[u8]) -> Result<(), ErrorCode> {
        rprintln!("Program Page addr:{} size:{}", addr, data.len());

        let status: u32 =
            unsafe { NOROM_FlashProgram(data.as_ptr(), addr, data.len().try_into().unwrap()) };

        match status {
            FAPI_STATUS_SUCCESS => Ok(()),
            _ => Err(ErrorCode::new(status).unwrap()),
        }
    }
}

impl Drop for Algorithm {
    fn drop(&mut self) {
        rprintln!("Deinit");
        // Renable the cache
        unsafe {
            NOROM_VIMSModeSafeSet(VIMS_BASE, VIMS_MODE_ENABLED, true);
            // Renable interrupts
            cortex_m::interrupt::enable();
        };
    }
}
