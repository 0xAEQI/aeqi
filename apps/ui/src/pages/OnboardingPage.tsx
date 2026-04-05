import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { api } from "@/lib/api";
import { useAuthStore } from "@/store/auth";

const AGENT_TEMPLATES = [
  { name: "Engineer", desc: "Writes code, reviews PRs, fixes bugs", template: "agents/engineer" },
  { name: "Researcher", desc: "Gathers context, compares options, synthesizes findings", template: "agents/researcher" },
  { name: "Designer", desc: "UI/UX, dashboards, landing pages, visual polish", template: "agents/designer" },
  { name: "Reviewer", desc: "Catches regressions, verifies quality, blocks bad merges", template: "agents/reviewer" },
];

export default function OnboardingPage() {
  const navigate = useNavigate();
  const fetchMe = useAuthStore((s) => s.fetchMe);
  const [step, setStep] = useState(1);
  const [companyName, setCompanyName] = useState("");
  const [tagline, setTagline] = useState("");
  const [selectedAgent, setSelectedAgent] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");

  const handleCreateCompany = async () => {
    if (!companyName.trim() || loading) return;
    setLoading(true);
    setError("");
    try {
      await api.createCompany({ name: companyName.trim(), tagline: tagline.trim() || undefined });
      setStep(2);
    } catch (e: any) {
      setError(e?.message || "Failed to create company");
    }
    setLoading(false);
  };

  const handleHireAgent = async () => {
    if (loading) return;
    setLoading(true);
    if (selectedAgent) {
      try {
        await api.spawnAgent({ template: selectedAgent, project: companyName.trim() });
      } catch {
        // Non-critical — agent can be hired later.
      }
    }
    await fetchMe();
    setStep(3);
    setLoading(false);
  };

  const handleFinish = () => {
    navigate("/", { replace: true });
  };

  return (
    <div className="auth-page">
      <div className="auth-container" style={{ maxWidth: 420 }}>
        {/* Progress dots */}
        <div style={{ display: "flex", justifyContent: "center", gap: 8, marginBottom: 40 }}>
          {[1, 2, 3].map((s) => (
            <div
              key={s}
              style={{
                width: 8,
                height: 8,
                borderRadius: "50%",
                background: s <= step ? "rgba(0,0,0,0.85)" : "rgba(0,0,0,0.1)",
                transition: "background 0.3s",
              }}
            />
          ))}
        </div>

        {step === 1 && (
          <>
            <h1 className="auth-heading">Create your company</h1>
            <p className="auth-subheading">A company is a workspace where your agents operate.</p>
            <div className="auth-form">
              <input
                className="auth-input"
                type="text"
                placeholder="Company name"
                value={companyName}
                onChange={(e) => setCompanyName(e.target.value)}
                onKeyDown={(e) => { if (e.key === "Enter" && companyName.trim()) handleCreateCompany(); }}
                autoFocus
              />
              <input
                className="auth-input"
                type="text"
                placeholder="Tagline (optional)"
                value={tagline}
                onChange={(e) => setTagline(e.target.value)}
                onKeyDown={(e) => { if (e.key === "Enter" && companyName.trim()) handleCreateCompany(); }}
              />
              {error && <div className="auth-error">{error}</div>}
              <button
                className="auth-btn-primary"
                onClick={handleCreateCompany}
                disabled={!companyName.trim() || loading}
              >
                {loading ? "Creating..." : "Continue"}
              </button>
            </div>
          </>
        )}

        {step === 2 && (
          <>
            <h1 className="auth-heading">Hire your first agent</h1>
            <p className="auth-subheading">Pick a role to get started. You can add more later.</p>
            <div style={{ display: "flex", flexDirection: "column", gap: 8, marginBottom: 24 }}>
              {AGENT_TEMPLATES.map((t) => (
                <button
                  key={t.name}
                  onClick={() => setSelectedAgent(selectedAgent === t.template ? null : t.template)}
                  style={{
                    display: "flex",
                    flexDirection: "column",
                    gap: 2,
                    padding: "14px 16px",
                    background: selectedAgent === t.template ? "rgba(0,0,0,0.04)" : "transparent",
                    border: `1px solid ${selectedAgent === t.template ? "rgba(0,0,0,0.2)" : "rgba(0,0,0,0.08)"}`,
                    borderRadius: 12,
                    cursor: "pointer",
                    textAlign: "left",
                    transition: "all 0.15s",
                    fontFamily: "var(--font-sans)",
                  }}
                >
                  <span style={{ fontSize: 14, fontWeight: 500, color: "rgba(0,0,0,0.85)" }}>{t.name}</span>
                  <span style={{ fontSize: 12, color: "rgba(0,0,0,0.4)" }}>{t.desc}</span>
                </button>
              ))}
            </div>
            <button
              className="auth-btn-primary"
              onClick={handleHireAgent}
              disabled={loading}
            >
              {loading ? "Setting up..." : selectedAgent ? "Hire & continue" : "Skip for now"}
            </button>
          </>
        )}

        {step === 3 && (
          <>
            <div style={{ fontSize: 48, textAlign: "center", marginBottom: 24 }}>
              <span role="img" aria-label="done">&#x2713;</span>
            </div>
            <h1 className="auth-heading">You're all set</h1>
            <p className="auth-subheading">
              {companyName} is ready. Assign work, chat with agents, and watch them operate.
            </p>
            <button className="auth-btn-primary" onClick={handleFinish}>
              Open dashboard
            </button>
          </>
        )}
      </div>
    </div>
  );
}
