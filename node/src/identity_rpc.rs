use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use liberland_node_runtime::{
    opaque::{Block, BlockId},
    pallet_identity::{IdentityPalletApi, IdentityType, PassportId},
    AccountId, Runtime,
};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use std::sync::Arc;

#[rpc]
pub trait IdentityRpc {
    #[rpc(name = "check_id_identity")]
    fn check_id_identity(&self, id: PassportId, id_type: IdentityType) -> Result<bool>;

    #[rpc(name = "check_account_identity")]
    fn check_account_identity(&self, account: AccountId, id_type: IdentityType) -> Result<bool>;
}

pub struct IdentityRpcImpl<C> {
    pub client: Arc<C>,
}

impl<C> IdentityRpc for IdentityRpcImpl<C>
where
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block>,
    C::Api: IdentityPalletApi<Block, Runtime>,
{
    fn check_id_identity(&self, id: PassportId, id_type: IdentityType) -> Result<bool> {
        let api = self.client.runtime_api();
        let best_hash = BlockId::hash(self.client.info().best_hash);
        let res = api.check_id_identity(&best_hash, id, id_type).unwrap();
        Ok(res)
    }

    fn check_account_identity(&self, account: AccountId, id_type: IdentityType) -> Result<bool> {
        let api = self.client.runtime_api();
        let best_hash = BlockId::hash(self.client.info().best_hash);
        let res = api
            .check_account_identity(&best_hash, account, id_type)
            .unwrap();
        Ok(res)
    }
}
