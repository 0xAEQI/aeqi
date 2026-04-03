import { useRef, useEffect, useCallback } from "react";

interface ParticleFieldProps {
  scrollProgress: number;
}

interface Particle {
  x: number;
  y: number;
  vx: number;
  vy: number;
  baseVx: number;
  baseVy: number;
  radius: number;
  opacity: number;
}

interface MouseState {
  x: number;
  y: number;
  active: boolean;
  influence: number;
}

function lerp(a: number, b: number, t: number): number {
  return a + (b - a) * t;
}

function clamp(v: number, min: number, max: number): number {
  return v < min ? min : v > max ? max : v;
}

function createParticles(width: number, height: number, count: number): Particle[] {
  const particles: Particle[] = [];
  for (let i = 0; i < count; i++) {
    const speed = 0.15 + Math.random() * 0.35;
    const angle = Math.random() * Math.PI * 2;
    const vx = Math.cos(angle) * speed;
    const vy = Math.sin(angle) * speed;
    particles.push({
      x: Math.random() * width,
      y: Math.random() * height,
      vx,
      vy,
      baseVx: vx,
      baseVy: vy,
      radius: 1 + Math.random(),
      opacity: 0.15 + Math.random() * 0.15,
    });
  }
  return particles;
}

export function ParticleField({ scrollProgress }: ParticleFieldProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const particlesRef = useRef<Particle[]>([]);
  const mouseRef = useRef<MouseState>({ x: 0, y: 0, active: false, influence: 0 });
  const frameRef = useRef<number>(0);
  const scrollRef = useRef(scrollProgress);

  scrollRef.current = scrollProgress;

  const getParticleCount = useCallback(() => {
    return window.innerWidth < 768 ? 80 : 150;
  }, []);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d", { alpha: true });
    if (!ctx) return;

    let width = 0;
    let height = 0;
    let dpr = 1;

    function resize() {
      dpr = window.devicePixelRatio || 1;
      width = window.innerWidth;
      height = window.innerHeight;
      canvas!.width = width * dpr;
      canvas!.height = height * dpr;
      canvas!.style.width = width + "px";
      canvas!.style.height = height + "px";
      ctx!.setTransform(dpr, 0, 0, dpr, 0, 0);

      const count = getParticleCount();
      const current = particlesRef.current;
      if (current.length === 0) {
        particlesRef.current = createParticles(width, height, count);
      } else if (current.length !== count) {
        if (count > current.length) {
          while (particlesRef.current.length < count) {
            const speed = 0.15 + Math.random() * 0.35;
            const angle = Math.random() * Math.PI * 2;
            const vx = Math.cos(angle) * speed;
            const vy = Math.sin(angle) * speed;
            particlesRef.current.push({
              x: Math.random() * width,
              y: Math.random() * height,
              vx, vy, baseVx: vx, baseVy: vy,
              radius: 1 + Math.random(),
              opacity: 0.15 + Math.random() * 0.15,
            });
          }
        } else {
          particlesRef.current.length = count;
        }
      }
    }

    const observer = new ResizeObserver(resize);
    observer.observe(document.documentElement);
    resize();

    function handleMouseMove(e: MouseEvent) {
      mouseRef.current.x = e.clientX;
      mouseRef.current.y = e.clientY;
      mouseRef.current.active = true;
    }

    function handleMouseLeave() {
      mouseRef.current.active = false;
    }

    window.addEventListener("mousemove", handleMouseMove);
    window.addEventListener("mouseleave", handleMouseLeave);

    function tick() {
      const particles = particlesRef.current;
      const mouse = mouseRef.current;
      const scroll = scrollRef.current;

      mouse.influence += (mouse.active ? 1 : 0 - mouse.influence) * 0.04;
      mouse.influence = clamp(mouse.influence, 0, 1);

      const slowdownFactor = scroll > 0.7 ? lerp(1, 0.4, (scroll - 0.7) / 0.3) : 1;
      const connectionBoost = scroll > 0.3 && scroll < 0.7
        ? lerp(1, 1.6, (scroll - 0.3) / 0.4)
        : scroll >= 0.7 ? 1.6 : 1;
      const tintAmount = scroll < 0.3 ? scroll / 0.3 : 1;

      for (let i = 0; i < particles.length; i++) {
        const p = particles[i];

        p.vx += (p.baseVx - p.vx) * 0.01;
        p.vy += (p.baseVy - p.vy) * 0.01;

        if (mouse.influence > 0.01) {
          const dx = mouse.x - p.x;
          const dy = mouse.y - p.y;
          const dist = Math.sqrt(dx * dx + dy * dy);
          if (dist < 200 && dist > 1) {
            const force = (1 - dist / 200) * 0.008 * mouse.influence;
            p.vx += (dx / dist) * force;
            p.vy += (dy / dist) * force;
          }
        }

        p.vx *= slowdownFactor;
        p.vy *= slowdownFactor;

        p.x += p.vx;
        p.y += p.vy;

        if (p.x < -10) p.x += width + 20;
        else if (p.x > width + 10) p.x -= width + 20;
        if (p.y < -10) p.y += height + 20;
        else if (p.y > height + 10) p.y -= height + 20;
      }

      ctx!.clearRect(0, 0, width, height);

      for (let i = 0; i < particles.length; i++) {
        for (let j = i + 1; j < particles.length; j++) {
          const a = particles[i];
          const b = particles[j];
          const dx = a.x - b.x;
          if (dx > 120 || dx < -120) continue;
          const dy = a.y - b.y;
          if (dy > 120 || dy < -120) continue;
          const dist = Math.sqrt(dx * dx + dy * dy);
          if (dist > 120) continue;
          const alpha = (1 - dist / 120) * 0.08 * connectionBoost;
          const r = Math.round(lerp(255, 99, tintAmount * 0.3));
          const g = Math.round(lerp(255, 102, tintAmount * 0.3));
          const b2 = Math.round(lerp(255, 241, tintAmount * 0.3));
          ctx!.beginPath();
          ctx!.moveTo(a.x, a.y);
          ctx!.lineTo(b.x, b.y);
          ctx!.strokeStyle = `rgba(${r},${g},${b2},${alpha})`;
          ctx!.lineWidth = 0.5;
          ctx!.stroke();
        }
      }

      if (mouse.influence > 0.01) {
        for (let i = 0; i < particles.length; i++) {
          const p = particles[i];
          const dx = mouse.x - p.x;
          const dy = mouse.y - p.y;
          const dist = Math.sqrt(dx * dx + dy * dy);
          if (dist > 150) continue;
          const alpha = (1 - dist / 150) * 0.12 * mouse.influence;
          const grad = ctx!.createLinearGradient(mouse.x, mouse.y, p.x, p.y);
          grad.addColorStop(0, `rgba(99,102,241,${alpha})`);
          grad.addColorStop(1, `rgba(45,212,191,${alpha})`);
          ctx!.beginPath();
          ctx!.moveTo(mouse.x, mouse.y);
          ctx!.lineTo(p.x, p.y);
          ctx!.strokeStyle = grad;
          ctx!.lineWidth = 0.8;
          ctx!.stroke();
        }
      }

      for (let i = 0; i < particles.length; i++) {
        const p = particles[i];
        const r = Math.round(lerp(255, 99, tintAmount * 0.25));
        const g = Math.round(lerp(255, 102, tintAmount * 0.25));
        const b = Math.round(lerp(255, 241, tintAmount * 0.25));
        ctx!.beginPath();
        ctx!.arc(p.x, p.y, p.radius, 0, Math.PI * 2);
        ctx!.fillStyle = `rgba(${r},${g},${b},${p.opacity})`;
        ctx!.fill();
      }

      frameRef.current = requestAnimationFrame(tick);
    }

    frameRef.current = requestAnimationFrame(tick);

    return () => {
      cancelAnimationFrame(frameRef.current);
      observer.disconnect();
      window.removeEventListener("mousemove", handleMouseMove);
      window.removeEventListener("mouseleave", handleMouseLeave);
    };
  }, [getParticleCount]);

  return (
    <canvas
      ref={canvasRef}
      style={{
        position: "fixed",
        inset: 0,
        zIndex: 0,
        pointerEvents: "none",
        width: "100%",
        height: "100%",
      }}
    />
  );
}
