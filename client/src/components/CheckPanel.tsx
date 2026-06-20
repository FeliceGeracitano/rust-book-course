import { useState } from 'react'
import confetti from 'canvas-confetti'
import { CheckResult, check } from '../api'

// Assertion diffs (left/right) print to cargo's stdout; compile errors print to
// stderr. Show both so the learner sees whatever actually went wrong.
function testOutput(r: CheckResult): string {
  return [r.stdout, r.stderr]
    .map((s) => s.trim())
    .filter(Boolean)
    .join('\n\n')
}

// Color each line by meaning so the signal (the diff, what FAILED) pops out of
// the noise (cargo's progress chatter).
function lineClass(line: string): string {
  const t = line.trimStart()
  if (/^left:/.test(t)) return 'text-rust-bright'
  if (/^right:/.test(t)) return 'text-ok'
  if (/^assertion|panicked at/.test(t)) return 'text-crab'
  if (/\bFAILED\b/.test(line) || /^error/.test(t)) return 'text-rust-bright font-semibold'
  if (/\.\.\. ok\b/.test(line) || /^test result: ok/.test(t)) return 'text-ok'
  if (/^(----|failures:|running |test result|note:|warning)/.test(t)) return 'text-muted'
  if (/^(Compiling|Finished|Running)/.test(t)) return 'text-muted'
  return 'text-paper/75'
}

function CheckOutput({ text }: { text: string }) {
  return (
    <div className="max-h-72 overflow-auto rounded-lg border border-edge bg-ink p-3 font-mono text-xs leading-relaxed">
      {text.split('\n').map((ln, i) => (
        <div key={i} className={`whitespace-pre-wrap ${lineClass(ln)}`}>
          {ln || ' '}
        </div>
      ))}
    </div>
  )
}

export default function CheckPanel({
  crate,
  onResult,
}: {
  crate: string | null
  onResult: (crate: string, pass: boolean) => void
}) {
  const [running, setRunning] = useState(false)
  const [result, setResult] = useState<CheckResult | null>(null)

  async function runCheck() {
    if (!crate) return
    setRunning(true)
    setResult(null)
    try {
      const r = await check(crate)
      setResult(r)
      onResult(crate, r.pass)
      if (r.pass) {
        confetti({
          particleCount: 120,
          spread: 70,
          origin: { y: 0.8 },
          colors: ['#ce422b', '#dea584', '#7bb661'],
        })
      }
    } catch (e) {
      setResult({ pass: false, stdout: '', stderr: String(e) })
    } finally {
      setRunning(false)
    }
  }

  return (
    <div className="border-t border-edge bg-ink-soft">
      <div className="mx-auto flex w-full max-w-5xl flex-col gap-3 px-6 py-4">
        <div className="flex items-center gap-3">
          <button
            disabled={!crate || running}
            onClick={runCheck}
            className="rounded-lg bg-rust px-4 py-2 text-sm font-semibold text-white transition hover:bg-rust-bright disabled:cursor-not-allowed disabled:opacity-40"
          >
            {running ? 'Running cargo test…' : 'Check my code'}
          </button>
          {crate && (
            <code className="font-mono text-xs text-muted">cargo test -p {crate}</code>
          )}
          {result && (
            <span
              className={`ml-auto text-sm font-semibold ${
                result.pass ? 'text-ok' : 'text-rust-bright'
              }`}
            >
              {result.pass ? '✓ passed' : '✗ not yet'}
            </span>
          )}
        </div>
        {result && !result.pass && testOutput(result) && (
          <CheckOutput text={testOutput(result)} />
        )}
      </div>
    </div>
  )
}
