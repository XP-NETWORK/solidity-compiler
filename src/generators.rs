use crate::*;
use reusable_fmt::fmt;

pub struct Generator;

impl Generator {
    pub fn payment_p2p(
        receiver: &str,
        amount: &str
    ) -> String {
        TEMPLATE_PAYMENT_SEND!(receiver, amount)
    }
}