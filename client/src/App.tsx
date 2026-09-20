import { useEffect, useRef, useState } from 'react'
import { course, lessons } from './content'
import { getProgress, rememberLesson, useProgress } from './progress'
import ChapterTree from './components/ChapterTree'
import LessonView from './components/LessonView'

function currentId() {
  return (
    window.location.hash.slice(1) || getProgress().lastLesson || lessons[0].id
  )
}

export default function App() {
  const [id, setId] = useState(currentId)
  const [sidebarOpen, setSidebarOpen] = useState(false)
  const progress = useProgress()
  const main = useRef<HTMLElement>(null)
  const index = lessons.findIndex((lesson) => lesson.id === id)
  const lesson = lessons[index]
  const completed = lessons.filter((item) =>
    progress.completed.includes(item.id),
  ).length

  useEffect(() => {
    const navigate = () => {
      setId(currentId())
      setSidebarOpen(false)
      main.current?.focus()
    }
    window.addEventListener('hashchange', navigate)
    return () => window.removeEventListener('hashchange', navigate)
  }, [])
  useEffect(() => {
    if (lesson) {
      // Give the initial/resumed lesson a stable history entry before lastLesson changes.
      if (!window.location.hash)
        window.history.replaceState(null, '', `#${lesson.id}`)
      rememberLesson(lesson.id)
      document.title = `${lesson.sub.title} · Rust Book Course`
    } else document.title = 'Lesson not found · Rust Book Course'
    main.current?.scrollTo?.(0, 0)
  }, [lesson])

  return (
    <div className="flex h-full flex-col">
      <a
        href="#lesson-content"
        onClick={(event) => {
          event.preventDefault()
          main.current?.focus()
        }}
        className="skip-link"
      >
        Skip to lesson
      </a>
      <header className="flex items-center gap-3 border-b border-edge bg-ink-soft px-4 py-4">
        <button
          onClick={() => setSidebarOpen(!sidebarOpen)}
          aria-expanded={sidebarOpen}
          aria-controls="chapter-sidebar"
          className="secondary lg:hidden"
          aria-label="Toggle chapters"
        >
          ☰
        </button>
        <span className="text-2xl" aria-hidden="true">
          🦀
        </span>
        <div>
          <span className="font-semibold tracking-tight">{course.title}</span>
          <p className="text-xs text-muted">Read. Predict. Explore.</p>
        </div>
        <span className="ml-auto text-right text-xs text-muted">
          {completed} / {lessons.length}
          <span className="hidden sm:inline"> lessons complete</span>
        </span>
        <a
          href="https://github.com/FeliceGeracitano/rust-book-course"
          className="text-link hidden sm:block"
        >
          Source
        </a>
      </header>
      <div className="relative flex min-h-0 flex-1">
        <div
          id="chapter-sidebar"
          className={`${sidebarOpen ? 'absolute inset-y-0 left-0 z-20 flex shadow-xl' : 'hidden'} w-[min(20rem,90vw)] shrink-0 lg:static lg:flex lg:w-72`}
        >
          <ChapterTree selectedId={id} />
        </div>
        {sidebarOpen && (
          <button
            aria-label="Close chapters"
            className="absolute inset-0 z-10 bg-black/50 lg:hidden"
            onClick={() => setSidebarOpen(false)}
          />
        )}
        <main
          id="lesson-content"
          ref={main}
          tabIndex={-1}
          className="min-w-0 flex-1 overflow-y-auto px-5 py-8 outline-none sm:px-10"
        >
          {lesson ? (
            <LessonView
              key={lesson.id}
              lesson={lesson}
              previous={lessons[index - 1]}
              next={lessons[index + 1]}
            />
          ) : (
            <div className="mx-auto max-w-3xl">
              <h1 className="text-2xl font-bold">Lesson not found</h1>
              <p className="my-4">
                This lesson link does not match the course.
              </p>
              <a className="text-link" href={`#${lessons[0].id}`}>
                Start the course
              </a>
            </div>
          )}
        </main>
      </div>
    </div>
  )
}
