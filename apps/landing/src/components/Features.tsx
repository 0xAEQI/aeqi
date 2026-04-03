import { motion } from "framer-motion";
import {
  Brain,
  GitBranch,
  Shield,
  Zap,
  Wallet,
  Network,
} from "lucide-react";
import type { LucideIcon } from "lucide-react";

const PROBLEMS = [
  "Stateless sessions, no continuity",
  "No coordination between agents",
  "No budget enforcement",
  "No safety middleware",
  "No organizational structure",
];

const SOLUTIONS = [
  "Entity-scoped memory persists forever",
  "Delegation with 5 response modes",
  "Per-agent and per-project budgets",
  "9-layer middleware on every execution",
  "Department hierarchy with escalation",
];

interface Feature {
  icon: LucideIcon;
  title: string;
  description: string;
}

const FEATURES: Feature[] = [
  {
    icon: Brain,
    title: "Persistent Memory",
    description:
      "Entity-scoped knowledge that survives across sessions, projects, and agent lifetimes.",
  },
  {
    icon: GitBranch,
    title: "Department Hierarchy",
    description:
      "Organizational structure with escalation paths, blackboard visibility, and clarification routing.",
  },
  {
    icon: Network,
    title: "Task Delegation",
    description:
      "One unified tool with five response modes. Named agents, departments, subagents.",
  },
  {
    icon: Shield,
    title: "Safety Middleware",
    description:
      "Nine layers of middleware on every execution path. Budget gates, content filters, audit trails.",
  },
  {
    icon: Zap,
    title: "Autonomous Triggers",
    description:
      "Cron schedules, event hooks, webhooks. Agents wake themselves when conditions are met.",
  },
  {
    icon: Wallet,
    title: "Budget Enforcement",
    description:
      "Per-agent and per-project spending limits. Real-time cost tracking. No runaway bills.",
  },
];

function FeatureCard({
  feature,
  index,
}: {
  feature: Feature;
  index: number;
}) {
  const Icon = feature.icon;

  return (
    <motion.div
      initial={{ opacity: 0, y: 30 }}
      whileInView={{ opacity: 1, y: 0 }}
      viewport={{ once: true, margin: "-100px" }}
      transition={{
        duration: 0.6,
        ease: [0.25, 0.46, 0.45, 0.94],
        delay: index * 0.1,
      }}
      className="group bg-white/[0.02] border border-white/[0.06] rounded-xl p-8 hover:bg-white/[0.04] hover:border-white/[0.1] transition-all duration-300"
    >
      <div className="relative mb-6 inline-flex items-center justify-center w-10 h-10 rounded-lg bg-gradient-to-br from-indigo-500/10 to-teal-400/10 border border-white/[0.06] group-hover:shadow-[0_0_20px_rgba(99,102,241,0.15)] transition-shadow duration-300">
        <Icon className="w-5 h-5 text-white/40 group-hover:text-white/60 transition-colors duration-300" />
      </div>
      <h3 className="text-[15px] font-medium text-white/60 mb-3">
        {feature.title}
      </h3>
      <p className="text-[14px] text-white/25 leading-relaxed">
        {feature.description}
      </p>
    </motion.div>
  );
}

export function Features() {
  return (
    <section className="relative z-10 max-w-5xl mx-auto px-8">
      <div className="py-32">
        <motion.p
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true, margin: "-100px" }}
          transition={{ duration: 0.6 }}
          className="text-[13px] tracking-[0.2em] uppercase text-white/20 mb-6"
        >
          The Problem
        </motion.p>

        <motion.h2
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true, margin: "-100px" }}
          transition={{ duration: 0.6, delay: 0.1 }}
          className="text-3xl sm:text-4xl font-light text-white/90 leading-snug mb-16"
        >
          AI agents forget everything
          <br />
          <span className="text-white/30">between sessions</span>
        </motion.h2>

        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true, margin: "-100px" }}
          transition={{ duration: 0.6, delay: 0.2 }}
          className="grid md:grid-cols-2 gap-16 text-[15px] leading-relaxed"
        >
          <div>
            <p className="text-[12px] tracking-[0.15em] uppercase text-white/20 mb-5">
              Typical Agent Frameworks
            </p>
            <ul className="space-y-3">
              {PROBLEMS.map((item) => (
                <li key={item} className="text-white/25 flex items-start gap-3">
                  <span className="mt-[7px] block w-1 h-1 rounded-full bg-white/15 shrink-0" />
                  {item}
                </li>
              ))}
            </ul>
          </div>
          <div>
            <p className="text-[12px] tracking-[0.15em] uppercase mb-5 bg-gradient-to-r from-indigo-500 to-teal-400 bg-clip-text text-transparent">
              AEQI
            </p>
            <ul className="space-y-3">
              {SOLUTIONS.map((item) => (
                <li key={item} className="text-white/50 flex items-start gap-3">
                  <span className="mt-[7px] block w-1 h-1 rounded-full bg-gradient-to-r from-indigo-500 to-teal-400 shrink-0" />
                  {item}
                </li>
              ))}
            </ul>
          </div>
        </motion.div>
      </div>

      <div className="h-px bg-white/[0.05] max-w-4xl mx-auto" />

      <div className="py-32">
        <motion.p
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true, margin: "-100px" }}
          transition={{ duration: 0.6 }}
          className="text-[13px] tracking-[0.2em] uppercase text-white/20 mb-6"
        >
          Capabilities
        </motion.p>

        <motion.h2
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true, margin: "-100px" }}
          transition={{ duration: 0.6, delay: 0.1 }}
          className="text-3xl font-light text-white/90 mb-3"
        >
          Built for production
        </motion.h2>

        <motion.p
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true, margin: "-100px" }}
          transition={{ duration: 0.6, delay: 0.15 }}
          className="text-[15px] text-white/30 mb-16"
        >
          Four primitives. Infinite compositions.
        </motion.p>

        <div className="grid sm:grid-cols-2 lg:grid-cols-3 gap-5">
          {FEATURES.map((feature, i) => (
            <FeatureCard key={feature.title} feature={feature} index={i} />
          ))}
        </div>
      </div>
    </section>
  );
}
