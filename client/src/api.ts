// Typed client for the Rust server's JSON API. Field names mirror
// content/course.json and the server's check/progress responses.

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

export async function getChapters(): Promise<Course> {
  const r = await fetch('/api/chapters')
  if (!r.ok) throw new Error('failed to load chapters')
  return r.json()
}

export async function getLesson(crate: string, sub: string): Promise<string> {
  const r = await fetch(`/api/lesson/${crate}/${sub}`)
  if (!r.ok) throw new Error('lesson not found')
  return r.text()
}

export async function check(crate: string): Promise<CheckResult> {
  const r = await fetch(`/api/check/${crate}`, { method: 'POST' })
  if (!r.ok) throw new Error('check failed to run')
  return r.json()
}

export async function clippy(crate: string): Promise<CheckResult> {
  const r = await fetch(`/api/clippy/${crate}`, { method: 'POST' })
  if (!r.ok) throw new Error('clippy failed to run')
  return r.json()
}

export async function getFile(crate: string): Promise<string> {
  const r = await fetch(`/api/file/${crate}`)
  if (!r.ok) throw new Error('file not found')
  return r.text()
}

export async function saveFile(crate: string, content: string): Promise<void> {
  const r = await fetch(`/api/file/${crate}`, { method: 'PUT', body: content })
  if (!r.ok) throw new Error('save failed')
}

export async function getSolution(crate: string): Promise<string> {
  const r = await fetch(`/api/solution/${crate}`)
  if (!r.ok) throw new Error('no solution')
  return r.text()
}

export async function resetChapter(crate: string): Promise<string> {
  const r = await fetch(`/api/reset/${crate}`, { method: 'POST' })
  if (!r.ok) throw new Error('reset failed')
  return r.text()
}

export async function getProgress(): Promise<Record<string, boolean>> {
  const r = await fetch('/api/progress')
  if (!r.ok) return {}
  return r.json()
}

export interface AppConfig {
  hostRepoDir: string
  editorScheme: string
  lspUrl: string
  chaptersDir: string
}

const DEFAULT_CONFIG: AppConfig = {
  hostRepoDir: '',
  editorScheme: 'vscode',
  lspUrl: '',
  chaptersDir: '',
}

export async function getConfig(): Promise<AppConfig> {
  const r = await fetch('/api/config')
  if (!r.ok) return DEFAULT_CONFIG
  return { ...DEFAULT_CONFIG, ...(await r.json()) }
}
