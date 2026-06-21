import { ComponentType, lazy } from 'react'

// Maps a chapter crate id (from course.json) to its interactive widget.
// Each widget is lazy-loaded (its own chunk) so it only downloads when you open
// that chapter — keeping the initial bundle small. LessonView renders it inside
// a <Suspense>.
const REGISTRY: Record<string, ComponentType> = {
  ch04_ownership: lazy(() => import('./OwnershipViz')),
  ch08_collections: lazy(() => import('./CollectionsViz')),
  ch15_smart_pointers: lazy(() => import('./SmartPointersViz')),
  ch16_concurrency: lazy(() => import('./ConcurrencyViz')),
  ch17_async: lazy(() => import('./AsyncViz')),
}

export function vizFor(crate: string | null): ComponentType | null {
  if (!crate) return null
  return REGISTRY[crate] ?? null
}
