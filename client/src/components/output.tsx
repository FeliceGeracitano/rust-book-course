import { CheckResult } from '../api'

// Assertion diffs (left/right) print to cargo's stdout; compile/lint errors print
// to stderr. Show both so the learner sees whatever actually went wrong.
export function testOutput(r: CheckResult): string {
  return [r.stdout, r.stderr]
    .map((s) => s.trim())
    .filter(Boolean)
    .join('\n\n')
}

// Color each line by meaning so the signal (the diff, what FAILED, lint notes)
// pops out of cargo's progress chatter.
function lineClass(line: string): string {
  const t = line.trimStart()
  if (/^left:/.test(t)) return 'text-rust-bright'
  if (/^right:/.test(t)) return 'text-ok'
  if (/^assertion|panicked at/.test(t)) return 'text-crab'
  if (/^warning/.test(t)) return 'text-crab'
  if (/\bFAILED\b/.test(line) || /^error/.test(t)) return 'text-rust-bright font-semibold'
  if (/\.\.\. ok\b/.test(line) || /^test result: ok/.test(t)) return 'text-ok'
  if (/^(----|failures:|running |test result|note:|help:)/.test(t)) return 'text-muted'
  if (/^(Compiling|Finished|Running|Checking)/.test(t)) return 'text-muted'
  return 'text-paper/75'
}

export function CheckOutput({ text }: { text: string }) {
  return (
    <div className="rounded-lg border border-edge bg-ink p-3 font-mono text-xs leading-relaxed">
      {text.split('\n').map((ln, i) => (
        <div key={i} className={`whitespace-pre-wrap ${lineClass(ln)}`}>
          {ln || ' '}
        </div>
      ))}
    </div>
  )
}
