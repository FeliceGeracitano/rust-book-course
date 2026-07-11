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
    return v >= 360 ? v : 640
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
        <a
          href="https://github.com/FeliceGeracitano/rust-book-course"
          target="_blank"
          rel="noopener noreferrer"
          className="text-muted transition hover:text-paper"
          title="View source on GitHub"
          aria-label="View source on GitHub"
        >
          <svg viewBox="0 0 16 16" width="20" height="20" fill="currentColor" aria-hidden="true">
            <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82a7.65 7.65 0 012-.27c.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z" />
          </svg>
        </a>
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
