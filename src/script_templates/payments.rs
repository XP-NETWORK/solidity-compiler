use rsevmasm::instructions::Instruction;
use once_cell::sync::OnceCell;

#[macro_export]
macro_rules! TEMPLATE_PAYMENT_SEND {
    ($receiver:expr, $amount:tt) => {
        fmt!(
            TEMPLATE_SCRIPT_MAIN,
            contract_name = "SendEth",
            decls = "",
            contract_body = fmt!(
                TEMPLATE_FUNCTION,
                name = "send",
                args = "",
                metas = "public payable",
                body = fmt!(
r#"
    address payable receiver = payable(0x{receiver});
    (bool sent, bytes memory _data) = receiver.call{{value: {amount}}}("");
    require(sent, "Failed to send Eth");
"#,
receiver = $receiver,
amount = $amount
                )
            )
        )
    }
}

static PAYMENT_SEND_RAW: OnceCell<[Instruction; 171]> = OnceCell::new();

pub fn payment_send_raw_temp() -> &'static [Instruction; 171] {
    PAYMENT_SEND_RAW.get_or_init(payment_raw_init)
}

fn payment_raw_init() -> [Instruction; 171] {
    [
Instruction::Push(vec![128]),
Instruction::Push(vec![64]),
Instruction::MStore,
Instruction::CallValue,
Instruction::Dup(0),
Instruction::IsZero,
Instruction::Push(vec![0, 16]),
Instruction::JumpIf,
Instruction::Push(vec![0]),
Instruction::Dup(0),
Instruction::Revert,
Instruction::JumpDest,
Instruction::Pop,
Instruction::Push(vec![1, 12]),
Instruction::Dup(0),
Instruction::Push(vec![0, 32]),
Instruction::Push(vec![0]),
Instruction::CodeCopy,
Instruction::Push(vec![0]),
Instruction::Return,
Instruction::Invalid,
Instruction::Push(vec![128]),
Instruction::Push(vec![64]),
Instruction::MStore,
Instruction::Push(vec![4]),
Instruction::CallDataSize,
Instruction::Lt,
Instruction::Push(vec![28]),
Instruction::JumpIf,
Instruction::Push(vec![0]),
Instruction::CallDataLoad,
Instruction::Push(vec![224]),
Instruction::Shr,
Instruction::Dup(0),
Instruction::Push(vec![180, 99, 0, 236]),
Instruction::EQ,
Instruction::Push(vec![33]),
Instruction::JumpIf,
Instruction::JumpDest,
Instruction::Push(vec![0]),
Instruction::Dup(0),
Instruction::Revert,
Instruction::JumpDest,
Instruction::Push(vec![39]),
Instruction::Push(vec![41]),
Instruction::Jump,
Instruction::JumpDest,
Instruction::Stop,
Instruction::JumpDest,
Instruction::Push(vec![64]),
Instruction::MLoad,
// Receiver address idx 51
Instruction::Invalid,
Instruction::Swap(1),
Instruction::Push(vec![0]),
Instruction::Swap(1),
Instruction::Dup(1),
Instruction::Swap(1),
Instruction::Dup(3),
Instruction::Swap(1),
// wei value idx 59
Instruction::Invalid,
Instruction::Swap(1),
Instruction::Dup(3),
Instruction::Dup(1),
Instruction::Dup(1),
Instruction::Dup(1),
Instruction::Dup(5),
Instruction::Dup(7),
Instruction::Gas,
Instruction::Call,
Instruction::Swap(3),
Instruction::Pop,
Instruction::Pop,
Instruction::Pop,
Instruction::ReturnDataSize,
Instruction::Dup(0),
Instruction::Push(vec![0]),
Instruction::Dup(1),
Instruction::EQ,
Instruction::Push(vec![130]),
Instruction::JumpIf,
Instruction::Push(vec![64]),
Instruction::MLoad,
Instruction::Swap(2),
Instruction::Pop,
Instruction::Push(vec![31]),
Instruction::Not,
Instruction::Push(vec![63]),
Instruction::ReturnDataSize,
Instruction::Add,
Instruction::And,
Instruction::Dup(2),
Instruction::Add,
Instruction::Push(vec![64]),
Instruction::MStore,
Instruction::ReturnDataSize,
Instruction::Dup(2),
Instruction::MStore,
Instruction::ReturnDataSize,
Instruction::Push(vec![0]),
Instruction::Push(vec![32]),
Instruction::Dup(4),
Instruction::Add,
Instruction::ReturnDataCopy,
Instruction::Push(vec![135]),
Instruction::Jump,
Instruction::JumpDest,
Instruction::Push(vec![96]),
Instruction::Swap(2),
Instruction::Pop,
Instruction::JumpDest,
Instruction::Pop,
Instruction::Swap(2),
Instruction::Pop,
Instruction::Swap(2),
Instruction::Pop,
Instruction::Dup(1),
Instruction::Push(vec![209]),
Instruction::JumpIf,
Instruction::Push(vec![64]),
Instruction::MLoad,
Instruction::Push(vec![70, 27, 205]),
Instruction::Push(vec![229]),
Instruction::Shl,
Instruction::Dup(1),
Instruction::MStore,
Instruction::Push(vec![32]),
Instruction::Push(vec![4]),
Instruction::Dup(2),
Instruction::Add,
Instruction::MStore,
Instruction::Push(vec![18]),
Instruction::Push(vec![36]),
Instruction::Dup(2),
Instruction::Add,
Instruction::MStore,
Instruction::Push(vec![8, 204, 45, 45, 140, 172, 132, 14, 141, 228, 14, 108, 173, 204, 132, 8, 174, 141]),
Instruction::Push(vec![115]),
Instruction::Shl,
Instruction::Push(vec![68]),
Instruction::Dup(2),
Instruction::Add,
Instruction::MStore,
Instruction::Push(vec![100]),
Instruction::Add,
Instruction::Push(vec![64]),
Instruction::MLoad,
Instruction::Dup(0),
Instruction::Swap(2),
Instruction::Sub,
Instruction::Swap(1),
Instruction::Revert,
Instruction::JumpDest,
Instruction::Pop,
Instruction::Pop,
Instruction::Pop,
Instruction::Jump,
Instruction::Invalid,
Instruction::Log(2),
Instruction::Push(vec![105, 112, 102, 115, 88]),
Instruction::Invalid,
Instruction::SLt,
Instruction::Sha3,
Instruction::CodeSize,
Instruction::Invalid,
Instruction::Invalid,
Instruction::Invalid,
Instruction::GasLimit,
Instruction::Swap(16),
Instruction::Push(vec![130, 161, 84, 17, 101, 224, 181]),
Instruction::Dup(11),
Instruction::Push(vec![88]),
]
}
