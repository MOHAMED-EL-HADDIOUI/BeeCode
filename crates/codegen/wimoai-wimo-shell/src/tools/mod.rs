//! All tool execution goes through `wimo ai-wimo-tools` via the `ToolBridge`.
//! Types (ToolOutput, ToolInput, TodoState, etc.) come from `wimo ai-wimo-tools` directly.

pub mod bridge;
pub mod config;
pub mod notification_bridge;
pub mod retry;
pub(crate) mod task_completed_frame;
pub mod todo;
pub mod tool_context;

pub use self::{
    config::{BashToolConfig, FileToolset, ShellToolsetConfig},
    retry::{RetryConfig, execute_with_retry},
    tool_context::ToolContext,
};

// Re-export key types from wimo ai-wimo-tools for convenience
pub use self::todo::{TodoId, TodoItem, TodoPriority, TodoStatus};
pub use wimo ai_wimo_tools::types::output::ToolOutput;
pub use wimo ai_wimo_tools::types::{MCPToolInput, ToolInput};
