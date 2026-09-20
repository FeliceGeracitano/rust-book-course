import { constants } from 'node:fs'
import { copyFile, mkdir, readdir } from 'node:fs/promises'
import { fileURLToPath } from 'node:url'
import { resolve, join } from 'node:path'

// Never overwrite learner work. Also usable from a different working directory.
export async function prepareExercises(directory) {
  let created = 0
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    if (!entry.isDirectory() || !entry.name.startsWith('ch')) continue
    const chapter = join(directory, entry.name)
    await mkdir(join(chapter, 'src'), { recursive: true })
    try {
      await copyFile(join(chapter, '.exercise.rs'), join(chapter, 'src/lib.rs'), constants.COPYFILE_EXCL)
      created++
    } catch (error) {
      if (error.code !== 'EEXIST') throw error
    }
  }
  return created
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const count = await prepareExercises(fileURLToPath(new URL('../chapters', import.meta.url)))
  console.log(`Prepared ${count} missing exercise files. Existing work was preserved.`)
}
