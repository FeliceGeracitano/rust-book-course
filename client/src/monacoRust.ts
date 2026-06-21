import type { Monaco } from '@monaco-editor/react'
import type { editor, Position } from 'monaco-editor'

// A curated (non-semantic) completion provider for the Rust editor: keywords,
// std types, Option/Result variants, macros, common methods, and snippets.
// Not rust-analyzer — but useful, dependency-free, and works offline.

let registered = false

const KEYWORDS = [
  'fn', 'let', 'mut', 'const', 'static', 'if', 'else', 'match', 'loop', 'while',
  'for', 'in', 'break', 'continue', 'return', 'struct', 'enum', 'trait', 'impl',
  'pub', 'use', 'mod', 'crate', 'self', 'Self', 'super', 'where', 'as', 'ref',
  'move', 'dyn', 'async', 'await', 'unsafe', 'type', 'default',
]

const TYPES = [
  'String', 'str', 'Vec', 'Option', 'Result', 'Box', 'Rc', 'Arc', 'RefCell',
  'Cell', 'HashMap', 'HashSet', 'BTreeMap', 'VecDeque', 'i8', 'i16', 'i32', 'i64',
  'i128', 'isize', 'u8', 'u16', 'u32', 'u64', 'u128', 'usize', 'f32', 'f64',
  'bool', 'char',
]

const VARIANTS = ['Some', 'None', 'Ok', 'Err']

const MACROS = [
  'println!', 'print!', 'eprintln!', 'format!', 'vec!', 'panic!', 'assert!',
  'assert_eq!', 'assert_ne!', 'todo!', 'unimplemented!', 'dbg!', 'write!',
]

const METHODS = [
  'iter', 'into_iter', 'iter_mut', 'map', 'filter', 'filter_map', 'collect',
  'fold', 'sum', 'count', 'enumerate', 'zip', 'rev', 'take', 'skip', 'find',
  'any', 'all', 'unwrap', 'unwrap_or', 'unwrap_or_else', 'expect', 'to_string',
  'to_owned', 'clone', 'into', 'as_str', 'len', 'is_empty', 'push', 'pop',
  'insert', 'get', 'contains', 'trim', 'split', 'parse',
]

interface Snippet {
  label: string
  insertText: string
  detail: string
}

const SNIPPETS: Snippet[] = [
  { label: 'fn', insertText: 'fn ${1:name}(${2}) {\n\t$0\n}', detail: 'function' },
  { label: 'pfn', insertText: 'pub fn ${1:name}(${2}) -> ${3:()} {\n\t$0\n}', detail: 'pub function' },
  { label: 'match', insertText: 'match ${1:expr} {\n\t${2:_} => ${3},\n}', detail: 'match expression' },
  { label: 'iflet', insertText: 'if let ${1:Some(x)} = ${2:expr} {\n\t$0\n}', detail: 'if let' },
  { label: 'letelse', insertText: 'let ${1:Some(x)} = ${2:expr} else {\n\t${3:return}\n};', detail: 'let...else' },
  { label: 'for', insertText: 'for ${1:item} in ${2:iter} {\n\t$0\n}', detail: 'for loop' },
  { label: 'while', insertText: 'while ${1:cond} {\n\t$0\n}', detail: 'while loop' },
  { label: 'struct', insertText: 'struct ${1:Name} {\n\t${2:field}: ${3:Type},\n}', detail: 'struct' },
  { label: 'enum', insertText: 'enum ${1:Name} {\n\t${2:Variant},\n}', detail: 'enum' },
  { label: 'impl', insertText: 'impl ${1:Type} {\n\t$0\n}', detail: 'impl block' },
  { label: 'trait', insertText: 'trait ${1:Name} {\n\t$0\n}', detail: 'trait' },
  { label: 'test', insertText: '#[test]\nfn ${1:name}() {\n\t$0\n}', detail: 'test function' },
  { label: 'println', insertText: 'println!("${1}"${2});', detail: 'print line' },
  { label: 'derive', insertText: '#[derive(${1:Debug})]', detail: 'derive attribute' },
]

export function registerRustCompletions(monaco: Monaco) {
  if (registered) return
  registered = true

  monaco.languages.registerCompletionItemProvider('rust', {
    provideCompletionItems(model: editor.ITextModel, position: Position) {
      const word = model.getWordUntilPosition(position)
      const range = {
        startLineNumber: position.lineNumber,
        endLineNumber: position.lineNumber,
        startColumn: word.startColumn,
        endColumn: word.endColumn,
      }
      const Kind = monaco.languages.CompletionItemKind
      const asSnippet = monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet

      const simple = (labels: string[], kind: number, detail: string) =>
        labels.map((label) => ({ label, kind, insertText: label, range, detail }))

      const suggestions = [
        ...simple(KEYWORDS, Kind.Keyword, 'keyword'),
        ...simple(TYPES, Kind.Struct, 'std type'),
        ...simple(VARIANTS, Kind.EnumMember, 'variant'),
        ...simple(MACROS, Kind.Function, 'macro'),
        ...simple(METHODS, Kind.Method, 'method'),
        ...SNIPPETS.map((s) => ({
          label: s.label,
          kind: Kind.Snippet,
          insertText: s.insertText,
          insertTextRules: asSnippet,
          detail: s.detail,
          range,
        })),
      ]
      return { suggestions }
    },
  })
}
