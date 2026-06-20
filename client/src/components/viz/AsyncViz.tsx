import { useState } from 'react'
import { AnimatePresence, motion } from 'framer-motion'
import { Box, Caption, Controls, Lane, Pill, VizButton, VizPanel } from './primitives'

type Tab = 'poll' | 'interleave' | 'stream'

export default function AsyncViz() {
  const [tab, setTab] = useState<Tab>('poll')
  return (
    <VizPanel title="Async: futures, interleaving & streams">
      <Controls>
        <VizButton tone="ghost" active={tab === 'poll'} onClick={() => setTab('poll')}>
          Poll
        </VizButton>
        <VizButton tone="ghost" active={tab === 'interleave'} onClick={() => setTab('interleave')}>
          Interleave
        </VizButton>
        <VizButton tone="ghost" active={tab === 'stream'} onClick={() => setTab('stream')}>
          Stream
        </VizButton>
      </Controls>
      {tab === 'poll' && <PollDemo />}
      {tab === 'interleave' && <InterleaveDemo />}
      {tab === 'stream' && <StreamDemo />}
    </VizPanel>
  )
}

// ── Poll ───────────────────────────────────────────────────────────────────
// A single future's state machine: poll() -> Pending, wake() arms it, the next
// poll() -> Ready(value).

type PollState = 'pending' | 'armed' | 'ready'

function PollDemo() {
  const [state, setState] = useState<PollState>('pending')

  const ready = state === 'ready'
  const armed = state === 'armed'

  const poll = () => {
    if (state === 'pending') return // still Pending — no work to do yet
    if (state === 'armed') setState('ready')
  }
  const wake = () => {
    if (state === 'pending') setState('armed')
  }
  const reset = () => setState('pending')

  return (
    <div>
      <Controls>
        <VizButton onClick={poll} disabled={ready}>
          poll()
        </VizButton>
        <VizButton onClick={wake} disabled={state !== 'pending'}>
          wake()
        </VizButton>
        <VizButton tone="ghost" onClick={reset}>
          reset
        </VizButton>
      </Controls>
      <div className="flex items-start gap-6">
        <Lane title="Future">
          <Box
            label="fetch_data()"
            value=".poll(cx)"
            tone={ready ? 'ok' : 'rust'}
            note={armed ? 'waker fired — data has arrived' : 'returns Poll<T>'}
          />
        </Lane>
        <Lane title="Result">
          <AnimatePresence mode="wait">
            {ready ? (
              <motion.div
                key="ready"
                layout
                initial={{ opacity: 0, y: 6 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, scale: 0.85 }}
                transition={{ type: 'spring', stiffness: 320, damping: 26 }}
              >
                <Box label="poll →" value="Ready(42)" tone="ok" note="the value is here" />
              </motion.div>
            ) : (
              <motion.div
                key="pending"
                layout
                initial={{ opacity: 0, y: 6 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, scale: 0.85 }}
                transition={{ type: 'spring', stiffness: 320, damping: 26 }}
              >
                <Box
                  label="poll →"
                  value="Pending"
                  tone={armed ? 'rust' : 'muted'}
                  note={armed ? 'poll again to make progress' : 'not ready — yielded back'}
                />
              </motion.div>
            )}
          </AnimatePresence>
        </Lane>
      </div>
      <Caption>
        A <code>Future</code> is polled; it returns <Pill tone="muted">Pending</Pill> until its value
        is ready, then <Pill tone="ok">Ready(T)</Pill>.
      </Caption>
    </div>
  )
}

// ── Interleave ───────────────────────────────────────────────────────────────
// Two futures A and B driven on one thread. Each step alternates polling A then
// B; each becomes Ready after a couple of polls.

type Which = 'A' | 'B'
type LogEntry = { id: number; future: Which; result: string }
const READY_AT = 2 // each future is Ready on its 2nd poll

function InterleaveDemo() {
  const [pollsA, setPollsA] = useState(0)
  const [pollsB, setPollsB] = useState(0)
  const [next, setNext] = useState<Which>('A')
  const [log, setLog] = useState<LogEntry[]>([])

  const doneA = pollsA >= READY_AT
  const doneB = pollsB >= READY_AT
  const allDone = doneA && doneB

  const step = () => {
    if (allDone) return
    // Pick the next future that still has work, alternating A then B.
    let which: Which = next
    if (which === 'A' && doneA) which = 'B'
    else if (which === 'B' && doneB) which = 'A'

    const count = (which === 'A' ? pollsA : pollsB) + 1
    const result = count >= READY_AT ? `Ready("${which.toLowerCase()}")` : 'Pending'
    if (which === 'A') setPollsA(count)
    else setPollsB(count)

    setLog((prev) => [...prev, { id: prev.length, future: which, result }])
    setNext(which === 'A' ? 'B' : 'A')
  }
  const reset = () => {
    setPollsA(0)
    setPollsB(0)
    setNext('A')
    setLog([])
  }

  return (
    <div>
      <Controls>
        <VizButton onClick={step} disabled={allDone}>
          step (poll next)
        </VizButton>
        <VizButton tone="ghost" onClick={reset}>
          reset
        </VizButton>
      </Controls>
      <div className="flex items-start gap-6">
        <Lane title="Executor thread">
          <Box
            label="future A"
            value={doneA ? 'Ready' : 'Pending'}
            tone={doneA ? 'ok' : 'rust'}
            note={`polled ${pollsA}×`}
          />
          <Box
            label="future B"
            value={doneB ? 'Ready' : 'Pending'}
            tone={doneB ? 'ok' : 'rust'}
            note={`polled ${pollsB}×`}
          />
        </Lane>
        <Lane title="Poll log">
          {log.length === 0 && <span className="text-xs text-muted">no polls yet</span>}
          <div className="flex flex-col gap-1.5">
            <AnimatePresence>
              {log.map((e) => (
                <motion.div
                  key={e.id}
                  layout
                  initial={{ opacity: 0, x: -8 }}
                  animate={{ opacity: 1, x: 0 }}
                  exit={{ opacity: 0 }}
                  transition={{ type: 'spring', stiffness: 320, damping: 26 }}
                  className="flex items-center gap-2"
                >
                  <Pill tone="crab">poll {e.future}</Pill>
                  <Pill tone={e.result.startsWith('Ready') ? 'ok' : 'muted'}>{e.result}</Pill>
                </motion.div>
              ))}
            </AnimatePresence>
          </div>
        </Lane>
      </div>
      <Caption>
        {allDone
          ? 'Both futures completed — the executor interleaved their polls on a single thread.'
          : 'async runs concurrently on one thread by interleaving polls — not parallel threads.'}
      </Caption>
    </div>
  )
}

// ── Stream ───────────────────────────────────────────────────────────────────
// A stream that yields 1, 2, 3 then None across successive next() polls.

const STREAM_ITEMS = [1, 2, 3] as const

function StreamDemo() {
  // index = how many items have been pulled (0..STREAM_ITEMS.length, then exhausted)
  const [pulled, setPulled] = useState(0)

  const exhausted = pulled > STREAM_ITEMS.length
  const current: number | null = pulled >= 1 && pulled <= STREAM_ITEMS.length ? STREAM_ITEMS[pulled - 1] : null

  const next = () => {
    if (exhausted) return
    setPulled((n) => n + 1)
  }
  const reset = () => setPulled(0)

  const yielded = STREAM_ITEMS.slice(0, Math.min(pulled, STREAM_ITEMS.length))

  return (
    <div>
      <Controls>
        <VizButton onClick={next} disabled={exhausted}>
          stream.next().await
        </VizButton>
        <VizButton tone="ghost" onClick={reset}>
          reset
        </VizButton>
      </Controls>
      <div className="flex items-start gap-6">
        <Lane title="Stream of i32">
          <Box
            label="counter()"
            value=".next()"
            tone={exhausted ? 'muted' : 'rust'}
            note={exhausted ? 'drained' : 'pulls one item per await'}
          />
        </Lane>
        <Lane title="Yielded so far">
          {pulled === 0 && <span className="text-xs text-muted">nothing pulled yet</span>}
          <AnimatePresence>
            {yielded.map((n) => (
              <motion.div
                key={n}
                layout
                initial={{ opacity: 0, y: 6 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, scale: 0.85 }}
                transition={{ type: 'spring', stiffness: 320, damping: 26 }}
              >
                <Box label={`item ${n}`} value={`Some(${n})`} tone="ok" />
              </motion.div>
            ))}
            {exhausted && (
              <motion.div
                key="none"
                layout
                initial={{ opacity: 0, y: 6 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, scale: 0.85 }}
                transition={{ type: 'spring', stiffness: 320, damping: 26 }}
              >
                <Box label="end" value="None" tone="muted" note="loop ends" />
              </motion.div>
            )}
          </AnimatePresence>
        </Lane>
      </div>
      <div className="mt-3 flex items-center gap-2">
        <Pill tone="crab">pulled {Math.min(pulled, STREAM_ITEMS.length)}</Pill>
        <Pill tone={current === null ? 'muted' : 'ok'}>
          last → {current === null ? (exhausted ? 'None' : '—') : `Some(${current})`}
        </Pill>
      </div>
      <Caption>
        A <code>Stream</code> is the async analogue of an iterator — many values over time.
      </Caption>
    </div>
  )
}
