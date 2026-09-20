import { Suspense, useEffect, useMemo, useState } from 'react'
import Markdown, { type Components } from 'react-markdown'
import remarkGfm from 'remark-gfm'
import { loadLesson, parseQuiz, parseTrace, type Lesson } from '../content'
import { markComplete, useProgress } from '../progress'
import CodeBlock from './CodeBlock'
import { Quiz, Trace } from './Discovery'
import { vizFor } from './viz/registry'

export default function LessonView({
  lesson,
  previous,
  next,
}: {
  lesson: Lesson
  previous?: Lesson
  next?: Lesson
}) {
  const [markdown, setMarkdown] = useState<string | null>(null)
  const [error, setError] = useState(false)
  const [attempt, setAttempt] = useState(0)
  const complete = useProgress().completed.includes(lesson.id)
  const Viz = vizFor(lesson.chapter.id)
  useEffect(() => {
    let active = true
    setError(false)
    loadLesson(lesson.id)
      .then((text) => {
        if (active) setMarkdown(text)
      })
      .catch(() => {
        if (active) setError(true)
      })
    return () => {
      active = false
    }
  }, [lesson.id, attempt])

  // Stable renderers preserve widget state when progress updates rerender the lesson.
  const components = useMemo<Components>(
    () => ({
      pre: ({ children }) => <>{children}</>,
      code: ({ className, children }) => {
        const raw = String(children ?? '').replace(/\n$/, '')
        try {
          if (className === 'language-quiz')
            return <Quiz data={parseQuiz(raw)} lessonId={lesson.id} />
          if (className === 'language-trace')
            return <Trace data={parseTrace(raw)} />
        } catch {
          return <p role="alert">This discovery block could not be loaded.</p>
        }
        return <CodeBlock className={className}>{children}</CodeBlock>
      },
    }),
    [lesson.id],
  )

  return (
    <div className="mx-auto max-w-3xl">
      <p className="eyebrow mb-5">
        {lesson.chapter.id === 'appendix'
          ? 'Reference'
          : `Chapter ${lesson.chapter.number}`}{' '}
        · {lesson.chapter.title}
      </p>
      {error ? (
        <div role="alert">
          <p>Could not load this lesson. Your progress is still saved.</p>
          <button
            className="secondary mt-3"
            onClick={() => setAttempt(attempt + 1)}
          >
            Retry loading lesson
          </button>
        </div>
      ) : markdown === null ? (
        <p role="status">Loading lesson…</p>
      ) : (
        <>
          <article className="prose-rust">
            <Markdown remarkPlugins={[remarkGfm]} components={components}>
              {markdown}
            </Markdown>
          </article>
          {Viz && (
            <details className="my-8">
              <summary className="cursor-pointer text-crab">
                Explore the chapter visualization
              </summary>
              <div className="mt-4">
                <Suspense fallback={<p>Loading visualization…</p>}>
                  <Viz />
                </Suspense>
              </div>
            </details>
          )}
          <div className="mt-10 flex flex-wrap items-center gap-4 border-t border-edge pt-6">
            <button
              className={complete ? 'secondary' : 'primary'}
              aria-pressed={complete}
              onClick={() => markComplete(lesson.id, !complete)}
            >
              {complete
                ? '✓ Completed · mark incomplete'
                : 'Mark lesson complete'}
            </button>
            <p className="text-xs text-muted">
              You decide when you’re ready to move on.
            </p>
          </div>
        </>
      )}
      <nav
        aria-label="Lesson navigation"
        className="mt-8 grid grid-cols-2 gap-5 border-t border-edge pt-6 pb-10 text-sm"
      >
        <div>
          {previous && (
            <a className="text-link" href={`#${previous.id}`}>
              <span className="block text-xs text-muted">← Previous</span>
              {previous.sub.title}
            </a>
          )}
        </div>
        <div className="text-right">
          {next ? (
            <a className="text-link" href={`#${next.id}`}>
              <span className="block text-xs text-muted">Next →</span>
              {next.sub.title}
            </a>
          ) : (
            <p className="text-crab">You’ve reached the end of the course.</p>
          )}
        </div>
      </nav>
    </div>
  )
}
