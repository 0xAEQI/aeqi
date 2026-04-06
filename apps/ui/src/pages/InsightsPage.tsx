import { useEffect, useRef, useState } from "react";
import { DataState } from "@/components/ui";
import { api } from "@/lib/api";
import { useChatStore } from "@/store/chat";

const CATEGORY_COLORS: Record<string, string> = {
  fact: "var(--info)",
  decision: "var(--accent)",
  preference: "var(--warning)",
  insight: "var(--success)",
};

const SCOPE_LABELS: Record<string, string> = {
  domain: "Domain",
  system: "System",
  self: "Self",
  personal: "Personal",
  session: "Session",
};

export default function InsightsPage() {
  const selectedAgent = useChatStore((s) => s.selectedAgent);
  const [insights, setInsights] = useState<any[]>([]);
  const [search, setSearch] = useState("");
  const [debouncedSearch, setDebouncedSearch] = useState("");
  const [loading, setLoading] = useState(true);
  const debounceRef = useRef<ReturnType<typeof setTimeout>>(undefined);

  // Debounce search input — 300ms
  useEffect(() => {
    debounceRef.current = setTimeout(() => setDebouncedSearch(search), 300);
    return () => clearTimeout(debounceRef.current);
  }, [search]);

  useEffect(() => {
    setLoading(true);
    api
      .getMemories({
        query: debouncedSearch || undefined,
        company: selectedAgent?.name || undefined,
        limit: 100,
      })
      .then((d) => {
        setInsights(d.memories || []);
        setLoading(false);
      })
      .catch(() => setLoading(false));
  }, [debouncedSearch, selectedAgent]);

  const filtered = insights;

  return (
    <div className="page-content">
      <div className="q-hero">
        <div className="q-hero-left">
          <h1 className="q-hero-title">Insights</h1>
          <p className="q-hero-subtitle">Knowledge your agents accumulate across sessions</p>
        </div>
      </div>

      <div className="filters">
        <input
          className="filter-input"
          style={{ flex: 1 }}
          placeholder="Search insights..."
          value={search}
          onChange={(e) => setSearch(e.target.value)}
        />
        {selectedAgent && (
          <span className="filter-agent-badge">
            Filtered: {selectedAgent.display_name || selectedAgent.name}
          </span>
        )}
        <span className="filter-count">{filtered.length} results</span>
      </div>

      <DataState
        loading={loading}
        empty={filtered.length === 0}
        emptyTitle="No insights"
        emptyDescription="No insights found. Insights are memories and knowledge stored by agents."
        loadingText="Loading insights..."
      >
        <div>
          {filtered.map((m: any) => (
            <div key={m.id} className="memory-entry" style={{ borderLeft: `3px solid ${CATEGORY_COLORS[m.category] || "var(--text-muted)"}` }}>
              <div className="memory-header">
                <code className="memory-key">{m.key}</code>
                <div className="memory-tags">
                  <span
                    className="memory-category"
                    style={{
                      color: CATEGORY_COLORS[m.category] || "var(--text-muted)",
                    }}
                  >
                    {m.category}
                  </span>
                  <span className="memory-scope">
                    {SCOPE_LABELS[m.scope] || m.scope}
                  </span>
                </div>
              </div>
              <div className="memory-content">{m.content}</div>
              <div className="memory-meta">
                {m.agent_id && <span>Agent: {m.agent_id}</span>}
                <span>
                  {new Date(m.created_at).toLocaleString("en-US", {
                    month: "short",
                    day: "numeric",
                    hour: "2-digit",
                    minute: "2-digit",
                  })}
                </span>
              </div>
            </div>
          ))}
        </div>
      </DataState>
    </div>
  );
}
