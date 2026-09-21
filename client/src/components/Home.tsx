import { chapters, course, lessons, type Chapter } from '../content'
import { useProgress } from '../progress'
import ProgressRing from './ProgressRing'

const label = (chapter: Chapter) =>
  chapter.id === 'appendix' ? 'Appendix' : `Chapter ${chapter.number}`

export default function Home() {
  const progress = useProgress()
  const doneCount = lessons.filter((lesson) =>
    progress.completed.includes(lesson.id),
  ).length
  const last = lessons.find((lesson) => lesson.id === progress.lastLesson)
  const first = lessons[0]
  return (
    <div className="mx-auto max-w-3xl">
      <section className="flex flex-col items-center gap-6 py-10 text-center sm:flex-row sm:text-left">
        <span className="text-8xl leading-none" aria-hidden="true">
          🦀
        </span>
        <div>
          <h1 className="text-4xl font-bold tracking-tight">{course.title}</h1>
          <p className="mt-3 max-w-xl text-muted">
            Learn Rust by reading and predicting: short explanations, discovery
            questions, step-through traces, and visualizations. No toolchain
            needed. Follows <em>The Rust Programming Language</em> chapter by
            chapter.
          </p>
          <div className="mt-5 flex flex-wrap items-center gap-3">
            {last ? (
              <>
                <a href={`#${last.id}`} className="primary font-semibold">
                  Continue: {last.sub.number} {last.sub.title} →
                </a>
                <a href={`#${first.id}`} className="text-link text-sm">
                  Start from the beginning
                </a>
              </>
            ) : (
              <a href={`#${first.id}`} className="primary font-semibold">
                Start the course →
              </a>
            )}
            <span className="text-sm text-muted">
              {doneCount} / {lessons.length} lessons done
            </span>
          </div>
        </div>
      </section>
      <section className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
        {chapters.map((chapter) => {
          const ids = chapter.subchapters.map((sub) => `${chapter.id}/${sub.id}`)
          const done = ids.filter((id) => progress.completed.includes(id)).length
          return (
            <a
              key={chapter.id}
              href={`#${ids[0]}`}
              className="rounded-2xl border border-edge bg-ink-soft p-5 transition hover:border-crab"
            >
              <div className="flex items-center gap-2 text-xs uppercase tracking-wide text-muted">
                <ProgressRing done={done} total={ids.length} />
                {label(chapter)}
              </div>
              <div className="mt-1 font-semibold">{chapter.title}</div>
              <div className="mt-2 text-sm text-muted">
                {ids.length} lessons · {done} done
              </div>
            </a>
          )
        })}
      </section>
    </div>
  )
}
