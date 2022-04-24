//! RPC interface for the valueflows action pallet.

use jsonrpc_core::rpc;
use pallet_valueflows_action::Action;

pub use pallet_::TransactionPaymentApi as TransactionPaymentRuntimeApi;

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

impl ActionApi<<B as BlockT>::Hash>
	for ActionRpcHandler<C, Block>
where
	Block: BlockT,
	C: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	C::Api: ActionRuntimeApi<Block>,
{
	fn all_actions(&self, at: Option<<Block as BlockT>::Hash>) -> Result<Vec<Action>> {
		let api = self.client.runtime_api();
		let block_hash = at.unwarp_or_else(|| self.client.info().best_hash);

		let actions = api.all_cations();

		Ok(actions)
	}
}
