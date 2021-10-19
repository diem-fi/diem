use std::{
    option::Option,
    result::Result,
    result::Result::Ok,
    string::String,
};
use diem_management::{error::Error, execute_command};
use structopt::StructOpt;
use diem_client::views::VMStatusView;
use diem_global_constants::{CONSENSUS_KEY, FULLNODE_NETWORK_KEY, OWNER_ACCOUNT, VALIDATOR_NETWORK_KEY};
use diem_management::config::ConfigPath;
use diem_management::secure_backend::ValidatorBackend;
use diem_management::storage::to_x25519;
use diem_types::{
    account_address::AccountAddress,
    network_address::NetworkAddress
};
use crate::{
    json_rpc::JsonRpcClientWrapper,
    validator_config::ValidatorConfig
};
use crate::validator_config::DecryptedValidatorConfig;

#[derive(Debug, StructOpt)]
pub struct VerifyValidatorState {
    #[structopt(long, required_unless = "config")]
    json_server: Option<String>,
    #[structopt(flatten)]
    validator_config: diem_management::validator_config::ValidatorConfig,
}

impl VerifyValidatorState {
    pub fn execute(self) -> Result<bool, Error> {
        println!("Got the verify-validator-state command, returning true");
        // Load the config, storage backend and create a json rpc client.
        let config = self
            .validator_config
            .config()?
            .override_json_server(&self.json_server);
        let storage = config.validator_backend();
        let encryptor = config.validator_backend().encryptor();
        let client = JsonRpcClientWrapper::new(config.json_server.clone());

        // Fetch the current on-chain validator config for the node
        let owner_account = storage.account_address(OWNER_ACCOUNT)?;
        let validator_config = client.validator_config(owner_account).and_then(|vc| {
            DecryptedValidatorConfig::from_validator_config_resource(&vc, owner_account, &encryptor)
        })?;

        // iterate over keys and validate if they match with the ones in storage
        let mut keys_match: bool = true;
        for key_name in [CONSENSUS_KEY, VALIDATOR_NETWORK_KEY, FULLNODE_NETWORK_KEY]{
            let storage_key = storage.ed25519_public_from_private(key_name)?;
            keys_match &= match key_name {
                CONSENSUS_KEY => storage_key == validator_config.consensus_public_key,
                VALIDATOR_NETWORK_KEY => {
                    Some(to_x25519(storage_key.clone())?)
                        == validator_config
                        .validator_network_address
                        .find_noise_proto()
                }
                FULLNODE_NETWORK_KEY => {
                    Some(to_x25519(storage_key.clone())?)
                        == validator_config.fullnode_network_address.find_noise_proto()
                }
                _ => {
                    return Err(Error::UnexpectedError(
                        "Rotate key was called with an unknown key name!".into(),
                    ));
                }
            };
        }

        Ok(keys_match)
    }
}
