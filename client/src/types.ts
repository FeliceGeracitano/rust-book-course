import { Chapter, Subchapter } from './api'

export interface Selection {
  chapter: Chapter
  sub: Subchapter | null
}
