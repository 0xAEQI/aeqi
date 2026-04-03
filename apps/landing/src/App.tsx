import { useState, useEffect } from "react";
import { ParticleField } from "./components/ParticleField";
import { Hero } from "./components/Hero";
import { Features } from "./components/Features";
import { SystemDemo, CallToAction } from "./components/SystemDemo";
import { Pricing } from "./components/Pricing";

function FilmGrain() {
  return (
    <div
      className="pointer-events-none fixed inset-0 z-[1] opacity-[0.03]"
      style={{
        backgroundImage: `url("data:image/svg+xml,%3Csvg viewBox='0 0 256 256' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='noise'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.9' numOctaves='4' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23noise)'/%3E%3C/svg%3E")`,
        backgroundRepeat: "repeat",
        backgroundSize: "256px 256px",
      }}
    />
  );
}

function Nav() {
  const [scrolled, setScrolled] = useState(false);

  useEffect(() => {
    const onScroll = () => setScrolled(window.scrollY > 50);
    window.addEventListener("scroll", onScroll, { passive: true });
    return () => window.removeEventListener("scroll", onScroll);
  }, []);

  return (
    <nav
      className={`fixed top-0 left-0 right-0 z-50 transition-all duration-500 ${
        scrolled
          ? "backdrop-blur-xl bg-[#0a0a0a]/70 border-b border-white/[0.04]"
          : "bg-transparent"
      }`}
    >
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
            className="hidden sm:inline hover:text-white/70 transition-colors duration-200"
          >
            Docs
          </a>
          <a
            href="https://app.aeqi.ai"
            className="px-5 py-2 rounded-lg text-[13px] font-medium text-white/70 border border-white/10 transition-all duration-300 hover:text-white hover:border-white/25 hover:bg-white/[0.04]"
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
    <div className="max-w-4xl mx-auto px-8 relative z-10">
      <div className="h-px bg-white/[0.05]" />
    </div>
  );
}

export default function App() {
  const [scrollProgress, setScrollProgress] = useState(0);

  useEffect(() => {
    const onScroll = () => {
      const max = document.documentElement.scrollHeight - window.innerHeight;
      setScrollProgress(max > 0 ? window.scrollY / max : 0);
    };
    window.addEventListener("scroll", onScroll, { passive: true });
    return () => window.removeEventListener("scroll", onScroll);
  }, []);

  return (
    <div className="relative min-h-screen">
      <ParticleField scrollProgress={scrollProgress} />
      <FilmGrain />
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
