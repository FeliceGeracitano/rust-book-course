import { beforeEach, expect, it, vi } from 'vitest'
import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { lessons, loadLesson } from '../content'
import LessonView from './LessonView'
import { reloadProgress } from '../progress'
vi.mock('../content', async (importOriginal) => ({
  ...(await importOriginal<typeof import('../content')>()),
  loadLesson: vi.fn(),
}))
beforeEach(() => {
  vi.mocked(loadLesson).mockReset()
  localStorage.clear()
  reloadProgress()
})
it('offers recovery when a static lesson chunk fails to load', async () => {
  vi.mocked(loadLesson)
    .mockRejectedValueOnce(new Error('offline'))
    .mockResolvedValueOnce('# Restored lesson')
  render(<LessonView lesson={lessons[0]} />)
  expect(await screen.findByRole('alert')).toHaveTextContent(
    'Could not load this lesson',
  )
  expect(
    screen.queryByRole('button', { name: 'Mark lesson complete' }),
  ).not.toBeInTheDocument()
  await userEvent.click(
    screen.getByRole('button', { name: 'Retry loading lesson' }),
  )
  expect(
    await screen.findByRole('heading', { name: 'Restored lesson' }),
  ).toBeInTheDocument()
})
it('shows a local error for a malformed block without losing the lesson', async () => {
  vi.mocked(loadLesson).mockResolvedValue('# Lesson\n\n```quiz\nnot JSON\n```')
  render(<LessonView lesson={lessons[0]} />)
  expect(await screen.findByRole('alert')).toHaveTextContent(
    'This discovery block could not be loaded',
  )
  expect(screen.getByRole('heading', { name: 'Lesson' })).toBeInTheDocument()
})
