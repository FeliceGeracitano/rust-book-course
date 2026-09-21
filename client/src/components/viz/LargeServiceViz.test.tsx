import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { expect, it } from 'vitest'
import LargeServiceViz from './LargeServiceViz'
import { vizFor } from './registry'

it('is registered for the large service chapter', () => {
  expect(vizFor('patterns_large_service')).not.toBeNull()
})

it('toggles between compile-time dependencies and a runtime request walk', async () => {
  render(<LargeServiceViz />)
  expect(screen.getByText('api → app')).toBeInTheDocument()
  await userEvent.click(screen.getByRole('button', { name: 'Runtime request' }))
  expect(screen.getByText('1 / 5')).toBeInTheDocument()
  await userEvent.click(screen.getByRole('button', { name: 'Next step' }))
  expect(screen.getByText('2 / 5')).toBeInTheDocument()
  expect(screen.getByText(/orders::Service::quote/)).toBeInTheDocument()
})
