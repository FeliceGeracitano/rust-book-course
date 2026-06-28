// Client API for the Vercel deployment.
//
// How this differs from the local `server` build (intentionally fewer features):
//  - Lessons / chapters / solutions are STATIC assets baked into the build under
//    /course (see scripts/prepare-content.mjs) and fetched directly.
//  - The editor's working copy and progress live in localStorage — there is no
//    server filesystem to write to.
//  - check/clippy POST the current code to a Vercel Function that compiles and
//    tests it in a Vercel Sandbox.
//  - No rust-analyzer LSP and no "open in your editor" deep link (the editor
//    falls back to the curated completion list).
//
// Signatures are kept identical to the local build so the React components are
// unchanged.

export interface Subchapter {
  id: string
  number: string
  title: string
}

export interface Chapter {
  id: string
  number: number
  title: string
  crate: string | null
  subchapters: Subchapter[]
}

export interface Course {
  title: string
  chapters: Chapter[]
}

export interface CheckResult {
  pass: boolean
  stdout: string
  stderr: string
}

const COURSE = '/course'
const codeKey = (crate: string) => `rbc:code:${crate}`
const PROGRESS_KEY = 'rbc:progress'

export async function getChapters(): Promise<Course> {
  const r = await fetch(`${COURSE}/course.json`)
  if (!r.ok) throw new Error('failed to load chapters')
  return r.json()
}

export async function getLesson(crate: string, sub: string): Promise<string> {
  const r = await fetch(`${COURSE}/${crate}/${sub}.md`)
  if (!r.ok) throw new Error('lesson not found')
  return r.text()
}

export async function getSolution(crate: string): Promise<string> {
  const r = await fetch(`${COURSE}/${crate}/SOLUTION.md`)
  if (!r.ok) throw new Error('no solution')
  return r.text()
}

// The pristine exercise shipped as a static asset: the seed for first load + Reset.
async function pristine(crate: string): Promise<string> {
  const r = await fetch(`${COURSE}/${crate}/exercise.rs`)
  if (!r.ok) throw new Error('exercise not found')
  return r.text()
}

export async function getFile(crate: string): Promise<string> {
  const saved = localStorage.getItem(codeKey(crate))
  if (saved !== null) return saved
  return pristine(crate)
}

export async function saveFile(crate: string, content: string): Promise<void> {
  localStorage.setItem(codeKey(crate), content)
}

export async function resetChapter(crate: string): Promise<string> {
  const content = await pristine(crate)
  localStorage.setItem(codeKey(crate), content)
  return content
}

function readProgress(): Record<string, boolean> {
  try {
    return JSON.parse(localStorage.getItem(PROGRESS_KEY) ?? '{}')
  } catch {
    return {}
  }
}

export async function getProgress(): Promise<Record<string, boolean>> {
  return readProgress()
}

function recordProgress(crate: string, pass: boolean): void {
  const p = readProgress()
  p[crate] = pass
  localStorage.setItem(PROGRESS_KEY, JSON.stringify(p))
}

// The local server read src/lib.rs from disk; here the editor's code (already
// persisted to localStorage by EditorPane's run()/autosave) is sent in the body.
async function runCargo(kind: 'check' | 'clippy', crate: string): Promise<CheckResult> {
  const code = localStorage.getItem(codeKey(crate)) ?? (await pristine(crate))
  const r = await fetch(`/api/${kind}/${crate}`, {
    method: 'POST',
    headers: { 'Content-Type': 'text/plain; charset=utf-8' },
    body: code,
  })
  if (!r.ok) throw new Error(`${kind} failed to run`)
  return r.json()
}

export async function check(crate: string): Promise<CheckResult> {
  const res = await runCargo('check', crate)
  recordProgress(crate, res.pass)
  return res
}

export async function clippy(crate: string): Promise<CheckResult> {
  return runCargo('clippy', crate)
}

export interface AppConfig {
  hostRepoDir: string
  editorScheme: string
  lspUrl: string
  chaptersDir: string
}

// Empty lspUrl => curated completions; empty hostRepoDir => no editor deep link.
const DEFAULT_CONFIG: AppConfig = {
  hostRepoDir: '',
  editorScheme: 'vscode',
  lspUrl: '',
  chaptersDir: '',
}

export async function getConfig(): Promise<AppConfig> {
  return DEFAULT_CONFIG
}
