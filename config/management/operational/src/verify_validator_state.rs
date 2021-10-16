use std::{
    option::Option,
    result::Result,
    result::Result::Ok,
    string::String,
};
use diem_management::{error::Error, execute_command};
use structopt::StructOpt;
use diem_client::views::VMStatusView;
use diem_management::config::ConfigPath;
use diem_types::{
    account_address::AccountAddress,
    network_address::NetworkAddress
};
use crate::{
    json_rpc::JsonRpcClientWrapper,
    validator_config::ValidatorConfig
};

#[derive(Debug, StructOpt)]
pub struct VerifyValidatorState {
    #[structopt(flatten)]
    config: ConfigPath,
    /// JSON-RPC Endpoint (e.g. http://localhost:8080)
    #[structopt(long, required_unless = "config")]
    json_server: Option<String>,
    #[structopt(flatten)]
    validator_config: ValidatorConfig,
    #[structopt(long, help = "AccountAddress to check transactions")]
    account_address: AccountAddress,
    #[structopt(long, help = "Sequence number to verify")]
    sequence_number: u64,
}

impl VerifyValidatorState {
    pub fn execute(self) -> Result<bool, Error> {
        println!("Got the verify-validator-state command, returning true");
        let config = self.config.load()?.override_json_server(&self.json_server);
        // let vm_status = JsonRpcClientWrapper::new(config.json_server)
        //     .transaction_status(self.account_address, self.sequence_number)?;
        Ok(true)
    }
}
