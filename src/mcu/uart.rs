//==============================================================================
// Notes
//==============================================================================
// mcu::uart.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use nrf52832_pac::{interrupt, uart0};

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
pub struct UartLine {
	pub cts_pin: Option<u8>,
	pub rts_pin: Option<u8>,
	pub rx_pin: u8,
	pub tx_pin: u8,
	pub baud: uart0::baudrate::BAUDRATE_A,
	pub parity: uart0::config::PARITY_A,
	pub echo_enabled: bool,
}

enum UartEvent{
	CTS,
	NCTS,
	RXDRDY,
	TXDRDY,
	ERROR,
	RXTO
}

//==============================================================================
// Macros
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
static _UART_EVENT: Option<UartEvent> = None;

//==============================================================================
// Implementations
//==============================================================================
pub fn init(p: &nrf52832_pac::Peripherals, uartline: &UartLine) {
	let uart = &p.UART0;

	nrf52832_pac::NVIC::mask(nrf52832_pac::Interrupt::UARTE0_UART0);

	uart.enable.write(|w| w.enable().disabled());

	if (uartline.cts_pin == None) || (uartline.rts_pin == None) {
		uart.config.write(|w| w.hwfc().disabled());
	}
	else {
		uart.pselcts.write(|w| unsafe { w.bits(uartline.cts_pin.unwrap() as u32) });
		uart.pselrts.write(|w| unsafe { w.bits(uartline.rts_pin.unwrap() as u32) });
		uart.config.write(|w| w.hwfc().enabled());
	}

	uart.pselrxd.write(|w| unsafe { w.bits(uartline.rx_pin as u32) });
	uart.pseltxd.write(|w| unsafe { w.bits(uartline.tx_pin as u32) });

	uart.config.modify(|_, w| w.parity().variant(uartline.parity));

	uart.baudrate.write(|w| w.baudrate().variant(uartline.baud ));

	uart.intenset.write(|w| w.rxdrdy().set());
	uart.events_rxdrdy.write(|w| unsafe { w.bits(1) });

	uart.enable.write(|w| w.enable().enabled());

	unsafe { nrf52832_pac::NVIC::unmask(nrf52832_pac::Interrupt::UARTE0_UART0); }
}

//==============================================================================
// Interrupt Handler
//==============================================================================
#[interrupt]
 fn UARTE0_UART0() {
	if 
}

//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler() {
	if let None = _UART_EVENT {
		return;
	}


}