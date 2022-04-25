//! RPC interface for the valueflows action pallet.

use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;

use pallet_valueflows_action::Action;
pub use pallet_valueflows_action_rpc_runtime_api::ActionRuntimeApi;

use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
	generic::BlockId,
	traits::Block as BlockT
};
use sp_std::prelude::Vec;

use std::sync::Arc;

#[rpc]
pub trait ActionApi<BlockHash> {
	#[rpc(name = "vf_allActions")]
	fn all_actions(&self, at: Option<BlockHash>) -> Result<Vec<Action>>;
}

pub struct ActionRpcHandler<C, P> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> ActionRpcHandler<C, P> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

const RUNTIME_ERROR: i64 = 8000;

impl<C, Block> ActionApi<<Block as BlockT>::Hash> for ActionRpcHandler<C, Block>
where
	Block: BlockT,
	C: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	C::Api: ActionRuntimeApi<Block>,
{
	fn all_actions(&self, at: Option<<Block as BlockT>::Hash>) -> Result<Vec<Action>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let actions = api.all_actions(&at).map_err(|e| RpcError {
			code: ErrorCode::ServerError(RUNTIME_ERROR),
			message: "Unable to query all actions.".into(),
			data: Some(e.to_string().into()),
		});

		actions
	}
}
