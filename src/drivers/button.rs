//==============================================================================
// Notes
//==============================================================================
// drivers::button.rs
// Wrapper around the gpio pins for handling button presses

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::config;
use crate::drivers::led;
use crate::mcu::{ gpio };

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Macros
//==============================================================================


//==============================================================================
// Variables
//==============================================================================


//==============================================================================
// Implementations
//==============================================================================
pub fn init(p: &nrf52832_pac::Peripherals) {
	for i in 0..config::BTN_PIN.len() {
		gpio::pin_setup(p, config::BTN_PIN[i], gpio::PinDirection::Input, gpio::PinPull::PullUp, gpio::PinState::PinHigh);
	}
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler(p: &nrf52832_pac::Peripherals) {
	static mut LAST_BTN_STATE: [gpio::PinState; config::BTN_PIN.len()] = [gpio::PinState::PinHigh; 4];
	
	for i in 0..config::BTN_PIN.len() { unsafe {
		if gpio::get_pin_state(p, config::BTN_PIN[i]) != LAST_BTN_STATE[i] {
			LAST_BTN_STATE[i] = gpio::get_pin_state(p, config::BTN_PIN[i]);
			led::set_led(p, i as u8, LAST_BTN_STATE[i]);
		}
	}}
}
