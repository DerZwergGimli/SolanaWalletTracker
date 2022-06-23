use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(rename = "blockTime")]
    pub block_time: i64,
    pub slot: i64,
    #[serde(rename = "txHash")]
    pub tx_hash: String,
    pub fee: i64,
    pub status: Status,
    pub lamport: i64,
    pub signer: Vec<String>,
    #[serde(rename = "parsedInstruction")]
    pub parsed_instruction: Vec<ParsedInstruction>,
    #[serde(rename = "includeSPLTransfer")]
    pub include_spl_transfer: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedInstruction {
    #[serde(rename = "programId")]
    pub program_id: String,
    #[serde(rename = "type")]
    pub parsed_instruction_type: Type,
    pub program: Option<Program>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "approve")]
    Approve,
    #[serde(rename = "createAccount")]
    CreateAccount,
    #[serde(rename = "createAssociatedAccount")]
    CreateAssociatedAccount,
    #[serde(rename = "matchOrders")]
    MatchOrders,
    #[serde(rename = "newOrderV3")]
    NewOrderV3,
    #[serde(rename = "settleFunds")]
    SettleFunds,
    #[serde(rename = "sol-transfer")]
    SolTransfer,
    #[serde(rename = "spl-transfer")]
    SplTransfer,
    #[serde(rename = "swap")]
    Swap,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Program {
    #[serde(rename = "spl-associated-token-account")]
    SplAssociatedTokenAccount,
    #[serde(rename = "spl-token")]
    SplToken,
    #[serde(rename = "system")]
    System,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Success,
}
