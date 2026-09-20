// @vitest-environment node
import { expect, it } from 'vitest'
import { mkdtemp, mkdir, readFile, rm, writeFile } from 'node:fs/promises'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
// @ts-expect-error The standalone Node CLI deliberately has no TypeScript build step.
import { prepareExercises } from '../../scripts/prepare-exercises.mjs'

it('initializes a fresh exercise and preserves learner edits on repeated setup', async () => {
  const root = await mkdtemp(join(tmpdir(), 'rust-exercises-'))
  try {
    await mkdir(join(root, 'ch01_example'))
    await writeFile(
      join(root, 'ch01_example/.exercise.rs'),
      'fn exercise() { todo!() }',
    )
    expect(await prepareExercises(root)).toBe(1)
    const working = join(root, 'ch01_example/src/lib.rs')
    expect(await readFile(working, 'utf8')).toContain('todo!()')
    await writeFile(working, 'fn exercise() { /* learner work */ }')
    expect(await prepareExercises(root)).toBe(0)
    expect(await readFile(working, 'utf8')).toContain('learner work')
  } finally {
    await rm(root, { recursive: true, force: true })
  }
})
