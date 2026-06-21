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
  const [config, setConfig] = useState<AppConfig>({ hostRepoDir: '', editorScheme: 'vscode' })
  const [error, setError] = useState<string | null>(null)

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
      .then(setConfig)
      .catch(() => {})
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
      <header className="flex items-center gap-3 border-b border-edge bg-ink-soft px-5 py-3">
        <span className="text-xl">🦀</span>
        <h1 className="text-lg font-semibold tracking-tight">{course.title}</h1>
        <span className="ml-auto text-xs text-muted">
          {completed} chapter{completed === 1 ? '' : 's'} complete
        </span>
      </header>

      <div className="flex min-h-0 flex-1">
        <ChapterTree
          course={course}
          selection={selection}
          progress={progress}
          onSelect={setSelection}
        />
        <main className="flex min-h-0 min-w-0 flex-1 flex-col">
          <LessonView selection={selection} />
        </main>
        <EditorPane crate={selection.chapter.crate} config={config} onResult={markProgress} />
      </div>
    </div>
  )
}
