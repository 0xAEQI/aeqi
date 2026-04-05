import { useNavigate } from "react-router-dom";
import "@/styles/welcome.css";

export default function WelcomePage() {
  const navigate = useNavigate();

  return (
    <div className="welcome">
      <div className="welcome-hero">
        <h1 className="welcome-title">
          aeqi<span className="welcome-dot">.ai</span>
        </h1>
        <p className="welcome-tagline">The agent runtime.</p>
      </div>

      <div className="welcome-grid">
        <div className="welcome-card" onClick={() => navigate("/agents")}>
          <span className="welcome-card-icon">✦</span>
          <div className="welcome-card-body">
            <h3>Agents</h3>
            <p>Autonomous entities that research, plan, implement, and verify. Organized in parent-child hierarchies.</p>
          </div>
          <span className="welcome-card-arrow">&rarr;</span>
        </div>

        <div className="welcome-card" onClick={() => navigate("/quests")}>
          <span className="welcome-card-icon">◆</span>
          <div className="welcome-card-body">
            <h3>Quests</h3>
            <p>Units of work tracked through your agent pipeline. Kanban board with priorities and acceptance criteria.</p>
          </div>
          <span className="welcome-card-arrow">&rarr;</span>
        </div>

        <div className="welcome-card" onClick={() => navigate("/events")}>
          <span className="welcome-card-icon">⚡</span>
          <div className="welcome-card-body">
            <h3>Events</h3>
            <p>Real-time activity stream. See decisions, messages, and approvals from your agents.</p>
          </div>
          <span className="welcome-card-arrow">&rarr;</span>
        </div>

        <div className="welcome-card" onClick={() => navigate("/insights")}>
          <span className="welcome-card-icon">◉</span>
          <div className="welcome-card-body">
            <h3>Insights</h3>
            <p>Knowledge your agents accumulate and share. Searchable memory across all sessions.</p>
          </div>
          <span className="welcome-card-arrow">&rarr;</span>
        </div>
      </div>

      <div className="welcome-footer">
        <p>Select an agent in the sidebar to start a session.</p>
      </div>
    </div>
  );
}
