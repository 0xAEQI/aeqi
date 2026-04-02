import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { useChatStore } from "@/store/chat";
import { api } from "@/lib/api";
import type { PersistentAgent } from "@/lib/types";

export default function AgentNav() {
  const navigate = useNavigate();
  const channel = useChatStore((s) => s.channel);
  const selectedAgent = useChatStore((s) => s.selectedAgent);
  const setSelectedAgent = useChatStore((s) => s.setSelectedAgent);
  const [agents, setAgents] = useState<PersistentAgent[]>([]);

  useEffect(() => {
    const load = () => {
      api
        .getAgents()
        .then((d: any) => {
          const list = d.agents || d.registry || [];
          setAgents(list.filter((a: PersistentAgent) => a.status === "Active" || a.status === "active"));
        })
        .catch(() => {});
    };
    load();
    const interval = setInterval(load, 20000);
    return () => clearInterval(interval);
  }, [channel]);

  const filtered = channel
    ? agents.filter((a) => a.project === channel || !a.project)
    : agents.filter((a) => !a.project);

  return (
    <nav className="agent-nav">
      <div
        className={`agent-row${!selectedAgent ? " active" : ""}`}
        onClick={() => { setSelectedAgent(null); navigate("/"); }}
      >
        AEQI Agent
      </div>

      {filtered.map((agent) => (
        <div
          key={agent.id}
          className={`agent-row${selectedAgent === agent.name ? " active" : ""}`}
          onClick={() => { setSelectedAgent(agent.name); navigate("/"); }}
        >
          {agent.display_name || agent.name}
        </div>
      ))}

      <div className="agent-nav-add" onClick={() => navigate("/agents")}>+</div>
    </nav>
  );
}
