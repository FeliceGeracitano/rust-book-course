import { beforeEach, expect, it } from 'vitest'
import { render, screen, within } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import ChapterTree from './ChapterTree'
import { chapters, type Part } from '../content'
import { markComplete, reloadProgress } from '../progress'

beforeEach(() => {
  localStorage.clear()
  reloadProgress()
})

const expanded = () => screen.queryAllByRole('button', { expanded: true })

it('starts with only the current chapter expanded and marks done lessons', () => {
  markComplete('ch01_getting_started/installation', true)
  render(<ChapterTree selectedId="ch01_getting_started/hello_world" />)
  expect(expanded()).toHaveLength(1)
  expect(expanded()[0]).toHaveTextContent('1. Getting Started')
  const done = screen.getByRole('link', { name: /1\.1 Installation/ })
  expect(done).toHaveAttribute('href', '#ch01_getting_started/installation')
  expect(done).toHaveAttribute('data-done', 'true')
  const current = screen.getByRole('link', { name: /1\.2 Hello, World!/ })
  expect(current).toHaveAttribute('aria-current', 'page')
  expect(current).toHaveAttribute('data-done', 'false')
  expect(
    screen.queryByRole('link', { name: /3\.1 Variables/ }),
  ).not.toBeInTheDocument()
})

it('shows a progress ring per chapter', () => {
  markComplete('ch01_getting_started/installation', true)
  render(<ChapterTree selectedId="ch01_getting_started/installation" />)
  expect(screen.getByLabelText('1 of 3 done')).toBeInTheDocument()
  expect(screen.getAllByLabelText(/^\d+ of \d+ done$/)).toHaveLength(
    chapters.length,
  )
})

it('opens every chapter with Expand all and closes them with Collapse all', async () => {
  const user = userEvent.setup()
  render(<ChapterTree selectedId="ch01_getting_started/installation" />)
  await user.click(screen.getByRole('button', { name: 'Expand all' }))
  expect(expanded()).toHaveLength(chapters.length)
  expect(screen.getByRole('link', { name: /21\.1/ })).toBeInTheDocument()
  expect(screen.getByRole('button', { name: 'Expand all' })).toBeDisabled()
  await user.click(screen.getByRole('button', { name: 'Collapse all' }))
  expect(expanded()).toHaveLength(0)
  expect(
    screen.queryByRole('link', { name: /1\.1 Installation/ }),
  ).not.toBeInTheDocument()
  expect(screen.getByRole('button', { name: 'Collapse all' })).toBeDisabled()
})

it('collapses the current chapter when its header is clicked', async () => {
  render(<ChapterTree selectedId="ch01_getting_started/installation" />)
  const header = screen.getByRole('button', { name: /Getting Started/ })
  await userEvent.click(header)
  expect(header).toHaveAttribute('aria-expanded', 'false')
  expect(
    screen.queryByRole('link', { name: /1\.1 Installation/ }),
  ).not.toBeInTheDocument()
})

it('expands the chapter being navigated into and keeps the previous one open', () => {
  const view = render(
    <ChapterTree selectedId="ch01_getting_started/installation" />,
  )
  view.rerender(<ChapterTree selectedId="ch03_common_concepts/functions" />)
  expect(
    screen.getByRole('button', { name: /Common Programming Concepts/ }),
  ).toHaveAttribute('aria-expanded', 'true')
  expect(
    screen.getByRole('button', { name: /Getting Started/ }),
  ).toHaveAttribute('aria-expanded', 'true')
})

const patterns: Part[] = [
  {
    id: 'patterns',
    title: 'Patterns & use cases',
    chapters: [
      {
        id: 'patterns_errors',
        title: 'Errors',
        subchapters: [{ id: 'wrapping', title: 'Wrapping errors' }],
      },
    ],
  },
]

it('shows one header per part', () => {
  render(<ChapterTree selectedId="ch01_getting_started/installation" />)
  expect(screen.getByText('Lessons')).toBeInTheDocument()
  expect(screen.queryByText('Your Rust journey')).not.toBeInTheDocument()
})

it('renders unnumbered chapters and lessons without a number prefix', () => {
  render(<ChapterTree selectedId="patterns_errors/wrapping" parts={patterns} />)
  expect(screen.getByText('Patterns & use cases')).toBeInTheDocument()
  const header = screen.getByRole('button', { name: /Errors/ })
  expect(header).toHaveTextContent(/^Errors/)
  expect(header).not.toHaveTextContent('undefined')
  expect(within(header).getByLabelText('0 of 1 done')).toBeInTheDocument()
  expect(
    screen.getByRole('link', { name: 'Wrapping errors' }),
  ).toHaveAttribute('href', '#patterns_errors/wrapping')
})
