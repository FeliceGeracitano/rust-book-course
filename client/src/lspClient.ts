import type { Monaco } from '@monaco-editor/react'
import type { editor as MEditor, languages, Position } from 'monaco-editor'

// A thin LSP-over-WebSocket client wired directly to Monaco — no
// monaco-languageclient, so no monaco version pinning. One JSON-RPC message per
// WS frame (matches the bridge's vscode-ws-jsonrpc reader/writer).

type Pending = { resolve: (v: unknown) => void; reject: (e: unknown) => void }
type LspKind = number

interface LspCompletionItem {
  label: string
  kind?: LspKind
  detail?: string
  insertText?: string
  insertTextFormat?: number
  textEdit?: { newText: string }
  documentation?: string | { value: string }
  sortText?: string
}

function mapKind(monaco: Monaco, k?: LspKind): number {
  const K = monaco.languages.CompletionItemKind
  const table: Record<number, number> = {
    1: K.Text, 2: K.Method, 3: K.Function, 4: K.Constructor, 5: K.Field,
    6: K.Variable, 7: K.Class, 8: K.Interface, 9: K.Module, 10: K.Property,
    11: K.Unit, 12: K.Value, 13: K.Enum, 14: K.Keyword, 15: K.Snippet,
    16: K.Color, 17: K.File, 18: K.Reference, 19: K.Folder, 20: K.EnumMember,
    21: K.Constant, 22: K.Struct, 23: K.Event, 24: K.Operator, 25: K.TypeParameter,
  }
  return (k && table[k]) || K.Text
}

function docString(d?: string | { value: string }): string | undefined {
  if (!d) return undefined
  return typeof d === 'string' ? d : d.value
}

class LspClient {
  private ws: WebSocket | null = null
  private monaco: Monaco | null = null
  private id = 0
  private pending = new Map<number, Pending>()
  private ready: Promise<void> | null = null
  private openUris = new Set<string>()
  private version = new Map<string, number>()
  private currentUri: string | null = null
  private getModel: (() => MEditor.ITextModel | null) | null = null

  connect(url: string, rootUri: string, monaco: Monaco): Promise<void> {
    if (this.ready) return this.ready
    this.monaco = monaco
    this.ready = new Promise((resolve, reject) => {
      let ws: WebSocket
      try {
        ws = new WebSocket(url)
      } catch (e) {
        reject(e)
        return
      }
      this.ws = ws
      ws.onmessage = (e) => this.onMessage(JSON.parse(e.data))
      ws.onerror = () => reject(new Error('LSP WebSocket error'))
      ws.onclose = () => {
        this.ws = null
        this.ready = null
        this.openUris.clear()
      }
      ws.onopen = async () => {
        try {
          await this.request('initialize', {
            processId: null,
            rootUri,
            workspaceFolders: [{ uri: rootUri, name: 'chapters' }],
            capabilities: {
              textDocument: {
                completion: { completionItem: { snippetSupport: true } },
                hover: { contentFormat: ['markdown', 'plaintext'] },
                publishDiagnostics: {},
              },
              workspace: { configuration: true, workspaceFolders: true },
            },
          })
          this.notify('initialized', {})
          this.registerProviders()
          resolve()
        } catch (err) {
          reject(err)
        }
      }
    })
    return this.ready
  }

  setModelResolver(fn: () => MEditor.ITextModel | null) {
    this.getModel = fn
  }

  private send(m: unknown) {
    this.ws?.send(JSON.stringify(m))
  }
  private notify(method: string, params: unknown) {
    this.send({ jsonrpc: '2.0', method, params })
  }
  private request(method: string, params: unknown): Promise<unknown> {
    const id = ++this.id
    return new Promise((resolve, reject) => {
      this.pending.set(id, { resolve, reject })
      this.send({ jsonrpc: '2.0', id, method, params })
    })
  }

  private onMessage(msg: any) {
    // server → client request: answer so rust-analyzer doesn't stall
    if (msg.method && msg.id !== undefined) {
      let result: unknown = null
      if (msg.method === 'workspace/configuration') {
        result = (msg.params?.items || []).map(() => ({}))
      }
      this.send({ jsonrpc: '2.0', id: msg.id, result })
      return
    }
    // response to our request
    if (msg.id !== undefined && !msg.method) {
      const p = this.pending.get(msg.id)
      if (p) {
        this.pending.delete(msg.id)
        if (msg.error) p.reject(msg.error)
        else p.resolve(msg.result)
      }
      return
    }
    // notifications
    if (msg.method === 'textDocument/publishDiagnostics') {
      this.applyDiagnostics(msg.params)
    }
  }

  openDoc(uri: string, text: string) {
    this.currentUri = uri
    if (!this.ws) return
    if (this.openUris.has(uri)) {
      this.changeDoc(uri, text)
      return
    }
    this.openUris.add(uri)
    this.version.set(uri, 1)
    this.notify('textDocument/didOpen', {
      textDocument: { uri, languageId: 'rust', version: 1, text },
    })
  }

  changeDoc(uri: string, text: string) {
    if (!this.ws) return
    if (!this.openUris.has(uri)) {
      this.openDoc(uri, text)
      return
    }
    const v = (this.version.get(uri) || 1) + 1
    this.version.set(uri, v)
    this.notify('textDocument/didChange', {
      textDocument: { uri, version: v },
      contentChanges: [{ text }],
    })
  }

  private applyDiagnostics(params: { uri: string; diagnostics: any[] }) {
    if (!this.monaco || !this.getModel) return
    const model = this.getModel()
    if (!model) return
    const sev = this.monaco.MarkerSeverity
    const markers = (params.diagnostics || []).map((d) => ({
      startLineNumber: d.range.start.line + 1,
      startColumn: d.range.start.character + 1,
      endLineNumber: d.range.end.line + 1,
      endColumn: d.range.end.character + 1,
      message: d.message,
      source: d.source || 'rust-analyzer',
      severity:
        d.severity === 1 ? sev.Error : d.severity === 2 ? sev.Warning : d.severity === 3 ? sev.Info : sev.Hint,
    }))
    this.monaco.editor.setModelMarkers(model, 'rust-analyzer', markers)
  }

  private registered = false
  private registerProviders() {
    if (this.registered || !this.monaco) return
    this.registered = true
    const monaco = this.monaco

    monaco.languages.registerCompletionItemProvider('rust', {
      triggerCharacters: ['.', ':', '(', "'"],
      provideCompletionItems: async (model: MEditor.ITextModel, position: Position) => {
        if (!this.currentUri) return { suggestions: [] }
        try {
          const res = (await this.request('textDocument/completion', {
            textDocument: { uri: this.currentUri },
            position: { line: position.lineNumber - 1, character: position.column - 1 },
          })) as { items?: LspCompletionItem[] } | LspCompletionItem[] | null
          const items = Array.isArray(res) ? res : res?.items || []
          const word = model.getWordUntilPosition(position)
          const range = {
            startLineNumber: position.lineNumber,
            endLineNumber: position.lineNumber,
            startColumn: word.startColumn,
            endColumn: word.endColumn,
          }
          const suggestions = items.map((it) => {
            const snippet = it.insertTextFormat === 2
            return {
              label: it.label,
              kind: mapKind(monaco, it.kind),
              detail: it.detail,
              insertText: it.textEdit?.newText ?? it.insertText ?? it.label,
              insertTextRules: snippet
                ? monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet
                : undefined,
              documentation: docString(it.documentation),
              sortText: it.sortText,
              range,
            } as languages.CompletionItem
          })
          return { suggestions }
        } catch {
          return { suggestions: [] }
        }
      },
    })

    monaco.languages.registerHoverProvider('rust', {
      provideHover: async (_model: MEditor.ITextModel, position: Position) => {
        if (!this.currentUri) return null
        try {
          const res = (await this.request('textDocument/hover', {
            textDocument: { uri: this.currentUri },
            position: { line: position.lineNumber - 1, character: position.column - 1 },
          })) as { contents?: unknown } | null
          const c = res?.contents
          if (!c) return null
          let value = ''
          if (typeof c === 'string') value = c
          else if (Array.isArray(c)) value = c.map((x) => (typeof x === 'string' ? x : (x as any).value)).join('\n')
          else value = (c as any).value || ''
          if (!value) return null
          return { contents: [{ value }] }
        } catch {
          return null
        }
      },
    })
  }
}

export const lsp = new LspClient()
