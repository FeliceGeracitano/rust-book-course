import { useState } from 'react'
import { motion } from 'framer-motion'
import { Caption, Controls, VizButton, VizPanel } from './primitives'

// The crate graph of examples/large-service, shown two ways: what Cargo.toml
// says at compile time, and what actually calls what while one request runs.

type Layer = 'binary' | 'wiring' | 'adapters' | 'domain' | 'leaf'
const CRATES: { id: string; layer: Layer }[] = [
  { id: 'api', layer: 'binary' },
  { id: 'app', layer: 'wiring' },
  { id: 'transport-http', layer: 'adapters' },
  { id: 'integrations', layer: 'adapters' },
  { id: 'orders', layer: 'domain' },
  { id: 'customers', layer: 'domain' },
  { id: 'billing', layer: 'domain' },
  { id: 'money', layer: 'leaf' },
  { id: 'observability', layer: 'leaf' },
]
const LAYERS: { id: Layer; title: string }[] = [
  { id: 'binary', title: 'binary' },
  { id: 'wiring', title: 'wiring' },
  { id: 'adapters', title: 'adapters' },
  { id: 'domain', title: 'domain (ports as traits, no HTTP)' },
  { id: 'leaf', title: 'leaf libraries' },
]
const COMPILE_EDGES: [string, string][] = [
  ['api', 'app'],
  ['app', 'transport-http'],
  ['app', 'integrations'],
  ['app', 'observability'],
  ['transport-http', 'orders'],
  ['transport-http', 'customers'],
  ['transport-http', 'billing'],
  ['transport-http', 'money'],
  ['integrations', 'orders'],
  ['integrations', 'customers'],
  ['integrations', 'billing'],
  ['integrations', 'money'],
  ['orders', 'customers'],
  ['orders', 'billing'],
  ['orders', 'money'],
  ['billing', 'money'],
]
const RUNTIME_STEPS: { title: string; detail: string; active: string[] }[] = [
  {
    title: 'GET /orders/quote?customer_id=c1&cents=1000',
    detail: 'transport-http parses the query, then calls the orders service. It never touches HTTP clients or providers.',
    active: ['transport-http'],
  },
  {
    title: 'orders::Service::quote',
    detail: 'Parses CustomerId and Cents first. Invalid input fails here, before any port is called.',
    active: ['orders', 'customers', 'money'],
  },
  {
    title: 'dyn CustomerDirectory',
    detail: 'orders asks its own port. integrations::identity implements it over the one shared reqwest::Client. An unknown customer stops the workflow: billing is never asked.',
    active: ['orders', 'integrations'],
  },
  {
    title: 'billing::Service::quote → dyn FeeProvider',
    detail: 'billing asks its port for the fee; integrations::payments answers. money enforces the total stays within bounds.',
    active: ['billing', 'integrations', 'money'],
  },
  {
    title: 'JSON response',
    detail: 'transport-http maps OrderQuote to JSON and each error to 400, 404, 502 or 504. observability logged method and path only.',
    active: ['transport-http', 'observability'],
  },
]

export default function LargeServiceViz() {
  const [mode, setMode] = useState<'compile' | 'runtime'>('compile')
  const [step, setStep] = useState(0)
  const active = mode === 'runtime' ? RUNTIME_STEPS[step].active : []
  return (
    <VizPanel title="Where the domains live">
      <Controls>
        <VizButton
          tone="ghost"
          active={mode === 'compile'}
          onClick={() => setMode('compile')}
        >
          Compile-time dependencies
        </VizButton>
        <VizButton
          tone="ghost"
          active={mode === 'runtime'}
          onClick={() => {
            setMode('runtime')
            setStep(0)
          }}
        >
          Runtime request
        </VizButton>
      </Controls>
      <div className="grid gap-2">
        {LAYERS.map((layer) => (
          <div key={layer.id} className="flex flex-wrap items-center gap-2">
            <span className="w-40 shrink-0 text-[10px] uppercase tracking-wider text-muted">
              {layer.title}
            </span>
            {CRATES.filter((crate) => crate.layer === layer.id).map((crate) => {
              const lit = active.includes(crate.id)
              return (
                <motion.span
                  key={crate.id}
                  animate={{ scale: lit ? 1.06 : 1 }}
                  className={`rounded-lg border px-2 py-1 font-mono text-xs ${lit ? 'border-rust/60 bg-rust/15 text-crab' : 'border-edge bg-ink-soft text-paper'}`}
                >
                  {crate.id}
                </motion.span>
              )
            })}
          </div>
        ))}
      </div>
      {mode === 'compile' ? (
        <div className="mt-4">
          <Caption>
            Arrows are Cargo.toml dependencies. Nothing points up: billing and
            customers never import orders, and no domain crate imports HTTP.
          </Caption>
          <ul className="mt-2 grid grid-cols-2 gap-x-4 gap-y-1 font-mono text-xs text-muted sm:grid-cols-3">
            {COMPILE_EDGES.map(([from, to]) => (
              <li key={`${from}-${to}`}>
                {from} → {to}
              </li>
            ))}
          </ul>
        </div>
      ) : (
        <div className="mt-4">
          <Controls>
            <VizButton
              tone="ghost"
              onClick={() => setStep(step - 1)}
              disabled={step === 0}
            >
              Previous step
            </VizButton>
            <VizButton
              onClick={() => setStep(step + 1)}
              disabled={step === RUNTIME_STEPS.length - 1}
            >
              Next step
            </VizButton>
            <span className="text-xs text-muted">
              {step + 1} / {RUNTIME_STEPS.length}
            </span>
          </Controls>
          <p className="font-mono text-sm text-crab">{RUNTIME_STEPS[step].title}</p>
          <Caption>{RUNTIME_STEPS[step].detail}</Caption>
        </div>
      )}
    </VizPanel>
  )
}
