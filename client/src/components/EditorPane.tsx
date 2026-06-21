import { useEffect, useRef, useState } from 'react'
import Editor from '@monaco-editor/react'
import Markdown from 'react-markdown'
import remarkGfm from 'remark-gfm'
import confetti from 'canvas-confetti'
import {
  AppConfig,
  CheckResult,
  check,
  clippy,
  getFile,
  getSolution,
  saveFile,
} from '../api'
import { CheckOutput, testOutput } from './output'
import CodeBlock from './CodeBlock'

type RunMode = 'test' | 'clippy'
type Busy = 'idle' | 'test' | 'clippy'

const AUTOSAVE_MS = 800
const MONACO_TIMEOUT_MS = 8000

function extractHints(code: string): string[] {
  const hints: string[] = []
  for (const line of code.split('\n')) {
    const comment = line.match(/\/\/\s*TODO:?\s*(.+)/i)
    if (comment) hints.push(comment[1].trim())
    const macro = line.match(/todo!\("(.+?)"\)/)
    if (macro) hints.push(macro[1].trim())
  }
  return [...new Set(hints)]
}

export default function EditorPane({
  crate,
  config,
  onResult,
}: {
  crate: string | null
  config: AppConfig
  onResult: (crate: string, pass: boolean) => void
}) {
  const [code, setCode] = useState('')
  const [saved, setSaved] = useState('')
  const [busy, setBusy] = useState<Busy>('idle')
  const [result, setResult] = useState<{ mode: RunMode; data: CheckResult } | null>(null)
  const [solution, setSolution] = useState<string | null>(null)
  const [showSolution, setShowSolution] = useState(false)
  const [showHints, setShowHints] = useState(false)
  const [loadError, setLoadError] = useState(false)
  const [editorReady, setEditorReady] = useState(false)
  const [editorFailed, setEditorFailed] = useState(false)

  // Latest snapshot, readable from async callbacks/listeners without stale closures.
  const latest = useRef({ crate, code, saved })
  latest.current = { crate, code, saved }

  // Load the file whenever the chapter changes.
  useEffect(() => {
    setResult(null)
    setSolution(null)
    setShowSolution(false)
    setShowHints(false)
    setLoadError(false)
    setBusy('idle')
    if (!crate) {
      setCode('')
      setSaved('')
      return
    }
    let alive = true
    getFile(crate)
      .then((c) => {
        if (!alive) return
        setCode(c)
        setSaved(c)
      })
      .catch(() => alive && setLoadError(true))
    return () => {
      alive = false
    }
  }, [crate])

  // Debounced autosave: switching chapters or closing the tab never loses edits.
  useEffect(() => {
    if (!crate || code === saved) return
    const t = setTimeout(() => {
      saveFile(crate, code)
        .then(() => setSaved(code))
        .catch(() => {})
    }, AUTOSAVE_MS)
    return () => clearTimeout(t)
  }, [code, saved, crate])

  // Returning to the tab picks up edits made in an external editor — but only
  // when the in-browser buffer is clean, so we never clobber unsaved work.
  useEffect(() => {
    function onFocus() {
      const cur = latest.current
      if (!cur.crate || cur.code !== cur.saved) return
      getFile(cur.crate)
        .then((c) => {
          if (latest.current.crate === cur.crate) {
            setCode(c)
            setSaved(c)
          }
        })
        .catch(() => {})
    }
    window.addEventListener('focus', onFocus)
    return () => window.removeEventListener('focus', onFocus)
  }, [])

  // If Monaco can't load (e.g. offline — it's fetched from a CDN), fall back to
  // a plain textarea so the pane is never stuck on "Loading…".
  useEffect(() => {
    if (editorReady) return
    const t = setTimeout(() => setEditorFailed(true), MONACO_TIMEOUT_MS)
    return () => clearTimeout(t)
  }, [editorReady])

  if (!crate) {
    return (
      <aside className="flex w-[44%] min-w-[360px] shrink-0 items-center justify-center border-l border-edge bg-ink-soft p-6 text-center text-sm text-muted">
        No exercise for this section — it's reference reading.
      </aside>
    )
  }

  const dirty = code !== saved
  const editorUrl = config.hostRepoDir
    ? `${config.editorScheme}://file${config.hostRepoDir}/chapters/${crate}/src/lib.rs`
    : null
  const hints = extractHints(code)

  async function run(mode: RunMode) {
    if (!crate) return
    const startCrate = crate
    setBusy(mode)
    setResult(null)
    try {
      await saveFile(startCrate, code)
      if (latest.current.crate === startCrate) setSaved(code)
      const data = mode === 'test' ? await check(startCrate) : await clippy(startCrate)
      if (mode === 'test') onResult(startCrate, data.pass) // attribute correctly even if navigated
      if (latest.current.crate !== startCrate) return // dropped stale UI update
      setResult({ mode, data })
      if (mode === 'test' && data.pass) {
        confetti({
          particleCount: 120,
          spread: 70,
          origin: { y: 0.7 },
          colors: ['#ce422b', '#dea584', '#7bb661'],
        })
      }
    } catch (e) {
      if (latest.current.crate === startCrate) {
        setResult({ mode, data: { pass: false, stdout: '', stderr: String(e) } })
      }
    } finally {
      if (latest.current.crate === startCrate) setBusy('idle')
    }
  }

  async function reveal() {
    if (!showSolution && solution === null && crate) {
      try {
        setSolution(await getSolution(crate))
      } catch {
        setSolution('_No solution available for this chapter._')
      }
    }
    setShowSolution((s) => !s)
  }

  return (
    <aside className="flex w-[44%] min-w-[360px] shrink-0 flex-col border-l border-edge bg-ink-soft">
      <div className="flex items-center gap-2 border-b border-edge px-3 py-2 text-xs">
        <span className="font-mono text-muted">chapters/{crate}/src/lib.rs</span>
        {dirty ? (
          <span className="text-crab">● unsaved</span>
        ) : (
          <span className="text-muted/60">saved</span>
        )}
        {editorUrl && (
          <a
            href={editorUrl}
            className="ml-auto rounded-md border border-edge px-2 py-0.5 font-medium text-crab transition hover:text-paper"
            title="Open this file in your local editor"
          >
            ‹/› editor ↗
          </a>
        )}
      </div>

      <div className="min-h-0 flex-1">
        {loadError ? (
          <div className="p-4 text-sm text-rust-bright">Couldn't load the file.</div>
        ) : editorFailed && !editorReady ? (
          <textarea
            value={code}
            onChange={(e) => setCode(e.target.value)}
            spellCheck={false}
            className="h-full w-full resize-none bg-ink p-4 font-mono text-sm text-paper outline-none"
          />
        ) : (
          <Editor
            height="100%"
            language="rust"
            theme="vs-dark"
            value={code}
            onChange={(v) => setCode(v ?? '')}
            onMount={() => setEditorReady(true)}
            loading={<div className="p-4 text-sm text-muted">Loading editor…</div>}
            options={{
              minimap: { enabled: false },
              fontSize: 13,
              fontFamily: 'JetBrains Mono, monospace',
              tabSize: 4,
              scrollBeyondLastLine: false,
              automaticLayout: true,
              padding: { top: 12 },
            }}
          />
        )}
      </div>

      <div className="max-h-[46%] shrink-0 overflow-auto border-t border-edge">
        <div className="flex flex-wrap items-center gap-2 px-3 py-2">
          <button
            disabled={busy !== 'idle'}
            onClick={() => run('test')}
            className="rounded-lg bg-rust px-3 py-1.5 text-xs font-semibold text-white transition hover:bg-rust-bright disabled:opacity-40"
          >
            {busy === 'test' ? 'Running…' : 'Check'}
          </button>
          <button
            disabled={busy !== 'idle'}
            onClick={() => run('clippy')}
            className="rounded-lg border border-edge px-3 py-1.5 text-xs font-semibold text-muted transition hover:text-paper disabled:opacity-40"
          >
            {busy === 'clippy' ? 'Linting…' : 'Clippy'}
          </button>
          <button
            onClick={() => setShowHints((s) => !s)}
            disabled={hints.length === 0}
            className="rounded-lg border border-edge px-3 py-1.5 text-xs font-semibold text-muted transition hover:text-paper disabled:opacity-40"
          >
            {showHints ? 'Hide hints' : `Hints${hints.length ? ` (${hints.length})` : ''}`}
          </button>
          <button
            onClick={reveal}
            className="rounded-lg border border-edge px-3 py-1.5 text-xs font-semibold text-muted transition hover:text-paper"
          >
            {showSolution ? 'Hide solution' : 'Reveal solution'}
          </button>
          {result && (
            <span
              className={`ml-auto text-xs font-semibold ${
                result.mode === 'clippy'
                  ? result.data.pass
                    ? 'text-ok'
                    : 'text-crab'
                  : result.data.pass
                    ? 'text-ok'
                    : 'text-rust-bright'
              }`}
            >
              {result.mode === 'clippy'
                ? result.data.pass
                  ? 'clippy: clean ✓'
                  : 'clippy: see notes'
                : result.data.pass
                  ? '✓ passed'
                  : '✗ not yet'}
            </span>
          )}
        </div>

        {showHints && hints.length > 0 && (
          <ul className="mx-3 mb-2 list-disc rounded-lg border border-edge bg-ink px-6 py-2 text-xs text-crab">
            {hints.map((h, i) => (
              <li key={i} className="my-0.5">
                {h}
              </li>
            ))}
          </ul>
        )}

        {result && testOutput(result.data) && (
          <div className="px-3 pb-2">
            <CheckOutput text={testOutput(result.data)} />
          </div>
        )}

        {showSolution && solution !== null && (
          <div className="px-3 pb-3">
            <div className="prose-rust rounded-lg border border-edge bg-ink p-3 text-sm">
              <Markdown
                remarkPlugins={[remarkGfm]}
                components={{ pre: ({ children }) => <>{children}</>, code: CodeBlock as never }}
              >
                {solution}
              </Markdown>
            </div>
          </div>
        )}
      </div>
    </aside>
  )
}
