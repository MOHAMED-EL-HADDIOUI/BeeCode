//! Session lifecycle event structs.
//!
//! The structs moved to `wimo ai-wimo-telemetry` in the telemetry crate split.
//! This module re-exports them so the existing import path in shell keeps working.

pub(crate) use wimo ai_wimo_telemetry::session_metrics::{
    DoomLoopDetected, DoomLoopRecovery, SessionContextSnapshot, SessionStartKind, SessionStarted,
    TraceUploadAttempted, TraceUploadFailed, TraceUploadSkipped, TraceUploadSucceeded, Turn,
    TurnCompletedLifecycle,
};
