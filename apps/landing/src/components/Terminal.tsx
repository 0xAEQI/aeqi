import { motion } from "framer-motion";

const lines = [
  { who: "you", text: "Help me with my codebase" },
  { who: "aeqi", text: "→ spawned engineer (workdir: ./src)" },
  { who: "", text: "" },
  { who: "you", text: "Review my PRs automatically" },
  { who: "aeqi", text: "→ spawned reviewer" },
  { who: "aeqi", text: "→ created trigger: github.pull_request" },
  { who: "", text: "" },
  { who: "you", text: "That reviewer is too strict" },
  { who: "aeqi", text: "→ updated reviewer/prompts" },
];

export function Terminal() {
  return (
    <section className="relative z-10 max-w-2xl mx-auto px-6 py-20">
      <motion.div
        className="rounded-lg border border-white/[0.06] bg-white/[0.02] backdrop-blur-sm overflow-hidden"
        initial={{ opacity: 0, y: 30 }}
        whileInView={{ opacity: 1, y: 0 }}
        viewport={{ once: true, margin: "-80px" }}
        transition={{ duration: 0.7, ease: "easeOut" }}
      >
        {/* Title bar */}
        <div className="flex items-center gap-2 px-4 py-3 border-b border-white/[0.04]">
          <div className="w-[7px] h-[7px] rounded-full bg-white/[0.08]" />
          <div className="w-[7px] h-[7px] rounded-full bg-white/[0.08]" />
          <div className="w-[7px] h-[7px] rounded-full bg-white/[0.08]" />
          <span
            className="ml-2 text-[11px] text-white/25"
            style={{ fontFamily: "'JetBrains Mono', monospace" }}
          >
            aeqi
          </span>
        </div>

        {/* Lines */}
        <div className="px-5 py-4 space-y-1">
          {lines.map((line, i) => {
            if (!line.text) return <div key={i} className="h-3" />;
            return (
              <motion.div
                key={i}
                className="flex gap-3"
                style={{ fontFamily: "'JetBrains Mono', monospace" }}
                initial={{ opacity: 0 }}
                whileInView={{ opacity: 1 }}
                viewport={{ once: true }}
                transition={{ duration: 0.4, delay: 0.1 + i * 0.08 }}
              >
                <span
                  className="text-[11px] w-7 shrink-0 text-right"
                  style={{ color: line.who === "you" ? "rgba(255,255,255,0.5)" : "#c0392b99" }}
                >
                  {line.who}
                </span>
                <span
                  className="text-[12px]"
                  style={{
                    color: line.who === "you" ? "rgba(255,255,255,0.7)" : "rgba(255,255,255,0.35)",
                  }}
                >
                  {line.text}
                </span>
              </motion.div>
            );
          })}
        </div>
      </motion.div>
    </section>
  );
}
