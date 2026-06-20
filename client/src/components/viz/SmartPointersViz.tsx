import { useState } from 'react'
import { AnimatePresence, motion } from 'framer-motion'
import { Box, Caption, Controls, Lane, Pill, VizButton, VizPanel } from './primitives'

type Tab = 'box' | 'rc' | 'refcell'

export default function SmartPointersViz() {
  const [tab, setTab] = useState<Tab>('box')
  return (
    <VizPanel title="Smart pointers: Box, Rc & RefCell">
      <Controls>
        <VizButton tone="ghost" active={tab === 'box'} onClick={() => setTab('box')}>
          Box
        </VizButton>
        <VizButton tone="ghost" active={tab === 'rc'} onClick={() => setTab('rc')}>
          Rc
        </VizButton>
        <VizButton tone="ghost" active={tab === 'refcell'} onClick={() => setTab('refcell')}>
          RefCell
        </VizButton>
      </Controls>
      {tab === 'box' && <BoxDemo />}
      {tab === 'rc' && <RcDemo />}
      {tab === 'refcell' && <RefCellDemo />}
    </VizPanel>
  )
}

function BoxDemo() {
  return (
    <div>
      <div className="flex items-center gap-6">
        <Lane title="Stack">
          <Box label="b" value="Box<i32>" tone="stack" note="owner → heap" />
        </Lane>
        <motion.div
          aria-hidden
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          className="font-mono text-sm text-crab"
        >
          →
        </motion.div>
        <Lane title="Heap">
          <Box label="0x…" value="5" tone="heap" note="single owner" />
        </Lane>
      </div>
      <Caption>
        <code>Box&lt;T&gt;</code> stores its value on the heap with a single owner.
      </Caption>
    </div>
  )
}

const RC_HANDLES = ['a', 'b', 'c', 'd'] as const

function RcDemo() {
  const [count, setCount] = useState(1)
  const freed = count === 0
  const clone = () => setCount((n) => Math.min(n + 1, RC_HANDLES.length))
  const drop = () => setCount((n) => Math.max(n - 1, 0))
  const reset = () => setCount(1)
  return (
    <div>
      <Controls>
        <VizButton onClick={clone} disabled={freed || count >= RC_HANDLES.length}>
          Rc::clone(&amp;a)
        </VizButton>
        <VizButton onClick={drop} disabled={freed}>
          drop
        </VizButton>
        <VizButton tone="ghost" onClick={reset}>
          reset
        </VizButton>
      </Controls>
      <div className="flex items-start gap-6">
        <Lane title="Stack handles">
          <AnimatePresence>
            {RC_HANDLES.slice(0, count).map((name) => (
              <motion.div
                key={name}
                layout
                initial={{ opacity: 0, y: 6 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, scale: 0.85 }}
                transition={{ type: 'spring', stiffness: 320, damping: 26 }}
              >
                <Pill tone="crab">{name}: Rc&lt;T&gt;</Pill>
              </motion.div>
            ))}
          </AnimatePresence>
          {freed && <span className="text-xs text-muted">all dropped</span>}
        </Lane>
        <Lane title="Heap">
          <AnimatePresence>
            {!freed && (
              <Box
                key="value"
                label="0x…"
                value={'"shared"'}
                tone="heap"
                note={
                  <span className="flex items-center gap-1">
                    <Pill tone="ok">strong_count {count}</Pill>
                  </span>
                }
              />
            )}
          </AnimatePresence>
          {freed && <span className="text-xs text-muted">value freed</span>}
        </Lane>
      </div>
      <Caption>
        {freed
          ? 'strong_count reached 0, so the heap value was dropped and freed.'
          : 'Rc<T> enables multiple owners; the value drops when the last Rc does.'}
      </Caption>
    </div>
  )
}

function RefCellDemo() {
  const [shared, setShared] = useState(0)
  const [mut, setMut] = useState(false)
  const [panic, setPanic] = useState(false)

  const borrow = () => {
    setPanic(false)
    setShared((n) => n + 1)
  }
  const borrowMut = () => {
    if (shared > 0 || mut) {
      setPanic(true)
      return
    }
    setPanic(false)
    setMut(true)
  }
  const release = () => {
    setShared(0)
    setMut(false)
    setPanic(false)
  }

  return (
    <div>
      <Controls>
        <VizButton onClick={borrow} disabled={mut}>
          borrow()
        </VizButton>
        <VizButton onClick={borrowMut}>borrow_mut()</VizButton>
        <VizButton tone="ghost" onClick={release}>
          release
        </VizButton>
      </Controls>
      <div className="flex items-start gap-6">
        <Lane title="Owner">
          <Box label="cell" value="RefCell<i32>" tone="ok" note="value: 5" />
        </Lane>
        <Lane title="Active borrows">
          <AnimatePresence>
            {mut && (
              <motion.div key="mut" layout initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }}>
                <Pill tone="rust">borrow_mut() — exclusive</Pill>
              </motion.div>
            )}
            {Array.from({ length: shared }).map((_, i) => (
              <motion.div key={i} layout initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }}>
                <Pill tone="crab">borrow() — shared</Pill>
              </motion.div>
            ))}
          </AnimatePresence>
          {!mut && shared === 0 && <span className="text-xs text-muted">none</span>}
        </Lane>
      </div>
      <AnimatePresence>
        {panic && (
          <motion.div
            key="panic"
            layout
            initial={{ opacity: 0, y: 4 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0 }}
            className="mt-3"
          >
            <Pill tone="rust">panic: already borrowed: BorrowMutError</Pill>
          </motion.div>
        )}
      </AnimatePresence>
      <Caption>
        <code>RefCell&lt;T&gt;</code> moves borrow checking to runtime: many shared borrows or one exclusive
        borrow_mut, never both.
      </Caption>
    </div>
  )
}
