import { useState, useEffect, useRef } from "react";
import { motion } from "framer-motion";

// ---------------------------------------------------------------------------
// Graph data
// ---------------------------------------------------------------------------

const NODES = [
  { id: "orchestrator", label: "Orchestrator", x: 250, y: 50, primary: true },
  { id: "cto", label: "CTO Agent", x: 120, y: 160 },
  { id: "coo", label: "COO Agent", x: 380, y: 160 },
  { id: "memory", label: "Memory", x: 80, y: 310 },
  { id: "tasks", label: "Tasks", x: 250, y: 310 },
  { id: "middleware", label: "Middleware", x: 420, y: 310 },
];

const EDGES: [string, string][] = [
  ["orchestrator", "cto"],
  ["orchestrator", "coo"],
  ["cto", "memory"],
  ["cto", "tasks"],
  ["coo", "tasks"],
  ["coo", "middleware"],
  ["tasks", "memory"],
  ["tasks", "middleware"],
];

const NODE_MAP = Object.fromEntries(NODES.map((n) => [n.id, n]));

// ---------------------------------------------------------------------------
// Terminal log data
// ---------------------------------------------------------------------------

const LOG_ENTRIES = [
  { time: "00:00:01", event: "daemon started", detail: "patrol interval: 30s" },
  { time: "00:00:02", event: "agent loaded", detail: "CTO (capable tier)" },
  { time: "00:00:02", event: "agent loaded", detail: "COO (balanced tier)" },
  {
    time: "00:00:03",
    event: "trigger fired",
    detail: "memory-consolidation",
  },
  { time: "00:00:04", event: "task created", detail: "sg-048 \u2192 CTO" },
  { time: "00:00:05", event: "middleware", detail: "9 layers attached" },
  {
    time: "00:00:12",
    event: "delegation",
    detail: "CTO \u2192 Backend (async)",
  },
  { time: "00:00:18", event: "task completed", detail: "cost: $0.042" },
  {
    time: "00:00:19",
    event: "memory stored",
    detail: "scope: entity, key: arch-decision",
  },
  {
    time: "00:00:30",
    event: "patrol cycle",
    detail: "3 active, 0 pending",
  },
];

// ---------------------------------------------------------------------------
// SVG Keyframes (injected once)
// ---------------------------------------------------------------------------

const SVG_STYLES = `
  @keyframes dash-flow {
    to {
      stroke-dashoffset: -24;
    }
  }
  @keyframes node-pulse {
    0%, 100% { opacity: 0.5; }
    50% { opacity: 1; }
  }
  @keyframes cursor-blink {
    0%, 49% { opacity: 1; }
    50%, 100% { opacity: 0; }
  }
  @keyframes log-slide {
    from {
      opacity: 0;
      transform: translateX(-6px);
    }
    to {
      opacity: 1;
      transform: translateX(0);
    }
  }
`;

// ---------------------------------------------------------------------------
// SystemGraph
// ---------------------------------------------------------------------------

function SystemGraph() {
  return (
    <svg viewBox="0 0 500 400" className="w-full h-auto">
      <defs>
        <linearGradient id="orch-grad" x1="0" y1="0" x2="1" y2="1">
          <stop offset="0%" stopColor="#6366f1" stopOpacity="0.35" />
          <stop offset="100%" stopColor="#2dd4bf" stopOpacity="0.25" />
        </linearGradient>
      </defs>

      {EDGES.map(([from, to], i) => {
        const a = NODE_MAP[from];
        const b = NODE_MAP[to];
        return (
          <g key={`edge-${i}`}>
            <line
              x1={a.x}
              y1={a.y}
              x2={b.x}
              y2={b.y}
              stroke="rgba(255,255,255,0.08)"
              strokeWidth="1"
            />
            <line
              x1={a.x}
              y1={a.y}
              x2={b.x}
              y2={b.y}
              stroke="rgba(255,255,255,0.2)"
              strokeWidth="1"
              strokeDasharray="4 20"
              style={{
                animation: "dash-flow 1.8s linear infinite",
                animationDelay: `${i * 0.25}s`,
              }}
            />
          </g>
        );
      })}

      {NODES.map((node) => (
        <g key={node.id}>
          {node.primary && (
            <circle
              cx={node.x}
              cy={node.y}
              r="14"
              fill="url(#orch-grad)"
              style={{ animation: "node-pulse 3s ease-in-out infinite" }}
            />
          )}
          <circle
            cx={node.x}
            cy={node.y}
            r="4"
            fill={node.primary ? "rgba(255,255,255,0.6)" : "rgba(255,255,255,0.3)"}
            style={
              !node.primary
                ? { animation: "node-pulse 4s ease-in-out infinite", animationDelay: `${Math.random() * 2}s` }
                : undefined
            }
          />
          <text
            x={node.x}
            y={node.y + 20}
            textAnchor="middle"
            fill="rgba(255,255,255,0.35)"
            fontSize="11"
            fontFamily="Inter, system-ui, sans-serif"
            fontWeight="400"
          >
            {node.label}
          </text>
        </g>
      ))}
    </svg>
  );
}

// ---------------------------------------------------------------------------
// LiveTerminal
// ---------------------------------------------------------------------------

function LiveTerminal() {
  const [count, setCount] = useState(0);
  const scrollRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const interval = setInterval(() => {
      setCount((c) => {
        if (c < LOG_ENTRIES.length) return c + 1;
        clearInterval(interval);
        return c;
      });
    }, 500);
    return () => clearInterval(interval);
  }, []);

  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [count]);

  const allVisible = count >= LOG_ENTRIES.length;

  return (
    <div className="border border-white/[0.06] bg-white/[0.03] rounded-xl overflow-hidden">
      <div className="flex items-center gap-2 px-5 py-3 border-b border-white/[0.06]">
        <span className="w-2.5 h-2.5 rounded-full bg-red-500/20" />
        <span className="w-2.5 h-2.5 rounded-full bg-yellow-500/20" />
        <span className="w-2.5 h-2.5 rounded-full bg-green-500/20" />
        <span className="ml-3 font-mono text-[12px] text-white/20 tracking-wider">
          aeqi daemon
        </span>
      </div>

      <div
        ref={scrollRef}
        className="px-5 py-4 max-h-[340px] overflow-y-auto font-mono text-[13px] leading-relaxed"
      >
        {LOG_ENTRIES.slice(0, count).map((entry, i) => (
          <div
            key={i}
            className="flex gap-4 py-1.5"
            style={{
              animation: "log-slide 0.35s ease forwards",
              opacity: 0,
            }}
          >
            <span className="text-white/15 shrink-0 w-[72px]">
              [{entry.time}]
            </span>
            <span className="text-white/35 shrink-0 w-[180px] truncate">
              {entry.event}
            </span>
            <span className="text-white/20">{entry.detail}</span>
          </div>
        ))}
        {allVisible && (
          <div className="flex gap-4 py-1.5 mt-1">
            <span className="text-white/25 font-mono">
              {">"}{" "}
              <span style={{ animation: "cursor-blink 1s step-end infinite" }}>
                _
              </span>
            </span>
          </div>
        )}
      </div>
    </div>
  );
}

// ---------------------------------------------------------------------------
// SystemDemo (exported)
// ---------------------------------------------------------------------------

export function SystemDemo() {
  return (
    <section className="relative z-10 max-w-6xl mx-auto px-8 py-32">
      <style>{SVG_STYLES}</style>

      <motion.p
        initial={{ opacity: 0, y: 20 }}
        whileInView={{ opacity: 1, y: 0 }}
        viewport={{ once: true, margin: "-100px" }}
        transition={{ duration: 0.6 }}
        className="text-[13px] tracking-[0.2em] uppercase text-white/20 mb-6"
      >
        System in Motion
      </motion.p>

      <motion.h2
        initial={{ opacity: 0, y: 20 }}
        whileInView={{ opacity: 1, y: 0 }}
        viewport={{ once: true, margin: "-100px" }}
        transition={{ duration: 0.6, delay: 0.1 }}
        className="text-3xl sm:text-4xl font-light text-white/90 leading-snug mb-16"
      >
        Watch the orchestrator work
      </motion.h2>

      <div className="grid lg:grid-cols-2 gap-10 items-start">
        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true, margin: "-100px" }}
          transition={{ duration: 0.7, delay: 0.15 }}
          className="border border-white/[0.06] bg-white/[0.03] rounded-xl p-6 sm:p-8"
        >
          <p className="text-[12px] tracking-[0.15em] uppercase text-white/20 mb-6">
            Architecture Graph
          </p>
          <SystemGraph />
          <p className="text-[13px] text-white/20 mt-5 leading-relaxed text-center">
            Animated data flow between orchestrator, agents, and infrastructure.
          </p>
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true, margin: "-100px" }}
          transition={{ duration: 0.7, delay: 0.3 }}
        >
          <LiveTerminal />
        </motion.div>
      </div>
    </section>
  );
}

// ---------------------------------------------------------------------------
// CallToAction (exported)
// ---------------------------------------------------------------------------

export function CallToAction() {
  return (
    <>
      <section className="relative z-10 max-w-4xl mx-auto px-8 pt-32 pb-24 text-center">
        <div
          className="pointer-events-none absolute inset-0"
          style={{
            background:
              "radial-gradient(ellipse 60% 50% at 50% 40%, rgba(99,102,241,0.07) 0%, transparent 70%)",
          }}
        />

        <motion.p
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true, margin: "-80px" }}
          transition={{ duration: 0.6 }}
          className="relative text-[13px] tracking-[0.3em] uppercase text-white/20 mb-8"
        >
          Ready?
        </motion.p>

        <motion.h2
          initial={{ opacity: 0, y: 24 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true, margin: "-80px" }}
          transition={{ duration: 0.7, delay: 0.1 }}
          className="relative text-4xl sm:text-5xl md:text-6xl font-light tracking-tight leading-[1.15] mb-6"
        >
          <span className="text-white/90">Start running your company</span>
          <br />
          <span className="bg-gradient-to-r from-indigo-500 to-teal-400 bg-clip-text text-transparent">
            like software
          </span>
        </motion.h2>

        <motion.p
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true, margin: "-80px" }}
          transition={{ duration: 0.6, delay: 0.2 }}
          className="relative text-[15px] text-white/30 mb-12"
        >
          Free tier. No credit card. Deploy in minutes.
        </motion.p>

        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true, margin: "-80px" }}
          transition={{ duration: 0.6, delay: 0.3 }}
          className="relative flex flex-col items-center gap-5"
        >
          <a
            href="https://app.aeqi.ai"
            className="inline-block px-10 py-5 text-lg font-medium rounded-xl text-white bg-gradient-to-r from-indigo-500 to-teal-400 hover:scale-[1.03] active:scale-[0.98] transition-transform duration-200 shadow-[0_0_40px_rgba(99,102,241,0.2)]"
          >
            Get Started Free
          </a>
          <a
            href="https://github.com/0xAEQI/aeqi/blob/main/docs/architecture.md"
            className="text-[14px] text-white/20 hover:text-white/40 transition-colors duration-200"
          >
            or explore the docs &rarr;
          </a>
        </motion.div>
      </section>

      <footer className="relative z-10 max-w-5xl mx-auto px-8 pb-16 pt-8">
        <div className="h-px bg-white/[0.04] mb-8" />
        <div className="flex items-center justify-between">
          <span className="text-[13px] text-white/15">aeqi.ai</span>
          <div className="flex gap-6 text-[13px] text-white/15">
            <a
              href="https://github.com/0xAEQI/aeqi"
              className="hover:text-white/30 transition-colors duration-200"
            >
              GitHub
            </a>
            <a
              href="https://github.com/0xAEQI/aeqi/blob/main/docs/architecture.md"
              className="hover:text-white/30 transition-colors duration-200"
            >
              Docs
            </a>
            <a
              href="#"
              className="hover:text-white/30 transition-colors duration-200"
            >
              Terms
            </a>
            <a
              href="#"
              className="hover:text-white/30 transition-colors duration-200"
            >
              Privacy
            </a>
          </div>
        </div>
      </footer>
    </>
  );
}
