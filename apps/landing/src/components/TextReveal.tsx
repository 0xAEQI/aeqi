"use client";

import { useEffect, useRef, useState } from "react";
import {
  motion,
  useInView,
  useMotionValue,
  useTransform,
  animate,
} from "framer-motion";

interface WordRevealProps {
  text: string;
  className?: string;
  delay?: number;
  stagger?: number;
  once?: boolean;
}

export function WordReveal({
  text,
  className,
  delay = 0,
  stagger = 0.05,
  once = true,
}: WordRevealProps) {
  const words = text.split(" ");

  return (
    <span className={className}>
      {words.map((word, i) => (
        <span
          key={i}
          style={{
            overflow: "hidden",
            display: "inline-block",
            marginRight: "0.25em",
          }}
        >
          <motion.span
            style={{ display: "inline-block" }}
            initial={{ y: "100%", opacity: 0 }}
            whileInView={{ y: 0, opacity: 1 }}
            viewport={{ once }}
            transition={{
              delay: delay + i * stagger,
              duration: 0.5,
              ease: [0.25, 0.4, 0.25, 1],
            }}
          >
            {word}
          </motion.span>
        </span>
      ))}
    </span>
  );
}

interface CharRevealProps {
  text: string;
  className?: string;
  delay?: number;
  stagger?: number;
  once?: boolean;
}

export function CharReveal({
  text,
  className,
  delay = 0,
  stagger = 0.02,
  once = true,
}: CharRevealProps) {
  const chars = text.split("");

  return (
    <span className={className}>
      {chars.map((char, i) => {
        if (char === " ") {
          return (
            <span
              key={i}
              style={{ display: "inline-block", width: "0.3em" }}
            />
          );
        }

        return (
          <span
            key={i}
            style={{ overflow: "hidden", display: "inline-block" }}
          >
            <motion.span
              style={{ display: "inline-block" }}
              initial={{ y: "100%", opacity: 0 }}
              whileInView={{ y: 0, opacity: 1 }}
              viewport={{ once }}
              transition={{
                delay: delay + i * stagger,
                duration: 0.5,
                ease: [0.25, 0.4, 0.25, 1],
              }}
            >
              {char}
            </motion.span>
          </span>
        );
      })}
    </span>
  );
}

interface CountUpProps {
  value: number;
  suffix?: string;
  prefix?: string;
  duration?: number;
  className?: string;
  once?: boolean;
}

export function CountUp({
  value,
  suffix = "",
  prefix = "",
  duration = 2,
  className,
  once = true,
}: CountUpProps) {
  const ref = useRef<HTMLSpanElement>(null);
  const isInView = useInView(ref, { once });
  const motionValue = useMotionValue(0);
  const rounded = useTransform(motionValue, (v) =>
    Math.round(v).toLocaleString()
  );
  const [display, setDisplay] = useState("0");

  useEffect(() => {
    if (!isInView) {
      if (!once) {
        motionValue.set(0);
        setDisplay("0");
      }
      return;
    }

    const controls = animate(motionValue, value, {
      duration,
      ease: "easeOut",
    });

    const unsubscribe = rounded.on("change", (v) => setDisplay(v));

    return () => {
      controls.stop();
      unsubscribe();
    };
  }, [isInView, value, duration, motionValue, rounded, once]);

  return (
    <span ref={ref} className={className}>
      {prefix}
      {display}
      {suffix}
    </span>
  );
}
