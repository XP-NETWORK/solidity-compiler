use crate::script_templates::payments::payment_send_raw_temp;
use crate::*;
use reusable_fmt::fmt;
use rsevmasm::instructions::Instruction;

#[cfg(not(feature = "std"))]
use alloc::{vec::Vec, string::String, format};

pub struct Generator;

impl Generator {
    pub fn payment_p2p(receiver: &str, amount: &str) -> String {
        TEMPLATE_PAYMENT_SEND!(receiver, amount)
    }

    pub fn payment_p2p_bytes(receiver: &[u8], amount: &[u8]) -> Vec<u8> {
        let mut temp = payment_send_raw_temp().to_vec();
        temp[59] = Instruction::Push(receiver.to_vec());
        temp[67] = Instruction::Push(amount.to_vec());
        rsevmasm::assemble_instructions(temp)
    }
}
