pub mod schema;
pub mod storage;
pub mod parser;
pub mod extract;
pub mod analysis;
pub mod index;
pub mod query;

pub use schema::{CodeEdge, CodeNode, EdgeType, NodeLabel, ResolutionTier};
pub use storage::{GraphStats, GraphStore, ImpactEntry, NodeContext};
pub use parser::{FileExtraction, LanguageProvider};
pub use parser::rust::RustProvider;
pub use parser::typescript::TypeScriptProvider;
pub use parser::solidity::SolidityProvider;
pub use extract::{SymbolTable, resolve_graph, TypeEnv, build_type_env_rust};
pub use analysis::community::{Community, detect_communities};
pub use analysis::process::{Process, ProcessType, detect_processes};
pub use analysis::synthesis::{SynthesizedSkill, synthesize_skill};
pub use index::{DiffImpact, Indexer, IndexResult};
