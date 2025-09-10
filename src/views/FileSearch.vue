<template>
  <div class="file-search-container">
    <div class="search-header">
      <div class="search-form">
        <!-- 搜索框和文件类型筛选 - 同一行，紧凑布局 -->
        <div class="search-row" ref="searchRowRef">
          <div class="search-input-container" :style="{ width: searchInputWidth + 'px' }">
            <el-input v-model="searchQuery" placeholder="搜索关键字" @keyup.enter="handleSearch" class="search-input"
              size="large" clearable>
            </el-input>
          </div>
          
          <!-- 拖动分隔条 -->
          <div 
            class="resize-handle" 
            @mousedown="startResize"
            title="拖动调整搜索框和筛选区域宽度"
          >
            <div class="resize-line"></div>
          </div>
          
          <!-- 文件类型筛选区域 -->
          <div class="file-type-filters" :style="{ width: filterWidth + 'px' }">
            <div class="filter-content">
              <el-radio-group v-model="selectedFileType">
                <el-radio label="">所有文件</el-radio>
                <el-radio
                  v-for="(typeConfig, key) in fileTypesStore.visibleFileTypes"
                  :key="key"
                  :label="key"
                >
                  {{ typeConfig.name }}
                </el-radio>
                <el-radio 
                  v-if="fileTypesStore.specialFilters.file.isVisible !== false"
                  label="file"
                >
                  仅文件
                </el-radio>
                <el-radio 
                  v-if="fileTypesStore.specialFilters.folder.isVisible !== false"
                  label="folder"
                >
                  仅文件夹
                </el-radio>
              </el-radio-group>
              
              <!-- 操作按钮与单选按钮在同一行 -->
              <div class="filter-actions">
                <el-button 
                  type="primary" 
                  link 
                  @click="showAdvancedOptions = !showAdvancedOptions"
                  class="action-btn"
                  size="small"
                >
                  <span>搜索选项</span>
                  <el-icon :class="{ 'rotate-180': showAdvancedOptions }">
                    <ArrowDown />
                  </el-icon>
                </el-button>
                <el-button 
                  type="primary" 
                  link 
                  @click="showFileTypesManager = true"
                  class="action-btn"
                  size="small"
                >
                  管理
                </el-button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 搜索选项 - 可折叠 -->
      <div class="search-options" v-show="showAdvancedOptions">
        <el-row :gutter="20">
          <el-col :span="6">
            <el-form-item label="匹配大小写">
              <el-switch v-model="matchCase" size="small" />
            </el-form-item>
          </el-col>
          <el-col :span="6">
            <el-form-item label="全字匹配">
              <el-switch v-model="wholeWord" size="small" />
            </el-form-item>
          </el-col>
          <el-col :span="6">
            <el-form-item label="匹配路径">
              <el-switch v-model="matchPath" size="small" />
            </el-form-item>
          </el-col>
          <el-col :span="6">
            <el-form-item label="正则表达式">
              <el-switch v-model="useRegex" size="small" />
            </el-form-item>
          </el-col>
        </el-row>
      </div>
    </div>

    <div class="search-results" ref="scrollContainer" @scroll="debouncedHandleScroll">
      <div v-if="loading" class="loading-container">
        <el-skeleton :rows="5" animated />
      </div>

      <div v-else-if="error" class="error-container">
        <el-empty description="搜索出错">
          <el-button type="primary" @click="handleSearch">重试</el-button>
        </el-empty>
      </div>

      <div v-else-if="results.length === 0 && hasSearched" class="empty-container">
        <el-empty description="未找到相关文件" />
      </div>

      <div v-else-if="results.length > 0" class="results-container">
        <div class="results-header">
          <span>共找到 {{ totalResults }} 个文件</span>
          <div class="results-controls">
            <el-button type="primary" size="small" @click="showColumnSettings = true" :icon="Setting">
              列设置
            </el-button>
          </div>
        </div>

        <el-table 
          :data="results" 
          style="width: 100%;height: 100%;" 
          stripe 
          border 
          @header-dragend="onColumnResize"
          ref="tableRef"
        >
          <!-- 动态渲染列，按配置的顺序 -->
          <template v-for="(column, index) in settingsStore.visibleColumns" :key="column">
            <!-- 名称列 -->
            <el-table-column 
              v-if="column === 'name'"
              prop="name" 
              label="名称"
              :width="settingsStore.columnWidths.name"
              :min-width="150"
              resizable
              show-overflow-tooltip
            >
              <template #header>
                <div class="sortable-header" :class="{ active: sortBy === 'name' }" @click="handleSort('name')">
                  <span>名称</span>
                  <div class="sort-indicator" v-if="sortBy === 'name'">
                    <i class="el-icon-caret-top" :class="{ active: sortOrder === 1 }"></i>
                    <i class="el-icon-caret-bottom" :class="{ active: sortOrder === 0 }"></i>
                  </div>
                </div>
              </template>
              <template #default="{ row }">
                <div 
                  @dblclick="openFileDefault(getFullFilePath(row.path, row.name), row.type)"
                  style="cursor: pointer; width: 100%; height: 100%; padding: 0;"
                  :title="'双击' + (row.type === 'folder' ? '打开文件夹' : '打开文件')"
                >
                  {{ row.name }}
                </div>
              </template>
            </el-table-column>
            
            <!-- 路径列 -->
            <el-table-column 
              v-else-if="column === 'path'"
              prop="path" 
              label="路径"
              :width="settingsStore.columnWidths.path"
              :min-width="200"
              show-overflow-tooltip
              resizable
            >
              <template #header>
                <div class="sortable-header" :class="{ active: sortBy === 'path' }" @click="handleSort('path')">
                  <span>路径</span>
                  <div class="sort-indicator" v-if="sortBy === 'path'">
                    <i class="el-icon-caret-top" :class="{ active: sortOrder === 1 }"></i>
                    <i class="el-icon-caret-bottom" :class="{ active: sortOrder === 0 }"></i>
                  </div>
                </div>
              </template>
              <template #default="{ row }">
                <div 
                  @dblclick="openFileDefault(row.path, 'folder')"
                  style="cursor: pointer; width: 100%; height: 100%; padding: 0;"
                  title="双击打开所在文件夹"
                >
                  {{ row.path }}
                </div>
              </template>
            </el-table-column>
            
            <!-- 大小列 -->
            <el-table-column 
              v-else-if="column === 'size'"
              prop="size" 
              label="大小"
              :width="settingsStore.columnWidths.size"
              align="right"
              resizable
            >
              <template #header>
                <div class="sortable-header" :class="{ active: sortBy === 'size' }" @click="handleSort('size')">
                  <span>大小</span>
                  <div class="sort-indicator" v-if="sortBy === 'size'">
                    <i class="el-icon-caret-top" :class="{ active: sortOrder === 1 }"></i>
                    <i class="el-icon-caret-bottom" :class="{ active: sortOrder === 0 }"></i>
                  </div>
                </div>
              </template>
              <template #default="{ row }">
                <div 
                  @dblclick="openFileDefault(getFullFilePath(row.path, row.name), row.type)"
                  style="cursor: pointer; width: 100%; height: 100%; padding: 0; text-align: right;"
                  :title="'双击' + (row.type === 'folder' ? '打开文件夹' : '打开文件')"
                >
                  {{ formatFileSize(row.size) }}
                </div>
              </template>
            </el-table-column>
            
            <!-- 修改时间列 -->
            <el-table-column 
              v-else-if="column === 'date_modified'"
              prop="date_modified" 
              label="修改时间"
              :width="settingsStore.columnWidths.date_modified"
              resizable
            >
              <template #header>
                <div class="sortable-header" :class="{ active: sortBy === 'date_modified' }" @click="handleSort('date_modified')">
                  <span>修改时间</span>
                  <div class="sort-indicator" v-if="sortBy === 'date_modified'">
                    <i class="el-icon-caret-top" :class="{ active: sortOrder === 1 }"></i>
                    <i class="el-icon-caret-bottom" :class="{ active: sortOrder === 0 }"></i>
                  </div>
                </div>
              </template>
              <template #default="{ row }">
                <div 
                  @dblclick="openFileDefault(getFullFilePath(row.path, row.name), row.type)"
                  style="cursor: pointer; width: 100%; height: 100%; padding: 0;"
                  :title="'双击' + (row.type === 'folder' ? '打开文件夹' : '打开文件')"
                >
                  {{ formatDate(row.date_modified) }}
                </div>
              </template>
            </el-table-column>
            
            <!-- 类型列 -->
            <el-table-column 
              v-else-if="column === 'type'"
              prop="type" 
              label="类型" 
              :width="settingsStore.columnWidths.type"
              align="center"
              resizable
            >
              <template #default="{ row }">
                <div 
                  @dblclick="openFileDefault(getFullFilePath(row.path, row.name), row.type)"
                  style="cursor: pointer; width: 100%; height: 100%; padding: 0; display: flex; justify-content: center; align-items: center;"
                  :title="'双击' + (row.type === 'folder' ? '打开文件夹' : '打开文件')"
                >
                  <el-tag :type="row.type === 'folder' ? 'success' : 'info'" size="small">
                    {{ row.type === 'folder' ? '文件夹' : '文件' }}
                  </el-tag>
                </div>
              </template>
            </el-table-column>
          </template>
        </el-table>

        <!-- 无限滚动加载提示 -->
        <div v-if="loadingMore" class="loading-more">
          <el-skeleton :rows="3" animated />
        </div>
    
        
        <div v-if="!hasMore && results.length > 0" class="no-more-data">
          <el-text type="info" size="small">已加载全部 {{ totalResults }} 个结果</el-text>
        </div>
      </div>
    </div>
  </div>
  
  <!-- 文件类型管理对话框 -->
  <FileTypesManager v-model:visible="showFileTypesManager" />
  
  <!-- 列设置对话框 -->
  <el-dialog v-model="showColumnSettings" title="列设置" width="500px">
    <div class="column-settings">
      <h4>显示列</h4>
      <el-space direction="vertical" style="width: 100%">
        <el-checkbox 
          v-model="settingsStore.columnVisibility.name" 
          @change="(val) => settingsStore.setColumnVisibility('name', val)"
          disabled
        >
          名称（必选）
        </el-checkbox>
        <el-checkbox 
          v-model="settingsStore.columnVisibility.path" 
          @change="(val) => settingsStore.setColumnVisibility('path', val)"
        >
          路径
        </el-checkbox>
        <el-checkbox 
          v-model="settingsStore.columnVisibility.size" 
          @change="(val) => settingsStore.setColumnVisibility('size', val)"
        >
          大小
        </el-checkbox>
        <el-checkbox 
          v-model="settingsStore.columnVisibility.date_modified" 
          @change="(val) => settingsStore.setColumnVisibility('date_modified', val)"
        >
          修改时间
        </el-checkbox>
        <el-checkbox 
          v-model="settingsStore.columnVisibility.type" 
          @change="(val) => settingsStore.setColumnVisibility('type', val)"
        >
          类型
        </el-checkbox>
      </el-space>
      
      <el-divider />
      
      <h4>自动刷新设置</h4>
      <el-space direction="vertical" style="width: 100%">
        <el-checkbox 
          v-model="settingsStore.searchSettings.autoRefreshOnDateSort" 
          @change="(val) => settingsStore.setSearchSetting('autoRefreshOnDateSort', val)"
        >
          按修改时间排序时自动刷新
        </el-checkbox>
        <div v-if="settingsStore.searchSettings.autoRefreshOnDateSort">
          <span>刷新间隔：</span>
          <el-input-number 
            v-model="settingsStore.searchSettings.refreshInterval" 
            @change="(val) => settingsStore.setSearchSetting('refreshInterval', val)"
            :min="500" 
            :max="10000" 
            :step="500"
            size="small"
            style="width: 120px; margin-left: 10px;"
          />
          <span style="margin-left: 5px;">毫秒</span>
        </div>
      </el-space>
    </div>
    
    <template #footer>
      <span class="dialog-footer">
        <el-button @click="settingsStore.resetToDefaults()">重置默认</el-button>
        <el-button type="primary" @click="showColumnSettings = false">确定</el-button>
      </span>
    </template>
  </el-dialog>
</template>

<script setup>
import { ref, computed, watch, onMounted, onUnmounted, nextTick } from "vue";
import { ElMessage } from "element-plus";
import { Search, ArrowDown, Setting } from "@element-plus/icons-vue";
import { invoke } from "@tauri-apps/api/core";
import { openPath } from "@tauri-apps/plugin-opener";
import { useFileTypesStore } from "@/stores/fileTypes";
import { useFileSearchSettingsStore } from "@/stores/fileSearchSettings";
import FileTypesManager from "@/components/FileTypesManager.vue";

// 状态管理
const searchQuery = ref("");
const loading = ref(false);
const error = ref(false);
const results = ref([]);
const totalResults = ref(0);
const hasSearched = ref(false);

// 无限滚动相关
const loadingMore = ref(false);
const hasMore = ref(true);
const currentPage = ref(1);
const pageSize = ref(50); // 每次加载50条数据

// 文件类型筛选
const selectedFileType = ref(''); // 使用单个值而不是数组
const fileTypesStore = useFileTypesStore();
const showFileTypesManager = ref(false);

// 设置存储
const settingsStore = useFileSearchSettingsStore();

// 排序和搜索设置
const sortBy = ref(""); // 空字符串表示无排序
const sortOrder = ref(1); // 1: 升序, 0: 降序
const matchCase = ref(settingsStore.searchSettings.matchCase);
const matchPath = ref(settingsStore.searchSettings.matchPath);
const wholeWord = ref(settingsStore.searchSettings.wholeWord);
const useRegex = ref(settingsStore.searchSettings.useRegex);
const showAdvancedOptions = ref(settingsStore.searchSettings.showAdvancedOptions);

// 列显示控制
const showColumnSettings = ref(false);

// 搜索行引用
const searchRowRef = ref(null);

// 宽度调整相关 - 使用持久化设置
const searchInputWidth = ref(settingsStore.layoutSettings.searchInputWidth);
const filterWidth = ref(settingsStore.layoutSettings.filterWidth);
const isResizing = ref(false);
const minSearchWidth = 200;
const minFilterWidth = 300;

// 自动刷新相关
let autoRefreshTimer = null;



// 表格引用
const tableRef = ref(null);

// 宽度监控定时器
let widthWatcher = null;

// 计算属性
const offset = computed(() => (currentPage.value - 1) * pageSize.value);

// 获取完整的文件路径
const getFullFilePath = (path, name) => {
  // 检查 path 是否已经以路径分隔符结尾
  const separator = path.includes('/') ? '/' : '\\';
  if (path.endsWith(separator)) {
    return path + name;
  } else {
    return path + separator + name;
  }
};

// 格式化文件大小
const formatFileSize = (sizeStr) => {
  if (!sizeStr || sizeStr === '') return '-';
  const bytes = parseInt(sizeStr);
  if (isNaN(bytes)) return sizeStr; // 如果不是数字，直接返回原值
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB", "TB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
};

// 格式化日期
const formatDate = (timestamp) => {
  if (!timestamp) return "";
  // Everything 返回的是 Windows 文件时间格式，需要转换
  let date;
  if (typeof timestamp === 'string' && timestamp.length > 10) {
    // Windows FILETIME 格式（100 纳秒间隔，从 1601年开始）
    const fileTime = parseInt(timestamp);
    const unixTime = (fileTime - 116444736000000000) / 10000;
    date = new Date(unixTime);
  } else {
    date = new Date(parseInt(timestamp));
  }

  if (isNaN(date.getTime())) return timestamp; // 如果无法解析，返回原值
  return date.toLocaleDateString() + " " + date.toLocaleTimeString();
};

// 构建文件类型筛选条件
const buildFileTypeFilter = () => {
  if (!selectedFileType.value) {
    return '';
  }
  
  if (selectedFileType.value === 'file') {
    return 'file:';
  } else if (selectedFileType.value === 'folder') {
    return 'folder:';
  } else if (fileTypesStore.fileTypes[selectedFileType.value]) {
    const extensions = fileTypesStore.fileTypes[selectedFileType.value].extensions.join(';');
    return `ext:${extensions}`;
  }
  
  return '';
};

// 停止自动刷新
const stopAutoRefresh = () => {
  if (autoRefreshTimer) {
    clearInterval(autoRefreshTimer);
    autoRefreshTimer = null;
  }
};

// 启动自动刷新
const startAutoRefresh = () => {
  if (sortBy.value === 'date_modified' && settingsStore.searchSettings.autoRefreshOnDateSort) {
    stopAutoRefresh(); // 先停止现有的定时器
    autoRefreshTimer = setInterval(() => {
      // 自动刷新时不显示Loading，避免闪烁
      handleSearchSilent();
    }, settingsStore.searchSettings.refreshInterval);
  }
};

// 列宽度调整事件处理
const onColumnResize = (newWidth, _oldWidth, column) => {
  const columnMap = {
    '名称': 'name',
    '类型': 'type', 
    '路径': 'path',
    '大小': 'size',
    '修改时间': 'date_modified'
  };
  
  const columnKey = columnMap[column.label];
  if (columnKey) {
    settingsStore.setColumnWidth(columnKey, newWidth);
  }
};

// 手动应用列宽度配置（使用CSS强制覆盖）
const applyColumnWidths = async () => {
  await nextTick();
  if (tableRef.value) {
    try {
      setTimeout(() => {
        const tableInstance = tableRef.value;
        
        // 移除之前的样式
        const existingStyle = document.getElementById('custom-table-width-style');
        if (existingStyle) {
          existingStyle.remove();
        }
        
        // 创建新的样式标签
        const styleTag = document.createElement('style');
        styleTag.id = 'custom-table-width-style';
        
        let cssRules = '';
        
        // 为每个列生成CSS规则
        settingsStore.visibleColumns.forEach((column, index) => {
          const savedWidth = settingsStore.columnWidths[column];
          if (savedWidth) {
            const selector = `.el-table th:nth-child(${index + 1}), .el-table td:nth-child(${index + 1})`;
            const rule = `
              ${selector} {
                width: ${savedWidth}px !important;
                min-width: ${savedWidth}px !important;
                max-width: ${savedWidth}px !important;
              }
            `;
            cssRules += rule;
          }
        });
        
        if (cssRules) {
          styleTag.textContent = cssRules;
          document.head.appendChild(styleTag);
        }
        
        // 强制重新布局
        if (tableInstance.doLayout) {
          tableInstance.doLayout();
        }
        
      }, 100);
    } catch (error) {
      console.error('应用列宽度失败:', error);
    }
  }
};

// 启动宽度监控器（仅在排序后短时间内监控）
const startWidthWatcher = () => {
  if (widthWatcher) {
    clearInterval(widthWatcher);
  }
  
  // 只监控短时间，避免持续性能问题
  let watchCount = 0;
  const maxWatchCount = 10; // 最多监控10次（5秒）
  
  widthWatcher = setInterval(() => {
    if (tableRef.value && watchCount < maxWatchCount) {
      applyColumnWidths();
      watchCount++;
    } else {
      // 超过最大监控次数后自动停止
      stopWidthWatcher();
    }
  }, 500);
};

// 停止宽度监控器
const stopWidthWatcher = () => {
  if (widthWatcher) {
    clearInterval(widthWatcher);
    widthWatcher = null;
  }
};

// 搜索文件（静默模式，不显示Loading）
const handleSearchSilent = async () => {
  // 允许空关键字搜索
  let searchTerm = searchQuery.value.trim() || '*';
  
  // 添加文件类型筛选条件
  const fileTypeFilter = buildFileTypeFilter();
  if (fileTypeFilter) {
    searchTerm = fileTypeFilter + ' ' + searchTerm;
  }

  try {
    const searchParams = {
      search: searchTerm,
      offset: offset.value,
      count: pageSize.value,
      case: matchCase.value,
      wholeword: wholeWord.value,
      path: matchPath.value,
      regex: useRegex.value,
      ...settingsStore.columnParams
    };
    
    if (sortBy.value) {
      searchParams.sort = sortBy.value;
      searchParams.ascending = sortOrder.value === 1;
    }

    const result = await invoke('search_everything', searchParams);
    results.value = result.results || [];
    totalResults.value = result.totalResults || result.total_results || 0;
    
    // 在静默刷新后重新启动自动刷新
    startAutoRefresh();
  } catch (err) {
    // 静默失败时不显示错误消息，避免繁频弹窗
  }
};

// 搜索文件（新搜索，清空结果）
const handleSearch = async () => {
  // 停止之前的自动刷新
  stopAutoRefresh();
  
  // 重置状态
  currentPage.value = 1;
  results.value = [];
  hasMore.value = true;
  
  // 执行搜索
  await loadMoreData(true);
};

// 加载更多数据
const loadMoreData = async (isNewSearch = false) => {
  if (isNewSearch) {
    loading.value = true;
  } else {
    loadingMore.value = true;
  }
  
  error.value = false;
  hasSearched.value = true;

  try {
    // 允许空关键字搜索
    let searchTerm = searchQuery.value.trim() || '*'; // 空关键字时使用 * 表示全部
    
    // 添加文件类型筛选条件
    const fileTypeFilter = buildFileTypeFilter();
    if (fileTypeFilter) {
      searchTerm = fileTypeFilter + ' ' + searchTerm;
    }

    const searchParams = {
      search: searchTerm,
      offset: (currentPage.value - 1) * pageSize.value,
      count: pageSize.value,
      case: matchCase.value,
      wholeword: wholeWord.value,
      path: matchPath.value,
      regex: useRegex.value,
      // 添加列控制参数
      ...settingsStore.columnParams
    };
    
    // 只有在有排序字段时才添加排序参数
    if (sortBy.value) {
      searchParams.sort = sortBy.value;
      searchParams.ascending = sortOrder.value === 1;
    }

    const result = await invoke('search_everything', searchParams);

    const newResults = result.results || [];
    totalResults.value = result.totalResults || result.total_results || 0;
    
    if (isNewSearch) {
      results.value = newResults;
    } else {
      results.value = [...results.value, ...newResults];
    }
    
    // 检查是否还有更多数据
    hasMore.value = results.value.length < totalResults.value;
    
    if (isNewSearch && results.value.length === 0) {
      ElMessage.info("未找到相关文件");
    }
    
    // 在搜索成功后启动自动刷新（如果需要）
    startAutoRefresh();
  } catch (err) {
    error.value = true;
    ElMessage.error("搜索失败，请检查Everything服务是否运行");
  } finally {
    loading.value = false;
    loadingMore.value = false;
  }
};

// 处理列头排序点击
const handleSort = (column) => {
  // 停止之前的自动刷新
  stopAutoRefresh();
  
  if (sortBy.value === column) {
    // 同一列：升序 → 降序 → 无排序
    if (sortOrder.value === 1) {
      sortOrder.value = 0; // 切换到降序
    } else {
      // 取消排序
      sortBy.value = "";
      sortOrder.value = 1;
    }
  } else {
    // 不同列：设置为当前列的升序
    sortBy.value = column;
    sortOrder.value = 1;
  }
  
  // 重新搜索
  currentPage.value = 1;
  handleSearch().then(() => {
    // 搜索完成后应用保存的列宽度
    applyColumnWidths();
    // 启动短期监控确保宽度保持
    startWidthWatcher();
  });
};

// 双击文件名或路径：使用默认方式打开文件或文件夹
const openFileDefault = async (filePath, fileType) => {
  try {
    // 统一使用 openPath 打开文件和文件夹
    try {
      await openPath(filePath);
    } catch (openerError) {
      // 如果 Opener 插件失败，使用系统 shell 命令打开
      await invoke("shell_open", { path: filePath });
    }
    
    ElMessage.success(`已打开${fileType === 'folder' ? '文件夹' : '文件'}`);
  } catch (err) {
    ElMessage.error(`无法打开${fileType === 'folder' ? '文件夹' : '文件'}: ${err.message || err}`);
  }
};

// 分页变化处理
const handleSizeChange = (newSize) => {
  pageSize.value = newSize;
  settingsStore.setPageSize(newSize); // 保存到设置
  currentPage.value = 1;
  if (hasSearched.value) {
    handleSearch().then(() => {
      applyColumnWidths();
    });
  }
};

const handleCurrentChange = (newPage) => {
  currentPage.value = newPage;
  if (hasSearched.value) {
    handleSearch().then(() => {
      applyColumnWidths();
    });
  }
};

// 监听筛选选项变化，立即搜索
watch([matchCase, matchPath, wholeWord, useRegex], (newValues, oldValues) => {
  // 保存设置到store
  settingsStore.setSearchSettings({
    matchCase: matchCase.value,
    matchPath: matchPath.value,
    wholeWord: wholeWord.value,
    useRegex: useRegex.value
  });
  
  // 搜索条件变化时立即搜索
  currentPage.value = 1;
  handleSearch().then(() => {
    applyColumnWidths();
  });
});

// 监听高级选项显示状态
watch(showAdvancedOptions, (newValue) => {
  settingsStore.setSearchSetting('showAdvancedOptions', newValue);
});

// 获取列显示名称
const getColumnDisplayName = (columnKey) => {
  const nameMap = {
    name: '名称',
    path: '路径',
    size: '大小',
    date_modified: '修改时间',
    type: '类型'
  };
  return nameMap[columnKey] || columnKey;
};

// 无限滚动相关
const scrollContainer = ref(null);

// 滚动事件处理
const handleScroll = () => {
  if (!scrollContainer.value || loadingMore.value || !hasMore.value) return;
  
  const container = scrollContainer.value;
  const { scrollTop, scrollHeight, clientHeight } = container;
  
  // 当滚动到距离底部 100px 以内时触发加载
  if (scrollTop + clientHeight >= scrollHeight - 100) {
    currentPage.value++;
    loadMoreData(false);
  }
};

// 防抖处理滚动事件
let scrollTimeout = null;
const debouncedHandleScroll = () => {
  if (scrollTimeout) {
    clearTimeout(scrollTimeout);
  }
  scrollTimeout = setTimeout(handleScroll, 150);
};

// 拖动调整宽度功能（基于容器实际宽度）
const startResize = (event) => {
  if (!searchRowRef.value) return;
  
  isResizing.value = true;
  const startX = event.clientX;
  const startSearchWidth = searchInputWidth.value;
  const startFilterWidth = filterWidth.value;
  
  // 获取容器实际宽度（减去拖动手柄和gap的宽度）
  const containerWidth = searchRowRef.value.offsetWidth - 8 - 8; // 8px 是拖动手柄宽度，8px 是 gap
  
  const onMouseMove = (e) => {
    if (!isResizing.value) return;
    
    const deltaX = e.clientX - startX;
    let newSearchWidth = startSearchWidth + deltaX;
    let newFilterWidth = startFilterWidth - deltaX;
    
    // 限制最小宽度
    if (newSearchWidth < minSearchWidth) {
      newSearchWidth = minSearchWidth;
      newFilterWidth = containerWidth - minSearchWidth;
    }
    if (newFilterWidth < minFilterWidth) {
      newFilterWidth = minFilterWidth;
      newSearchWidth = containerWidth - minFilterWidth;
    }
    
    searchInputWidth.value = newSearchWidth;
    filterWidth.value = newFilterWidth;
  };
  
  const onMouseUp = () => {
    if (isResizing.value) {
      isResizing.value = false;
      // 保存新的宽度设置到store
      settingsStore.setLayoutSizes(searchInputWidth.value, filterWidth.value);
      document.removeEventListener('mousemove', onMouseMove);
      document.removeEventListener('mouseup', onMouseUp);
    }
  };
  
  document.addEventListener('mousemove', onMouseMove);
  document.addEventListener('mouseup', onMouseUp);
};

// 监听搜索条件变化，重置滚动状态并立即搜索
watch([selectedFileType, searchQuery], () => {
  currentPage.value = 1;
  hasMore.value = true;
  // 搜索条件变化时立即搜索
  handleSearch();
});

// 组件挂载时的初始化
onMounted(() => {
  // 为滚动容器添加滚动监听
  if (scrollContainer.value) {
    scrollContainer.value.addEventListener('scroll', debouncedHandleScroll, { passive: true });
  }
  
  // 页面加载时自动执行一次空关键字搜索初始化结果列表
  handleSearch();
});

// 清理工作
onUnmounted(() => {
  stopAutoRefresh();
  stopWidthWatcher();
  
  // 清理滚动监听
  if (scrollContainer.value) {
    scrollContainer.value.removeEventListener('scroll', debouncedHandleScroll);
  }
  
  // 清理滚动防抖定时器
  if (scrollTimeout) {
    clearTimeout(scrollTimeout);
  }
});
</script>

<style lang="scss" src="../assets/styles/flie-search.scss" scoped/>