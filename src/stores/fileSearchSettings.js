import { defineStore } from 'pinia'

export const useFileSearchSettingsStore = defineStore('fileSearchSettings', {
  state: () => ({
    // 表格列显示配置
    columnVisibility: {
      name: true,
      path: true,
      size: true,
      date_modified: true,
      type: true
    },
    
    // 表格列宽度配置
    columnWidths: {
      name: 200,
      path: 300, 
      size: 100,
      date_modified: 180,
      type: 80
    },
    
    // 表格列顺序配置（根据规范，类型列应该在最后）
    columnOrder: ['name', 'path', 'size', 'date_modified', 'type'],
    
    // 搜索设置
    searchSettings: {
      matchCase: false,
      wholeWord: false,
      matchPath: false,
      useRegex: false,
      showAdvancedOptions: false,
      autoRefreshOnDateSort: true, // 按修改时间排序时是否自动刷新
      refreshInterval: 1000 // 自动刷新间隔（毫秒）
    },
    
    // 分页设置
    paginationSettings: {
      pageSize: 20,
      pageSizes: [20, 50, 100]
    },
    
    // 界面布局设置
    layoutSettings: {
      searchInputWidth: 400,
      filterWidth: 600
    },
    
    // Everything服务配置
    everythingSettings: {
      host: 'localhost',
      port: 8080
    },
    
    // 界面外观设置
    appearanceSettings: {
      tableFontFamily: 'inherit',
      tableFontSize: '14px'
    }
  }),

  getters: {
    // 获取可见列配置（按顺序）
    visibleColumns: (state) => {
      return state.columnOrder
        .filter(column => state.columnVisibility[column])
    },
    
    // 获取列控制参数（用于后端API）
    columnParams: (state) => ({
      path_column: state.columnVisibility.path ? 1 : 0,
      size_column: state.columnVisibility.size ? 1 : 0,
      date_modified_column: state.columnVisibility.date_modified ? 1 : 0
    })
  },

  actions: {
    // 设置列显示状态
    setColumnVisibility(column, visible) {
      this.columnVisibility[column] = visible
    },
    
    // 设置列宽度
    setColumnWidth(column, width) {
      this.columnWidths[column] = width
    },
    
    // 批量设置列宽度
    setColumnWidths(widths) {
      this.columnWidths = { ...this.columnWidths, ...widths }
    },
    
    // 设置列顺序
    setColumnOrder(order) {
      this.columnOrder = [...order]
    },
    
    // 移动列位置
    moveColumn(fromIndex, toIndex) {
      const newOrder = [...this.columnOrder]
      const [movedColumn] = newOrder.splice(fromIndex, 1)
      newOrder.splice(toIndex, 0, movedColumn)
      this.columnOrder = newOrder
    },
    
    // 设置搜索选项
    setSearchSetting(key, value) {
      this.searchSettings[key] = value
    },
    
    // 批量设置搜索选项
    setSearchSettings(settings) {
      this.searchSettings = { ...this.searchSettings, ...settings }
    },
    
    // 设置分页大小
    setPageSize(size) {
      this.paginationSettings.pageSize = size
    },
    
    // 设置搜索输入框宽度
    setSearchInputWidth(width) {
      this.layoutSettings.searchInputWidth = width
    },
    
    // 设置筛选区域宽度
    setFilterWidth(width) {
      this.layoutSettings.filterWidth = width
    },
    
    // 设置Everything服务配置
    setEverythingHost(host) {
      this.everythingSettings.host = host
    },
    
    setEverythingPort(port) {
      this.everythingSettings.port = port
    },
    
    setEverythingSettings(settings) {
      this.everythingSettings = { ...this.everythingSettings, ...settings }
    },
    
    // 设置界面外观
    setTableFontFamily(fontFamily) {
      this.appearanceSettings.tableFontFamily = fontFamily
    },
    
    setTableFontSize(fontSize) {
      this.appearanceSettings.tableFontSize = fontSize
    },
    
    setAppearanceSettings(settings) {
      this.appearanceSettings = { ...this.appearanceSettings, ...settings }
    },
    
    // 批量设置布局尺寸
    setLayoutSizes(searchInputWidth, filterWidth) {
      this.layoutSettings.searchInputWidth = searchInputWidth
      this.layoutSettings.filterWidth = filterWidth
    },
    
    // 切换列显示状态
    toggleColumnVisibility(column) {
      this.columnVisibility[column] = !this.columnVisibility[column]
    },
    
    // 重置为默认配置
    resetToDefaults() {
      this.columnVisibility = {
        name: true,
        path: true,
        size: true,
        date_modified: true,
        type: true
      }
      
      this.columnWidths = {
        name: 200,
        path: 300,
        size: 100,
        date_modified: 180,
        type: 80
      }
      
      this.columnOrder = ['name', 'path', 'size', 'date_modified', 'type']
      
      this.searchSettings = {
        matchCase: false,
        wholeWord: false,
        matchPath: false,
        useRegex: false,
        showAdvancedOptions: false,
        autoRefreshOnDateSort: true,
        refreshInterval: 1000
      }
      
      this.paginationSettings = {
        pageSize: 20,
        pageSizes: [20, 50, 100]
      }
      
      this.layoutSettings = {
        searchInputWidth: 400,
        filterWidth: 600
      }
      
      this.everythingSettings = {
        host: 'localhost',
        port: 8080
      }
      
      this.appearanceSettings = {
        tableFontFamily: 'inherit',
        tableFontSize: '14px'
      }
    }
  },

  persist: {
    storage: localStorage,
    key: 'wem-file-search-settings'
  }
})