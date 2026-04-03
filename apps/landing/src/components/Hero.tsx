import { motion } from "framer-motion";
import { ChevronDown } from "lucide-react";
import { WordReveal, CharReveal, CountUp } from "./TextReveal";

function ScrollIndicator() {
  return (
    <motion.div
      className="absolute bottom-8 left-1/2 -translate-x-1/2 flex flex-col items-center gap-2"
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ delay: 2.5, duration: 1 }}
    >
      <motion.div
        animate={{ y: [0, 6, 0] }}
        transition={{ duration: 2, repeat: Infinity, ease: "easeInOut" }}
      >
        <ChevronDown className="w-4 h-4 text-white/15" strokeWidth={1.5} />
      </motion.div>
      <div className="w-px h-8 bg-gradient-to-b from-white/10 to-transparent" />
    </motion.div>
  );
}

export function Hero() {
  return (
    <section className="relative min-h-screen flex flex-col items-center justify-center overflow-hidden">
      <div className="relative z-10 flex flex-col items-center text-center px-6">
        <div className="mb-10">
          <CharReveal
            text="PERSISTENT AGENT ORCHESTRATION"
            className="text-[11px] tracking-[0.3em] uppercase text-white/25 font-medium"
            delay={0.3}
            stagger={0.015}
          />
        </div>

        <h1 className="text-5xl sm:text-6xl md:text-7xl lg:text-8xl font-light tracking-tight leading-[1.08] mb-8">
          <WordReveal
            text="Run your company"
            className="text-white/90 block"
            delay={0.6}
            stagger={0.08}
          />
          <WordReveal
            text="like software"
            className="bg-gradient-to-r from-indigo-400 to-teal-400 bg-clip-text text-transparent block"
            delay={1.0}
            stagger={0.1}
          />
        </h1>

        <motion.p
          className="text-base sm:text-lg text-white/30 max-w-lg leading-relaxed mb-14"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ delay: 1.6, duration: 1 }}
        >
          Agents that remember. Departments that coordinate.
          <br className="hidden sm:block" />
          Triggers that execute. One Rust runtime, infinite scale.
        </motion.p>

        <motion.div
          className="flex flex-wrap items-center justify-center gap-4 mb-20"
          initial={{ opacity: 0, y: 16 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 2, duration: 0.8, ease: [0.25, 0.4, 0.25, 1] as [number, number, number, number] }}
        >
          <a
            href="https://app.aeqi.ai"
            className="group relative px-10 py-4 rounded-xl text-[14px] font-medium text-white overflow-hidden transition-transform duration-200 hover:scale-[1.05] active:scale-[0.98]"
          >
            <span className="absolute inset-0 bg-gradient-to-r from-indigo-500 to-teal-400 rounded-xl" />
            <span className="absolute inset-0 bg-gradient-to-r from-indigo-400 to-teal-300 rounded-xl opacity-0 group-hover:opacity-100 transition-opacity duration-300" />
            <span className="absolute inset-0 rounded-xl shadow-[0_0_40px_rgba(99,102,241,0.3)] opacity-0 group-hover:opacity-100 transition-opacity duration-300" />
            <span className="relative z-10">Start Free</span>
          </a>
          <a
            href="https://docs.aeqi.ai"
            className="px-10 py-4 rounded-xl text-[14px] font-medium text-white/40 border border-white/10 transition-all duration-300 hover:text-white/80 hover:border-white/25 hover:bg-white/[0.03]"
          >
            View Documentation
          </a>
        </motion.div>

        <motion.div
          className="flex items-center gap-6 sm:gap-10 font-mono text-[13px]"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ delay: 2.2, duration: 0.8 }}
        >
          <div className="text-center">
            <div className="text-white/40 font-medium">
              <CountUp value={5800} suffix="+" />
            </div>
            <div className="text-white/15 text-[11px] mt-1">Graph Nodes</div>
          </div>
          <div className="w-px h-6 bg-white/[0.06]" />
          <div className="text-center">
            <div className="text-white/40 font-medium">
              <CountUp value={634} />
            </div>
            <div className="text-white/15 text-[11px] mt-1">Tests</div>
          </div>
          <div className="w-px h-6 bg-white/[0.06]" />
          <div className="text-center">
            <div className="text-white/40 font-medium">
              <CountUp value={9} />
            </div>
            <div className="text-white/15 text-[11px] mt-1">Middleware Layers</div>
          </div>
        </motion.div>
      </div>

      <ScrollIndicator />
    </section>
  );
}
