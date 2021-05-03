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
