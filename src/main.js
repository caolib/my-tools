import { createApp } from 'vue'
import { createPinia } from 'pinia'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import 'element-plus/theme-chalk/dark/css-vars.css'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import App from './App.vue'
import router from './router'
import './assets/styles/global.scss'
import { useSettingsStore } from './stores/settings'

const app = createApp(App)
const pinia = createPinia()

pinia.use(piniaPluginPersistedstate)

// 注册所有图标
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component)
}

app.use(pinia)
app.use(ElementPlus)
app.use(router)

// 初始化主题
const settingsStore = useSettingsStore()
// 应用保存的主题设置
document.documentElement.setAttribute('data-theme', settingsStore.theme)
if (settingsStore.theme === 'dark') {
  document.documentElement.classList.add('dark')
}

app.mount('#app')
