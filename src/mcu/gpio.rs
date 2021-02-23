//==============================================================================
// Notes
//==============================================================================
// mcu::gpio.rs
// Basic control over gpio pins

//==============================================================================
// Crates and Mods
//==============================================================================
use nrf52832_pac;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum PinDirection{
	Input,
	Output
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum PinPull{
	PullUp,
	PullDown,
	PullDisabled
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum PinState{
	PinLow,
	PinHigh
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
#[allow(dead_code)]
pub fn get_pin_state(p: &nrf52832_pac::Peripherals, pin: u8) -> PinState {
	let p0 = &p.P0;
	match p0.in_.read().bits() & (1 << pin) {
		0 => PinState::PinLow,
		_ => PinState::PinHigh
	}
}

#[allow(dead_code)]
pub fn pin_setup(p: &nrf52832_pac::Peripherals, pin: u8, dir: PinDirection, pull: PinPull, state: PinState){
	let p0 = &p.P0;
	
	// Set direction bit
	match dir{
		PinDirection::Input => {
			p0.pin_cnf[pin as usize].modify(|_, w| w.dir().input());
			p0.pin_cnf[pin as usize].modify(|_, w| w.input().connect());
		},
		PinDirection::Output => {
			p0.pin_cnf[pin as usize].modify(|_, w| w.dir().output());
			
			match state {
				PinState::PinLow => p0.outclr.write(|w| unsafe {w.bits(1 << pin)}),
				PinState::PinHigh => p0.outset.write(|w| unsafe {w.bits(1 << pin)})
			}
		}
	};
	
	// Set pin pull
	match pull{
		PinPull::PullUp => p0.pin_cnf[pin as usize].modify(|_, w| w.pull().pullup()),
		PinPull::PullDown => p0.pin_cnf[pin as usize].modify(|_, w| w.pull().pulldown()),
		PinPull::PullDisabled => p0.pin_cnf[pin as usize].modify(|_, w| w.pull().disabled()),
	};
}

#[allow(dead_code)]
pub fn set_pin_state(p: &nrf52832_pac::Peripherals, pin: u8, state: PinState){
	let p0 = &p.P0;
	match state {
		PinState::PinLow => p0.outclr.write(|w| unsafe {w.bits(1 << pin)}),
		PinState::PinHigh => p0.outset.write(|w| unsafe {w.bits(1 << pin)})
	}
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
