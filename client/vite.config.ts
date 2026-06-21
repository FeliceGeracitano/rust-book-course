import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'

// During `npm run dev`, proxy API calls to the Rust server on :8080.
// In production the Rust server serves the built `dist/` itself.
export default defineConfig({
  plugins: [react(), tailwindcss()],
  server: {
    port: 5173,
    proxy: {
      '/api': 'http://localhost:8080',
    },
  },
  build: {
    outDir: 'dist',
    rollupOptions: {
      output: {
        // Split big deps into their own cacheable chunks; the per-chapter viz
        // widgets are already lazy chunks (React.lazy).
        manualChunks: {
          'vendor-react': ['react', 'react-dom'],
          'vendor-shiki': ['shiki'],
          'vendor-motion': ['framer-motion'],
          'vendor-markdown': ['react-markdown', 'remark-gfm'],
        },
      },
    },
  },
})
