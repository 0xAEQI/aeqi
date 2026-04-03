use anyhow::Result;
use std::path::PathBuf;

use crate::cli::NotesAction;
use crate::helpers::load_config;
use aeqi_orchestrator::notes::{Notes, ClaimResult, EntryDurability};

pub(crate) async fn cmd_notes(
    config_path: &Option<PathBuf>,
    action: NotesAction,
) -> Result<()> {
    let (config, _) = load_config(config_path)?;
    let data_dir = config.data_dir();
    let bb_path = PathBuf::from(&data_dir).join("notes.db");
    let orch = &config.orchestrator;
    let bb = Notes::open(
        &bb_path,
        orch.notes_transient_ttl_hours,
        orch.notes_durable_ttl_days,
        orch.notes_claim_ttl_hours,
    )?;

    match action {
        NotesAction::List { company, limit } => {
            let entries = bb.list_project(&company, limit)?;
            if entries.is_empty() {
                println!("No note entries for company '{company}'.");
                return Ok(());
            }
            for entry in &entries {
                let tags_str = if entry.tags.is_empty() {
                    "-".to_string()
                } else {
                    entry.tags.join(", ")
                };
                println!(
                    "[{}] {} ({:?}) by {} | tags: {} | {}",
                    entry.created_at.format("%Y-%m-%d %H:%M"),
                    entry.key,
                    entry.durability,
                    entry.agent,
                    tags_str,
                    entry.content,
                );
            }
        }
        NotesAction::Post {
            company,
            key,
            content,
            tags,
            durability,
        } => {
            let dur = match durability.as_str() {
                "durable" => EntryDurability::Durable,
                _ => EntryDurability::Transient,
            };
            let entry = bb.post(&key, &content, "cli", &company, &tags, dur)?;
            println!(
                "Posted {} (expires {})",
                entry.key,
                entry.expires_at.format("%Y-%m-%d %H:%M")
            );
        }
        NotesAction::Query {
            company,
            tags,
            limit,
        } => {
            let entries = bb.query(&company, &tags, limit)?;
            if entries.is_empty() {
                println!("No matching entries.");
                return Ok(());
            }
            for entry in &entries {
                println!("{}: {} (by {})", entry.key, entry.content, entry.agent);
            }
        }
        NotesAction::Get { company, key } => match bb.get_by_key(&company, &key)? {
            Some(entry) => {
                println!(
                    "{}: {} (by {}, expires {})",
                    entry.key,
                    entry.content,
                    entry.agent,
                    entry.expires_at.format("%Y-%m-%d %H:%M"),
                );
            }
            None => println!("No entry found for key '{key}'."),
        },
        NotesAction::Claim {
            company,
            resource,
            content,
            agent,
        } => {
            let agent = agent.as_deref().unwrap_or("cli");
            match bb.claim(&resource, agent, &company, &content)? {
                ClaimResult::Acquired => {
                    println!("Claimed: {resource}");
                }
                ClaimResult::Renewed => {
                    println!("Renewed claim: {resource}");
                }
                ClaimResult::Held { holder, content } => {
                    println!("Held by {holder}: {content}");
                }
            }
        }
        NotesAction::Release {
            company,
            resource,
            agent,
            force,
        } => {
            let agent = agent.as_deref().unwrap_or("cli");
            if bb.release(&resource, agent, &company, force)? {
                println!("Released: {resource}");
            } else {
                println!("No claim found for '{resource}' (or not owned by {agent}).");
            }
        }
        NotesAction::Delete { company, key } => {
            if bb.delete_by_key(&company, &key)? {
                println!("Deleted: {key}");
            } else {
                println!("No entry found for key '{key}'.");
            }
        }
    }

    Ok(())
}
