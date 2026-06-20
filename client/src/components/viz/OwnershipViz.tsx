import { useState } from 'react'
import { AnimatePresence, motion } from 'framer-motion'
import { Box, Caption, Controls, Lane, Pill, VizButton, VizPanel } from './primitives'

type Tab = 'move' | 'borrow' | 'slice'

export default function OwnershipViz() {
  const [tab, setTab] = useState<Tab>('move')
  return (
    <VizPanel title="Ownership, borrowing & slices">
      <Controls>
        <VizButton tone="ghost" active={tab === 'move'} onClick={() => setTab('move')}>
          Move
        </VizButton>
        <VizButton tone="ghost" active={tab === 'borrow'} onClick={() => setTab('borrow')}>
          Borrow
        </VizButton>
        <VizButton tone="ghost" active={tab === 'slice'} onClick={() => setTab('slice')}>
          Slice
        </VizButton>
      </Controls>
      {tab === 'move' && <MoveDemo />}
      {tab === 'borrow' && <BorrowDemo />}
      {tab === 'slice' && <SliceDemo />}
    </VizPanel>
  )
}

function MoveDemo() {
  const [moved, setMoved] = useState(false)
  return (
    <div>
      <Controls>
        <VizButton onClick={() => setMoved(true)} disabled={moved}>
          let s2 = s1;
        </VizButton>
        <VizButton tone="ghost" onClick={() => setMoved(false)}>
          reset
        </VizButton>
      </Controls>
      <div className="flex gap-6">
        <Lane title="Stack">
          <Box
            label="s1"
            value="String"
            tone={moved ? 'muted' : 'stack'}
            faded={moved}
            note={moved ? 'moved — using s1 is a compile error' : 'owner → heap'}
          />
          <AnimatePresence>
            {moved && (
              <Box key="s2" label="s2" value="String" tone="rust" note="new owner → heap" />
            )}
          </AnimatePresence>
        </Lane>
        <Lane title="Heap">
          <Box label="0x…" value={'"hello"'} tone="heap" note="one owner at a time" />
        </Lane>
      </div>
      <Caption>
        {moved
          ? 'A move transfers ownership. s1 is invalidated so the heap value is never freed twice.'
          : 'String keeps its bytes on the heap; s1 owns them. Assigning to s2 moves ownership.'}
      </Caption>
    </div>
  )
}

function BorrowDemo() {
  const [shared, setShared] = useState(0)
  const [mut, setMut] = useState(false)
  const reset = () => {
    setShared(0)
    setMut(false)
  }
  return (
    <div>
      <Controls>
        <VizButton onClick={() => setShared((n) => n + 1)} disabled={mut}>
          &amp;s (shared)
        </VizButton>
        <VizButton onClick={() => setMut(true)} disabled={mut || shared > 0}>
          &amp;mut s
        </VizButton>
        <VizButton tone="ghost" onClick={reset}>
          release all
        </VizButton>
      </Controls>
      <div className="flex items-start gap-6">
        <Lane title="Owner">
          <Box label="s" value={'"hello"'} tone="ok" />
        </Lane>
        <Lane title="Active borrows">
          <AnimatePresence>
            {mut && (
              <motion.div key="mut" layout initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }}>
                <Pill tone="rust">&amp;mut s — exclusive</Pill>
              </motion.div>
            )}
            {Array.from({ length: shared }).map((_, i) => (
              <motion.div key={i} layout initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }}>
                <Pill tone="crab">&amp;s</Pill>
              </motion.div>
            ))}
          </AnimatePresence>
          {!mut && shared === 0 && <span className="text-xs text-muted">none</span>}
        </Lane>
      </div>
      <Caption>
        The rule: <span className="text-paper">any number of shared <code>&amp;</code> references</span>, OR{' '}
        <span className="text-paper">exactly one mutable <code>&amp;mut</code></span> — never both at once.
      </Caption>
    </div>
  )
}

const TEXT = 'hello world'
type Range = { start: number; end: number; label: string }
const RANGES: Range[] = [
  { start: 0, end: 5, label: '&s[0..5]' },
  { start: 6, end: 11, label: '&s[6..11]' },
  { start: 0, end: 11, label: '&s[..]' },
]

function SliceDemo() {
  const [range, setRange] = useState<Range>(RANGES[0])
  return (
    <div>
      <Controls>
        {RANGES.map((r) => (
          <VizButton key={r.label} tone="ghost" active={range.label === r.label} onClick={() => setRange(r)}>
            {r.label}
          </VizButton>
        ))}
      </Controls>
      <div className="flex flex-wrap gap-1">
        {TEXT.split('').map((ch, i) => {
          const inRange = i >= range.start && i < range.end
          return (
            <motion.div
              key={i}
              animate={{
                backgroundColor: inRange ? 'rgba(222,165,132,0.22)' : 'rgba(0,0,0,0)',
                color: inRange ? '#e8e2da' : '#a89f93',
              }}
              className="flex h-8 w-6 items-center justify-center rounded border border-edge font-mono text-sm"
            >
              {ch === ' ' ? '␣' : ch}
            </motion.div>
          )
        })}
      </div>
      <div className="mt-3 flex items-center gap-2">
        <Pill tone="crab">ptr → byte {range.start}</Pill>
        <Pill tone="muted">len {range.end - range.start}</Pill>
        <Pill tone="ok">"{TEXT.slice(range.start, range.end)}"</Pill>
      </div>
      <Caption>
        A string slice <code>&amp;str</code> is a borrowed view — a pointer and a length into the original
        data. No bytes are copied.
      </Caption>
    </div>
  )
}
