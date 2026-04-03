import { Hero } from "./components/Hero";
import { Features } from "./components/Features";
import { SystemDemo, CallToAction } from "./components/SystemDemo";
import { Pricing } from "./components/Pricing";

function GridOverlay() {
  return (
    <div className="pointer-events-none fixed inset-0 z-0 overflow-hidden">
      <div
        className="absolute inset-0"
        style={{
          backgroundImage: `
            linear-gradient(rgba(255,255,255,0.015) 1px, transparent 1px),
            linear-gradient(90deg, rgba(255,255,255,0.015) 1px, transparent 1px)
          `,
          backgroundSize: "80px 80px",
        }}
      />
    </div>
  );
}

function Nav() {
  return (
    <nav className="fixed top-0 left-0 right-0 z-50 backdrop-blur-md bg-[#0a0a0a]/80 border-b border-white/[0.04]">
      <div className="flex items-center justify-between px-8 py-4 max-w-6xl mx-auto">
        <a
          href="/"
          className="text-[15px] font-medium tracking-[0.15em] text-white/80 hover:text-white transition-colors duration-200"
        >
          AEQI
        </a>
        <div className="flex items-center gap-8 text-[13px] text-white/40">
          <a
            href="#features"
            className="hidden sm:inline hover:text-white/70 transition-colors duration-200"
          >
            Features
          </a>
          <a
            href="#pricing"
            className="hidden sm:inline hover:text-white/70 transition-colors duration-200"
          >
            Pricing
          </a>
          <a
            href="https://github.com/0xAEQI/aeqi"
            className="hover:text-white/70 transition-colors duration-200"
          >
            GitHub
          </a>
          <a
            href="https://github.com/0xAEQI/aeqi/blob/main/docs/architecture.md"
            className="hover:text-white/70 transition-colors duration-200"
          >
            Docs
          </a>
          <a
            href="https://app.aeqi.ai"
            className="px-4 py-1.5 border border-white/10 hover:border-white/20 hover:text-white/70 transition-colors duration-200 rounded-lg"
          >
            Enter
          </a>
        </div>
      </div>
    </nav>
  );
}

function Divider() {
  return (
    <div className="max-w-4xl mx-auto px-8">
      <div className="h-px bg-white/[0.05]" />
    </div>
  );
}

export default function App() {
  return (
    <div className="relative min-h-screen">
      <GridOverlay />
      <Nav />

      <Hero />

      <div id="features">
        <Features />
      </div>

      <Divider />

      <SystemDemo />

      <Divider />

      <div id="pricing">
        <Pricing />
      </div>

      <Divider />

      <CallToAction />
    </div>
  );
}
