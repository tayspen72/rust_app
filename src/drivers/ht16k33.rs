//==============================================================================
// Notes
//==============================================================================
// drivers::ht16k33.rs
// 4 Digit 14-Segment Adafruit Backpack Driver

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::config;
use crate::mcu::i2c;
use crate::drivers::ascii;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
enum BlinkFrequency{
	Off,
	Hz2,
	Hz1,
	Hz05
}

#[allow(dead_code)]
enum DimmingLevel{
	Level1,
	Level2,
	Level3,
	Level4,
	Level5,
	Level6,
	Level7,
	Level8,
	Level9,
	Level10,
	Level11,
	Level12,
	Level13,
	Level14,
	Level15,
	Level16	
}

#[allow(dead_code)]
enum DisplayStatus{
	DisplayOn,
	DisplayOff
}

#[allow(dead_code)]
enum SystemMode{
	OscillatorStandby,
	OscillatorNormal
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
pub fn init(p: &nrf52832_pac::Peripherals) {
	let line = get_i2cline();
	
	i2c::init(p, &line);
	
	begin(p, &line);
	
	write_display(p, get_i2cline(), ['C', 'L', 'A', 'Y']);	
}

fn get_i2cline() -> &'static i2c::I2cLine {
	static I2CLINE: i2c::I2cLine = i2c:: I2cLine {
		scl_pin: config::I2C_SCL_PIN,
		sda_pin: config::I2C_SDA_PIN,
		frequency: config::I2C_FREQUENCY,
		address: config::I2C_ADDRESS,
	};
	
	&I2CLINE
}

fn begin(p: &nrf52832_pac::Peripherals, i2cline: &i2c::I2cLine) -> Option<bool> {
	
	if set_system_setup(p, i2cline, SystemMode::OscillatorNormal, true, true) == None { return None; }
	if set_int_set(p, i2cline, false, false, true, true) == None { return None; }
	if set_dimming(p, i2cline, DimmingLevel::Level8, true, true) == None { return None; }
	if write_display(p, i2cline, [ ' ', ' ', ' ', ' ']) == None { return None; }
	if set_display_setup(p, i2cline, DisplayStatus::DisplayOn, BlinkFrequency::Off, true, true) == None { return None; }
	
	Some(true)
}

fn set_dimming(p: &nrf52832_pac::Peripherals, i2cline: &i2c::I2cLine, level: DimmingLevel, send_start: bool, send_stop: bool) -> Option<bool>{
	let level = 0xE0 | match level{
		DimmingLevel::Level1 => 0x00,
		DimmingLevel::Level2 => 0x01,
		DimmingLevel::Level3 => 0x02,
		DimmingLevel::Level4 => 0x03,
		DimmingLevel::Level5 => 0x04,
		DimmingLevel::Level6 => 0x05,
		DimmingLevel::Level7 => 0x06,
		DimmingLevel::Level8 => 0x07,
		DimmingLevel::Level9 => 0x08,
		DimmingLevel::Level10 => 0x09,
		DimmingLevel::Level11 => 0x0A,
		DimmingLevel::Level12 => 0x0B,
		DimmingLevel::Level13 => 0x0C,
		DimmingLevel::Level14 => 0x0D,
		DimmingLevel::Level15 => 0x0E,
		DimmingLevel::Level16 => 0x0F,
	};

	i2c::write_byte(p, i2cline, level, send_start, send_stop)
}

fn set_display_address(p: &nrf52832_pac::Peripherals, i2cline: &i2c::I2cLine, address: u8, send_start: bool, send_stop: bool) -> Option<bool> {
	i2c::write_byte(p, i2cline, address & 0x0F, send_start, send_stop)
}

fn set_display_setup(p: &nrf52832_pac::Peripherals, i2cline: &i2c::I2cLine, status: DisplayStatus, blink: BlinkFrequency, send_start: bool, send_stop: bool) -> Option<bool> {
	let mut setup = match blink{
		BlinkFrequency::Off => 0x80,
		BlinkFrequency::Hz2 => 0x82,
		BlinkFrequency::Hz1 => 0x84,
		BlinkFrequency::Hz05 => 0x86		
	};
	
	setup |= match status {
		DisplayStatus::DisplayOff => 0,
		DisplayStatus::DisplayOn => 0x01,
	};
	
	i2c::write_byte(p, i2cline, setup, send_start, send_stop)
}

#[allow(dead_code)]
fn set_int_address(p: &nrf52832_pac::Peripherals, i2cline: &i2c::I2cLine, send_start: bool, send_stop: bool) -> Option<bool> {
	i2c::write_byte(p, i2cline, 0x60, send_start, send_stop)
}

fn set_int_set(p: &nrf52832_pac::Peripherals, i2cline: &i2c::I2cLine, polarity: bool, enable: bool, send_start: bool, send_stop: bool) -> Option<bool> {
	let mut set = match polarity {
		false => 0xC0,
		true => 0xC2
	};
	
	set |= match enable{
		false => 0x0, 
		true => 0x1
	};
	
	i2c::write_byte(p, i2cline, set, send_start, send_stop)
}

#[allow(dead_code)]
fn set_key_address(p: &nrf52832_pac::Peripherals, i2cline: &i2c::I2cLine, address: u8, send_start: bool, send_stop: bool) -> Option<bool> {
	i2c::write_byte(p, i2cline, 0x40 | (address & 0x07), send_start, send_stop)
}

fn set_system_setup(p: &nrf52832_pac::Peripherals, i2cline: &i2c::I2cLine, mode: SystemMode, send_start: bool, send_stop: bool) -> Option<bool> {
	let mode: u8 = match mode {
		SystemMode::OscillatorNormal => 0x21,
		SystemMode::OscillatorStandby => 0x20,
	};
	
	i2c::write_byte(p, i2cline, mode, send_start, send_stop)
}

#[allow(dead_code)]
fn write_digit(p: &nrf52832_pac::Peripherals, i2cline: &i2c::I2cLine, c: &char) -> Option<bool> {
	let val = ascii::from_ascii(c);
	
	if i2c::write_byte(p, i2cline, (val & 0x00FF) as u8, true, false) == None {
		return None;
	}
	
	i2c::write_byte(p, i2cline, ((val & 0xFF00) >> 8) as u8, false, true)
}

#[allow(dead_code)]
fn write_display(p: &nrf52832_pac::Peripherals, i2cline: &i2c::I2cLine, vals: [char; 4]) -> Option<bool> {
	let buf: [u16; 4] = [
		ascii::from_ascii(&vals[0]),
		ascii::from_ascii(&vals[1]),
		ascii::from_ascii(&vals[2]),
		ascii::from_ascii(&vals[3]),
	];
	
	let data: [u8; 8] = [
		(buf[0] & 0x00FF) as u8, ((buf[0] & 0xFF00) >> 8) as u8,
		(buf[1] & 0x00FF) as u8, ((buf[1] & 0xFF00) >> 8) as u8,
		(buf[2] & 0x00FF) as u8, ((buf[2] & 0xFF00) >> 8) as u8,
		(buf[3] & 0x00FF) as u8, ((buf[3] & 0xFF00) >> 8) as u8,
	];
	
	set_display_address(p, i2cline, 0x00, true, false);
	i2c::write_data(p, i2cline, &data, false, true)
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================

