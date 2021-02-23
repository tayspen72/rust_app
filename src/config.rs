//==============================================================================
// Notes
//==============================================================================
// config.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use nrf52832_pac::{twi0, uart0};

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Macros
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
pub const BTN_PIN: [u8; 4] = [13, 14, 15, 16];
pub const LED_PIN: [u8; 4] = [17, 18, 19, 20];

pub const I2C_SCL_PIN: u8 = 31;
pub const I2C_SDA_PIN: u8 = 30;
pub const I2C_FREQUENCY: twi0::frequency::FREQUENCY_A = twi0::frequency::FREQUENCY_A::K400;
pub const I2C_ADDRESS: u8 = 0x70;

pub const UART_CTS_PIN: Option<u8> = None;
pub const UART_RTS_PIN: Option<u8> = None;
pub const UART_RX_PIN: u8 = 1;
pub const UART_TX_PIN: u8 = 2;
pub const UART_BAUD: uart0::baudrate::BAUDRATE_A = uart0::baudrate::BAUDRATE_A::BAUD115200;
pub const UART_PARITY: uart0::config::PARITY_A = uart0::config::PARITY_A::EXCLUDED;
pub const UART_ECHO: bool = true;

//==============================================================================
// Implementations
//==============================================================================


//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
