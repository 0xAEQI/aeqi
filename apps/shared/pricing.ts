/** Central pricing config. Imported by both landing page and dashboard app. */

export const TRIAL = {
  days: 3,
  companies: 1,
  agents: 3,
  tokens: "3M",
};

export const PLANS = [
  {
    id: "starter" as const,
    name: "Starter",
    price: 29,
    popular: false,
    tagline: "Ship your first autonomous company.",
    desc: "For individuals and small teams getting started with autonomous agents.",
    features: [
      "Up to 2 companies",
      "Up to 10 agents",
      "50M LLM tokens included",
      "On-chain cap table",
      "Economy listing",
      "Bring your own LLM key",
    ],
    /** Short features for the dashboard billing page */
    short: [
      "2 companies",
      "10 agents per company",
      "50M tokens / month",
      "Email support",
    ],
  },
  {
    id: "growth" as const,
    name: "Growth",
    price: 79,
    popular: true,
    tagline: "Run a portfolio. No limits.",
    desc: "For teams that need full autonomy at scale with priority support.",
    features: [
      "Everything in Starter",
      "Unlimited companies",
      "Unlimited agents",
      "500M LLM tokens included",
      "Priority support",
      "Custom agent templates",
    ],
    short: [
      "Unlimited companies",
      "Unlimited agents",
      "500M tokens / month",
      "Priority support",
      "Custom agent templates",
    ],
  },
] as const;

export type PlanId = (typeof PLANS)[number]["id"];
