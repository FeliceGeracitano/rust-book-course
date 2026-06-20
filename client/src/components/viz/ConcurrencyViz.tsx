import { useState } from 'react'
import { AnimatePresence, motion } from 'framer-motion'
import { Box, Caption, Controls, Lane, Pill, VizButton, VizPanel } from './primitives'

type Tab = 'channel' | 'mutex'

export default function ConcurrencyViz() {
  const [tab, setTab] = useState<Tab>('channel')
  return (
    <VizPanel title="Fearless concurrency">
      <Controls>
        <VizButton tone="ghost" active={tab === 'channel'} onClick={() => setTab('channel')}>
          Channel
        </VizButton>
        <VizButton tone="ghost" active={tab === 'mutex'} onClick={() => setTab('mutex')}>
          Mutex
        </VizButton>
      </Controls>
      {tab === 'channel' && <ChannelDemo />}
      {tab === 'mutex' && <MutexDemo />}
    </VizPanel>
  )
}

type Worker = { id: number; value: string }
const WORKERS: Worker[] = [
  { id: 0, value: '"hi"' },
  { id: 1, value: '"from"' },
  { id: 2, value: '"thread"' },
]

function ChannelDemo() {
  // `sent` counts how many values have already moved through the channel.
  const [sent, setSent] = useState<number>(0)
  const next: Worker | undefined = WORKERS[sent]
  const received: Worker[] = WORKERS.slice(0, sent)

  const step = () => setSent((n) => Math.min(n + 1, WORKERS.length))
  const reset = () => setSent(0)
  const done = sent >= WORKERS.length

  return (
    <div>
      <Controls>
        <VizButton onClick={step} disabled={done}>
          {next ? `tx.send(${next.value})` : 'all sent'}
        </VizButton>
        <VizButton tone="ghost" onClick={reset}>
          reset
        </VizButton>
      </Controls>
      <div className="flex items-start gap-6">
        <Lane title="Threads">
          <AnimatePresence>
            {WORKERS.map((w) => {
              const moved = w.id < sent
              const isNext = w.id === sent
              return (
                <Box
                  key={w.id}
                  label={`thread ${w.id}`}
                  value={w.value}
                  tone={moved ? 'muted' : isNext ? 'rust' : 'stack'}
                  faded={moved}
                  note={moved ? 'moved into channel' : isNext ? 'next to send' : 'owns its value'}
                />
              )
            })}
          </AnimatePresence>
        </Lane>
        <Lane title="Channel">
          <Box
            label="mpsc"
            value="tx → rx"
            tone="heap"
            note={done ? 'closed' : `${WORKERS.length - sent} left to send`}
          />
        </Lane>
        <Lane title="Received">
          <AnimatePresence>
            {received.map((w) => (
              <Box key={w.id} label={`from ${w.id}`} value={w.value} tone="ok" note="owned by main" />
            ))}
          </AnimatePresence>
          {received.length === 0 && <span className="text-xs text-muted">none yet</span>}
        </Lane>
      </div>
      <Caption>
        mpsc channels <span className="text-paper">move ownership</span> of values between threads. Once
        sent, a value belongs to the receiver — the sending thread can no longer use it.
      </Caption>
    </div>
  )
}

const MUTEX_THREADS: number[] = [0, 1, 2]

function MutexDemo() {
  // `acquired` is how many threads have already taken the lock and incremented.
  const [acquired, setAcquired] = useState<number>(0)
  const counter: number = acquired
  const activeId: number | undefined = MUTEX_THREADS[acquired]

  const step = () => setAcquired((n) => Math.min(n + 1, MUTEX_THREADS.length))
  const reset = () => setAcquired(0)
  const done = acquired >= MUTEX_THREADS.length

  return (
    <div>
      <Controls>
        <VizButton onClick={step} disabled={done}>
          {activeId !== undefined ? `thread ${activeId}: lock().unwrap() += 1` : 'all done'}
        </VizButton>
        <VizButton tone="ghost" onClick={reset}>
          reset
        </VizButton>
      </Controls>
      <div className="flex items-start gap-6">
        <Lane title="Arc<Mutex<i32>>">
          <motion.div layout>
            <Box
              label="counter"
              value={
                <AnimatePresence mode="popLayout">
                  <motion.span
                    key={counter}
                    initial={{ opacity: 0, y: -8 }}
                    animate={{ opacity: 1, y: 0 }}
                    exit={{ opacity: 0, y: 8 }}
                    className="inline-block"
                  >
                    {counter}
                  </motion.span>
                </AnimatePresence>
              }
              tone={done ? 'ok' : 'rust'}
              note={done ? 'all increments applied' : `held by ${activeId !== undefined ? `thread ${activeId}` : '—'}`}
            />
          </motion.div>
        </Lane>
        <Lane title="Threads">
          {MUTEX_THREADS.map((id) => {
            const finished = id < acquired
            const isActive = id === acquired && !done
            const tone = finished ? 'muted' : isActive ? 'rust' : 'stack'
            const pillTone = finished ? 'ok' : isActive ? 'rust' : 'muted'
            const status = finished ? 'released' : isActive ? 'holds lock' : 'waiting'
            return (
              <Box
                key={id}
                label={`thread ${id}`}
                value={`+1`}
                tone={tone}
                faded={finished}
                note={<Pill tone={pillTone}>{status}</Pill>}
              />
            )
          })}
        </Lane>
      </div>
      <Caption>
        <code>Mutex&lt;T&gt;</code> guarantees{' '}
        <span className="text-paper">one thread mutates the data at a time</span>; <code>Arc</code> shares
        ownership across threads. After all steps the counter equals {MUTEX_THREADS.length}.
      </Caption>
    </div>
  )
}
