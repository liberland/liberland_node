use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use liberland_node_runtime::{
    opaque::{Block, BlockId},
    pallet_min_interior::{KycRequest, MinInteriorPalletApi},
    AccountId, Runtime,
};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_std::collections::btree_set::BTreeSet;
use std::sync::Arc;

#[rpc]
pub trait MinInteriorRpc {
    #[rpc(name = "get_all_requests")]
    fn get_all_requests(&self) -> Result<BTreeSet<KycRequest<AccountId>>>;
}

pub struct MinInteriorRpcImpl<C> {
    pub client: Arc<C>,
}

impl<C> MinInteriorRpc for MinInteriorRpcImpl<C>
where
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block>,
    C::Api: MinInteriorPalletApi<Block, Runtime>,
{
    fn get_all_requests(&self) -> Result<BTreeSet<KycRequest<AccountId>>> {
        let api = self.client.runtime_api();
        let best_hash = BlockId::hash(self.client.info().best_hash);
        let res = api.get_all_requests(&best_hash).unwrap();
        Ok(res)
    }
}
