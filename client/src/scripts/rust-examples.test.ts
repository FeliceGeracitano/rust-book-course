import { describe, expect, it } from 'vitest'
import { classify, collect } from '../../scripts/rust-examples.mjs'

describe('classify', () => {
  it('detects programs with declared output', () => {
    const result = classify('fn main() {\n    println!("hi");\n}\n// Output:\n// hi')
    expect(result).toMatchObject({ kind: 'program', expected: 'hi' })
    expect((result as { code: string }).code).not.toContain('// Output')
  })
  it('detects programs without output, tests, compile-fail, excerpts, and fragments', () => {
    expect(classify('fn main() {}')).toMatchObject({ kind: 'program', expected: null })
    expect(classify('#[test]\nfn adds() { assert_eq!(1 + 1, 2); }')).toMatchObject({ kind: 'test' })
    expect(classify('#[tokio::test]\nasync fn waits() {}')).toMatchObject({ kind: 'test' })
    expect(
      classify(
        "// compile-fail: E0499\nfn main() { let mut s = String::new(); let a = &mut s; let b = &mut s; a.push('x'); b.push('y'); }",
      ),
    ).toMatchObject({ kind: 'compile-fail', errorCode: 'E0499' })
    expect(
      classify('// Source: examples/large-service/crates/money/src/lib.rs\npub struct Cents(i64);'),
    ).toEqual({
      kind: 'excerpt',
      code: 'pub struct Cents(i64);',
      source: 'examples/large-service/crates/money/src/lib.rs',
    })
    expect(classify('let x = 1;')).toEqual({ kind: 'skip' })
    expect(classify('// no-check\nfn main() {}')).toEqual({ kind: 'skip' })
  })
})

it('collect returns rust fences with line numbers and ignores other languages', () => {
  const markdown = '# T\n\n```bash\ncargo test\n```\n\n```rust\nfn main() {}\n```\n'
  expect(collect(markdown, 'x.md')).toEqual([
    { file: 'x.md', line: 7, kind: 'program', code: 'fn main() {}', expected: null },
  ])
})
