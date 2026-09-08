pub mod conversation;
pub mod error;
pub mod types;

// `Client` is the legacy alias used throughout the shell; it points at the sampler crate's `SamplingClient`
// The two have identical method sets, so call sites compile unchanged
pub use self::conversation::*;
pub use self::error::{ResponseModelMetadata, Result, SamplingError};
pub use self::types::*;
pub use wimo ai_wimo_sampler::ApiBackend;
pub use wimo ai_wimo_sampler::SamplingClient as Client;

// Re-export async-openai Responses API types under `rs` namespace
pub use async_openai::types::responses as rs;

// ---------------------------------------------------------------------------
// wimo ai-wimo-sampler re-exports
// ---------------------------------------------------------------------------
//
// The actual streaming / retry / HTTP-client logic lives in the `wimo ai-wimo-sampler` crate
// These re-exports keep `crate::sampling::{SamplerHandle, SamplerConfig, ...}` paths working for callers not yet ported to `wimo ai_wimo_sampler::*`
// There is no shell-side `sampling::client::Config` composite anymore; `MvpAgent` holds session-snapshot state in a `RefCell<SamplerConfig>`
pub use wimo ai_wimo_sampler::{
    InferenceLatencyStats, OriginClientInfo, RequestId, SamplerActor, SamplerConfig, SamplerHandle,
    SamplingChannel, SamplingClient, SamplingErrorInfo, SamplingErrorKind, SamplingEvent,
};
