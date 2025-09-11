import { defineStore } from 'pinia'

export const useSettingsStore = defineStore('settings', {
  state: () => ({
    exportPath: '',
    autoOpenFolder: true,
    theme: 'light', // 主题：light 或 dark
    collapsedKeys: [], // 存储折叠面板的key
    // 预览面板设置
    previewSettings: {
      enabled: false, // 预览模式是否启用
      panelWidth: 400, // 预览面板宽度
      panelHeight: 300, // 预览面板高度
      imageModeHeight: 0.5, // 图片模式高度比例（相对于窗口高度）
      documentModeHeight: 0.9, // 文档模式高度比例（相对于窗口高度）
      widthRatio: 1 / 3, // 预览面板宽度比例（相对于窗口宽度）
      codeTheme: 'auto', // 代码高亮主题：'auto' | 'vs2015' | 'github' | 'atom-one-dark' | 'atom-one-light' | 'monokai'
      fontSize: '13px', // 代码字体大小
      fontFamily: 'Consolas, Monaco, "Courier New", monospace', // 代码字体
      wordWrap: false // 是否自动换行，默认关闭
    }
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
      // 同时更新html的data-theme属性和Element Plus的dark类名
      document.documentElement.setAttribute('data-theme', theme)
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

    // 预览设置相关方法
    setPreviewEnabled(enabled) {
      this.previewSettings.enabled = enabled
    },

    setPreviewPanelSize(width, height) {
      this.previewSettings.panelWidth = width
      this.previewSettings.panelHeight = height
    },

    setPreviewModeRatios(imageModeHeight, documentModeHeight, widthRatio) {
      this.previewSettings.imageModeHeight = imageModeHeight
      this.previewSettings.documentModeHeight = documentModeHeight
      this.previewSettings.widthRatio = widthRatio
    },

    setPreviewCodeTheme(theme) {
      this.previewSettings.codeTheme = theme
    },

    setPreviewCodeFont(fontSize, fontFamily) {
      this.previewSettings.fontSize = fontSize
      if (fontFamily) {
        this.previewSettings.fontFamily = fontFamily
      }
    },

    setPreviewWordWrap(wordWrap) {
      this.previewSettings.wordWrap = wordWrap
    },

    // 获取当前应该使用的代码主题
    getCurrentCodeTheme() {
      if (this.previewSettings.codeTheme === 'auto') {
        return this.theme === 'dark' ? 'vs2015' : 'github'
      }
      return this.previewSettings.codeTheme
    },

    getDefaultPreviewSize(fileType, windowWidth, windowHeight) {
      const width = Math.floor(windowWidth * this.previewSettings.widthRatio)
      let height

      if (fileType === 'image') {
        height = Math.floor(windowHeight * this.previewSettings.imageModeHeight)
      } else {
        height = Math.floor(windowHeight * this.previewSettings.documentModeHeight) - 100 // 留一些边距
      }

      return { width, height }
    },

    resetToDefaults() {
      this.exportPath = ''
      this.autoOpenFolder = true
      this.theme = 'light'
      this.collapsedKeys = []
      this.previewSettings = {
        enabled: false,
        panelWidth: 400,
        panelHeight: 300,
        imageModeHeight: 0.5,
        documentModeHeight: 0.9,
        widthRatio: 1 / 3,
        codeTheme: 'auto',
        fontSize: '13px',
        fontFamily: 'Consolas, Monaco, "Courier New", monospace',
        wordWrap: false
      }
    }
  },

  persist: {
    storage: localStorage,
    key: 'wem-settings'
  }
})