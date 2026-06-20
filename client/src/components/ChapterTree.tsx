import { Course } from '../api'
import { Selection } from '../types'

export default function ChapterTree({
  course,
  selection,
  progress,
  onSelect,
}: {
  course: Course
  selection: Selection
  progress: Record<string, boolean>
  onSelect: (s: Selection) => void
}) {
  return (
    <nav className="w-80 shrink-0 overflow-y-auto border-r border-edge bg-ink-soft px-2 py-3">
      {course.chapters.map((ch) => {
        const done = ch.crate ? progress[ch.crate] : false
        const active = selection.chapter.id === ch.id
        return (
          <div key={ch.id} className="mb-1">
            <button
              onClick={() => onSelect({ chapter: ch, sub: ch.subchapters[0] ?? null })}
              className={`flex w-full items-center gap-2 rounded-lg px-3 py-2 text-left text-sm transition ${
                active
                  ? 'bg-ink-card text-paper'
                  : 'text-muted hover:bg-ink-card/60 hover:text-paper'
              }`}
            >
              <span className="w-6 font-mono text-xs text-crab">{ch.number}</span>
              <span className="flex-1">{ch.title}</span>
              {done && <span className="text-ok">✓</span>}
            </button>
            {active && ch.subchapters.length > 0 && (
              <div className="mt-0.5 ml-5 border-l border-edge pl-2">
                {ch.subchapters.map((sub) => {
                  const subActive = selection.sub?.id === sub.id
                  return (
                    <button
                      key={sub.id}
                      onClick={() => onSelect({ chapter: ch, sub })}
                      className={`block w-full rounded-md px-2 py-1 text-left text-xs transition ${
                        subActive ? 'text-rust-bright' : 'text-muted hover:text-paper'
                      }`}
                    >
                      <span className="font-mono">{sub.number}</span> {sub.title}
                    </button>
                  )
                })}
              </div>
            )}
          </div>
        )
      })}
    </nav>
  )
}
