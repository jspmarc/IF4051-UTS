import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  build: {
    outDir: '../back-end/src/view',
    emptyOutDir: true,
  },
  server: {
    host: '0.0.0.0',
    strictPort: true,
  }
})
