import { useState } from 'react'
import { AnimatePresence, motion } from 'framer-motion'
import { Box, Caption, Controls, Lane, Pill, VizButton, VizPanel } from './primitives'

type Tab = 'vec' | 'string' | 'hashmap'

export default function CollectionsViz() {
  const [tab, setTab] = useState<Tab>('vec')
  return (
    <VizPanel title="Common collections: Vec, String & HashMap">
      <Controls>
        <VizButton tone="ghost" active={tab === 'vec'} onClick={() => setTab('vec')}>
          Vec&lt;T&gt;
        </VizButton>
        <VizButton tone="ghost" active={tab === 'string'} onClick={() => setTab('string')}>
          String
        </VizButton>
        <VizButton tone="ghost" active={tab === 'hashmap'} onClick={() => setTab('hashmap')}>
          HashMap
        </VizButton>
      </Controls>
      {tab === 'vec' && <VecDemo />}
      {tab === 'string' && <StringDemo />}
      {tab === 'hashmap' && <HashMapDemo />}
    </VizPanel>
  )
}

// ─── Vec ──────────────────────────────────────────────────────────────────────

function VecDemo() {
  const [len, setLen] = useState<number>(0)
  const [cap, setCap] = useState<number>(0)
  // Tracks whether the most recent push triggered a reallocation, so we can
  // explain the doubling in the caption and tint the heap lane.
  const [reallocated, setReallocated] = useState<boolean>(false)

  const push = (): void => {
    if (len === cap) {
      // Vec grows by doubling: 0 → 1 → 2 → 4 → 8 …
      setCap(cap === 0 ? 1 : cap * 2)
      setReallocated(true)
    } else {
      setReallocated(false)
    }
    setLen(len + 1)
  }

  const pop = (): void => {
    if (len === 0) return
    setReallocated(false)
    setLen(len - 1)
  }

  const reset = (): void => {
    setLen(0)
    setCap(0)
    setReallocated(false)
  }

  return (
    <div>
      <Controls>
        <VizButton onClick={push}>v.push(_)</VizButton>
        <VizButton onClick={pop} disabled={len === 0}>
          v.pop()
        </VizButton>
        <VizButton tone="ghost" onClick={reset}>
          reset
        </VizButton>
      </Controls>
      <div className="flex flex-wrap gap-6">
        <Lane title="Stack — handle">
          <Box
            label="v"
            value={
              <span className="flex flex-col gap-0.5">
                <span>ptr →</span>
                <span>len {len}</span>
                <span>cap {cap}</span>
              </span>
            }
            tone="stack"
            note="fixed-size handle"
          />
        </Lane>
        <Lane title="Heap — buffer">
          <motion.div
            layout
            animate={{
              backgroundColor: reallocated ? 'rgba(222,165,132,0.18)' : 'rgba(0,0,0,0)',
            }}
            transition={{ duration: 0.4 }}
            className="flex flex-wrap items-start gap-2 rounded-xl p-1"
          >
            <AnimatePresence mode="popLayout">
              {Array.from({ length: len }).map((_, i) => (
                <Box key={`el-${i}`} label={`[${i}]`} value={i} tone="heap" />
              ))}
              {Array.from({ length: Math.max(cap - len, 0) }).map((_, i) => (
                <Box key={`spare-${i}`} value="·" tone="muted" note="spare cap" />
              ))}
            </AnimatePresence>
            {cap === 0 && <span className="text-xs text-muted">no allocation yet</span>}
          </motion.div>
        </Lane>
      </div>
      <div className="mt-3 flex items-center gap-2">
        <Pill tone="crab">len {len}</Pill>
        <Pill tone="muted">cap {cap}</Pill>
        {reallocated && <Pill tone="rust">reallocated → cap doubled</Pill>}
      </div>
      <Caption>
        {reallocated
          ? 'len reached cap, so Vec allocated a new, larger buffer (capacity doubles) and moved the elements.'
          : 'push appends to the heap buffer; when len would exceed cap, Vec grows by doubling capacity. Greyed cells are spare capacity.'}
      </Caption>
    </div>
  )
}

// ─── String ───────────────────────────────────────────────────────────────────

type StringView = 'bytes' | 'chars'

// "héllo": é is U+00E9, encoded as two UTF-8 bytes (0xC3 0xA9).
const CHARS: string[] = ['h', 'é', 'l', 'l', 'o']
const BYTES: { hex: string; ch: string }[] = [
  { hex: 'h', ch: 'h' },
  { hex: 'C3', ch: 'é' },
  { hex: 'A9', ch: 'é' },
  { hex: 'l', ch: 'l' },
  { hex: 'l', ch: 'l' },
  { hex: 'o', ch: 'o' },
]

function StringDemo() {
  const [view, setView] = useState<StringView>('bytes')
  const isBytes = view === 'bytes'
  return (
    <div>
      <Controls>
        <VizButton tone="ghost" active={isBytes} onClick={() => setView('bytes')}>
          byte view (6)
        </VizButton>
        <VizButton tone="ghost" active={!isBytes} onClick={() => setView('chars')}>
          char view (5)
        </VizButton>
      </Controls>
      <Lane title={isBytes ? 'UTF-8 bytes' : 'Unicode scalar values (char)'}>
        <AnimatePresence mode="popLayout">
          {isBytes
            ? BYTES.map((b, i) => (
                <Box
                  key={`byte-${i}`}
                  label={`b${i}`}
                  value={b.hex}
                  tone={b.ch === 'é' ? 'rust' : 'heap'}
                  note={b.ch}
                />
              ))
            : CHARS.map((c, i) => (
                <Box key={`char-${i}`} label={`[${i}]`} value={c} tone="heap" />
              ))}
        </AnimatePresence>
      </Lane>
      <div className="mt-3 flex items-center gap-2">
        <Pill tone="crab">"héllo"</Pill>
        <Pill tone="muted">.len() = 6</Pill>
        <Pill tone="ok">.chars().count() = 5</Pill>
      </div>
      <Caption>
        A String is a UTF-8 byte vector. <code>len()</code> counts <span className="text-paper">bytes</span>,
        not characters — <code>é</code> takes 2 bytes. You cannot index a String by integer
        (<code>s[0]</code> won't compile); iterate with <code>.chars()</code> or <code>.bytes()</code> instead.
      </Caption>
    </div>
  )
}

// ─── HashMap ──────────────────────────────────────────────────────────────────

const BUCKET_COUNT = 4
type Entry = { key: string; value: number; bucket: number }
// Deterministic preset entries; the "bucket" is a fixed teaching index, not a
// real hash — it just shows keys distributing across buckets.
const PRESET: Entry[] = [
  { key: 'blue', value: 10, bucket: 1 },
  { key: 'red', value: 50, bucket: 3 },
  { key: 'green', value: 20, bucket: 1 },
]

function HashMapDemo() {
  const [count, setCount] = useState<number>(0)
  const [overwritten, setOverwritten] = useState<boolean>(false)
  // The last inserted key reuses bucket 1, demonstrating a collision chain.
  const inserted = PRESET.slice(0, count)

  const insert = (): void => {
    if (count < PRESET.length) setCount(count + 1)
  }
  const overwrite = (): void => {
    // Re-inserting "blue" with a new value overwrites in place (no new entry).
    if (count >= 1) setOverwritten(true)
  }
  const reset = (): void => {
    setCount(0)
    setOverwritten(false)
  }

  const next = PRESET[count]

  return (
    <div>
      <Controls>
        <VizButton onClick={insert} disabled={count >= PRESET.length}>
          {next ? `insert "${next.key}" → ${next.value}` : 'all inserted'}
        </VizButton>
        <VizButton onClick={overwrite} disabled={count < 1 || overwritten}>
          insert "blue" → 25 (overwrite)
        </VizButton>
        <VizButton tone="ghost" onClick={reset}>
          reset
        </VizButton>
      </Controls>
      <div className="flex flex-wrap gap-3">
        {Array.from({ length: BUCKET_COUNT }).map((_, b) => {
          const here = inserted.filter((e) => e.bucket === b)
          return (
            <Lane key={b} title={`bucket ${b}`}>
              <motion.div layout className="flex flex-col gap-2">
                <AnimatePresence mode="popLayout">
                  {here.length === 0 ? (
                    <Box key="empty" value="∅" tone="muted" note="empty" />
                  ) : (
                    here.map((e) => {
                      const isBlue = e.key === 'blue'
                      const shownValue = isBlue && overwritten ? 25 : e.value
                      return (
                        <Box
                          key={e.key}
                          label={e.key}
                          value={
                            <motion.span
                              key={shownValue}
                              initial={{ opacity: 0, y: -4 }}
                              animate={{ opacity: 1, y: 0 }}
                            >
                              {shownValue}
                            </motion.span>
                          }
                          tone={isBlue && overwritten ? 'rust' : 'heap'}
                          note={isBlue && overwritten ? 'overwritten' : undefined}
                        />
                      )
                    })
                  )}
                </AnimatePresence>
              </motion.div>
            </Lane>
          )
        })}
      </div>
      <div className="mt-3 flex flex-wrap items-center gap-2">
        {inserted.map((e) => (
          <Pill key={e.key} tone="crab">
            "{e.key}" → bucket {e.bucket}
          </Pill>
        ))}
        {inserted.length === 0 && <span className="text-xs text-muted">map is empty</span>}
      </div>
      <Caption>
        Each key is hashed to a bucket index. Inserting an existing key{' '}
        <span className="text-paper">overwrites</span> its value; <code>.entry(k).or_insert(v)</code> instead
        inserts only when the key is absent. <code>"blue"</code> and <code>"green"</code> share bucket 1 — a
        collision the map chains together.
      </Caption>
    </div>
  )
}
