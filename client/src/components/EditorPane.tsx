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
import { registerRustCompletions } from '../monacoRust'

type RunMode = 'test' | 'clippy'
type Busy = 'idle' | 'test' | 'clippy'
type Tab = 'output' | 'hints' | 'solution'

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
  width,
  onResult,
}: {
  crate: string | null
  config: AppConfig
  width: number
  onResult: (crate: string, pass: boolean) => void
}) {
  const [code, setCode] = useState('')
  const [saved, setSaved] = useState('')
  const [busy, setBusy] = useState<Busy>('idle')
  const [result, setResult] = useState<{ mode: RunMode; data: CheckResult } | null>(null)
  const [solution, setSolution] = useState<string | null>(null)
  const [activeTab, setActiveTab] = useState<Tab>('output')
  const [loadError, setLoadError] = useState(false)
  const [editorReady, setEditorReady] = useState(false)
  const [editorFailed, setEditorFailed] = useState(false)
  const [panelHeight, setPanelHeight] = useState(() => {
    const v = Number(localStorage.getItem('panelHeight'))
    return v >= 80 ? v : 220
  })
  const [panelDragging, setPanelDragging] = useState(false)
  const asideRef = useRef<HTMLElement>(null)

  // Latest snapshot, readable from async callbacks/listeners without stale closures.
  const latest = useRef({ crate, code, saved })
  latest.current = { crate, code, saved }

  // Drag the divider between the editor and the results panel to resize it.
  useEffect(() => {
    if (!panelDragging) return
    function onMove(e: MouseEvent) {
      const rect = asideRef.current?.getBoundingClientRect()
      if (!rect) return
      const h = rect.bottom - e.clientY
      const max = Math.max(80, rect.height - 160)
      setPanelHeight(Math.max(80, Math.min(h, max)))
    }
    function onUp() {
      setPanelDragging(false)
    }
    window.addEventListener('mousemove', onMove)
    window.addEventListener('mouseup', onUp)
    return () => {
      window.removeEventListener('mousemove', onMove)
      window.removeEventListener('mouseup', onUp)
    }
  }, [panelDragging])

  useEffect(() => {
    localStorage.setItem('panelHeight', String(panelHeight))
  }, [panelHeight])

  // Load the file whenever the chapter changes.
  useEffect(() => {
    setResult(null)
    setSolution(null)
    setActiveTab('output')
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
      <aside
        style={{ width }}
        className="flex shrink-0 items-center justify-center border-l border-edge bg-ink-soft p-6 text-center text-sm text-muted"
      >
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
    setActiveTab('output')
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

  function openTab(tab: Tab) {
    setActiveTab(tab)
    if (tab === 'solution' && solution === null && crate) {
      getSolution(crate)
        .then(setSolution)
        .catch(() => setSolution('_No solution available for this chapter._'))
    }
  }

  return (
    <aside
      ref={asideRef}
      style={{ width }}
      className="flex shrink-0 flex-col border-l border-edge bg-ink-soft"
    >
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
            onMount={(_editor, monaco) => {
              setEditorReady(true)
              registerRustCompletions(monaco)
            }}
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

      <div
        onMouseDown={() => setPanelDragging(true)}
        className={`h-1 shrink-0 cursor-row-resize transition-colors ${
          panelDragging ? 'bg-rust' : 'bg-edge hover:bg-rust/60'
        }`}
        title="Drag to resize results"
      />

      <div style={{ height: panelHeight }} className="flex shrink-0 flex-col">
        <div className="flex flex-wrap items-center gap-1.5 border-b border-edge px-3 py-2">
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

          <div className="mx-1 h-5 w-px bg-edge" />

          {([
            ['output', 'Output'],
            ['hints', `Hints${hints.length ? ` (${hints.length})` : ''}`],
            ['solution', 'Solution'],
          ] as [Tab, string][]).map(([id, label]) => (
            <button
              key={id}
              onClick={() => openTab(id)}
              className={`rounded-md px-2.5 py-1 text-xs font-semibold transition ${
                activeTab === id ? 'bg-ink-card text-paper' : 'text-muted hover:text-paper'
              }`}
            >
              {label}
            </button>
          ))}

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

        <div className="min-h-0 flex-1 overflow-auto p-3">
          {activeTab === 'output' &&
            (result && testOutput(result.data) ? (
              <CheckOutput text={testOutput(result.data)} />
            ) : (
              <p className="text-xs text-muted">Run Check or Clippy to see results here.</p>
            ))}

          {activeTab === 'hints' &&
            (hints.length > 0 ? (
              <ul className="list-disc rounded-lg border border-edge bg-ink px-6 py-2 text-xs text-crab">
                {hints.map((h, i) => (
                  <li key={i} className="my-0.5">
                    {h}
                  </li>
                ))}
              </ul>
            ) : (
              <p className="text-xs text-muted">No hints for this exercise.</p>
            ))}

          {activeTab === 'solution' &&
            (solution !== null ? (
              <div className="prose-rust rounded-lg border border-edge bg-ink p-3 text-sm">
                <Markdown
                  remarkPlugins={[remarkGfm]}
                  components={{ pre: ({ children }) => <>{children}</>, code: CodeBlock as never }}
                >
                  {solution}
                </Markdown>
              </div>
            ) : (
              <p className="text-xs text-muted">Loading solution…</p>
            ))}
        </div>
      </div>
      {panelDragging && <div className="fixed inset-0 z-50 cursor-row-resize" />}
    </aside>
  )
}
