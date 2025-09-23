import { defineStore } from 'pinia'

export const useSettingsStore = defineStore('settings', {
  state: () => ({
    exportPath: '',
    autoOpenFolder: true,
    vscodeStoragePath: '', // 自定义 VSCode storage.json 路径，留空则自动推断
    traeStoragePath: '', // 自定义 Trae storage.json 路径
    qoderStoragePath: '', // 自定义 Qoder storage.json 路径
    ideaStoragePath: '', // 自定义 IDEA recentProjects.xml 路径
    vscodeExecutablePath: '', // 自定义 VSCode 可执行文件路径
    traeExecutablePath: '', // 自定义 Trae 可执行文件路径
    qoderExecutablePath: '', // 自定义 Qoder 可执行文件路径
    ideaExecutablePath: '', // 自定义 IDEA 可执行文件路径
    theme: 'light', // 主题：light 或 dark
    collapsedKeys: [], // 存储折叠面板的key
    currentRoute: 'FileSearch', // 当前页面路由，默认为文件搜索页面
    // 窗口状态管理
    windowState: {
      position: null, // 窗口位置 {x: number, y: number}
      size: null, // 窗口大小 {width: number, height: number}
      isMaximized: false, // 是否最大化
      isMinimized: false, // 是否最小化
    },
    // 预览面板设置
    previewSettings: {
      enabled: false, // 预览模式是否启用
      codeTheme: 'auto', // 代码高亮主题：'auto' | 'vs2015' | 'github' | 'atom-one-dark' | 'atom-one-light' | 'monokai'
      fontSize: '13px', // 代码字体大小
      fontFamily: 'Consolas, Monaco, "Courier New", monospace', // 代码字体
      wordWrap: false, // 是否自动换行，默认关闭
      // 按文件类型分别配置预览窗口
      image: {
        panelWidth: 400,
        panelHeight: 300,
        widthRatio: 1 / 3,
        heightRatio: 0.5
      },
      text: {
        panelWidth: 500,
        panelHeight: 400,
        widthRatio: 1 / 3,
        heightRatio: 0.8
      },
      pdf: {
        panelWidth: 600,
        panelHeight: 800,
        widthRatio: 0.4,
        heightRatio: 0.9
      },
      table: {
        panelWidth: 700,
        panelHeight: 500,
        widthRatio: 0.5,
        heightRatio: 0.8
      }
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

    setCurrentRoute(route) {
      this.currentRoute = route
    },

    // 窗口状态管理方法
    setWindowPosition(position) {
      this.windowState.position = position
    },

    setWindowSize(size) {
      this.windowState.size = size
    },

    setWindowMaximized(isMaximized) {
      this.windowState.isMaximized = isMaximized
    },

    setWindowMinimized(isMinimized) {
      this.windowState.isMinimized = isMinimized
    },

    updateWindowState(state) {
      Object.assign(this.windowState, state)
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

    setPreviewPanelSize(fileType, width, height) {
      if (this.previewSettings[fileType]) {
        this.previewSettings[fileType].panelWidth = width
        this.previewSettings[fileType].panelHeight = height
      }
    },

    setPreviewPanelRatios(fileType, widthRatio, heightRatio) {
      if (this.previewSettings[fileType]) {
        this.previewSettings[fileType].widthRatio = widthRatio
        this.previewSettings[fileType].heightRatio = heightRatio
      }
    },

    getPreviewConfig(fileType) {
      // 确保配置存在，如果不存在则使用默认值
      if (!this.previewSettings[fileType]) {
        const defaults = {
          image: {
            panelWidth: 400,
            panelHeight: 300,
            widthRatio: 1 / 3,
            heightRatio: 0.5
          },
          text: {
            panelWidth: 500,
            panelHeight: 400,
            widthRatio: 1 / 3,
            heightRatio: 0.8
          },
          pdf: {
            panelWidth: 600,
            panelHeight: 800,
            widthRatio: 0.4,
            heightRatio: 0.9
          }
        }
        return defaults[fileType] || defaults.text
      }
      return this.previewSettings[fileType]
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
      const config = this.getPreviewConfig(fileType)
      const width = Math.floor(windowWidth * config.widthRatio)
      const height = Math.floor(windowHeight * config.heightRatio)

      return { width, height }
    },

    resetToDefaults() {
      this.exportPath = ''
      this.autoOpenFolder = true
      this.vscodeStoragePath = ''
      this.traeStoragePath = ''
      this.vscodeExecutablePath = ''
      this.traeExecutablePath = ''
      this.theme = 'light'
      this.collapsedKeys = []
      this.currentRoute = 'FileSearch'
      this.windowState = {
        position: null,
        size: null,
        isMaximized: false,
        isMinimized: false,
      }
      this.previewSettings = {
        enabled: false,
        codeTheme: 'auto',
        fontSize: '13px',
        fontFamily: 'Consolas, Monaco, "Courier New", monospace',
        wordWrap: false,
        image: {
          panelWidth: 400,
          panelHeight: 300,
          widthRatio: 1 / 3,
          heightRatio: 0.5
        },
        text: {
          panelWidth: 500,
          panelHeight: 400,
          widthRatio: 1 / 3,
          heightRatio: 0.8
        },
        pdf: {
          panelWidth: 600,
          panelHeight: 800,
          widthRatio: 0.4,
          heightRatio: 0.9
        },
        table: {
          panelWidth: 700,
          panelHeight: 500,
          widthRatio: 0.5,
          heightRatio: 0.8
        }
      }
    }
  },

  persist: {
    storage: localStorage,
    key: 'wem-settings'
  }
})