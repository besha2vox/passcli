export default defineNuxtConfig({
  ssr: false,

  css: ['~/assets/css/global.css', '@fortawesome/fontawesome-free/css/all.min.css'],

  devtools: { enabled: true },

  typescript: {
    strict: true,
  },

  vite: {
    clearScreen: false,
    envPrefix: ['VITE_', 'TAURI_'],
    server: {
      strictPort: true,
      port: 3000,
      watch: {
        ignored: ['**/src-tauri/**'],
      },
    },
  },

  compatibilityDate: '2024-11-01',
})
