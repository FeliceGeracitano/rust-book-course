import { beforeEach, describe, expect, it, vi } from 'vitest'
import {
  getProgress,
  markComplete,
  recordAnswer,
  reloadProgress,
  rememberLesson,
  STORAGE_KEY,
} from './progress'

beforeEach(() => {
  localStorage.clear()
  reloadProgress()
})
describe('browser progress', () => {
  it('persists answers, attempts, completion and last lesson independently', () => {
    recordAnswer('a#quiz', 1)
    recordAnswer('a#quiz', 0)
    markComplete('a', true)
    markComplete('a', true)
    rememberLesson('b')
    reloadProgress()
    expect(getProgress()).toEqual({
      completed: ['a'],
      answers: { 'a#quiz': { choice: 0, attempts: 2 } },
      lastLesson: 'b',
    })
    markComplete('a', false)
    expect(getProgress().completed).toEqual([])
    expect(getProgress().answers['a#quiz'].attempts).toBe(2)
  })
  it.each([
    '{broken',
    '{}',
    '{"completed":"wrong","answers":{},"lastLesson":null}',
  ])('recovers from invalid stored data: %s', (raw) => {
    localStorage.setItem(STORAGE_KEY, raw)
    reloadProgress()
    expect(getProgress()).toEqual({
      completed: [],
      answers: {},
      lastLesson: null,
    })
  })
  it('keeps working in memory when storage is blocked or full', () => {
    const get = vi
      .spyOn(Storage.prototype, 'getItem')
      .mockImplementation(() => {
        throw new Error('blocked')
      })
    const set = vi
      .spyOn(Storage.prototype, 'setItem')
      .mockImplementation(() => {
        throw new Error('full')
      })
    try {
      reloadProgress()
      markComplete('a', true)
      recordAnswer('a#quiz', 2)
      expect(getProgress().completed).toEqual(['a'])
      expect(getProgress().answers['a#quiz'].choice).toBe(2)
    } finally {
      get.mockRestore()
      set.mockRestore()
    }
  })
})
