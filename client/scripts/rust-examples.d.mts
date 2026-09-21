export type Example =
  | { kind: 'program'; code: string; expected: string | null }
  | { kind: 'test'; code: string }
  | { kind: 'compile-fail'; code: string; errorCode: string }
  | { kind: 'excerpt'; code: string; source: string }
  | { kind: 'skip' }
export function stripOutput(code: string): { code: string; expected: string | null }
export function classify(code: string): Example
export function collect(markdown: string, file: string): (Example & { file: string; line: number })[]
