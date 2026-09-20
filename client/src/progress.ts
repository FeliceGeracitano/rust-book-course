import { useSyncExternalStore } from 'react'
import { z } from 'zod'

export const STORAGE_KEY = 'rust-book-course:discovery:v1'
const Schema = z.object({
  completed: z.array(z.string()),
  answers: z.record(
    z.string(),
    z.object({
      choice: z.number().int().min(0),
      attempts: z.number().int().min(1),
    }),
  ),
  lastLesson: z.string().nullable(),
})
type Progress = z.infer<typeof Schema>
const empty = (): Progress => ({ completed: [], answers: {}, lastLesson: null })
function read(): Progress {
  try {
    return Schema.parse(JSON.parse(localStorage.getItem(STORAGE_KEY) ?? 'null'))
  } catch {
    return empty()
  }
}
let state = read()
const listeners = new Set<() => void>()
function commit(next: Progress) {
  state = next
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(state))
  } catch {
    /* Keep progress in memory if storage is unavailable. */
  }
  listeners.forEach((listener) => listener())
}
export function getProgress() {
  return state
}
export function reloadProgress() {
  state = read()
  listeners.forEach((listener) => listener())
}
export function markComplete(id: string, complete: boolean) {
  commit({
    ...state,
    completed: complete
      ? [...new Set([...state.completed, id])]
      : state.completed.filter((value) => value !== id),
  })
}
export function rememberLesson(id: string) {
  if (state.lastLesson !== id) commit({ ...state, lastLesson: id })
}
export function recordAnswer(key: string, choice: number) {
  commit({
    ...state,
    answers: {
      ...state.answers,
      [key]: { choice, attempts: (state.answers[key]?.attempts ?? 0) + 1 },
    },
  })
}
export function useProgress() {
  return useSyncExternalStore(
    (listener) => {
      listeners.add(listener)
      return () => {
        listeners.delete(listener)
      }
    },
    getProgress,
    getProgress,
  )
}
