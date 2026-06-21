// WebSocket ⇄ rust-analyzer bridge.
//
// Each browser connection gets its own rust-analyzer process. The browser speaks
// LSP JSON-RPC over the socket (one JSON message per WS frame); we forward it to
// rust-analyzer's stdio (with Content-Length framing handled by the helper).
import { WebSocketServer } from 'ws'
import { WebSocketMessageReader, WebSocketMessageWriter } from 'vscode-ws-jsonrpc'
import { createConnection, createServerProcess, forward } from 'vscode-ws-jsonrpc/server'

const PORT = Number(process.env.LSP_PORT || 3030)
const CHAPTERS_DIR = process.env.CHAPTERS_DIR || '/app/chapters'

const wss = new WebSocketServer({ port: PORT, host: '0.0.0.0' })
console.log(`LSP bridge listening on ws://0.0.0.0:${PORT} (rust-analyzer cwd=${CHAPTERS_DIR})`)

wss.on('connection', (webSocket) => {
  const socket = {
    send: (content) => webSocket.send(content),
    onMessage: (cb) => webSocket.on('message', (data) => cb(data.toString())),
    onError: (cb) => webSocket.on('error', cb),
    onClose: (cb) => webSocket.on('close', cb),
    dispose: () => webSocket.close(),
  }

  const reader = new WebSocketMessageReader(socket)
  const writer = new WebSocketMessageWriter(socket)
  const socketConnection = createConnection(reader, writer, () => webSocket.close())

  const serverConnection = createServerProcess('rust-analyzer', 'rust-analyzer', [], {
    cwd: CHAPTERS_DIR,
    env: { ...process.env, CARGO_TARGET_DIR: process.env.CARGO_TARGET_DIR || '/ra-target' },
  })

  if (!serverConnection) {
    console.error('failed to spawn rust-analyzer')
    webSocket.close()
    return
  }

  forward(socketConnection, serverConnection, (message) => message)
  webSocket.on('close', () => {
    try {
      serverConnection.dispose()
    } catch {
      /* ignore */
    }
  })
})
