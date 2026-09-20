import { course } from '../content'
import { useProgress } from '../progress'

export default function ChapterTree({ selectedId }: { selectedId: string }) {
  const { completed } = useProgress()
  return (
    <nav
      aria-label="Course chapters"
      className="w-full overflow-y-auto border-r border-edge bg-ink-soft px-3 py-5"
    >
      <p className="eyebrow px-2 pb-4">Your Rust journey</p>
      {course.chapters.map((chapter) => {
        const active = selectedId.startsWith(`${chapter.id}/`)
        const done = chapter.subchapters.filter((sub) =>
          completed.includes(`${chapter.id}/${sub.id}`),
        ).length
        return (
          <details key={chapter.id} open={active || undefined} className="mb-2">
            <summary
              className={`flex cursor-pointer items-center gap-2 rounded-lg px-2 py-2 text-sm ${active ? 'bg-ink-card text-crab' : 'text-muted'}`}
            >
              <span className="w-6 shrink-0 font-mono text-xs">
                {chapter.id === 'appendix'
                  ? 'A'
                  : String(chapter.number).padStart(2, '0')}
              </span>
              <span className="flex-1">{chapter.title}</span>
              <span
                className="text-xs"
                aria-label={`${done} of ${chapter.subchapters.length} complete`}
              >
                {done === chapter.subchapters.length
                  ? '✓'
                  : `${done}/${chapter.subchapters.length}`}
              </span>
            </summary>
            <ul className="ml-5 mt-1 border-l border-edge pl-2">
              {chapter.subchapters.map((sub) => {
                const id = `${chapter.id}/${sub.id}`
                return (
                  <li key={id}>
                    <a
                      href={`#${id}`}
                      aria-current={selectedId === id ? 'page' : undefined}
                      className={`block rounded px-2 py-2 text-xs leading-relaxed ${selectedId === id ? 'bg-rust/10 text-crab' : 'text-muted hover:text-paper'}`}
                    >
                      <span className="font-mono">{sub.number}</span>{' '}
                      {sub.title}
                      {completed.includes(id) && (
                        <span className="ml-2 text-ok" aria-label="Completed">
                          ✓
                        </span>
                      )}
                    </a>
                  </li>
                )
              })}
            </ul>
          </details>
        )
      })}
      <p className="mt-6 px-2 text-xs leading-relaxed text-muted">
        Progress is saved in this browser. Explore at your own pace.
      </p>
    </nav>
  )
}
