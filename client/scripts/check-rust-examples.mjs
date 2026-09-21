// Verify Rust snippets and excerpts in Part 2 / Part 3 lessons, then the example workspace.
// Snippets compile inside examples/snippets (pinned deps); excerpts must match their source
// file under examples/large-service; the workspace must pass test, clippy, and fmt.
import { readFile, writeFile, mkdir, mkdtemp, rm } from 'node:fs/promises'
import { execFile } from 'node:child_process'
import { promisify } from 'node:util'
import { join, resolve, sep } from 'node:path'
import { tmpdir } from 'node:os'
import { collect } from './rust-examples.mjs'

const exec = promisify(execFile)
const repo = resolve(import.meta.dirname, '../..')
const content = join(repo, 'content')
const snippets = join(repo, 'examples/snippets')
const service = join(repo, 'examples/large-service')
const CHECKED_PARTS = ['patterns', 'toolchain']
const failures = []

const course = JSON.parse(await readFile(join(content, 'course.json'), 'utf8'))
const lessons = course.parts
  .filter((part) => CHECKED_PARTS.includes(part.id))
  .flatMap((part) => part.chapters.flatMap((chapter) => chapter.subchapters.map((sub) => `${chapter.id}/${sub.id}.md`)))
const items = []
for (const file of lessons) items.push(...collect(await readFile(join(content, file), 'utf8'), file))
const byKind = (kind) => items.filter((item) => item.kind === kind)
const programs = byKind('program')
const tests = byKind('test')
const compileFails = byKind('compile-fail')
const excerpts = byKind('excerpt')
const where = (item) => `${item.file}:${item.line}`
const cargo = (args, cwd) => exec('cargo', args, { cwd, timeout: 900_000, maxBuffer: 64 * 1024 * 1024 })
const describe = (error) => (error.stderr || error.stdout || error.message).toString().trim().split('\n').slice(-25).join('\n')

// 1. Standalone programs and test files, compiled together in the pinned snippets crate.
const binDir = join(snippets, 'src/bin')
const testDir = join(snippets, 'tests')
for (const dir of [binDir, testDir]) {
  await rm(dir, { recursive: true, force: true })
  await mkdir(dir, { recursive: true })
}
const target = (item, index) => `${item.file.replace(/\.md$/, '').replace(/[^a-z0-9]+/g, '_')}__${index}`
programs.forEach((item, index) => (item.target = target(item, index)))
tests.forEach((item, index) => (item.target = target(item, index)))
for (const item of programs) await writeFile(join(binDir, `${item.target}.rs`), `${item.code}\n`)
for (const item of tests) await writeFile(join(testDir, `${item.target}.rs`), `${item.code}\n`)
let verified = 0
if (programs.length || tests.length) {
  try {
    await cargo(['build', '--bins', '--tests', '--locked'], snippets)
    for (const item of programs) {
      try {
        const { stdout } = await exec(join(snippets, 'target/debug', item.target), [], { cwd: snippets, timeout: 10_000 })
        if (item.expected !== null && stdout.trimEnd() !== item.expected) {
          failures.push(`${where(item)}: expected output ${JSON.stringify(item.expected)}, got ${JSON.stringify(stdout.trimEnd())}`)
        } else verified++
      } catch (error) {
        failures.push(`${where(item)}: program failed: ${describe(error)}`)
      }
    }
    if (tests.length) {
      try {
        await cargo(['test', '--locked', '--tests'], snippets)
        verified += tests.length
      } catch (error) {
        failures.push(`snippet tests failed (${tests.map(where).join(', ')}): ${describe(error)}`)
      }
    }
  } catch (error) {
    failures.push(`snippets did not compile (${[...programs, ...tests].map(where).join(', ')}): ${describe(error)}`)
  }
}

// 2. Deliberately invalid snippets must fail with the declared error code.
const scratch = await mkdtemp(join(tmpdir(), 'rust-compile-fail-'))
try {
  for (const [index, item] of compileFails.entries()) {
    const file = join(scratch, `fail_${index}.rs`)
    await writeFile(file, `${item.code}\n`)
    try {
      await exec('rustc', ['--edition', '2024', '--crate-type', 'lib', '--emit=metadata', '-o', join(scratch, `fail_${index}.rmeta`), file], { timeout: 60_000 })
      failures.push(`${where(item)}: expected ${item.errorCode} but the snippet compiled`)
    } catch (error) {
      if (!String(error.stderr).includes(`error[${item.errorCode}]`)) failures.push(`${where(item)}: expected ${item.errorCode}, got: ${describe(error)}`)
    }
  }
} finally {
  await rm(scratch, { recursive: true, force: true })
}

// 3. Excerpts must still match the example source, ignoring trailing whitespace.
const normalize = (text) => text.split('\n').map((line) => line.trimEnd()).join('\n')
let matched = 0
for (const item of excerpts) {
  const path = resolve(repo, item.source)
  if (!path.startsWith(service + sep)) {
    failures.push(`${where(item)}: source path escapes the example: ${item.source}`)
    continue
  }
  let actual
  try {
    actual = await readFile(path, 'utf8')
  } catch {
    failures.push(`${where(item)}: source file not found: ${item.source}`)
    continue
  }
  if (normalize(actual).includes(normalize(item.code))) matched++
  else failures.push(`${where(item)}: excerpt differs from ${item.source}`)
}

// 4. The example workspace itself.
let workspaceOk = true
for (const args of [['test', '--workspace', '--locked'], ['clippy', '--workspace', '--locked', '--', '-D', 'warnings'], ['fmt', '--check']]) {
  try {
    await cargo(args, service)
  } catch (error) {
    workspaceOk = false
    failures.push(`large-service: cargo ${args.join(' ')} failed: ${describe(error)}`)
  }
}

if (items.length === 0) failures.push('nothing checked: no rust snippets or excerpts found in the checked parts')
console.log(
  `${verified}/${programs.length + tests.length} snippets verified, ${compileFails.length} compile-fail checked, ${matched}/${excerpts.length} excerpts matched, large-service ${workspaceOk ? 'passed' : 'FAILED'} (test, clippy, fmt).`,
)
for (const failure of failures) console.error(failure)
if (failures.length) process.exitCode = 1
