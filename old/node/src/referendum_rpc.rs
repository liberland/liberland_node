use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use liberland_node_runtime::{
    opaque::{Block, BlockId},
    pallet_referendum::{ReferendumPalletApi, Suggestion},
    Hash, Runtime,
};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_std::collections::btree_map::BTreeMap;
use std::sync::Arc;

#[rpc]
pub trait ReferendumRpc {
    #[rpc(name = "get_active_petitions")]
    fn get_active_petitions(&self) -> Result<BTreeMap<Hash, Suggestion>>;

    #[rpc(name = "get_active_referendums")]
    fn get_active_referendums(&self) -> Result<BTreeMap<Hash, Suggestion>>;

    #[rpc(name = "get_successfull_referendums")]
    fn get_successfull_referendums(&self) -> Result<BTreeMap<Hash, Suggestion>>;
}

pub struct ReferendumRpcImpl<C> {
    pub client: Arc<C>,
}

impl<C> ReferendumRpc for ReferendumRpcImpl<C>
where
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block>,
    C::Api: ReferendumPalletApi<Block, Runtime>,
{
    fn get_active_petitions(&self) -> Result<BTreeMap<Hash, Suggestion>> {
        let api = self.client.runtime_api();
        let best_hash = BlockId::hash(self.client.info().best_hash);
        let res = api.get_active_petitions(&best_hash).unwrap();
        Ok(res)
    }

    fn get_active_referendums(&self) -> Result<BTreeMap<Hash, Suggestion>> {
        let api = self.client.runtime_api();
        let best_hash = BlockId::hash(self.client.info().best_hash);
        let res = api.get_active_referendums(&best_hash).unwrap();
        Ok(res)
    }

    fn get_successfull_referendums(&self) -> Result<BTreeMap<Hash, Suggestion>> {
        let api = self.client.runtime_api();
        let best_hash = BlockId::hash(self.client.info().best_hash);
        let res = api.get_successfull_referendums(&best_hash).unwrap();
        Ok(res)
    }
}
