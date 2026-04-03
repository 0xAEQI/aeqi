import { motion } from "framer-motion";
import { Check } from "lucide-react";
import { GlowCard } from "./GlowCard";
import { WordReveal } from "./TextReveal";

const fadeUp = {
  hidden: { opacity: 0, y: 24 },
  visible: (delay: number) => ({
    opacity: 1,
    y: 0,
    transition: {
      duration: 0.7,
      ease: [0.25, 0.4, 0.25, 1] as [number, number, number, number],
      delay,
    },
  }),
};

interface Tier {
  name: string;
  price: string;
  period?: string;
  subtitle: string;
  features: string[];
  cta: string;
  href: string;
  highlighted?: boolean;
  badge?: string;
}

const tiers: Tier[] = [
  {
    name: "Starter",
    price: "$0",
    period: "/month",
    subtitle: "For exploration",
    features: [
      "10,000 credits/month",
      "2 agents",
      "Community support",
      "Basic memory (7 day retention)",
      "Single project",
    ],
    cta: "Get Started",
    href: "https://app.aeqi.ai",
  },
  {
    name: "Pro",
    price: "$49",
    period: "/month",
    subtitle: "For teams shipping with agents",
    features: [
      "500,000 credits/month",
      "Unlimited agents",
      "Priority support",
      "Persistent memory (unlimited)",
      "Unlimited projects",
      "Department hierarchy",
      "Custom triggers",
    ],
    cta: "Start Free Trial",
    href: "https://app.aeqi.ai/trial",
    highlighted: true,
    badge: "POPULAR",
  },
  {
    name: "Enterprise",
    price: "Custom",
    subtitle: "For organizations at scale",
    features: [
      "Unlimited credits",
      "Dedicated infrastructure",
      "99.9% SLA",
      "SSO / SAML",
      "Audit logs & compliance",
      "Custom middleware",
      "Dedicated support engineer",
    ],
    cta: "Contact Sales",
    href: "mailto:sales@aeqi.ai",
  },
];

function PricingCard({ tier, index }: { tier: Tier; index: number }) {
  const staggerBase = 0.15;
  const delay = tier.highlighted
    ? staggerBase
    : index === 0
      ? staggerBase + 0.08
      : staggerBase + 0.16;

  const cardContent = (
    <div className="p-8 flex flex-col h-full">
      {tier.badge && (
        <div className="absolute -top-3 left-1/2 -translate-x-1/2 z-20">
          <span className="px-3 py-1 text-[10px] font-medium tracking-[0.15em] uppercase bg-gradient-to-r from-indigo-500 to-teal-400 text-white rounded-full">
            {tier.badge}
          </span>
        </div>
      )}

      <div className="mb-6">
        <h3 className="text-[14px] font-medium tracking-wide text-white/50 mb-4">
          {tier.name}
        </h3>
        <div className="flex items-baseline gap-1">
          <span className="text-4xl font-light text-white/90">
            {tier.price}
          </span>
          {tier.period && (
            <span className="text-[14px] text-white/30">{tier.period}</span>
          )}
        </div>
        <p className="text-[13px] text-white/25 mt-2">{tier.subtitle}</p>
      </div>

      <div className="h-px bg-white/[0.06] mb-6" />

      <ul className="flex-1 space-y-3 mb-8">
        {tier.features.map((feature) => (
          <li key={feature} className="flex items-start gap-3">
            <Check
              className={`w-4 h-4 mt-0.5 shrink-0 ${
                tier.highlighted ? "text-teal-400/70" : "text-white/20"
              }`}
              strokeWidth={2}
            />
            <span className="text-[14px] text-white/40 leading-snug">
              {feature}
            </span>
          </li>
        ))}
      </ul>

      {tier.highlighted ? (
        <a
          href={tier.href}
          className="group relative block w-full py-3 rounded-lg text-center text-[13px] font-medium text-white overflow-hidden transition-transform duration-200 hover:scale-[1.03] active:scale-[0.98]"
        >
          <span className="absolute inset-0 bg-gradient-to-r from-indigo-500 to-teal-400 rounded-lg" />
          <span className="absolute inset-0 bg-gradient-to-r from-indigo-400 to-teal-300 rounded-lg opacity-0 group-hover:opacity-100 transition-opacity duration-300" />
          <span className="relative z-10">{tier.cta}</span>
        </a>
      ) : (
        <a
          href={tier.href}
          className="block w-full py-3 rounded-lg text-center text-[13px] font-medium text-white/50 border border-white/10 transition-colors duration-200 hover:text-white/80 hover:border-white/25"
        >
          {tier.cta}
        </a>
      )}
    </div>
  );

  if (tier.highlighted) {
    return (
      <motion.div
        className="relative lg:-mt-4 lg:mb-4"
        variants={fadeUp}
        initial="hidden"
        whileInView="visible"
        viewport={{ once: true, margin: "-60px" }}
        custom={delay}
      >
        <div className="rounded-2xl p-px bg-gradient-to-br from-indigo-500 to-teal-400">
          <GlowCard
            className="shadow-2xl shadow-indigo-500/5"
            glowColor="rgba(45, 212, 191, 0.12)"
          >
            {cardContent}
          </GlowCard>
        </div>
      </motion.div>
    );
  }

  return (
    <motion.div
      variants={fadeUp}
      initial="hidden"
      whileInView="visible"
      viewport={{ once: true, margin: "-60px" }}
      custom={delay}
    >
      <GlowCard>
        {cardContent}
      </GlowCard>
    </motion.div>
  );
}

function FlowDiagram() {
  return (
    <motion.div
      className="flex items-center justify-center gap-3 font-mono text-[13px] text-white/15 mt-8 flex-wrap"
      variants={fadeUp}
      initial="hidden"
      whileInView="visible"
      viewport={{ once: true, margin: "-40px" }}
      custom={0.3}
    >
      <span className="px-3 py-1.5 border border-white/[0.06] rounded bg-white/[0.02]">
        Your Credits
      </span>
      <span className="text-white/10">&rarr;</span>
      <span className="px-3 py-1.5 border border-white/[0.06] rounded bg-white/[0.02]">
        AEQI Runtime
      </span>
      <span className="text-white/10">&rarr;</span>
      <span className="px-3 py-1.5 border border-white/[0.06] rounded bg-white/[0.02]">
        Direct Provider API
      </span>
    </motion.div>
  );
}

function TokenCalculator() {
  return (
    <motion.div
      className="max-w-lg mx-auto mt-24"
      variants={fadeUp}
      initial="hidden"
      whileInView="visible"
      viewport={{ once: true, margin: "-40px" }}
      custom={0.1}
    >
      <h3 className="text-[13px] tracking-[0.15em] uppercase text-white/25 mb-6 text-center">
        Estimate your usage
      </h3>
      <GlowCard glowColor="rgba(99, 102, 241, 0.08)">
        <div className="p-6">
          <div className="space-y-3 font-mono text-[13px] text-white/15">
            <div className="flex items-center justify-between py-1.5 border-b border-white/[0.04]">
              <span>1 credit</span>
              <span className="text-white/25">=</span>
              <span>1 LLM token</span>
            </div>
            <div className="flex items-center justify-between py-1.5 border-b border-white/[0.04]">
              <span>Average agent task</span>
              <span className="text-white/25">&asymp;</span>
              <span>5,000 credits</span>
            </div>
            <div className="flex items-center justify-between py-1.5">
              <span>Pro plan</span>
              <span className="text-white/25">&asymp;</span>
              <span>100 agent tasks/month</span>
            </div>
          </div>
        </div>
      </GlowCard>
    </motion.div>
  );
}

export function Pricing() {
  return (
    <section className="relative z-10 max-w-6xl mx-auto px-6 sm:px-8 py-32">
      <div className="max-w-2xl mx-auto text-center mb-20">
        <motion.p
          className="text-[11px] tracking-[0.2em] uppercase text-white/20 font-medium mb-6"
          variants={fadeUp}
          initial="hidden"
          whileInView="visible"
          viewport={{ once: true, margin: "-40px" }}
          custom={0}
        >
          Credits
        </motion.p>
        <h2 className="text-3xl sm:text-4xl font-light leading-snug mb-6">
          <WordReveal
            text="Pay for what you use"
            className="text-white/90"
            stagger={0.07}
          />
        </h2>
        <motion.p
          className="text-[15px] text-white/30 leading-relaxed max-w-lg mx-auto"
          variants={fadeUp}
          initial="hidden"
          whileInView="visible"
          viewport={{ once: true, margin: "-40px" }}
          custom={0.2}
        >
          AEQI credits map directly to LLM tokens. No middleman markup from
          aggregators. Direct provider pricing, passed through with transparent
          margin.
        </motion.p>
        <FlowDiagram />
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 lg:gap-5 items-start">
        {tiers.map((tier, i) => (
          <PricingCard key={tier.name} tier={tier} index={i} />
        ))}
      </div>

      <TokenCalculator />
    </section>
  );
}
