use crate::{config, onchainos, rpc};

pub async fn run() -> anyhow::Result<()> {
    println!("=== EigenLayer Active Operators (Ethereum Mainnet) ===");
    println!();

    for op in config::KNOWN_OPERATORS {
        // Query isDelegated — actually we want to check if operator is registered
        // getOperatorDetails(address) returns (address delegationApprover, uint32 stakerOptOutWindowBlocks, uint32 maxMagnitude)
        let calldata = rpc::calldata_single_address(config::SEL_GET_OPERATOR_DETAILS, op.address);
        let result = onchainos::eth_call(config::CHAIN_ID, config::DELEGATION_MANAGER, &calldata);

        let approver = if let Ok(res) = result {
            if let Ok(data) = rpc::extract_return_data(&res) {
                // First word is the delegationApprover address
                rpc::decode_address(&data)
            } else {
                "unknown".to_string()
            }
        } else {
            "unknown".to_string()
        };

        let open = approver == "0x0000000000000000000000000000000000000000";
        let approval_label = if open { "Open (no approval required)" } else { "Requires approval" };

        println!("  Name:      {}", op.name);
        println!("  Address:   {}", op.address);
        println!("  Approval:  {}", approval_label);
        if !open {
            println!("  Approver:  {}", approver);
        }
        println!();
    }

    println!("To delegate to an operator: eigencloud delegate --operator <ADDRESS>");
    Ok(())
}
