import { motion } from "framer-motion";
import { ChevronDown } from "lucide-react";

const fadeUp = {
  hidden: { opacity: 0, y: 24 },
  visible: (delay: number) => ({
    opacity: 1,
    y: 0,
    transition: {
      duration: 0.8,
      ease: [0.25, 0.4, 0.25, 1] as [number, number, number, number],
      delay,
    },
  }),
};

function AuroraBackground() {
  return (
    <>
      <style>{`
        @keyframes aurora-drift-1 {
          0%, 100% {
            transform: translate(0%, 0%) scale(1);
          }
          25% {
            transform: translate(5%, -8%) scale(1.1);
          }
          50% {
            transform: translate(-3%, 4%) scale(0.95);
          }
          75% {
            transform: translate(7%, 2%) scale(1.05);
          }
        }

        @keyframes aurora-drift-2 {
          0%, 100% {
            transform: translate(0%, 0%) scale(1);
          }
          25% {
            transform: translate(-6%, 5%) scale(1.08);
          }
          50% {
            transform: translate(4%, -6%) scale(0.92);
          }
          75% {
            transform: translate(-2%, 8%) scale(1.12);
          }
        }

        @keyframes aurora-drift-3 {
          0%, 100% {
            transform: translate(0%, 0%) scale(1.05);
          }
          25% {
            transform: translate(8%, 3%) scale(0.95);
          }
          50% {
            transform: translate(-5%, -7%) scale(1.1);
          }
          75% {
            transform: translate(3%, -4%) scale(1);
          }
        }

        @keyframes aurora-drift-4 {
          0%, 100% {
            transform: translate(0%, 0%) scale(0.95);
          }
          25% {
            transform: translate(-4%, -6%) scale(1.05);
          }
          50% {
            transform: translate(6%, 3%) scale(1);
          }
          75% {
            transform: translate(-7%, 5%) scale(1.08);
          }
        }

        .aurora-layer-1 {
          animation: aurora-drift-1 20s ease-in-out infinite;
        }
        .aurora-layer-2 {
          animation: aurora-drift-2 25s ease-in-out infinite;
        }
        .aurora-layer-3 {
          animation: aurora-drift-3 18s ease-in-out infinite;
        }
        .aurora-layer-4 {
          animation: aurora-drift-4 22s ease-in-out infinite;
        }
      `}</style>
      <div className="absolute inset-0 overflow-hidden">
        <div
          className="aurora-layer-1 absolute w-[80%] h-[70%] top-[5%] left-[10%]"
          style={{
            background:
              "radial-gradient(ellipse at 50% 50%, rgba(99, 102, 241, 0.18) 0%, transparent 70%)",
          }}
        />
        <div
          className="aurora-layer-2 absolute w-[70%] h-[60%] top-[15%] left-[25%]"
          style={{
            background:
              "radial-gradient(ellipse at 40% 60%, rgba(45, 212, 191, 0.15) 0%, transparent 65%)",
          }}
        />
        <div
          className="aurora-layer-3 absolute w-[90%] h-[80%] top-[-5%] left-[-5%]"
          style={{
            background:
              "radial-gradient(ellipse at 60% 30%, rgba(139, 92, 246, 0.12) 0%, transparent 60%)",
          }}
        />
        <div
          className="aurora-layer-4 absolute w-[60%] h-[50%] top-[25%] left-[5%]"
          style={{
            background:
              "radial-gradient(ellipse at 30% 70%, rgba(99, 102, 241, 0.1) 0%, transparent 55%)",
          }}
        />
        <div
          className="absolute inset-0"
          style={{
            background:
              "radial-gradient(ellipse at 50% 100%, rgba(10, 10, 10, 1) 0%, transparent 60%)",
          }}
        />
      </div>
    </>
  );
}

function ScrollIndicator() {
  return (
    <motion.div
      className="absolute bottom-8 left-1/2 -translate-x-1/2 flex flex-col items-center gap-2"
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ delay: 2, duration: 1 }}
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
      <AuroraBackground />

      <div className="relative z-10 flex flex-col items-center text-center px-6">
        <motion.p
          className="text-[11px] tracking-[0.3em] uppercase text-white/25 font-medium mb-10"
          variants={fadeUp}
          initial="hidden"
          animate="visible"
          custom={0}
        >
          Persistent Agent Orchestration
        </motion.p>

        <motion.h1
          className="text-5xl sm:text-6xl md:text-7xl font-light tracking-tight leading-[1.08] mb-8"
          variants={fadeUp}
          initial="hidden"
          animate="visible"
          custom={0.15}
        >
          <span className="text-white/90">Run your company</span>
          <br />
          <span className="bg-gradient-to-r from-indigo-400 to-teal-400 bg-clip-text text-transparent">
            like software
          </span>
        </motion.h1>

        <motion.p
          className="text-lg text-white/35 max-w-xl leading-relaxed mb-12"
          variants={fadeUp}
          initial="hidden"
          animate="visible"
          custom={0.3}
        >
          Agents that remember. Departments that coordinate.
          Triggers that execute. One Rust runtime, infinite scale.
        </motion.p>

        <motion.div
          className="flex flex-wrap items-center justify-center gap-4 mb-16"
          variants={fadeUp}
          initial="hidden"
          animate="visible"
          custom={0.45}
        >
          <a
            href="https://app.aeqi.ai"
            className="relative px-8 py-4 rounded-lg text-[14px] font-medium text-white overflow-hidden transition-transform duration-200 hover:scale-[1.04] active:scale-[0.98]"
          >
            <span className="absolute inset-0 bg-gradient-to-r from-indigo-500 to-teal-400 rounded-lg" />
            <span className="relative z-10">Start Free</span>
          </a>
          <a
            href="https://docs.aeqi.ai"
            className="px-8 py-4 rounded-lg text-[14px] font-medium text-white/50 border border-white/15 transition-colors duration-200 hover:text-white/80 hover:border-white/25"
          >
            View Documentation
          </a>
        </motion.div>

        <motion.div
          className="flex items-center gap-3 font-mono text-[12px] text-white/20"
          variants={fadeUp}
          initial="hidden"
          animate="visible"
          custom={0.6}
        >
          <span>5,800+ Graph Nodes</span>
          <span className="text-white/10">&#183;</span>
          <span>634 Tests</span>
          <span className="text-white/10">&#183;</span>
          <span>9 Middleware Layers</span>
        </motion.div>
      </div>

      <ScrollIndicator />
    </section>
  );
}
