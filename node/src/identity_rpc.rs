use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use liberland_node_runtime::{
    opaque::{Block, BlockId},
    pallet_identity::{IdentityPalletApi, IdentityType, PassportId},
    AccountId, Runtime,
};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_std::collections::btree_set::BTreeSet;
use std::sync::Arc;

#[rpc]
pub trait IdentityRpc {
    #[rpc(name = "get_passport_id")]
    fn get_passport_id(&self, account: AccountId) -> Result<Option<PassportId>>;

    #[rpc(name = "get_id_identities")]
    fn get_id_identities(&self, id: PassportId) -> Result<BTreeSet<IdentityType>>;

    #[rpc(name = "check_id_identity")]
    fn check_id_identity(&self, id: PassportId, id_type: IdentityType) -> Result<bool>;

    #[rpc(name = "get_account_identities")]
    fn get_account_identities(&self, account: AccountId) -> Result<BTreeSet<IdentityType>>;

    #[rpc(name = "check_account_indetity")]
    fn check_account_indetity(&self, account: AccountId, id_type: IdentityType) -> Result<bool>;
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
    fn get_passport_id(&self, account: AccountId) -> Result<Option<PassportId>> {
        let api = self.client.runtime_api();
        let best_hash = BlockId::hash(self.client.info().best_hash);
        let res = api.get_passport_id(&best_hash, account).unwrap();
        Ok(res)
    }

    fn get_id_identities(&self, id: PassportId) -> Result<BTreeSet<IdentityType>> {
        let api = self.client.runtime_api();
        let best_hash = BlockId::hash(self.client.info().best_hash);
        let res = api.get_id_identities(&best_hash, id).unwrap();
        Ok(res)
    }

    fn check_id_identity(&self, id: PassportId, id_type: IdentityType) -> Result<bool> {
        let api = self.client.runtime_api();
        let best_hash = BlockId::hash(self.client.info().best_hash);
        let res = api.check_id_identity(&best_hash, id, id_type).unwrap();
        Ok(res)
    }

    fn get_account_identities(&self, account: AccountId) -> Result<BTreeSet<IdentityType>> {
        let api = self.client.runtime_api();
        let best_hash = BlockId::hash(self.client.info().best_hash);
        let res = api.get_account_identities(&best_hash, account).unwrap();
        Ok(res)
    }

    fn check_account_indetity(&self, account: AccountId, id_type: IdentityType) -> Result<bool> {
        let api = self.client.runtime_api();
        let best_hash = BlockId::hash(self.client.info().best_hash);
        let res = api
            .check_account_indetity(&best_hash, account, id_type)
            .unwrap();
        Ok(res)
    }
}
