import { beforeEach, expect, it, vi } from 'vitest'
import { act, render, screen, within, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import App from './App'
import { markComplete, reloadProgress, STORAGE_KEY } from './progress'
vi.mock('./components/CodeBlock', () => ({
  default: ({ children }: { children: string }) => <code>{children}</code>,
}))
beforeEach(() => {
  window.history.replaceState(null, '', '/')
  localStorage.clear()
  reloadProgress()
})
async function startCourse() {
  await screen.findByRole('heading', { name: 'Rust Book Course' })
  await userEvent.click(screen.getByRole('link', { name: /Start the course/ }))
  await screen.findByRole('heading', { name: '1.1 Installation' })
}
it('loads without an API, navigates, saves progress and offers to continue after remount', async () => {
  const fetch = vi
    .spyOn(window, 'fetch')
    .mockRejectedValue(new Error('No backend'))
  try {
    const user = userEvent.setup()
    const app = render(<App />)
    await startCourse()
    expect(window.location.hash).toBe('#ch01_getting_started/installation')
    await user.click(screen.getByRole('button', { name: 'A. rustup' }))
    expect(screen.getByRole('status')).toHaveTextContent('Correct')
    await user.click(
      screen.getByRole('button', { name: 'Mark lesson complete' }),
    )
    expect(screen.getByText(/1 \/ 93/)).toBeInTheDocument()
    const navigation = screen.getByRole('navigation', {
      name: 'Lesson navigation',
    })
    await user.click(
      within(navigation).getByRole('link', { name: /Next.*Hello, World!/ }),
    )
    await screen.findByRole('heading', { name: '1.2 Hello, World!' })
    expect(screen.queryByRole('status')).not.toBeInTheDocument()
    expect(window.location.hash).toBe('#ch01_getting_started/hello_world')
    expect(fetch).not.toHaveBeenCalled()
    app.unmount()
    window.history.replaceState(null, '', '/')
    reloadProgress()
    render(<App />)
    await user.click(
      await screen.findByRole('link', { name: /Continue: 1\.2 Hello, World!/ }),
    )
    await screen.findByRole('heading', { name: '1.2 Hello, World!' })
    expect(JSON.parse(localStorage.getItem(STORAGE_KEY)!).completed).toEqual([
      'ch01_getting_started/installation',
    ])
  } finally {
    fetch.mockRestore()
  }
})
it('lists every chapter with its progress on the landing page', async () => {
  markComplete('ch01_getting_started/installation', true)
  render(<App />)
  const card = await screen.findByRole('link', { name: /Getting Started/ })
  expect(card).toHaveAttribute('href', '#ch01_getting_started/installation')
  expect(card).toHaveTextContent('3 lessons · 1 done')
  expect(screen.getByRole('link', { name: /Appendix/ })).toHaveAttribute(
    'href',
    '#appendix/a_keywords',
  )
  expect(screen.getByText('1 / 93 lessons done')).toBeInTheDocument()
  expect(
    screen.queryByRole('link', { name: /Start from the beginning/ }),
  ).not.toBeInTheDocument()
})
it('offers to continue from the last lesson or restart from the header link', async () => {
  window.history.replaceState(null, '', '/#ch03_common_concepts/data_types')
  render(<App />)
  await screen.findByRole('heading', { name: '3.2 Data Types' })
  await userEvent.click(screen.getByRole('link', { name: 'Rust Book Course' }))
  await screen.findByRole('heading', { name: 'Rust Book Course' })
  expect(
    screen.getByRole('link', { name: /Continue: 3\.2 Data Types/ }),
  ).toHaveAttribute('href', '#ch03_common_concepts/data_types')
  expect(
    screen.getByRole('link', { name: 'Start from the beginning' }),
  ).toHaveAttribute('href', '#ch01_getting_started/installation')
  await waitFor(() => expect(document.title).toBe('Rust Book Course'))
})
it('opens direct links, reacts to history and recovers from unknown lessons', async () => {
  window.history.replaceState(null, '', '/#ch03_common_concepts/data_types')
  render(<App />)
  await screen.findByRole('heading', { name: '3.2 Data Types' })
  act(() => {
    window.location.hash = 'does-not-exist'
    window.dispatchEvent(new HashChangeEvent('hashchange'))
  })
  expect(
    screen.getByRole('heading', { name: 'Lesson not found' }),
  ).toBeInTheDocument()
  await userEvent.click(screen.getByRole('link', { name: 'Start the course' }))
  await screen.findByRole('heading', { name: '1.1 Installation' })
  await waitFor(() =>
    expect(document.title).toBe('Installation · Rust Book Course'),
  )
})
it('allows completion to be undone and exposes the mobile chapter menu state', async () => {
  render(<App />)
  await startCourse()
  await userEvent.click(screen.getByRole('button', { name: 'Toggle chapters' }))
  expect(
    screen.getByRole('button', { name: 'Toggle chapters' }),
  ).toHaveAttribute('aria-expanded', 'true')
  await userEvent.click(screen.getByRole('button', { name: 'Close chapters' }))
  expect(
    screen.getByRole('button', { name: 'Toggle chapters' }),
  ).toHaveAttribute('aria-expanded', 'false')
  await userEvent.click(
    screen.getByRole('button', { name: 'Mark lesson complete' }),
  )
  await userEvent.click(
    screen.getByRole('button', { name: /Completed · mark incomplete/ }),
  )
  expect(screen.getByText(/0 \/ 93/)).toBeInTheDocument()
})
it('preserves the active trace step when quiz and completion progress change', async () => {
  window.history.replaceState(null, '', '/#ch04_ownership/what_is_ownership')
  render(<App />)
  await screen.findByRole('heading', { name: '4.1 What is Ownership?' })
  await userEvent.click(screen.getByRole('button', { name: 'Next step' }))
  expect(screen.getByText('2 / 4')).toBeInTheDocument()
  const quiz = screen.getByRole('region', {
    name: 'Which binding can still be used after this move?',
  })
  await userEvent.click(
    within(quiz).getByRole('button', { name: /second only/ }),
  )
  expect(screen.getByText('2 / 4')).toBeInTheDocument()
  await userEvent.click(
    screen.getByRole('button', { name: 'Mark lesson complete' }),
  )
  expect(screen.getByText('2 / 4')).toBeInTheDocument()
})
it('walks Back through lessons to the landing page', async () => {
  render(<App />)
  await startCourse()
  await userEvent.click(
    within(
      screen.getByRole('navigation', { name: 'Lesson navigation' }),
    ).getByRole('link', { name: /Next.*Hello, World!/ }),
  )
  await screen.findByRole('heading', { name: '1.2 Hello, World!' })
  act(() => window.history.back())
  await screen.findByRole('heading', { name: '1.1 Installation' })
  act(() => window.history.back())
  await screen.findByRole('heading', { name: 'Rust Book Course' })
  expect(
    screen.getByRole('link', { name: /Continue: 1\.1 Installation/ }),
  ).toBeInTheDocument()
})
it('collapses the desktop sidebar and remembers the choice across reloads', async () => {
  const app = render(<App />)
  await screen.findByRole('heading', { name: 'Rust Book Course' })
  expect(
    screen.getByRole('navigation', { name: 'Course chapters' }),
  ).toBeInTheDocument()
  const toggle = screen.getByRole('button', { name: 'Toggle sidebar' })
  expect(toggle).toHaveAttribute('aria-expanded', 'true')
  await userEvent.click(toggle)
  expect(toggle).toHaveAttribute('aria-expanded', 'false')
  expect(
    screen.queryByRole('navigation', { name: 'Course chapters' }),
  ).not.toBeInTheDocument()
  expect(localStorage.getItem('rust-book-course:sidebar')).toBe('closed')
  app.unmount()
  render(<App />)
  await screen.findByRole('heading', { name: 'Rust Book Course' })
  expect(
    screen.getByRole('button', { name: 'Toggle sidebar' }),
  ).toHaveAttribute('aria-expanded', 'false')
  expect(
    screen.queryByRole('navigation', { name: 'Course chapters' }),
  ).not.toBeInTheDocument()
})
it('closes the mobile chapter drawer after choosing a lesson', async () => {
  render(<App />)
  await startCourse()
  await userEvent.click(screen.getByRole('button', { name: 'Toggle chapters' }))
  const drawer = document.getElementById('chapter-drawer')!
  await userEvent.click(
    within(drawer).getByRole('link', { name: /1\.2 Hello, World!/ }),
  )
  await screen.findByRole('heading', { name: '1.2 Hello, World!' })
  expect(
    screen.getByRole('button', { name: 'Toggle chapters' }),
  ).toHaveAttribute('aria-expanded', 'false')
  expect(document.getElementById('chapter-drawer')).toBeNull()
})
it('links to each part from the header and marks the current one', async () => {
  window.history.replaceState(null, '', '/#ch03_common_concepts/data_types')
  render(<App />)
  await screen.findByRole('heading', { name: '3.2 Data Types' })
  const nav = screen.getByRole('navigation', { name: 'Course parts' })
  const link = within(nav).getByRole('link', { name: 'Lessons' })
  expect(link).toHaveAttribute('href', '#ch01_getting_started/installation')
  expect(link).toHaveAttribute('aria-current', 'true')
})
it('shows one card per part on the landing page', async () => {
  render(<App />)
  const parts = await screen.findByRole('region', { name: 'Parts' })
  const card = within(parts).getByRole('link', { name: /Part 1.*Lessons/ })
  expect(card).toHaveTextContent('87 lessons · 0 done')
  expect(card).toHaveAttribute('href', '#ch01_getting_started/installation')
})
