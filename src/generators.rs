use crate::*;
use rsevmasm::instructions::Instruction;
use reusable_fmt::fmt;
use crate::script_templates::payments::payment_send_raw_temp;

pub struct Generator;

impl Generator {
    pub fn payment_p2p(
        receiver: &str,
        amount: &str
    ) -> String {
        TEMPLATE_PAYMENT_SEND!(receiver, amount)
    }

    pub fn payment_p2p_bytes(
        receiver: &[u8],
        amount: &[u8]
    ) -> Vec<u8> {
        let mut temp = payment_send_raw_temp().to_vec();
        temp[51] = Instruction::Push(receiver.to_vec());
        temp[59] = Instruction::Push(amount.to_vec());
        rsevmasm::assemble_instructions(temp)
    }
}
