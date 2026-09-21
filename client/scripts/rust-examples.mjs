// Pure classification of the ```rust fences in Part 2 / Part 3 lessons.
// The markers are documented in content/README.md; the runner is check-rust-examples.mjs.
import { unified } from 'unified'
import remarkParse from 'remark-parse'

const OUTPUT_TRAILER = /\n\/\/ Output:\s*\n([\s\S]*)$/

export function stripOutput(code) {
  const match = code.match(OUTPUT_TRAILER)
  if (!match) return { code: code.trimEnd(), expected: null }
  const expected = match[1]
    .split('\n')
    .map((line) => line.replace(/^\/\/ ?/, ''))
    .join('\n')
    .trimEnd()
  return { code: code.slice(0, match.index).trimEnd(), expected }
}

export function classify(raw) {
  const code = raw.replace(/\r\n/g, '\n')
  const first = code.split('\n')[0].trim()
  if (first === '// no-check') return { kind: 'skip' }
  const compileFail = first.match(/^\/\/ compile-fail: (E\d{4})$/)
  if (compileFail) return { kind: 'compile-fail', code: code.trimEnd(), errorCode: compileFail[1] }
  const source = first.match(/^\/\/ Source: (examples\/large-service\/\S+)$/)
  if (source) {
    return { kind: 'excerpt', code: code.split('\n').slice(1).join('\n').trimEnd(), source: source[1] }
  }
  if (/^\s*fn main\s*\(/m.test(code)) {
    const { code: body, expected } = stripOutput(code)
    return { kind: 'program', code: body, expected }
  }
  if (/^\s*#\[(tokio::)?test\]/m.test(code)) return { kind: 'test', code: code.trimEnd() }
  return { kind: 'skip' }
}

export function collect(markdown, file) {
  const found = []
  const visit = (node) => {
    if (node.type === 'code' && node.lang === 'rust') {
      const item = classify(node.value)
      if (item.kind !== 'skip') found.push({ file, line: node.position.start.line, ...item })
    }
    for (const child of node.children ?? []) visit(child)
  }
  visit(unified().use(remarkParse).parse(markdown))
  return found
}
