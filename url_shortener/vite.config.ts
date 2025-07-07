import { defineConfig } from 'vite'
import tailwindcss from 'tailwindcss'
import autoprefixer from 'autoprefixer'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  css: {
    postcss: {
      plugins: [
        tailwindcss(),
        autoprefixer()
      ]
    }
  },
  /*
  server: {
    proxy: {
      '/tinify': {
        target: 'http://0.0.0.0:8000',
        changeOrigin: true,
      }
    },
    secure: false,
    configure: (proxy) => {
      proxy.on('proxyReq', (proxyReq, req) => {
        console.log(`[proxy] -> ${req.method} ${req.url}`)
      })
    }
  }
  */
})
