// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  imports: {
    dirs: ['composables']
  },
  ssr: false,
  compatibilityDate: '2024-04-03',
  devtools: { enabled: true },
  modules: ['@nuxtjs/google-fonts', 'nuxt-proxy'],
  googleFonts: {
    families: {
      "JetBrains Mono": [400, 500, 600, 700, 800]
    }
  },
  css: ['~/assets/main.css'],
  runtimeConfig: {
    apiUrl: 'http://api:8080/'
  }
})