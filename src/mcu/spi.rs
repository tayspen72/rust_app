//==============================================================================
// Notes
//==============================================================================
// mcu::spi.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use nrf52832_pac::spi0;
use crate::mcu::gpio;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
pub struct SpiLine{
	pub sclk_pin: u8,
	pub sel_pin: u8,
	pub mosi_pin: u8,
	pub miso_pin: u8,
	pub frequency: spi0::frequency::FREQUENCY_A,
	pub order: spi0::config::ORDER_A,
	pub cpha: spi0::config::CPHA_A,
	pub cpol: spi0::config::CPOL_A
}

//==============================================================================
// Macros
//==============================================================================


//==============================================================================
// Variables
//==============================================================================


//==============================================================================
// Implementations
//==============================================================================
pub fn init(p: &nrf52832_pac::Peripherals, spiline: &SpiLine) {
	let spi = &p.SPI0;

	spi.enable.write(|w| w.enable().disabled());

	// Configure MOSI pin
	gpio::pin_setup(p, spiline.mosi_pin, gpio::PinDirection::Output, gpio::PinPull::PullDisabled, gpio::PinState::PinLow);
	spi.psel.mosi.write(|w| unsafe { w.bits(spiline.mosi_pin as u32) });

	// Configure MISO pin
	gpio::pin_setup(p, spiline.mosi_pin, gpio::PinDirection::Output, gpio::PinPull::PullDisabled, gpio::PinState::PinLow);
	spi.psel.miso.write(|w| unsafe { w.bits(spiline.miso_pin as u32) });

	// Configure SCLK pin
	spi.psel.sck.write(|w| unsafe { w.bits(spiline.sclk_pin as u32) });

	// Configure SEL pin
	gpio::pin_setup(p, spiline.sel_pin, gpio::PinDirection::Output, gpio::PinPull::PullDisabled, gpio::PinState::PinHigh);

	spi.frequency.write(|w| w.frequency().variant(spiline.frequency));
	spi.config.write(|w| w
		.order().variant(spiline.order)
		.cpha().variant(spiline.cpha)
		.cpol().variant(spiline.cpol)
	);
}

pub fn txrx_byte(p: &nrf52832_pac::Peripherals, byte: u8)->  Option<u8> {
	p.SPI0.txd.write(|w| unsafe { w.txd().bits(byte) });

	while p.SPI0.events_ready.read().bits() != 0 {};

	Some((p.SPI0.rxd.read().bits() & 0xFF) as u8)
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
