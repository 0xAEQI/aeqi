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
    desc: "A node in the tree with a name, prompts, and a position. Everything else — departments, teams, specialists — emerges from arrangement.",
  },
  {
    letter: "E",
    word: "Event",
    color: "#c0392b",
    role: "happened",
    desc: "Everything that happens is one row in one table. The audit log is a query. The cost report is a query. The system is its own observability.",
  },
  {
    letter: "Q",
    word: "Quest",
    color: "#c0392b",
    role: "what",
    desc: "A unit of work, decomposed from intent. Dependencies resolve. Agents claim. Results compound.",
  },
  {
    letter: "I",
    word: "Insight",
    color: "#c0392b",
    role: "known",
    desc: "Searchable knowledge that walks the tree. An agent sees its own, its parent's, up to root. Context flows like thought through a nervous system.",
  },
];

export function Process() {
  return (
    <section className="relative z-10 max-w-5xl mx-auto px-8 py-28">
      <motion.div {...fadeUp()} className="text-center mb-20">
        <p
          className="text-[11px] uppercase tracking-[0.25em] text-white/15 mb-6"
          style={{ fontFamily: "'Space Grotesk', sans-serif" }}
        >
          Four Tables
        </p>
        <p className="text-[15px] text-white/30 max-w-md mx-auto leading-relaxed">
          Four primitives. One loop. No schema for what you're building
          — just a tree that can become anything.
        </p>
      </motion.div>

      <div className="grid grid-cols-1 md:grid-cols-4 gap-10 md:gap-6">
        {primitives.map((p, i) => (
          <motion.div key={i} {...fadeUp(0.08 * i)} className="group">
            <div
              className="border-t pt-6 transition-colors duration-500"
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
                <span
                  style={{ color: p.color, textShadow: `0 0 20px ${p.color}30` }}
                >
                  {p.letter}
                </span>
                {p.word.slice(1)}
              </h3>
              <p className="text-[13px] leading-relaxed text-white/25">
                {p.desc}
              </p>
            </div>
          </motion.div>
        ))}
      </div>
    </section>
  );
}
