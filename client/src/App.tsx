import { useEffect, useState } from 'react'
import { AppConfig, Course, getChapters, getConfig, getProgress } from './api'
import { Selection } from './types'
import ChapterTree from './components/ChapterTree'
import LessonView from './components/LessonView'
import EditorPane from './components/EditorPane'

export default function App() {
  const [course, setCourse] = useState<Course | null>(null)
  const [selection, setSelection] = useState<Selection | null>(null)
  const [progress, setProgress] = useState<Record<string, boolean>>({})
  const [config, setConfig] = useState<AppConfig>({
    hostRepoDir: '',
    editorScheme: 'vscode',
    lspUrl: '',
    chaptersDir: '',
  })
  const [configLoaded, setConfigLoaded] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [editorWidth, setEditorWidth] = useState(() => {
    const v = Number(localStorage.getItem('editorWidth'))
    return v >= 360 ? v : 560
  })
  const [dragging, setDragging] = useState(false)
  const [sidebarOpen, setSidebarOpen] = useState(() => localStorage.getItem('sidebarOpen') !== 'false')

  useEffect(() => {
    localStorage.setItem('sidebarOpen', String(sidebarOpen))
  }, [sidebarOpen])

  useEffect(() => {
    if (!dragging) return
    function onMove(e: MouseEvent) {
      const desired = window.innerWidth - e.clientX
      const max = Math.max(360, window.innerWidth - 520)
      setEditorWidth(Math.max(360, Math.min(desired, max)))
    }
    function onUp() {
      setDragging(false)
    }
    window.addEventListener('mousemove', onMove)
    window.addEventListener('mouseup', onUp)
    return () => {
      window.removeEventListener('mousemove', onMove)
      window.removeEventListener('mouseup', onUp)
    }
  }, [dragging])

  useEffect(() => {
    localStorage.setItem('editorWidth', String(editorWidth))
  }, [editorWidth])

  useEffect(() => {
    getChapters()
      .then((c) => {
        setCourse(c)
        const first = c.chapters[0]
        setSelection({ chapter: first, sub: first.subchapters[0] ?? null })
      })
      .catch((e) => setError(String(e)))
    getProgress()
      .then(setProgress)
      .catch(() => {})
    getConfig()
      .then((c) => {
        setConfig(c)
        setConfigLoaded(true)
      })
      .catch(() => setConfigLoaded(true))
  }, [])

  function markProgress(crate: string, pass: boolean) {
    setProgress((p) => ({ ...p, [crate]: pass }))
  }

  if (error)
    return (
      <div className="p-8 text-rust-bright">
        {error}. Is the server running?
      </div>
    )
  if (!course || !selection)
    return <div className="p-8 text-muted">Loading…</div>

  const completed = Object.values(progress).filter(Boolean).length

  return (
    <div className="flex h-full flex-col">
      <header className="flex items-center gap-3 border-b border-edge bg-ink-soft px-4 py-3">
        <button
          onClick={() => setSidebarOpen((o) => !o)}
          className="rounded-md border border-edge px-2 py-1 text-sm text-muted transition hover:text-paper"
          title={sidebarOpen ? 'Hide chapters' : 'Show chapters'}
          aria-label="Toggle chapter sidebar"
        >
          ☰
        </button>
        <span className="text-xl">🦀</span>
        <h1 className="text-lg font-semibold tracking-tight">{course.title}</h1>
        <span className="ml-auto text-xs text-muted">
          {completed} chapter{completed === 1 ? '' : 's'} complete
        </span>
      </header>

      <div className="flex min-h-0 flex-1">
        {sidebarOpen && (
          <ChapterTree
            course={course}
            selection={selection}
            progress={progress}
            onSelect={setSelection}
          />
        )}
        <main className="flex min-h-0 min-w-0 flex-1 flex-col">
          <LessonView selection={selection} />
        </main>
        <div
          onMouseDown={() => setDragging(true)}
          className={`w-1 shrink-0 cursor-col-resize transition-colors ${
            dragging ? 'bg-rust' : 'bg-edge hover:bg-rust/60'
          }`}
          title="Drag to resize"
        />
        <EditorPane
          crate={selection.chapter.crate}
          config={config}
          configLoaded={configLoaded}
          width={editorWidth}
          onResult={markProgress}
        />
        {dragging && <div className="fixed inset-0 z-50 cursor-col-resize" />}
      </div>
    </div>
  )
}
