import Header from "@/components/Header";
import { useDaemonStore } from "@/store/daemon";
import { useChatStore } from "@/store/chat";
import { timeAgo } from "@/lib/format";
import { DataState } from "@/components/ui";

function formatDecisionType(type: string): string {
  return type.replace(/_/g, " ");
}

export default function EventsPage() {
  const events = useDaemonStore((s) => s.events);
  const loading = useDaemonStore((s) => s.loading);
  const selectedAgent = useChatStore((s) => s.selectedAgent);

  let filtered = events;
  if (selectedAgent) {
    filtered = filtered.filter(
      (e: any) => e.agent === selectedAgent.name || e.agent?.includes(selectedAgent.name)
    );
  }

  return (
    <div className="page-content">
      <Header title="Events" />

      {selectedAgent && (
        <div className="filters">
          <span className="filter-agent-badge">
            Filtered: {selectedAgent.display_name || selectedAgent.name}
          </span>
        </div>
      )}

      <DataState
        loading={loading}
        empty={filtered.length === 0}
        emptyTitle="No events"
        emptyDescription="No events recorded yet."
        loadingText="Loading events..."
      >
        <div className="events-stream">
          {filtered.map((event: any, i: number) => (
            <div key={event.id || i} className="event-row">
              <span className="event-time">{timeAgo(event.timestamp || event.created_at)}</span>
              <span className="event-type-badge">
                {formatDecisionType(event.decision_type || event.event_type || "event")}
              </span>
              <span className="event-agent">{event.agent || "\u2014"}</span>
              <span className="event-summary">
                {event.summary || event.reasoning || event.description || "\u2014"}
              </span>
              {event.task_id && (
                <code className="event-quest-id">{event.task_id}</code>
              )}
            </div>
          ))}
        </div>
      </DataState>
    </div>
  );
}
