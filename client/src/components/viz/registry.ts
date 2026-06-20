import { ComponentType } from 'react'
import OwnershipViz from './OwnershipViz'

// Maps a chapter crate id (from course.json) to its interactive widget.
// LessonView renders the matching widget above the lesson prose.
const REGISTRY: Record<string, ComponentType> = {
  ch04_ownership: OwnershipViz,
}

export function vizFor(crate: string | null): ComponentType | null {
  if (!crate) return null
  return REGISTRY[crate] ?? null
}
