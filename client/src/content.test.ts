import { describe, expect, it } from 'vitest'
import { unified } from 'unified'
import remarkParse from 'remark-parse'
import type { Root, RootContent } from 'mdast'
import {
  course,
  chapters,
  lessons,
  markdownFiles,
  loadLesson,
  parseQuiz,
  parseTrace,
  CourseSchema,
} from './content'

function blocks(markdown: string) {
  const found: { lang: string; value: string }[] = []
  function visit(node: Root | RootContent) {
    if (node.type === 'code')
      found.push({ lang: node.lang ?? '', value: node.value })
    if ('children' in node) node.children.forEach(visit)
  }
  visit(unified().use(remarkParse).parse(markdown))
  return found
}

describe('published curriculum', () => {
  it('keeps the 87 Book lessons first with unchanged ids, then the pattern chapters', () => {
    expect(course.parts.map((part) => part.id)).toEqual(['lessons', 'patterns'])
    expect(course.parts.map((part) => part.title)).toEqual([
      'Lessons',
      'Patterns & use cases',
    ])
    expect(chapters).toHaveLength(23)
    expect(lessons).toHaveLength(93)
    expect(lessons.slice(0, 87).every((l) => l.part.id === 'lessons')).toBe(true)
    expect(lessons.slice(87).every((l) => l.part.id === 'patterns')).toBe(true)
    expect(lessons[0].id).toBe('ch01_getting_started/installation')
    expect(lessons[86].id).toBe('appendix/g_nightly')
    expect(lessons[87].id).toBe('patterns_large_service/boundaries')
    expect(lessons[87].sub.problem).toMatch(/crate boundaries/)
    expect(new Set(chapters.map((ch) => ch.id)).size).toBe(chapters.length)
    expect(new Set(lessons.map((l) => l.id)).size).toBe(lessons.length)
    const paths = Object.keys(markdownFiles)
      .filter((path) => !path.endsWith('/README.md'))
      .sort()
    expect(paths).toEqual(lessons.map((l) => `../../content/${l.id}.md`).sort())
  })
  it('accepts unnumbered chapters with problem and ref, and rejects bad refs', () => {
    const part = {
      id: 'patterns',
      title: 'Patterns & use cases',
      chapters: [
        {
          id: 'patterns_errors',
          title: 'Errors',
          subchapters: [
            {
              id: 'wrapping',
              title: 'Wrapping',
              problem: 'Add context.',
              ref: 'https://docs.rs/anyhow',
            },
          ],
        },
      ],
    }
    expect(() =>
      CourseSchema.parse({ title: 'x', parts: [part] }),
    ).not.toThrow()
    const bad = structuredClone(part)
    bad.chapters[0].subchapters[0].ref = 'not a url'
    expect(() => CourseSchema.parse({ title: 'x', parts: [bad] })).toThrow()
    expect(() =>
      CourseSchema.parse({
        title: 'x',
        parts: [{ id: 'p', title: 'P', chapters: [] }],
      }),
    ).toThrow()
  })
  it.each(lessons)(
    '$id loads with valid interactive discovery content',
    async (lesson) => {
      const markdown = await loadLesson(lesson.id)
      expect(markdown).toMatch(/^# /)
      const widgets = blocks(markdown)
      const quizzes = widgets
        .filter((block) => block.lang === 'quiz')
        .map((block) => parseQuiz(block.value))
      expect(quizzes.length).toBeGreaterThan(0)
      expect(new Set(quizzes.map((q) => q.id)).size).toBe(quizzes.length)
      for (const quiz of quizzes)
        expect(new Set(quiz.options).size).toBe(quiz.options.length)
      for (const trace of widgets.filter((block) => block.lang === 'trace'))
        expect(() => parseTrace(trace.value)).not.toThrow()
      expect(markdown).not.toMatch(
        /hit.*Check|Docker container|chapter shows ✅/i,
      )
    },
  )
  it('rejects missing lessons and malformed discovery data', async () => {
    await expect(loadLesson('unknown/lesson')).rejects.toThrow(
      'Lesson not found',
    )
    expect(() => parseQuiz('not JSON')).toThrow()
    expect(() =>
      parseQuiz(
        JSON.stringify({
          id: 'a',
          question: '?',
          options: ['a', 'b'],
          answer: 2,
          explain: 'why',
        }),
      ),
    ).toThrow()
    expect(() =>
      parseTrace(
        JSON.stringify({
          title: 't',
          code: 'line',
          steps: [{ line: 2, note: 'n', state: {} }],
        }),
      ),
    ).toThrow()
    expect(() =>
      parseTrace(JSON.stringify({ title: 't', code: 'line', steps: [] })),
    ).toThrow()
  })
})
