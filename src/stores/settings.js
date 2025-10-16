import { defineStore } from 'pinia'

export const useSettingsStore = defineStore('settings', {
  state: () => ({
    exportPath: '',
    autoOpenFolder: true,
    afterOpenProjectBehavior: 'minimize', // 打开项目后的行为：'minimize'（最小化到托盘）、'quit'（退出应用）、'none'（无动作，默认）
    afterCopyCommitBehavior: 'minimize', // 复制提交信息后的行为：'minimize'（最小化到托盘）、'quit'（退出应用）、'none'（无动作，默认）
    vscodeStoragePath: '', // 自定义 VSCode storage.json 路径，留空则自动推断
    traeStoragePath: '', // 自定义 Trae storage.json 路径
    qoderStoragePath: '', // 自定义 Qoder storage.json 路径
    ideaStoragePath: '', // 自定义 IDEA recentProjects.xml 路径
    webstormStoragePath: '', // 自定义 WebStorm recentProjects.xml 路径
    pycharmStoragePath: '', // 自定义 PyCharm recentProjects.xml 路径
    vscodeExecutablePath: '', // 自定义 VSCode 可执行文件路径
    traeExecutablePath: '', // 自定义 Trae 可执行文件路径
    qoderExecutablePath: '', // 自定义 Qoder 可执行文件路径
    ideaExecutablePath: '', // 自定义 IDEA 可执行文件路径
    webstormExecutablePath: '', // 自定义 WebStorm 可执行文件路径
    pycharmExecutablePath: '', // 自定义 PyCharm 可执行文件路径
    theme: 'light', // 主题：light 或 dark
    collapsedKeys: [], // 存储折叠面板的key
    currentRoute: 'FileSearch', // 当前页面路由，默认为文件搜索页面
    lastCacheClearTime: null, // 上次清理缓存的时间
    // 全局快捷键设置
    globalShortcuts: {
      envVarManager: '', // 打开环境变量管理界面的快捷键
      fileSearch: '', // 打开文件搜索界面的快捷键
      projects: '', // 打开项目管理界面的快捷键
      commitGenerator: '', // 打开提交生成器界面的快捷键
    },
    // 窗口关闭行为设置
    closeToTray: true, // 点击关闭按钮时是否最小化到托盘，默认为 true（最小化到托盘）

    // 提交生成器设置
    commitGenerator: {
      useEmoji: true, // 是否使用 Emoji 表情，默认为 true
      autoClassify: true, // 是否启用自动分类，默认为 true
      // 自动分类规则配置
      classifyRules: {
        fix: {
          startsWith: ['修复', '修正', 'fix'],
          contains: ['bug', '错误', '问题'],
          endsWith: []
        },
        feat: {
          startsWith: ['新增', '添加', 'feat', 'feature'],
          contains: ['功能', '特性', '实现'],
          endsWith: []
        },
        docs: {
          startsWith: ['文档', 'doc', 'docs'],
          contains: ['注释', '说明'],
          endsWith: []
        },
        style: {
          startsWith: ['样式', 'style'],
          contains: ['格式', '美化', 'ui'],
          endsWith: []
        },
        refactor: {
          startsWith: ['重构', 'refactor'],
          contains: ['优化代码', '代码优化'],
          endsWith: []
        },
        perf: {
          startsWith: ['性能', 'performance', 'perf'],
          contains: ['优化性能', '提升性能'],
          endsWith: []
        },
        test: {
          startsWith: ['测试', 'test'],
          contains: ['单元测试'],
          endsWith: []
        },
        chore: {
          startsWith: ['构建', 'build', 'chore'],
          contains: ['配置', '依赖'],
          endsWith: []
        },
        revert: {
          startsWith: ['回退', '撤销', 'revert'],
          contains: ['还原'],
          endsWith: []
        }
      }
    },

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

    setAfterOpenProjectBehavior(value) {
      this.afterOpenProjectBehavior = value
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

    setLastCacheClearTime(time) {
      this.lastCacheClearTime = time
    },

    // 全局快捷键管理
    setGlobalShortcut(key, shortcut) {
      if (this.globalShortcuts.hasOwnProperty(key)) {
        this.globalShortcuts[key] = shortcut
      }
    },

    getGlobalShortcut(key) {
      return this.globalShortcuts[key] || ''
    },

    clearGlobalShortcut(key) {
      if (this.globalShortcuts.hasOwnProperty(key)) {
        this.globalShortcuts[key] = ''
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

    // 提交生成器设置相关方法
    setCommitGeneratorUseEmoji(useEmoji) {
      this.commitGenerator.useEmoji = useEmoji
    },

    setCommitGeneratorAutoClassify(autoClassify) {
      this.commitGenerator.autoClassify = autoClassify
    },

    // 设置自动分类规则
    setClassifyRules(rules) {
      this.commitGenerator.classifyRules = rules
    },

    // 更新特定类型的分类规则
    updateClassifyRule(type, ruleType, keywords) {
      if (!this.commitGenerator.classifyRules[type]) {
        this.commitGenerator.classifyRules[type] = {
          startsWith: [],
          contains: [],
          endsWith: []
        }
      }
      this.commitGenerator.classifyRules[type][ruleType] = keywords
    },

    // 重置分类规则为默认值
    resetClassifyRules() {
      this.commitGenerator.classifyRules = {
        fix: {
          startsWith: ['修复', '修正', 'fix'],
          contains: ['bug', '错误', '问题'],
          endsWith: []
        },
        feat: {
          startsWith: ['新增', '添加', 'feat', 'feature'],
          contains: ['功能', '特性', '实现'],
          endsWith: []
        },
        docs: {
          startsWith: ['文档', 'doc', 'docs'],
          contains: ['注释', '说明'],
          endsWith: []
        },
        style: {
          startsWith: ['样式', 'style'],
          contains: ['格式', '美化', 'ui'],
          endsWith: []
        },
        refactor: {
          startsWith: ['重构', 'refactor'],
          contains: ['优化代码', '代码优化'],
          endsWith: []
        },
        perf: {
          startsWith: ['性能', 'performance', 'perf'],
          contains: ['优化性能', '提升性能'],
          endsWith: []
        },
        test: {
          startsWith: ['测试', 'test'],
          contains: ['单元测试'],
          endsWith: []
        },
        chore: {
          startsWith: ['构建', 'build', 'chore'],
          contains: ['配置', '依赖'],
          endsWith: []
        },
        revert: {
          startsWith: ['回退', '撤销', 'revert'],
          contains: ['还原'],
          endsWith: []
        }
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
      this.afterOpenProjectBehavior = 'minimize'
      this.vscodeStoragePath = ''
      this.traeStoragePath = ''
      this.qoderStoragePath = ''
      this.ideaStoragePath = ''
      this.webstormStoragePath = ''
      this.vscodeExecutablePath = ''
      this.traeExecutablePath = ''
      this.qoderExecutablePath = ''
      this.ideaExecutablePath = ''
      this.webstormExecutablePath = ''
      this.theme = 'light'
      this.collapsedKeys = []
      this.currentRoute = 'FileSearch'
      this.lastCacheClearTime = null
      this.closeToTray = true
      this.commitGenerator = {
        useEmoji: true,
      }
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