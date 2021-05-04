use crate::*;

#[cfg(feature = "test_generated")]
#[test]
fn test_transfer_p2p() {
    println!(
        "{}",
        generators::Generator::payment_p2p("106Ca83003090c63B03d3fE3A9EE3B5E36C155CD", "32")
    );
}
