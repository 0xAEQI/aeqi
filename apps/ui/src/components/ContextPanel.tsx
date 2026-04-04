import { useEffect, useState } from "react";
import { useChatStore } from "@/store/chat";
import { useDaemonStore } from "@/store/daemon";
import { api } from "@/lib/api";
import type { Agent } from "@/lib/types";

function timeAgo(ts: string | undefined | null): string {
  if (!ts) return "";
  const diff = Date.now() - new Date(ts).getTime();
  if (diff < 0) return "now";
  const sec = Math.floor(diff / 1000);
  if (sec < 60) return `${sec}s`;
  const min = Math.floor(sec / 60);
  if (min < 60) return `${min}m`;
  const hr = Math.floor(min / 60);
  if (hr < 24) return `${hr}h`;
  return `${Math.floor(hr / 24)}d`;
}

type Tab = "prompts" | "quests" | "insights" | "events";

function PromptsSection({ agentName, allAgents }: { agentName: string; allAgents: Agent[] }) {
  const [identityFiles, setIdentityFiles] = useState<Record<string, string>>({});
  const [ancestorChain, setAncestorChain] = useState<string[]>([]);

  useEffect(() => {
    // Build ancestor chain
    const chain: string[] = [];
    const byName = new Map<string, Agent>();
    for (const a of allAgents) byName.set(a.name, a);
    const byId = new Map<string, Agent>();
    for (const a of allAgents) byId.set(a.id, a);

    let current = byName.get(agentName);
    while (current) {
      chain.unshift(current.name);
      if (current.parent_id) {
        current = byId.get(current.parent_id);
      } else {
        break;
      }
    }
    setAncestorChain(chain);

    // Fetch identity/prompt files
    api.getAgentIdentity(agentName)
      .then((d: any) => {
        if (d.ok && d.files) setIdentityFiles(d.files);
        else setIdentityFiles({});
      })
      .catch(() => setIdentityFiles({}));
  }, [agentName, allAgents]);

  const fileNames = Object.keys(identityFiles);

  return (
    <div className="ctx-tab-content">
      {/* Ancestor chain */}
      {ancestorChain.length > 1 && (
        <div className="ctx-section">
          <div className="ctx-section-title">Prompt Chain</div>
          <div className="ctx-chain">
            {ancestorChain.map((name, i) => (
              <span key={name} className={`ctx-chain-item${name === agentName ? " ctx-chain-active" : ""}`}>
                {i > 0 && <span className="ctx-chain-arrow">&rarr;</span>}
                {name}
              </span>
            ))}
          </div>
        </div>
      )}

      {/* Identity files */}
      {fileNames.length > 0 ? (
        fileNames.map((filename) => (
          <div key={filename} className="ctx-section">
            <div className="ctx-section-title">{filename}</div>
            <pre className="ctx-pre">{identityFiles[filename]}</pre>
          </div>
        ))
      ) : (
        <div className="ctx-empty-state">No prompt files found</div>
      )}
    </div>
  );
}

function QuestsSection({ agentName }: { agentName: string }) {
  const quests = useDaemonStore((s) => s.quests);

  const agentQuests = quests.filter((q: any) => {
    const assignee = (q.assignee || q.agent || "").toLowerCase();
    return assignee.includes(agentName.toLowerCase());
  });

  const active = agentQuests.filter((q: any) => q.status === "in_progress" || q.status === "pending");
  const blocked = agentQuests.filter((q: any) => q.status === "blocked");
  const done = agentQuests.filter((q: any) => q.status === "done").slice(0, 5);

  return (
    <div className="ctx-tab-content">
      {active.length > 0 && (
        <div className="ctx-section">
          <div className="ctx-section-title">Active ({active.length})</div>
          <div className="ctx-list">
            {active.map((q: any) => (
              <div key={q.id} className="ctx-quest-row">
                <span className="ctx-quest-status active" />
                <div className="ctx-quest-info">
                  <span className="ctx-quest-id">{q.id}</span>
                  <span className="ctx-quest-subject">{q.subject}</span>
                </div>
                <span className="ctx-quest-time">{timeAgo(q.started_at || q.updated_at)}</span>
              </div>
            ))}
          </div>
        </div>
      )}
      {blocked.length > 0 && (
        <div className="ctx-section">
          <div className="ctx-section-title">Blocked ({blocked.length})</div>
          <div className="ctx-list">
            {blocked.map((q: any) => (
              <div key={q.id} className="ctx-quest-row blocked">
                <span className="ctx-quest-status blocked" />
                <div className="ctx-quest-info">
                  <span className="ctx-quest-id">{q.id}</span>
                  <span className="ctx-quest-subject">{q.subject}</span>
                  {q.blocked_reason && <span className="ctx-quest-reason">{q.blocked_reason}</span>}
                </div>
              </div>
            ))}
          </div>
        </div>
      )}
      {done.length > 0 && (
        <div className="ctx-section">
          <div className="ctx-section-title">Recent Done</div>
          <div className="ctx-list">
            {done.map((q: any) => (
              <div key={q.id} className="ctx-quest-row done">
                <span className="ctx-quest-status done" />
                <div className="ctx-quest-info">
                  <span className="ctx-quest-subject">{q.subject}</span>
                </div>
                <span className="ctx-quest-time">{timeAgo(q.updated_at || q.closed_at)}</span>
              </div>
            ))}
          </div>
        </div>
      )}
      {agentQuests.length === 0 && (
        <div className="ctx-empty-state">No quests for this agent</div>
      )}
    </div>
  );
}

function InsightsSection({ agentName }: { agentName: string }) {
  const [insights, setInsights] = useState<any[]>([]);
  const [search, setSearch] = useState("");

  useEffect(() => {
    api.getMemories({ query: agentName, limit: 20 })
      .then((d: any) => setInsights(d.memories || d.items || []))
      .catch(() => setInsights([]));
  }, [agentName]);

  const handleSearch = (q: string) => {
    setSearch(q);
    const query = q || agentName;
    api.getMemories({ query, limit: 20 })
      .then((d: any) => setInsights(d.memories || d.items || []))
      .catch(() => {});
  };

  return (
    <div className="ctx-tab-content">
      <div className="ctx-section">
        <input
          className="ctx-search"
          placeholder="Search insights..."
          value={search}
          onChange={(e) => handleSearch(e.target.value)}
        />
      </div>
      {insights.length > 0 ? (
        <div className="ctx-section">
          <div className="ctx-list">
            {insights.map((item: any) => (
              <div key={item.id || item.key} className="ctx-insight-row">
                <span className="ctx-insight-key">{item.key || item.title || "insight"}</span>
                <span className="ctx-insight-content">{(item.content || "").slice(0, 120)}</span>
              </div>
            ))}
          </div>
        </div>
      ) : (
        <div className="ctx-empty-state">No insights found</div>
      )}
    </div>
  );
}

function EventsSection({ agentName }: { agentName: string }) {
  const events = useDaemonStore((s) => s.events);

  const agentEvents = events.filter((e: any) => {
    const agent = (e.agent || e.actor || "").toLowerCase();
    return agent.includes(agentName.toLowerCase());
  });

  return (
    <div className="ctx-tab-content">
      {agentEvents.length > 0 ? (
        <div className="ctx-list">
          {agentEvents.slice(0, 20).map((e: any, i: number) => (
            <div key={e.id || i} className="ctx-event-row">
              <span className="ctx-event-time">{timeAgo(e.timestamp || e.created_at)}</span>
              <span className="ctx-event-type">{e.decision_type || e.event_type || ""}</span>
              <span className="ctx-event-summary">
                {e.summary || e.reasoning || e.description || "\u2014"}
              </span>
            </div>
          ))}
        </div>
      ) : (
        <div className="ctx-empty-state">No events for this agent</div>
      )}
    </div>
  );
}

export default function ContextPanel() {
  const selectedAgent = useChatStore((s) => s.selectedAgent);
  const allAgents = useDaemonStore((s) => s.agents);
  const agentName = selectedAgent?.name || "";
  const [tab, setTab] = useState<Tab>("quests");

  if (!selectedAgent) return null;

  return (
    <aside className="context-panel">
      <div className="context-panel-header">
        <button className={`ctx-tab${tab === "prompts" ? " ctx-tab-active" : ""}`} onClick={() => setTab("prompts")}>Prompts</button>
        <button className={`ctx-tab${tab === "quests" ? " ctx-tab-active" : ""}`} onClick={() => setTab("quests")}>Quests</button>
        <button className={`ctx-tab${tab === "insights" ? " ctx-tab-active" : ""}`} onClick={() => setTab("insights")}>Insights</button>
        <button className={`ctx-tab${tab === "events" ? " ctx-tab-active" : ""}`} onClick={() => setTab("events")}>Events</button>
      </div>
      {tab === "prompts" && <PromptsSection agentName={agentName} allAgents={allAgents} />}
      {tab === "quests" && <QuestsSection agentName={agentName} />}
      {tab === "insights" && <InsightsSection agentName={agentName} />}
      {tab === "events" && <EventsSection agentName={agentName} />}
    </aside>
  );
}
