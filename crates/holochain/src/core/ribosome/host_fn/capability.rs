use crate::core::ribosome::error::RibosomeResult;
use crate::core::ribosome::host_fn::HostContext;
use crate::core::ribosome::wasm_ribosome::WasmRibosome;
use holochain_zome_types::CapabilityInput;
use holochain_zome_types::CapabilityOutput;
use std::sync::Arc;

pub async fn capability(
    _ribosome: Arc<WasmRibosome<'_>>,
    _host_context: Arc<HostContext<'_>>,
    _input: CapabilityInput,
) -> RibosomeResult<CapabilityOutput> {
    unimplemented!();
}
