import { useState } from "react";
import { motion } from "framer-motion";

/* ─── Fade-in helper ─── */
const fade = (delay = 0) => ({
  initial: { opacity: 0, y: 8 } as const,
  animate: { opacity: 1, y: 0 } as const,
  transition: { duration: 0.5, ease: "easeOut" as const, delay },
});

const fadeView = (delay = 0) => ({
  initial: { opacity: 0, y: 12 } as const,
  whileInView: { opacity: 1, y: 0 } as const,
  viewport: { once: true, margin: "-60px" } as const,
  transition: { duration: 0.5, ease: "easeOut" as const, delay },
});

/* ─── Nav ─── */
function Nav() {
  return (
    <motion.nav
      className="fixed top-0 left-0 right-0 z-50 backdrop-blur-lg bg-white/80 border-b border-black/5"
      {...fade(0.1)}
    >
      <div className="max-w-5xl mx-auto px-6 h-14 flex items-center justify-between">
        <a href="/" className="text-[18px] font-semibold tracking-tight text-black">
          aeqi
        </a>
        <div className="flex items-center gap-5">
          <a
            href="https://github.com/0xAEQI/aeqi/blob/main/docs/architecture.md"
            className="text-[14px] text-black/40 hover:text-black/70 transition-colors"
          >
            docs
          </a>
          <a
            href="https://github.com/0xAEQI/aeqi"
            className="text-[14px] text-black/40 hover:text-black/70 transition-colors"
          >
            github
          </a>
          <a
            href="https://app.aeqi.ai"
            className="bg-black text-white rounded-full px-5 py-2 text-[14px] font-medium hover:bg-black/85 transition-colors"
          >
            Get Started
          </a>
        </div>
      </div>
    </motion.nav>
  );
}

/* ─── Hero ─── */
function Hero() {
  const [copied, setCopied] = useState(false);

  const copy = () => {
    navigator.clipboard.writeText("cargo install aeqi");
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <section className="pt-36 pb-24 px-6">
      <div className="max-w-3xl mx-auto text-center">
        <motion.h1
          className="text-5xl md:text-7xl font-bold tracking-tight text-black leading-[1.08]"
          {...fade(0.2)}
        >
          aeqi
        </motion.h1>

        <motion.p
          className="mt-5 text-lg md:text-xl text-black/40 tracking-wide"
          {...fade(0.35)}
        >
          agent orchestration kernel
        </motion.p>

        <motion.p
          className="mt-8 text-[15px] text-black/30 tracking-[0.04em]"
          {...fade(0.5)}
        >
          agent&ensp;&middot;&ensp;event&ensp;&middot;&ensp;quest&ensp;&middot;&ensp;insight
        </motion.p>

        <motion.div className="mt-10" {...fade(0.6)}>
          <button
            onClick={copy}
            className="group inline-flex items-center gap-3 bg-black/[0.03] hover:bg-black/[0.06] rounded-lg px-5 py-3 transition-colors cursor-pointer"
          >
            <code className="text-[14px] font-mono text-black/50">
              <span className="text-black/25 select-none">$&nbsp;</span>
              cargo install aeqi
            </code>
            <span className="text-[12px] text-black/20 group-hover:text-black/40 transition-colors">
              {copied ? "copied" : "copy"}
            </span>
          </button>
        </motion.div>

        <motion.div className="mt-8" {...fade(0.7)}>
          <a
            href="https://app.aeqi.ai"
            className="inline-block bg-black text-white rounded-full px-7 py-2.5 text-[15px] font-medium hover:bg-black/85 transition-colors"
          >
            Get Started
          </a>
        </motion.div>
      </div>
    </section>
  );
}

/* ─── Features: The four primitives ─── */
const primitives = [
  {
    name: "Agent",
    role: "who",
    desc: "A node in a tree. Teams, specialists, departments — patterns that emerge from how agents arrange themselves.",
  },
  {
    name: "Event",
    role: "happened",
    desc: "One row, one table. Audit log, cost report, session transcript — all queries on the same stream.",
  },
  {
    name: "Quest",
    role: "what",
    desc: "A unit of work decomposed from intent. Dependencies resolve. Agents claim. Results compound.",
  },
  {
    name: "Insight",
    role: "known",
    desc: "Knowledge that walks the tree. Each agent sees its own, its parent's, up to root.",
  },
];

function Features() {
  return (
    <section className="py-24 px-6 border-t border-black/5">
      <div className="max-w-5xl mx-auto">
        <motion.div {...fadeView()} className="text-center mb-16">
          <p className="text-[11px] uppercase tracking-[0.25em] text-black/25 mb-4">
            Four Tables
          </p>
          <p className="text-[15px] text-black/40 max-w-sm mx-auto leading-relaxed">
            The entire system. No schema for what you're building.
          </p>
        </motion.div>

        <div className="grid grid-cols-1 md:grid-cols-4 gap-10 md:gap-8">
          {primitives.map((p, i) => (
            <motion.div key={p.name} {...fadeView(0.08 * i)}>
              <div className="border-t border-black/10 pt-6">
                <span className="text-[11px] font-mono block mb-2 text-black/25">
                  {p.role}
                </span>
                <h3 className="text-[18px] font-semibold mb-3 text-black">
                  {p.name}
                </h3>
                <p className="text-[13px] leading-relaxed text-black/40">
                  {p.desc}
                </p>
              </div>
            </motion.div>
          ))}
        </div>
      </div>
    </section>
  );
}

/* ─── Schema block ─── */
function Schema() {
  const code = `agents    { id, parent_id, name, model, prompts }
events    { id, agent_id, kind, payload, ts }
quests    { id, agent_id, intent, status, deps }
insights  { id, agent_id, scope, content }`;

  return (
    <section className="py-20 px-6">
      <div className="max-w-2xl mx-auto">
        <motion.div {...fadeView()} className="text-center mb-10">
          <p className="text-[11px] uppercase tracking-[0.25em] text-black/25">
            Schema
          </p>
        </motion.div>
        <motion.pre
          {...fadeView(0.1)}
          className="bg-black/[0.03] border border-black/5 rounded-lg px-6 py-5 font-mono text-[13px] leading-relaxed text-black/50 overflow-x-auto"
        >
          {code}
        </motion.pre>
      </div>
    </section>
  );
}

/* ─── Footer ─── */
function Footer() {
  return (
    <footer className="border-t border-black/5 py-8 px-6">
      <div className="max-w-5xl mx-auto flex items-center justify-between text-[13px] text-black/25">
        <span className="font-medium tracking-tight">aeqi</span>
        <div className="flex items-center gap-5">
          <a
            href="https://github.com/0xAEQI/aeqi"
            className="hover:text-black/50 transition-colors"
          >
            github
          </a>
          <a
            href="https://github.com/0xAEQI/aeqi/blob/main/docs/architecture.md"
            className="hover:text-black/50 transition-colors"
          >
            docs
          </a>
          <span className="text-black/15">open source &middot; rust</span>
        </div>
      </div>
    </footer>
  );
}

/* ─── App ─── */
export default function App() {
  return (
    <div className="min-h-screen bg-white">
      <Nav />
      <Hero />
      <Features />
      <Schema />
      <Footer />
    </div>
  );
}
