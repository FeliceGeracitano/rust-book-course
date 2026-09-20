import { beforeEach, expect, it, vi } from 'vitest'
import { render, screen, within } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { Quiz, Trace } from './Discovery'
import { reloadProgress, getProgress } from '../progress'
vi.mock('./CodeBlock', () => ({
  default: ({ children }: { children: string }) => <pre>{children}</pre>,
}))
beforeEach(() => {
  localStorage.clear()
  reloadProgress()
})
const quiz = {
  id: 'q',
  question: 'What changes?',
  options: ['Ownership', 'Nothing'],
  answer: 0,
  explain: 'The String moves.',
}
it('gives feedback, locks the attempt, supports retry and restores the answer', async () => {
  const user = userEvent.setup()
  const first = render(<Quiz data={quiz} lessonId="a" />)
  await user.click(screen.getByRole('button', { name: 'B. Nothing' }))
  expect(screen.getByRole('status')).toHaveTextContent('Not quite')
  expect(screen.getByRole('status')).toHaveTextContent('The String moves.')
  expect(screen.getByRole('button', { name: /A. Ownership/ })).toBeDisabled()
  await user.click(screen.getByRole('button', { name: 'Try again' }))
  await user.click(screen.getByRole('button', { name: 'A. Ownership' }))
  expect(screen.getByRole('status')).toHaveTextContent('Correct')
  expect(getProgress().completed).toEqual([])
  first.unmount()
  render(<Quiz data={quiz} lessonId="a" />)
  expect(screen.getByRole('status')).toHaveTextContent('2 attempts')
})
it('does not share answers between lessons with the same question ID', async () => {
  render(
    <>
      <Quiz data={quiz} lessonId="a" />
      <Quiz data={quiz} lessonId="b" />
    </>,
  )
  const [a, b] = screen.getAllByRole('region', { name: 'What changes?' })
  await userEvent.click(within(a).getByRole('button', { name: 'A. Ownership' }))
  expect(within(b).queryByRole('status')).not.toBeInTheDocument()
})
it('steps forward, back and restarts without accumulating stale output', async () => {
  render(
    <Trace
      data={{
        title: 'Move',
        code: 'let a = 1;\nprintln!("{a}");',
        steps: [
          { line: 1, note: 'Bind a.', state: { a: '1' }, output: '' },
          { line: 2, note: 'Print a.', state: { a: '1' }, output: '1\n' },
        ],
      }}
    />,
  )
  expect(screen.getByRole('button', { name: 'Previous step' })).toBeDisabled()
  await userEvent.click(screen.getByRole('button', { name: 'Next step' }))
  expect(screen.getByText('Print a.')).toBeInTheDocument()
  expect(screen.getByRole('button', { name: 'Next step' })).toBeDisabled()
  await userEvent.click(screen.getByRole('button', { name: 'Previous step' }))
  expect(screen.getByText('(no output)')).toBeInTheDocument()
  await userEvent.click(screen.getByRole('button', { name: 'Next step' }))
  await userEvent.click(screen.getByRole('button', { name: 'Restart' }))
  expect(screen.getByText('Bind a.')).toBeInTheDocument()
})
