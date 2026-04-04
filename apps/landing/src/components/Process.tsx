import { motion } from "framer-motion";

const fadeUp = (delay = 0) => ({
  initial: { opacity: 0, y: 24 } as const,
  whileInView: { opacity: 1, y: 0 } as const,
  viewport: { once: true, margin: "-60px" } as const,
  transition: { duration: 0.6, ease: "easeOut" as const, delay },
});

const primitives = [
  {
    letter: "A",
    word: "Agent",
    color: "#c0392b",
    role: "who",
    desc: "A node in a tree. Everything else — teams, specialists, departments — emerges from how agents arrange themselves.",
  },
  {
    letter: "E",
    word: "Event",
    color: "#c0392b",
    role: "happened",
    desc: "One row, one table. Audit log, cost report, session transcript — all queries on the same stream.",
  },
  {
    letter: "Q",
    word: "Quest",
    color: "#c0392b",
    role: "what",
    desc: "A unit of work decomposed from intent. Dependencies resolve. Agents claim. Results compound.",
  },
  {
    letter: "I",
    word: "Insight",
    color: "#c0392b",
    role: "known",
    desc: "Knowledge that walks the tree. Each agent sees its own, its parent's, up to root.",
  },
];

export function Process() {
  return (
    <section className="relative z-10 max-w-5xl mx-auto px-8 py-24">
      <motion.div {...fadeUp()} className="text-center mb-16">
        <p
          className="text-[11px] uppercase tracking-[0.25em] text-white/15 mb-5"
          style={{ fontFamily: "'Space Grotesk', sans-serif" }}
        >
          Four Tables
        </p>
        <p className="text-[15px] text-white/25 max-w-sm mx-auto leading-relaxed">
          The entire system. No schema for what you're building.
        </p>
      </motion.div>

      <div className="grid grid-cols-1 md:grid-cols-4 gap-10 md:gap-6">
        {primitives.map((p, i) => (
          <motion.div key={i} {...fadeUp(0.08 * i)}>
            <div
              className="border-t pt-6"
              style={{ borderColor: `${p.color}15` }}
            >
              <span
                className="text-[11px] font-mono block mb-3"
                style={{ color: `${p.color}50` }}
              >
                {p.role}
              </span>
              <h3
                className="text-[18px] font-semibold mb-3"
                style={{
                  fontFamily: "'Space Grotesk', sans-serif",
                  color: `${p.color}cc`,
                }}
              >
                <span style={{ color: p.color, textShadow: `0 0 20px ${p.color}30` }}>
                  {p.letter}
                </span>
                {p.word.slice(1)}
              </h3>
              <p className="text-[13px] leading-relaxed text-white/20">
                {p.desc}
              </p>
            </div>
          </motion.div>
        ))}
      </div>
    </section>
  );
}
