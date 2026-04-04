import { useEffect, useState } from "react";
import Header from "@/components/Header";
import StatusBadge from "@/components/StatusBadge";
import { DataState } from "@/components/ui";
import { PRIORITY_COLORS } from "@/lib/constants";
import { useDaemonStore } from "@/store/daemon";
import { useChatStore } from "@/store/chat";
import { runtimeLabel, summarizeTaskRuntime } from "@/lib/runtime";

export default function QuestsPage() {
  const quests = useDaemonStore((s) => s.quests);
  const loading = useDaemonStore((s) => s.loading);
  const selectedAgent = useChatStore((s) => s.selectedAgent);
  const [statusFilter, setStatusFilter] = useState("");

  let filtered = quests;

  // Filter by selected agent
  if (selectedAgent) {
    filtered = filtered.filter(
      (q: any) =>
        q.assignee === selectedAgent.name ||
        q.agent_id === selectedAgent.id
    );
  }

  // Filter by status
  if (statusFilter) {
    filtered = filtered.filter((q: any) => q.status === statusFilter);
  }

  return (
    <div className="page-content">
      <Header title="Quests" />

      <div className="filters">
        <select
          className="filter-select"
          value={statusFilter}
          onChange={(e) => setStatusFilter(e.target.value)}
        >
          <option value="">All statuses</option>
          <option value="pending">Pending</option>
          <option value="in_progress">In Progress</option>
          <option value="done">Done</option>
          <option value="blocked">Blocked</option>
          <option value="cancelled">Cancelled</option>
        </select>
        {selectedAgent && (
          <span className="filter-agent-badge">
            Filtered: {selectedAgent.display_name || selectedAgent.name}
          </span>
        )}
        <span className="filter-count">
          {filtered.length} quests
        </span>
      </div>

      <DataState
        loading={loading}
        empty={filtered.length === 0}
        emptyTitle="No quests"
        emptyDescription="No quests match the current filters."
        loadingText="Loading quests..."
      >
        <div className="task-table">
          {filtered.map((quest: any) => {
            const label = runtimeLabel(quest.runtime);
            const detail = summarizeTaskRuntime(quest.runtime, quest.closed_reason);

            return (
              <div key={quest.id} className="task-row">
                <span
                  className="task-priority-bar"
                  style={{ backgroundColor: PRIORITY_COLORS[quest.priority] || "var(--text-primary)" }}
                />
                <code className="task-id">{quest.id}</code>
                <div className="task-row-detail">
                  <span className="task-subject">{quest.subject}</span>
                  {(label || detail) && (
                    <span className="task-runtime">
                      {[label, detail].filter(Boolean).join(" \u2022 ")}
                    </span>
                  )}
                </div>
                <div className="task-meta">
                  <StatusBadge status={quest.status} size="sm" />
                  <span>{quest.assignee || "\u2014"}</span>
                </div>
              </div>
            );
          })}
        </div>
      </DataState>
    </div>
  );
}
