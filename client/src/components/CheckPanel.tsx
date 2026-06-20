import { useState } from 'react'
import confetti from 'canvas-confetti'
import { CheckResult, check } from '../api'

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
      <div className="mx-auto flex max-w-3xl flex-col gap-3 px-8 py-4">
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
        {result && !result.pass && (result.stderr || result.stdout) && (
          <pre className="max-h-48 overflow-auto rounded-lg border border-edge bg-ink p-3 font-mono text-xs text-muted">
            {result.stderr || result.stdout}
          </pre>
        )}
      </div>
    </div>
  )
}
