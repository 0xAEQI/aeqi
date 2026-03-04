//! Tool implementations for the `Tool` trait, available to agent workers.
//!
//! Provides shell execution ([`ShellTool`]), file read/write ([`FileReadTool`],
//! [`FileWriteTool`], [`ListDirTool`]), git worktree management ([`GitWorktreeTool`]),
//! task CRUD ([`BeadsCreateTool`] et al.), cross-agent delegation ([`DelegateTool`]),
//! DNS management via Porkbun ([`PorkbunTool`]), and skill invocation ([`Skill`]).

pub mod beads;
pub mod delegate;
pub mod file;
pub mod git;
pub mod porkbun;
pub mod shell;
pub mod skill;

pub use beads::{BeadsCreateTool, BeadsReadyTool, BeadsUpdateTool, BeadsCloseTool, BeadsShowTool, BeadsDepTool};
pub use delegate::DelegateTool;
pub use file::{FileReadTool, FileWriteTool, ListDirTool};
pub use git::GitWorktreeTool;
pub use porkbun::PorkbunTool;
pub use shell::ShellTool;
pub use skill::Skill;
