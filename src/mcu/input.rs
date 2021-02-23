//==============================================================================
// Notes
//==============================================================================
// mcu::input.rs
// Watcher and handler for a GPIO pin defined as an input

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::mcu::gpio;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[derive(Copy, Clone)]
pub struct Input {
	pub pin: u8,
	pub state: gpio::PinState,
	pub pull: gpio::PinPull,
	pub callback: Callback,
}

type Callback = fn(Input);

//==============================================================================
// Macros
//==============================================================================
// #[macro_export]
// macro_rules! input_create_new {
// 	(name: ident, pin: item, pull: item, callback: item) => {
// 		static mut ident: Input = Input{ 
// 			pin: pin,
// 			state: gpio::PinStateL::PinLow,
// 			pull: pull,
// 			callback: callback
// 		}
// 	};
// }! Input

//==============================================================================
// Variables
//==============================================================================


//==============================================================================
// Implementations
//==============================================================================
#[allow(dead_code)]
pub fn init(p: &nrf52832_pac::Peripherals, input: &Input) {
	gpio::pin_setup(p, input.pin, gpio::PinDirection::Input, input.pull, input.state);
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
