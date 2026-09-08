use std::path::PathBuf;

pub mod info;

pub use info::Info;

// Re-export shared feedback wire types used by downstream crates (e.g. wimoai-wimo-pager-render).
pub use prod_mc_cli_chat_proxy_types::feedback_types::FeedbackTerminalInfo;

pub fn session_dir(info: &Info) -> PathBuf {
    wimoai_wimo_tools::util::wimo_home::sessions_cwd_dir(&info.cwd).join(info.id.to_string())
}
