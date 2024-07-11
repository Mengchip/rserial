use std::mem::MaybeUninit;
use std::time::Duration;

use crate::{DataBits, FlowCtrl, Parity, StopBits};
use crate::Result;
use crate::windows::ffi::*;

/// # Device control Block

/// # Get DCB
pub fn get_dcb(handle: HANDLE) -> Result<DCB> {
    unsafe {
        let mut dcb: DCB = MaybeUninit::zeroed().assume_init();
        let _ = GetCommState(handle, &mut dcb);
        Ok(dcb)
    }
}

/// # Set DCB
pub fn set_dcb(handle: HANDLE, mut dcb: DCB) -> Result<()> {
    unsafe {
        SetCommState(handle, &mut dcb as *mut _);
    }
    Ok(())
}

/// # Set baud rate
pub fn set_baud_rate(dcb: &mut DCB, baud_rate: u32) {
    dcb.BaudRate = baud_rate;
}

/// # Set data bits
pub fn set_data_bits(dcb: &mut DCB, data_bits: DataBits) {
    dcb.ByteSize = match data_bits {
        DataBits::Five => 5,
        DataBits::Six => 6,
        DataBits::Seven => 7,
        DataBits::Eight => 8
    }
}

/// # Set stop bits
pub fn set_stop_bits(dcb: &mut DCB, stop_bits: StopBits) {
    dcb.StopBits = match stop_bits {
        StopBits::One => ONESTOPBITS,
        StopBits::Two => TWOSTOPBITS
    }
}

/// # Set parity
pub fn set_parity(dcb: &mut DCB, parity: Parity) {
    dcb.Parity = match parity {
        Parity::None => NOPARITY,
        Parity::Odd => ODDPARITY,
        Parity::Even => EVENPARITY,
        Parity::Mark => MARKPARITY,
        Parity::Space => SPACEPARITY
    };

    if parity == Parity::None {
        dcb.fBits &= !fParity;
    } else {
        dcb.fBits |= fParity;
    }
}

/// # Set flow control
pub fn set_flow_control(dcb: &mut DCB, flow_ctrl: FlowCtrl) {
    unsafe {
        match flow_ctrl {
            FlowCtrl::None => {
                dcb.fBits &= !(fOutxCtsFlow | fRtsControl);
                dcb.fBits &= !(fOutX | fInX);
            }
            FlowCtrl::XOnXOff => {
                dcb.fBits |= fOutX | fInX;
            }
            FlowCtrl::RtsCts => {
                dcb.fBits &= !(fOutxCtsFlow | fRtsControl);
            }
            FlowCtrl::DtrDsr => {
                dcb.fBits &= !(fOutxDsrFlow | fDtrControl);
            }
            FlowCtrl::RtsCtsXOnXOff => {}
            FlowCtrl::DtrDsrXOnXOff => {}
        }
    }
}

pub fn set_timeouts(handle: HANDLE, timeout: Duration) {
    unsafe {
        let mills = timeout.as_secs() * 1000 + timeout.subsec_nanos() as u64 / 1_000_000;
        let timeouts = COMMTIMEOUTS {
            ReadIntervalTimeout: 0,
            ReadTotalTimeoutMultiplier: 0,
            ReadTotalTimeoutConstant: mills as DWORD,
            WriteTotalTimeoutMultiplier: 0,
            WriteTotalTimeoutConstant: 0,
        };
        SetCommTimeouts(handle, &timeouts);
    }
}
