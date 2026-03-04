//! Gacha companion system with rarity tiers, archetypes, and fusion mechanics.
//!
//! Companions have a rich trait system — [`Rarity`] (C/B/A/S/SS), [`Archetype`] (8 types),
//! [`DereType`] (7 personality modes), [`Region`], [`Aesthetic`], and anime inspirations.
//! The [`GachaEngine`] handles weighted pulls with pity tracking. [`fuse`] and [`fuse_multi`]
//! combine companions into higher-rarity results. [`CompanionStore`] persists everything in SQLite.

pub mod anime;
pub mod companion;
pub mod fusion;
pub mod gacha;
pub mod names;
pub mod relationship;
pub mod store;

pub use anime::{AnimeGenre, AnimeInspiration};
pub use companion::{Archetype, Aesthetic, Companion, DereType, PersonaStatus, PortraitStatus, Rarity, Region};
pub use fusion::{fuse, fuse_multi, fusion_preview_text, validate_fusion, FusionError};
pub use gacha::{GachaEngine, GachaRates, PityState};
pub use relationship::{AgentRelationship, seed_from_traits};
pub use store::{CollectionStats, CompanionStore};
