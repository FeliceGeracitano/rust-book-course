import { useState } from 'react'
import { course, type Chapter } from '../content'
import { useProgress } from '../progress'
import ProgressRing from './ProgressRing'

const allChapterIds = course.chapters.map((chapter) => chapter.id)
const label = (chapter: Chapter) =>
  chapter.id === 'appendix' ? 'A' : String(chapter.number)

export default function ChapterTree({ selectedId }: { selectedId: string }) {
  const chapterId = selectedId.split('/')[0]
  const { completed } = useProgress()
  const [open, setOpen] = useState(() => new Set([chapterId]))
  // Expand the chapter being navigated into. Adjusting state during render
  // (rather than in an effect) avoids a frame where it is still collapsed.
  const [seenChapter, setSeenChapter] = useState(chapterId)
  if (chapterId !== seenChapter) {
    setSeenChapter(chapterId)
    if (!open.has(chapterId)) setOpen(new Set(open).add(chapterId))
  }
  const toggle = (id: string) =>
    setOpen((current) => {
      const next = new Set(current)
      if (!next.delete(id)) next.add(id)
      return next
    })
  const allOpen = allChapterIds.every((id) => open.has(id))
  const noneOpen = allChapterIds.every((id) => !open.has(id))
  const done = (id: string) => completed.includes(id)

  return (
    <nav
      aria-label="Course chapters"
      className="w-full overflow-y-auto border-r border-edge bg-ink-soft px-3 py-4 text-sm"
    >
      <div className="mb-2 flex items-center justify-end gap-1 px-1 text-xs text-muted">
        <BulkButton
          onClick={() => setOpen(new Set(allChapterIds))}
          disabled={allOpen}
        >
          Expand all
        </BulkButton>
        <span aria-hidden="true">·</span>
        <BulkButton onClick={() => setOpen(new Set())} disabled={noneOpen}>
          Collapse all
        </BulkButton>
      </div>
      <p className="eyebrow mb-1 px-2">Your Rust journey</p>
      {course.chapters.map((chapter) => (
        <ChapterBlock
          key={chapter.id}
          chapter={chapter}
          open={open.has(chapter.id)}
          onToggle={() => toggle(chapter.id)}
          selectedId={selectedId}
          done={done}
        />
      ))}
      <p className="mt-6 px-2 text-xs leading-relaxed text-muted">
        Progress is saved in this browser. Explore at your own pace.
      </p>
    </nav>
  )
}

function BulkButton({
  onClick,
  disabled,
  children,
}: {
  onClick: () => void
  disabled: boolean
  children: string
}) {
  return (
    <button
      type="button"
      onClick={onClick}
      disabled={disabled}
      className="rounded px-1.5 py-0.5 hover:bg-ink-card hover:text-paper disabled:opacity-40 disabled:hover:bg-transparent disabled:hover:text-muted"
    >
      {children}
    </button>
  )
}

function ChapterBlock({
  chapter,
  open,
  onToggle,
  selectedId,
  done,
}: {
  chapter: Chapter
  open: boolean
  onToggle: () => void
  selectedId: string
  done: (lessonId: string) => boolean
}) {
  const lessonId = (subId: string) => `${chapter.id}/${subId}`
  const doneCount = chapter.subchapters.filter((sub) =>
    done(lessonId(sub.id)),
  ).length
  const active = selectedId.startsWith(`${chapter.id}/`)
  return (
    <div>
      <button
        type="button"
        onClick={onToggle}
        aria-expanded={open}
        className={`flex w-full items-center gap-2 rounded-lg px-2 py-1.5 text-left hover:bg-ink-card ${active ? 'text-crab' : 'text-paper'}`}
      >
        <ProgressRing done={doneCount} total={chapter.subchapters.length} />
        <span className="min-w-0 flex-1">
          {label(chapter)}. {chapter.title}
        </span>
        <span className="text-muted" aria-hidden="true">
          {open ? '▾' : '▸'}
        </span>
      </button>
      {open && (
        <ul className="mb-1 ml-3 border-l border-edge pl-2">
          {chapter.subchapters.map((sub) => {
            const id = lessonId(sub.id)
            const isDone = done(id)
            const current = selectedId === id
            return (
              <li key={id}>
                <a
                  href={`#${id}`}
                  aria-current={current ? 'page' : undefined}
                  data-done={isDone}
                  className={`flex items-start gap-2 rounded-md px-2 py-1 text-xs leading-relaxed ${current ? 'bg-rust/10 text-crab' : 'text-muted hover:bg-ink-card hover:text-paper'}`}
                >
                  <span
                    className={`w-3 shrink-0 text-center ${isDone ? 'text-ok' : 'text-edge'}`}
                    aria-hidden="true"
                  >
                    {isDone ? '✓' : '·'}
                  </span>
                  <span className="min-w-0">
                    <span className="font-mono">{sub.number}</span> {sub.title}
                  </span>
                </a>
              </li>
            )
          })}
        </ul>
      )}
    </div>
  )
}
