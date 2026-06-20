import { ReactNode, useEffect, useState } from 'react'
import { getHighlighter, langOrText, THEME } from '../highlighter'

// Copy with the async Clipboard API when available (needs a secure context),
// falling back to a hidden textarea + execCommand for everything else.
async function copyText(text: string): Promise<boolean> {
  try {
    if (navigator.clipboard?.writeText) {
      await navigator.clipboard.writeText(text)
      return true
    }
  } catch {
    // fall through to the legacy path
  }
  try {
    const ta = document.createElement('textarea')
    ta.value = text
    ta.style.position = 'fixed'
    ta.style.opacity = '0'
    document.body.appendChild(ta)
    ta.focus()
    ta.select()
    const ok = document.execCommand('copy')
    document.body.removeChild(ta)
    return ok
  } catch {
    return false
  }
}

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

  async function copy() {
    const ok = await copyText(text)
    if (ok) {
      setCopied(true)
      setTimeout(() => setCopied(false), 1400)
    }
  }

  return (
    <div className="group relative my-4">
      <button
        onClick={copy}
        aria-label="Copy code"
        className={`absolute right-2 top-2 z-10 rounded-md border px-2 py-1 text-xs font-medium opacity-70 transition group-hover:opacity-100 ${
          copied
            ? 'border-ok/50 bg-ok/15 text-ok'
            : 'border-edge bg-ink-soft text-muted hover:text-paper'
        }`}
      >
        {copied ? '✓ Copied' : 'Copy'}
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
