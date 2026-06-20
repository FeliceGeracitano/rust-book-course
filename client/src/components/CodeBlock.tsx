import { ReactNode, useEffect, useState } from 'react'
import { getHighlighter, langOrText, THEME } from '../highlighter'

// react-markdown renders this for every <code>. Inline code (no language class)
// is styled simply; fenced blocks get Shiki highlighting + a copy button.
export default function CodeBlock({
  className,
  children,
}: {
  className?: string
  children?: ReactNode
}) {
  const match = /language-(\w+)/.exec(className || '')
  if (!match) return <InlineCode>{children}</InlineCode>
  const text = String(children ?? '').replace(/\n$/, '')
  return <BlockCode lang={match[1]} text={text} />
}

function InlineCode({ children }: { children?: ReactNode }) {
  return (
    <code className="rounded bg-ink-soft px-1.5 py-0.5 font-mono text-[0.85em] text-crab">
      {children}
    </code>
  )
}

function BlockCode({ lang, text }: { lang: string; text: string }) {
  const [html, setHtml] = useState<string>('')
  const [copied, setCopied] = useState(false)

  useEffect(() => {
    let alive = true
    getHighlighter()
      .then((hl) => {
        if (alive) setHtml(hl.codeToHtml(text, { lang: langOrText(lang), theme: THEME }))
      })
      .catch(() => alive && setHtml(''))
    return () => {
      alive = false
    }
  }, [text, lang])

  function copy() {
    navigator.clipboard.writeText(text).then(() => {
      setCopied(true)
      setTimeout(() => setCopied(false), 1200)
    })
  }

  return (
    <div className="group relative my-4">
      <button
        onClick={copy}
        className="absolute right-2 top-2 z-10 rounded-md border border-edge bg-ink-soft px-2 py-1 text-xs text-muted opacity-0 transition group-hover:opacity-100 hover:text-paper"
      >
        {copied ? 'copied' : 'copy'}
      </button>
      {html ? (
        <div
          className="overflow-x-auto rounded-xl border border-edge text-sm [&_pre]:m-0 [&_pre]:rounded-xl [&_pre]:p-4"
          dangerouslySetInnerHTML={{ __html: html }}
        />
      ) : (
        <pre className="overflow-x-auto rounded-xl border border-edge bg-ink-soft p-4 text-sm">
          <code className="font-mono">{text}</code>
        </pre>
      )}
    </div>
  )
}
