import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'

export default defineConfig({
  plugins: [react(), tailwindcss()],
  // Old checkouts may contain generated runner assets in public/course.
  // All current assets are imported explicitly; never publish those stale files.
  publicDir: false,
  base: './',
  server: { port: 5173, fs: { allow: ['..'] } },
  build: {
    rollupOptions: {
      output: {
        // Vite 8 bundles with Rolldown, which wants manualChunks as a function
        // (not the old object map). Split the big deps into cacheable chunks; the
        // per-chapter viz widgets are already lazy chunks (React.lazy).
        manualChunks(id) {
          if (!id.includes('/node_modules/')) return
          if (/[\\/]node_modules[\\/](react|react-dom|scheduler)[\\/]/.test(id)) return 'vendor-react'
          if (id.includes('/node_modules/shiki/') || id.includes('/node_modules/@shikijs/')) return 'vendor-shiki'
          if (id.includes('/node_modules/framer-motion/') || id.includes('/node_modules/motion')) return 'vendor-motion'
          if (/[\\/]node_modules[\\/](react-markdown|remark|mdast|micromark|hast|unist|vfile|property-information|character-entities|decode-named|space-separated|comma-separated|trim-lines|devlop|bail|trough|is-plain-obj|html-url-attributes|estree)/.test(id)) {
            return 'vendor-markdown'
          }
        },
      },
    },
  },
})
