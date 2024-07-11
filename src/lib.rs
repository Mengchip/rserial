use std::io::{Read, Write};
use std::marker::Send;
use std::time::Duration;

use windows::COMPort;

mod windows;

#[derive(Debug)]
pub enum ErrorKind {
    NotFound,
    TimeOut,
    Unknown,
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    desc: String,
}

impl Error {
    fn new(kind: ErrorKind, desc: impl Into<String>) -> Self {
        Self {
            kind,
            desc: desc.into(),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Port: Send + Read + Write {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DataBits {
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StopBits {
    One = 0,
    // OnePointFive = 1,
    Two = 2,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Parity {
    None,
    Odd,
    Even,
    Mark,
    Space,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FlowCtrl {
    None,
    XOnXOff,
    RtsCts,
    DtrDsr,
    RtsCtsXOnXOff,
    DtrDsrXOnXOff,
}

#[derive(Debug, Clone)]
pub struct Serial {
    path: String,
    baud_rate: u32,
    data_bit: DataBits,
    stop_bit: StopBits,
    parity: Parity,
    flow_ctrl: FlowCtrl,
    timeout: Duration,
}

impl Serial {
    pub fn new(path: impl Into<String>, baud_rate: u32) -> Self {
        Serial {
            path: path.into(),
            baud_rate,
            data_bit: DataBits::Eight,
            stop_bit: StopBits::One,
            parity: Parity::None,
            flow_ctrl: FlowCtrl::None,
            timeout: Duration::from_millis(500),
        }
    }

    pub fn open(&self) -> Result<COMPort> {
        COMPort::open(self)
    }
}
