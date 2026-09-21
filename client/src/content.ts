import manifest from '../../content/course.json'
import { z } from 'zod'

const Subchapter = z.object({
  id: z.string().regex(/^[a-z0-9_]+$/),
  number: z.string().optional(),
  title: z.string().min(1),
  problem: z.string().min(1).optional(),
  ref: z.string().url().optional(),
})
const Chapter = z.object({
  id: z.string().regex(/^[a-z0-9_]+$/),
  number: z.number().optional(),
  title: z.string().min(1),
  subchapters: z.array(Subchapter).min(1),
})
const Part = z.object({
  id: z.string().regex(/^[a-z0-9_]+$/),
  title: z.string().min(1),
  chapters: z.array(Chapter).min(1),
})
export const CourseSchema = z.object({
  title: z.string(),
  parts: z.array(Part).min(1),
})
export const course = CourseSchema.parse(manifest)
export type Part = z.infer<typeof Part>
export type Chapter = z.infer<typeof Chapter>
export const chapters = course.parts.flatMap((part) => part.chapters)
export function partLabel(part: Part) {
  return part.title === 'Patterns & use cases' ? 'Patterns' : part.title
}
export const lessons = course.parts.flatMap((part) =>
  part.chapters.flatMap((chapter) =>
    chapter.subchapters.map((sub) => ({
      part,
      chapter,
      sub,
      id: `${chapter.id}/${sub.id}`,
    })),
  ),
)
export type Lesson = (typeof lessons)[number]
export const markdownFiles = import.meta.glob<string>(
  ['../../content/**/*.md', '!../../content/README.md'],
  { query: '?raw', import: 'default' },
)

export function loadLesson(id: string): Promise<string> {
  const load = markdownFiles[`../../content/${id}.md`]
  return load ? load() : Promise.reject(new Error('Lesson not found'))
}

const text = z.string().trim().min(1)
export const QuizSchema = z
  .object({
    id: text,
    question: text,
    code: text.optional(),
    options: z.array(text).min(2),
    answer: z.number().int().min(0),
    explain: text,
  })
  .refine((q) => q.answer < q.options.length, 'Answer must reference an option')
export type QuizData = z.infer<typeof QuizSchema>
export const TraceSchema = z
  .object({
    title: text,
    code: text,
    steps: z
      .array(
        z.object({
          line: z.number().int().min(1),
          note: text,
          state: z.record(z.string(), z.string()),
          output: z.string().optional(),
        }),
      )
      .min(1),
  })
  .refine(
    (t) => t.steps.every((s) => s.line <= t.code.split('\n').length),
    'Step line is outside the code',
  )
export type TraceData = z.infer<typeof TraceSchema>

export function parseQuiz(raw: string): QuizData {
  return QuizSchema.parse(JSON.parse(raw))
}
export function parseTrace(raw: string): TraceData {
  return TraceSchema.parse(JSON.parse(raw))
}
