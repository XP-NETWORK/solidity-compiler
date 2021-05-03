pub mod payments;

use reusable_fmt::fmt_reuse;

// Template for a default Solidity script
fmt_reuse! {
    TEMPLATE_SCRIPT_MAIN = r#"
pragma solidity >=0.7.0 <0.9.0;

contract {contract_name} {{
    {decls}

    {contract_body}
}}
"#;
}

fmt_reuse! {
    TEMPLATE_FUNCTION = r#"
function {name}({args}) {metas} {{
    {body}
}}
"#;
}
