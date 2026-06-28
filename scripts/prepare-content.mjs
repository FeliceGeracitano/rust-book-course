// Bake the course's read-only content into the client build as static assets.
//
// The local server served lessons/solutions/exercises from disk via /api/*; the
// Vercel build has no such server, so we copy them under client/public/course/
// (-> dist/course/* -> served at /course/*) and the client fetches them directly.
//
// Run automatically by the client's prebuild/predev scripts. Output is git-ignored.

import {
  cpSync, mkdirSync, readdirSync, existsSync, statSync, copyFileSync, rmSync,
} from 'node:fs'
import { join, dirname } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const contentDir = join(root, 'content')
const chaptersDir = join(root, 'chapters')
const out = join(root, 'client', 'public', 'course')

const isDir = (p) => existsSync(p) && statSync(p).isDirectory()

rmSync(out, { recursive: true, force: true })
mkdirSync(out, { recursive: true })

// Table of contents.
copyFileSync(join(contentDir, 'course.json'), join(out, 'course.json'))

// Lesson markdown: content/<key>/*.md  (key = crate, or chapter id for the appendix).
for (const dir of readdirSync(contentDir)) {
  const src = join(contentDir, dir)
  if (!isDir(src)) continue
  const dest = join(out, dir)
  mkdirSync(dest, { recursive: true })
  for (const f of readdirSync(src)) {
    if (f.endsWith('.md')) copyFileSync(join(src, f), join(dest, f))
  }
}

// Per-chapter solution + pristine exercise (the seed for first load and Reset).
let exercises = 0
for (const dir of readdirSync(chaptersDir)) {
  const src = join(chaptersDir, dir)
  if (!isDir(src)) continue
  const dest = join(out, dir)
  mkdirSync(dest, { recursive: true })
  const solution = join(src, 'SOLUTION.md')
  if (existsSync(solution)) copyFileSync(solution, join(dest, 'SOLUTION.md'))
  const exercise = join(src, '.exercise.rs')
  if (existsSync(exercise)) {
    copyFileSync(exercise, join(dest, 'exercise.rs'))
    exercises++
  }
}

// Self-check: a broken copy would otherwise ship a blank app.
const required = [
  join(out, 'course.json'),
  join(out, 'ch02_guessing_game', 'exercise.rs'),
  join(out, 'ch02_guessing_game', 'guessing_game.md'),
]
for (const p of required) {
  if (!existsSync(p)) throw new Error(`prepare-content: missing expected output ${p}`)
}
if (exercises < 20) throw new Error(`prepare-content: only ${exercises} exercises copied (expected 21)`)

console.log(`prepare-content: ${exercises} exercises + lessons -> client/public/course`)
