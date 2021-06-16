use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use liberland_node_runtime::{
    opaque::{Block, BlockId},
    pallet_kyc::{KycPalletApi, KycRequest},
    AccountId, BlockNumber, Runtime,
};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use std::sync::Arc;

#[rpc]
pub trait KycRpc {
    #[rpc(name = "get_earliest_request")]
    fn get_earliest_request(&self) -> Result<Option<KycRequest<AccountId, BlockNumber>>>;
}

pub struct KycRpcImpl<C> {
    pub client: Arc<C>,
}

impl<C> KycRpc for KycRpcImpl<C>
where
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block>,
    C::Api: KycPalletApi<Block, Runtime>,
{
    fn get_earliest_request(&self) -> Result<Option<KycRequest<AccountId, BlockNumber>>> {
        let api = self.client.runtime_api();
        let best_hash = BlockId::hash(self.client.info().best_hash);
        let res = api.get_earliest_request(&best_hash).unwrap();
        Ok(res)
    }
}
