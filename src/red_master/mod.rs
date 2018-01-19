use red_buffer::RedBuffer;
use action::ActionErr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RedMaster {
    pub buffers: Vec<RedBuffer>,
    current_buffer: usize
}

impl RedMaster {
    pub fn empty() -> RedMaster {
        RedMaster { buffers: vec![ RedBuffer::empty() ], current_buffer: 0 }
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
