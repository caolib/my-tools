import { defineStore } from 'pinia'

export const useSettingsStore = defineStore('settings', {
  state: () => ({
    exportPath: '',
    autoOpenFolder: true,
    theme: 'light', // 主题：light 或 dark
    collapsedKeys: [] // 存储折叠面板的key
  }),

  actions: {
    setExportPath(path) {
      this.exportPath = path
    },
    
    setAutoOpenFolder(value) {
      this.autoOpenFolder = value
    },

    setTheme(theme) {
      this.theme = theme
      // 同时更新html的class
      if (theme === 'dark') {
        document.documentElement.classList.add('dark')
      } else {
        document.documentElement.classList.remove('dark')
      }
    },

    setCollapsedKeys(keys) {
      this.collapsedKeys = keys
    },

    addCollapsedKey(key) {
      if (!this.collapsedKeys.includes(key)) {
        this.collapsedKeys.push(key)
      }
    },

    removeCollapsedKey(key) {
      const index = this.collapsedKeys.indexOf(key)
      if (index > -1) {
        this.collapsedKeys.splice(index, 1)
      }
    },

    toggleCollapsedKey(key) {
      if (this.collapsedKeys.includes(key)) {
        this.removeCollapsedKey(key)
      } else {
        this.addCollapsedKey(key)
      }
    },

    resetToDefaults() {
      this.exportPath = ''
      this.autoOpenFolder = true
      this.theme = 'light'
      this.collapsedKeys = []
    }
  },

  persist: {
    storage: localStorage,
    key: 'wem-settings'
  }
})