use std::collections::HashMap;
use std::convert::From;
use std::ops::Deref;

use red_buffer::RedBuffer;
use action::ActionErr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RedMaster {
    pub buffers: Vec<RedBuffer>,
    current_buffer: usize,
    pub registers: HashMap<Register, Vec<String>>
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Register(String);

impl From<String> for Register {
    fn from(x: String) -> Register {
        if x == "" {
            Register("'".into())
        } else {
            Register(x)
        }
    }
}

impl Deref for Register {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl RedMaster {
    pub fn empty() -> RedMaster {
        RedMaster { buffers: vec![ RedBuffer::empty() ], current_buffer: 0, registers: HashMap::new() }
    }

    pub fn curr_buf(&self) -> &RedBuffer {
        &self.buffers[self.current_buffer]
    }

    pub fn curr_buf_idx(&self) -> &usize {
        &self.current_buffer
    }

    pub fn curr_buf_mut(&mut self) -> &mut RedBuffer {
        &mut self.buffers[self.current_buffer]
    }

    pub fn change_buffer(&mut self, idx: usize) -> Result<(), ActionErr> {
        if idx >= self.buffers.len() {
            Err(ActionErr::OutOfBounds)
        } else {
            self.current_buffer = idx;
            Ok(())
        }
    }

}
