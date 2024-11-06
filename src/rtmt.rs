#[repr(C)]
#[derive(Debug)]
pub struct RtmtFrame<'a> {
    pub entry_header: u8,
    pub timestamp_entry: u32,
    pub buffer: &'a [u8],
    pub timestamp_exit: u32,
}

impl<'a> RtmtFrame<'a> {
    pub fn try_from_bytes(bytes: &'a [u8]) -> Result<Self, TryFromSliceError> {
        Ok(RtmtFrame {
            // [Id[3:0]|Priority[3:0]|Timestamp Entry[31:0]|Payload[?:0]|Timestamp Exit[31:0]|Last zero offset[8:0]|Sentinel[8:0]]
            entry_header: bytes[0],
            timestamp_entry: u32::from_be_bytes(bytes[1..5].try_into()?),
            buffer: &bytes[5..bytes.len() - 4],
            timestamp_exit: u32::from_be_bytes(bytes[bytes.len() - 4..].try_into()?),
        })
    }
}

use std::array::TryFromSliceError;
use std::fmt::{Display, Formatter};
use std::str;

impl Display for RtmtFrame<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let id = self.entry_header >> 4;
        let priority = self.entry_header & 0xF;

        write!(
            f,
            "[SOF Interrupt Id: {}, Interrupt Priority: {}]{{{}}}:{}[EOF]{{{}}}",
            id,
            priority,
            self.timestamp_entry,
            str::from_utf8(self.buffer).unwrap(),
            self.timestamp_exit
        )
    }
}
