import { ComponentType } from 'react'
import OwnershipViz from './OwnershipViz'
import CollectionsViz from './CollectionsViz'
import SmartPointersViz from './SmartPointersViz'
import ConcurrencyViz from './ConcurrencyViz'
import AsyncViz from './AsyncViz'

// Maps a chapter crate id (from course.json) to its interactive widget.
// LessonView renders the matching widget above the lesson prose.
const REGISTRY: Record<string, ComponentType> = {
  ch04_ownership: OwnershipViz,
  ch08_collections: CollectionsViz,
  ch15_smart_pointers: SmartPointersViz,
  ch16_concurrency: ConcurrencyViz,
  ch17_async: AsyncViz,
}

export function vizFor(crate: string | null): ComponentType | null {
  if (!crate) return null
  return REGISTRY[crate] ?? null
}
