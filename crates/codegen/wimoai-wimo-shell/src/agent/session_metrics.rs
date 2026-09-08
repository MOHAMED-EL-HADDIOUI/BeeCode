//! Session lifecycle event structs.
//!
//! The structs moved to `wimoai-wimo-telemetry` in the telemetry crate split.
//! This module re-exports them so the existing import path in shell keeps working.

pub(crate) use wimoai_wimo_telemetry::session_metrics::{
    DoomLoopDetected, DoomLoopRecovery, SessionContextSnapshot, SessionStartKind, SessionStarted,
    TraceUploadAttempted, TraceUploadFailed, TraceUploadSkipped, TraceUploadSucceeded, Turn,
    TurnCompletedLifecycle,
};
