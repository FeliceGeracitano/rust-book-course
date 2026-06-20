import { ReactNode } from 'react'
import { motion } from 'framer-motion'

// Shared building blocks so every chapter visualization speaks the same visual
// and interaction language. Widgets compose these; they never invent new colors.

export function VizPanel({ title, children }: { title: string; children: ReactNode }) {
  return (
    <section className="mb-6 rounded-2xl border border-edge bg-ink-card p-4">
      <div className="mb-3 flex items-center gap-2">
        <span className="text-sm font-semibold text-paper">{title}</span>
        <span className="rounded-full bg-rust/15 px-2 py-0.5 text-[10px] font-medium uppercase tracking-wider text-crab">
          interactive
        </span>
      </div>
      {children}
    </section>
  )
}

export function Controls({ children }: { children: ReactNode }) {
  return <div className="mb-4 flex flex-wrap items-center gap-2">{children}</div>
}

export function VizButton({
  onClick,
  children,
  disabled,
  tone = 'rust',
  active,
}: {
  onClick: () => void
  children: ReactNode
  disabled?: boolean
  tone?: 'rust' | 'ghost'
  active?: boolean
}) {
  const base =
    'rounded-lg px-3 py-1.5 text-xs font-semibold transition disabled:cursor-not-allowed disabled:opacity-40'
  const tones = {
    rust: 'bg-rust text-white hover:bg-rust-bright',
    ghost: active
      ? 'border border-rust/60 bg-rust/10 text-crab'
      : 'border border-edge text-muted hover:text-paper',
  }
  return (
    <button onClick={onClick} disabled={disabled} className={`${base} ${tones[tone]}`}>
      {children}
    </button>
  )
}

export type BoxTone = 'stack' | 'heap' | 'rust' | 'ok' | 'muted'

const BOX_TONES: Record<BoxTone, string> = {
  stack: 'border-edge bg-ink-soft',
  heap: 'border-crab/40 bg-crab/10',
  rust: 'border-rust/50 bg-rust/10',
  ok: 'border-ok/40 bg-ok/10',
  muted: 'border-edge bg-ink-soft opacity-60',
}

export function Box({
  label,
  value,
  tone = 'stack',
  faded,
  note,
}: {
  label?: string
  value: ReactNode
  tone?: BoxTone
  faded?: boolean
  note?: ReactNode
}) {
  return (
    <motion.div
      layout
      initial={{ opacity: 0, y: 6 }}
      animate={{ opacity: faded ? 0.3 : 1, y: 0 }}
      exit={{ opacity: 0, scale: 0.85 }}
      transition={{ type: 'spring', stiffness: 320, damping: 26 }}
      className={`min-w-[84px] rounded-xl border px-3 py-2 ${BOX_TONES[tone]}`}
    >
      {label && (
        <div className="mb-0.5 font-mono text-[10px] uppercase tracking-wider text-muted">{label}</div>
      )}
      <div className="font-mono text-sm text-paper">{value}</div>
      {note && <div className="mt-1 text-[10px] leading-tight text-muted">{note}</div>}
    </motion.div>
  )
}

export function Lane({ title, children }: { title: string; children: ReactNode }) {
  return (
    <div className="flex-1">
      <div className="mb-2 text-[10px] font-semibold uppercase tracking-wider text-muted">{title}</div>
      <div className="flex flex-wrap items-start gap-3">{children}</div>
    </div>
  )
}

export function Pill({
  children,
  tone = 'muted',
}: {
  children: ReactNode
  tone?: 'muted' | 'ok' | 'rust' | 'crab'
}) {
  const tones = {
    muted: 'bg-ink-soft text-muted',
    ok: 'bg-ok/15 text-ok',
    rust: 'bg-rust/15 text-crab',
    crab: 'bg-crab/15 text-crab',
  }
  return <span className={`rounded-md px-2 py-0.5 font-mono text-xs ${tones[tone]}`}>{children}</span>
}

// A short, monospaced explanation line shown under the animation.
export function Caption({ children }: { children: ReactNode }) {
  return <p className="mt-3 min-h-[1.25rem] text-xs leading-relaxed text-muted">{children}</p>
}
