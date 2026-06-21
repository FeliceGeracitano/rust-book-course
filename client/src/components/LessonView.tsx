import { useEffect, useState } from 'react'
import Markdown from 'react-markdown'
import remarkGfm from 'remark-gfm'
import { getLesson } from '../api'
import { Selection } from '../types'
import CodeBlock from './CodeBlock'
import { vizFor } from './viz/registry'

type Status = 'loading' | 'ok' | 'missing'

export default function LessonView({ selection }: { selection: Selection }) {
  const { chapter, sub } = selection
  const Viz = vizFor(chapter.crate)
  // Content dirs are named by crate; the appendix has no crate so it uses its id.
  const contentKey = chapter.crate ?? chapter.id
  const [markdown, setMarkdown] = useState('')
  const [status, setStatus] = useState<Status>('loading')

  useEffect(() => {
    if (!sub) {
      setStatus('missing')
      setMarkdown('')
      return
    }
    setStatus('loading')
    let alive = true
    getLesson(contentKey, sub.id)
      .then((md) => {
        if (!alive) return
        setMarkdown(md)
        setStatus('ok')
      })
      .catch(() => alive && setStatus('missing'))
    return () => {
      alive = false
    }
  }, [contentKey, sub?.id])

  return (
    <div className="min-h-0 flex-1 overflow-y-auto px-8 py-6">
      <div className="mx-auto max-w-3xl">
        <div className="mb-4 font-mono text-xs uppercase tracking-wider text-crab">
          Chapter {chapter.number} · {chapter.title}
        </div>

        {Viz && <Viz />}

        {status === 'loading' && <p className="text-muted">Loading lesson…</p>}

        {status === 'missing' && (
          <div className="prose-rust">
            <h1>{sub ? `${sub.number} ${sub.title}` : chapter.title}</h1>
            <p className="text-muted">
              {chapter.crate ? (
                <>
                  Lesson text for this section is coming soon. You can still work the
                  exercise — edit <code>chapters/{chapter.crate}/src/lib.rs</code>, make
                  its tests pass, then hit <strong>Check</strong>.
                </>
              ) : (
                <>This is reference reading — content coming soon.</>
              )}
            </p>
          </div>
        )}

        {status === 'ok' && (
          <article className="prose-rust">
            <Markdown
              remarkPlugins={[remarkGfm]}
              components={{
                pre: ({ children }) => <>{children}</>,
                code: CodeBlock as never,
              }}
            >
              {markdown}
            </Markdown>
          </article>
        )}
      </div>
    </div>
  )
}
