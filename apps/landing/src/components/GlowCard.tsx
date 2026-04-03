"use client";

import { useRef, useCallback } from "react";

interface GlowCardProps {
  children: React.ReactNode;
  className?: string;
  glowColor?: string;
  glowSize?: number;
}

export function GlowCard({
  children,
  className = "",
  glowColor = "rgba(99, 102, 241, 0.15)",
  glowSize = 200,
}: GlowCardProps) {
  const cardRef = useRef<HTMLDivElement>(null);
  const glowRef = useRef<HTMLDivElement>(null);

  const handleMouseMove = useCallback(
    (e: React.MouseEvent<HTMLDivElement>) => {
      const card = cardRef.current;
      if (!card) return;
      const rect = card.getBoundingClientRect();
      const x = e.clientX - rect.left;
      const y = e.clientY - rect.top;
      card.style.setProperty("--glow-x", `${x}px`);
      card.style.setProperty("--glow-y", `${y}px`);
    },
    [],
  );

  const handleMouseEnter = useCallback(() => {
    if (glowRef.current) {
      glowRef.current.style.opacity = "1";
    }
  }, []);

  const handleMouseLeave = useCallback(() => {
    if (glowRef.current) {
      glowRef.current.style.opacity = "0";
    }
  }, []);

  return (
    <div
      ref={cardRef}
      onMouseMove={handleMouseMove}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      className={`relative overflow-hidden rounded-2xl border border-white/[0.06] bg-white/[0.02] transition-colors duration-300 hover:border-white/[0.1] ${className}`}
    >
      <div
        ref={glowRef}
        style={{
          background: `radial-gradient(circle ${glowSize}px at var(--glow-x, 50%) var(--glow-y, 50%), ${glowColor}, transparent)`,
          opacity: 0,
          transition: "opacity 300ms ease",
        }}
        className="pointer-events-none absolute inset-0 rounded-[inherit]"
      />
      <div className="relative z-10">{children}</div>
    </div>
  );
}

interface GlowCardGridProps {
  children: React.ReactNode;
  className?: string;
  columns?: number;
}

export function GlowCardGrid({
  children,
  className = "",
  columns = 3,
}: GlowCardGridProps) {
  return (
    <div
      className={`grid grid-cols-1 gap-6 md:grid-cols-2 ${className}`}
      style={{
        "--lg-cols": columns,
      } as React.CSSProperties}
    >
      <style>{`
        @media (min-width: 1024px) {
          [style*="--lg-cols"] {
            grid-template-columns: repeat(var(--lg-cols), minmax(0, 1fr)) !important;
          }
        }
      `}</style>
      {children}
    </div>
  );
}
