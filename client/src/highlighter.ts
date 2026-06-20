import { createHighlighterCore, type HighlighterCore } from 'shiki/core'
import { createJavaScriptRegexEngine } from 'shiki/engine/javascript'
import rust from 'shiki/langs/rust.mjs'
import bash from 'shiki/langs/bash.mjs'
import toml from 'shiki/langs/toml.mjs'
import json from 'shiki/langs/json.mjs'
import oneDarkPro from 'shiki/themes/one-dark-pro.mjs'

// Fine-grained Shiki: only the languages/theme we use are bundled, and the
// pure-JS regex engine avoids shipping the Oniguruma wasm. Keeps dist small.
export const SUPPORTED_LANGS = ['rust', 'bash', 'toml', 'json'] as const
export const THEME = 'one-dark-pro'

let highlighterPromise: Promise<HighlighterCore> | null = null

export function getHighlighter(): Promise<HighlighterCore> {
  if (!highlighterPromise) {
    highlighterPromise = createHighlighterCore({
      themes: [oneDarkPro],
      langs: [rust, bash, toml, json],
      engine: createJavaScriptRegexEngine(),
    })
  }
  return highlighterPromise
}

export function langOrText(lang: string): string {
  return (SUPPORTED_LANGS as readonly string[]).includes(lang) ? lang : 'text'
}
