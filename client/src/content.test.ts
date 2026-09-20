import { describe, expect, it } from 'vitest'
import { unified } from 'unified'
import remarkParse from 'remark-parse'
import type { Root, RootContent } from 'mdast'
import {
  course,
  lessons,
  markdownFiles,
  loadLesson,
  parseQuiz,
  parseTrace,
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
  it('preserves all 87 lessons with unique stable paths and no orphaned content', () => {
    expect(course.chapters).toHaveLength(22)
    expect(lessons).toHaveLength(87)
    expect(new Set(course.chapters.map((ch) => ch.id)).size).toBe(
      course.chapters.length,
    )
    expect(new Set(lessons.map((l) => l.id)).size).toBe(lessons.length)
    const paths = Object.keys(markdownFiles)
      .filter((path) => !path.endsWith('/README.md'))
      .sort()
    expect(paths).toEqual(lessons.map((l) => `../../content/${l.id}.md`).sort())
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
