use crate::core::ribosome::error::RibosomeResult;
use crate::core::ribosome::host_fn::HostContext;
use crate::core::ribosome::wasm_ribosome::WasmRibosome;
use holochain_zome_types::DecryptInput;
use holochain_zome_types::DecryptOutput;
use std::sync::Arc;

pub async fn decrypt(
    _ribosome: Arc<WasmRibosome<'_>>,
    _host_context: Arc<HostContext<'_>>,
    _input: DecryptInput,
) -> RibosomeResult<DecryptOutput> {
    unimplemented!();
}
