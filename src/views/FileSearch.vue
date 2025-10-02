<template>
  <div class="file-search-container">
    <div class="search-header">
      <div class="search-form">
        <!-- 搜索框和文件类型筛选 - 同一行，紧凑布局 -->
        <div class="search-row" ref="searchRowRef">
          <div class="search-input-container" :style="{ width: searchInputWidth + 'px' }">
            <el-input ref="searchInputRef" v-model="searchQuery" placeholder="搜索关键字" @keyup.enter="handleSearch"
              class="search-input" size="large" clearable>
              <template #append>
                <el-button :icon="Folder" @click="selectFolder" title="选择文件夹" class="folder-select-btn">
                </el-button>
              </template>
            </el-input>
          </div>

          <!-- 拖动分隔条 -->
          <div class="resize-handle" @mousedown="startResize" title="拖动调整搜索框和筛选区域宽度">
            <div class="resize-line"></div>
          </div>

          <!-- 文件类型筛选区域 -->
          <div class="file-type-filters" :style="{ width: filterWidth + 'px' }">
            <div class="filter-content">
              <el-radio-group v-model="selectedFileType">
                <el-radio label="">所有文件</el-radio>
                <el-radio v-for="(typeConfig, key) in fileTypesStore.visibleFileTypes" :key="key" :label="key">
                  {{ typeConfig.name }}
                </el-radio>
                <el-radio v-if="fileTypesStore.specialFilters.file.isVisible !== false" label="file">
                  仅文件
                </el-radio>
                <el-radio v-if="fileTypesStore.specialFilters.folder.isVisible !== false" label="folder">
                  仅文件夹
                </el-radio>
              </el-radio-group>

              <!-- 操作按钮与单选按钮在同一行 -->
              <div class="filter-actions">
                <el-button type="primary" link @click="showAdvancedOptions = !showAdvancedOptions" class="action-btn"
                  size="small">
                  <span>搜索选项</span>
                  <el-icon :class="{ 'rotate-180': showAdvancedOptions }">
                    <ArrowDown />
                  </el-icon>
                </el-button>
                <el-button type="primary" link @click="showFileTypesManager = true" class="action-btn" size="small">
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

    <!-- 设置区域，始终可见 -->
    <div class="settings-toolbar">
      <div class="toolbar-content">
        <div class="search-status">
          <span v-if="hasSearched && !loading">
            <span v-if="error">搜索服务不可用</span>
            <span v-else-if="results.length === 0">未找到相关文件</span>
            <span v-else>共找到 {{ totalResults }} 个文件</span>
          </span>
        </div>
        <div class="toolbar-buttons">
          <!-- 预览切换按钮 -->
          <el-button :type="previewEnabled ? 'primary' : 'info'" size="small"
            @click="handlePreviewToggle(!previewEnabled)" :title="previewEnabled ? '关闭预览' : '开启预览'"
            style="margin-right: 12px;">
            <el-icon>
              <View />
            </el-icon>
          </el-button>

          <!-- 设置下拉菜单 -->
          <el-dropdown trigger="hover" @command="handleSettingsCommand">
            <el-button type="warning" size="small" :icon="Setting">
              设置
              <el-icon class="el-icon--right">
                <ArrowDown />
              </el-icon>
            </el-button>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item command="exportSettings" :icon="Download">导出设置</el-dropdown-item>
                <el-dropdown-item command="importSettings" :icon="Upload">导入设置</el-dropdown-item>
                <el-dropdown-item divided command="appearance" :icon="Brush">界面设置</el-dropdown-item>
                <el-dropdown-item command="everything" :icon="Tools">服务设置</el-dropdown-item>
                <el-dropdown-item command="columns" :icon="Setting">列设置</el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
        </div>
      </div>
    </div>

    <div class="search-results" ref="scrollContainer" @scroll="debouncedHandleScroll">
      <div v-if="loading" class="loading-container">
        <el-skeleton :rows="5" animated />
      </div>

      <div v-else-if="error" class="error-container">
        <el-empty description="Everything搜索服务连接失败">
        </el-empty>
      </div>

      <div v-else-if="results.length === 0 && hasSearched" class="empty-container">
        <el-empty description="未找到相关文件" />
      </div>

      <div v-else-if="results.length > 0" class="results-section">

        <div class="results-container" @keydown="handleKeyDown" @click="focusResultsContainer" tabindex="0"
          ref="resultsContainer">
          <el-table :data="results" style="width: 100%;height: 100%;" stripe border @header-dragend="onColumnResize"
            @row-click="handleRowClick" :row-class-name="getRowClassName" ref="tableRef">
            <!-- 动态渲染列，按配置的顺序 -->
            <template v-for="column in settingsStore.visibleColumns" :key="column">
              <!-- 名称列 -->
              <el-table-column v-if="column === 'name'" prop="name" label="名称" :width="settingsStore.columnWidths.name"
                :min-width="150" resizable show-overflow-tooltip>
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
                  <div @dblclick="openFileDefault(getFullFilePath(row.path, row.name), row.type)"
                    style="cursor: pointer; width: 100%; height: 100%; padding: 0; display: flex; align-items: center; gap: 8px;"
                    :title="'单击选中，双击' + (row.type === 'folder' ? '打开文件夹' : '打开文件')">
                    <FileIcon :file-path="getFullFilePath(row.path, row.name)" :file-name="row.name"
                      :file-type="row.type" :size="16" />
                    <span style="flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;"
                      v-html="highlightMatchedText(row.name)"></span>
                  </div>
                </template>
              </el-table-column>

              <!-- 路径列 -->
              <el-table-column v-else-if="column === 'path'" prop="path" label="路径"
                :width="settingsStore.columnWidths.path" :min-width="200" show-overflow-tooltip resizable>
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
                  <div @dblclick="openFileDefault(row.path, 'folder')"
                    style="cursor: pointer; width: 100%; height: 100%; padding: 0; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;"
                    title="双击打开所在文件夹" v-html="matchPath ? highlightMatchedText(row.path) : row.path">
                  </div>
                </template>
              </el-table-column>

              <!-- 大小列 -->
              <el-table-column v-else-if="column === 'size'" prop="size" label="大小"
                :width="settingsStore.columnWidths.size" align="right" resizable>
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
                  <div @dblclick="openFileDefault(getFullFilePath(row.path, row.name), row.type)"
                    style="cursor: pointer; width: 100%; height: 100%; padding: 0; text-align: right; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;"
                    :title="'双击' + (row.type === 'folder' ? '打开文件夹' : '打开文件')">
                    {{ formatFileSize(row.size) }}
                  </div>
                </template>
              </el-table-column>

              <!-- 修改时间列 -->
              <el-table-column v-else-if="column === 'date_modified'" prop="date_modified" label="修改时间"
                :width="settingsStore.columnWidths.date_modified" resizable>
                <template #header>
                  <div class="sortable-header" :class="{ active: sortBy === 'date_modified' }"
                    @click="handleSort('date_modified')">
                    <span>修改时间</span>
                    <div class="sort-indicator" v-if="sortBy === 'date_modified'">
                      <i class="el-icon-caret-top" :class="{ active: sortOrder === 1 }"></i>
                      <i class="el-icon-caret-bottom" :class="{ active: sortOrder === 0 }"></i>
                    </div>
                  </div>
                </template>
                <template #default="{ row }">
                  <div @dblclick="openFileDefault(getFullFilePath(row.path, row.name), row.type)"
                    style="cursor: pointer; width: 100%; height: 100%; padding: 0; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;"
                    :title="'双击' + (row.type === 'folder' ? '打开文件夹' : '打开文件')">
                    {{ formatDate(row.date_modified) }}
                  </div>
                </template>
              </el-table-column>

              <!-- 类型列 -->
              <el-table-column v-else-if="column === 'type'" prop="type" label="类型"
                :width="settingsStore.columnWidths.type" align="center" resizable>
                <template #default="{ row }">
                  <div @dblclick="openFileDefault(getFullFilePath(row.path, row.name), row.type)"
                    style="cursor: pointer; width: 100%; height: 100%; padding: 0; display: flex; justify-content: center; align-items: center; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;"
                    :title="'双击' + (row.type === 'folder' ? '打开文件夹' : '打开文件')">
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
  </div>

  <!-- 界面设置对话框 -->
  <el-dialog v-model="showAppearanceSettings" title="界面外观设置" width="600px">
    <el-form :model="appearanceForm" label-width="120px">
      <el-form-item label="表格字体">
        <el-input v-model="appearanceForm.tableFontFamily" placeholder="多个字体用;分隔" clearable />
      </el-form-item>
      <el-form-item label="字体大小">
        <el-input-number v-model="fontSizeNumber" :min="1" :max="100" :step="1" placeholder="字体大小，单位px"
          style="width: 100%" />
      </el-form-item>

      <!-- 代码预览主题设置 -->
      <el-divider>代码预览设置</el-divider>
      <el-form-item label="代码主题">
        <el-select v-model="codeThemeSelection" placeholder="选择代码高亮主题" style="width: 100%">
          <el-option label="跟随应用主题" value="auto" />
          <el-option label="VS2015 (深色)" value="vs2015" />
          <el-option label="GitHub (浅色)" value="github" />
          <el-option label="Atom One Dark" value="atom-one-dark" />
          <el-option label="Atom One Light" value="atom-one-light" />
          <el-option label="Monokai" value="monokai" />
          <el-option label="Tomorrow Night" value="tomorrow-night" />
          <el-option label="Dracula" value="dracula" />
        </el-select>
      </el-form-item>
      <el-form-item>
        <el-text type="info" size="small">
          💡 代码预览字体会跟随表格字体设置，主题可独立选择
        </el-text>
      </el-form-item>
    </el-form>

    <template #footer>
      <span class="dialog-footer">
        <el-button @click="resetAppearanceSettings">重置默认</el-button>
        <el-button @click="showAppearanceSettings = false">取消</el-button>
        <el-button type="primary" @click="saveAppearanceSettings">保存</el-button>
      </span>
    </template>
  </el-dialog>

  <!-- Everything设置对话框 -->
  <el-dialog v-model="showEverythingSettings" title="Everything服务设置" width="400px">
    <el-form :model="everythingForm" label-width="80px">
      <el-form-item label="主机地址">
        <el-input v-model="everythingForm.host" placeholder="请输入主机地址" clearable />
      </el-form-item>
      <el-form-item label="端口号">
        <el-input-number v-model="everythingForm.port" :min="1" :max="65535" placeholder="请输入端口号" style="width: 100%" />
      </el-form-item>
      <el-form-item>
        <el-text type="info" size="small">
          everything -> 工具 -> 选项 -> HTTP服务器
        </el-text>
      </el-form-item>
    </el-form>

    <template #footer>
      <span class="dialog-footer">
        <el-button @click="resetEverythingSettings">重置默认</el-button>
        <el-button @click="showEverythingSettings = false">取消</el-button>
        <el-button type="primary" @click="saveEverythingSettings">保存</el-button>
      </span>
    </template>
  </el-dialog>

  <!-- 文件类型管理对话框 -->
  <FileTypesManager v-model:visible="showFileTypesManager" />

  <!-- 列设置对话框 -->
  <el-dialog v-model="showColumnSettings" title="列设置" width="500px">
    <div class="column-settings">
      <h4>显示列</h4>
      <el-space direction="vertical" style="width: 100%">
        <el-checkbox v-model="settingsStore.columnVisibility.name"
          @change="(val) => settingsStore.setColumnVisibility('name', val)" disabled>
          名称（必选）
        </el-checkbox>
        <el-checkbox v-model="settingsStore.columnVisibility.path"
          @change="(val) => settingsStore.setColumnVisibility('path', val)">
          路径
        </el-checkbox>
        <el-checkbox v-model="settingsStore.columnVisibility.size"
          @change="(val) => settingsStore.setColumnVisibility('size', val)">
          大小
        </el-checkbox>
        <el-checkbox v-model="settingsStore.columnVisibility.date_modified"
          @change="(val) => settingsStore.setColumnVisibility('date_modified', val)">
          修改时间
        </el-checkbox>
        <el-checkbox v-model="settingsStore.columnVisibility.type"
          @change="(val) => settingsStore.setColumnVisibility('type', val)">
          类型
        </el-checkbox>
      </el-space>

      <el-divider />

      <h4>自动刷新设置</h4>
      <el-space direction="vertical" style="width: 100%">
        <el-checkbox v-model="settingsStore.searchSettings.autoRefreshOnDateSort"
          @change="(val) => settingsStore.setSearchSetting('autoRefreshOnDateSort', val)">
          按修改时间排序时自动刷新
        </el-checkbox>
        <div v-if="settingsStore.searchSettings.autoRefreshOnDateSort">
          <span>刷新间隔：</span>
          <el-input-number v-model="settingsStore.searchSettings.refreshInterval"
            @change="(val) => settingsStore.setSearchSetting('refreshInterval', val)" :min="500" :max="10000"
            :step="500" size="small" style="width: 120px; margin-left: 10px;" />
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

  <!-- 预览面板 -->
  <PreviewPanel :visible="showPreview" :file-path="previewFilePath" :file-name="previewFileName"
    @close="closePreview" />
</template>

<script setup>
import { ref, computed, watch, onMounted, onUnmounted, nextTick } from "vue";
import { ElMessage, ElMessageBox } from "element-plus";
import { Search, ArrowDown, Setting, Folder, Tools, Brush, Download, Upload, View } from "@element-plus/icons-vue";
import { invoke } from "@tauri-apps/api/core";
import { openPath } from "@tauri-apps/plugin-opener";
import { open } from "@tauri-apps/plugin-dialog";
import { useFileTypesStore } from "@/stores/fileTypes";
import { useFileSearchSettingsStore } from "@/stores/fileSearchSettings";
import { useSettingsStore } from "@/stores/settings";
import FileTypesManager from "@/components/FileTypesManager.vue";
import FileIcon from "@/components/FileIcon.vue";
import PreviewPanel from "@/components/PreviewPanel.vue";

// 状态管理
const searchQuery = ref("");
const searchInputRef = ref(null);
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
const globalSettingsStore = useSettingsStore();

// 文件选中和预览相关
const selectedRowIndex = ref(-1);
const previewEnabled = ref(globalSettingsStore.previewSettings.enabled); // 预览模式开关 - 从store加载
const showPreview = ref(false); // 当前是否显示预览
const previewFilePath = ref('');
const previewFileName = ref('');

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
const showEverythingSettings = ref(false);
const showAppearanceSettings = ref(false);

// Everything设置表单
const everythingForm = ref({
  host: settingsStore.everythingSettings.host,
  port: settingsStore.everythingSettings.port
});

// 界面设置表单
const appearanceForm = ref({
  tableFontFamily: settingsStore.appearanceSettings.tableFontFamily,
  tableFontSize: settingsStore.appearanceSettings.tableFontSize
});

// 字体大小数字值
const fontSizeNumber = ref(parseInt(settingsStore.appearanceSettings.tableFontSize));

// 代码主题选择
const codeThemeSelection = ref(globalSettingsStore.previewSettings.codeTheme);

// 搜索行引用
const searchRowRef = ref(null);

// 宽度调整相关 - 使用持久化设置
const searchInputWidth = ref(settingsStore.layoutSettings.searchInputWidth);
const filterWidth = ref(settingsStore.layoutSettings.filterWidth);
const isResizing = ref(false);
const minSearchWidth = 100;
const minFilterWidth = 150;

// 自动刷新相关
let autoRefreshTimer = null;



// 表格引用
const tableRef = ref(null);
const resultsContainer = ref(null);

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

// 高亮匹配的文本
const highlightMatchedText = (text) => {
  // 安全检查
  if (!text || typeof text !== 'string') {
    return text || '';
  }

  if (!searchQuery.value || !searchQuery.value.trim()) {
    return text;
  }

  let query = searchQuery.value.trim();

  // 移除可能的文件夹路径（用引号包围的部分）
  query = query.replace(/"[^"]*"/g, '').trim();

  // 移除文件类型前缀（如 ext:jpg）
  query = query.replace(/^(ext|file|folder):\S*\s*/i, '').trim();

  // 如果查询为空或者是通配符，不进行高亮
  if (!query || query === '*') {
    return text;
  }

  try {
    // 处理正则表达式模式
    if (useRegex.value) {
      try {
        const flags = matchCase.value ? 'g' : 'gi';
        const regex = new RegExp(query, flags);
        return text.replace(regex, (match) => `<span class="highlight-match">${escapeHtml(match)}</span>`);
      } catch (e) {
        // 如果正则表达式无效，fallback 到普通匹配
        return highlightNormalText(text, query);
      }
    } else {
      return highlightNormalText(text, query);
    }
  } catch (error) {
    // 如果高亮过程中出现任何错误，返回原始文本
    console.warn('文本高亮失败:', error);
    return text;
  }
};

// 普通文本高亮
const highlightNormalText = (text, query) => {
  try {
    // 如果查询包含多个词，分别高亮每个词
    const words = query.split(/\s+/).filter(word => word.length > 0);
    let result = text;

    words.forEach(word => {
      // 转义特殊字符
      const escapedWord = word.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');

      let flags = 'g';
      if (!matchCase.value) {
        flags += 'i';
      }

      // 如果启用全字匹配，添加单词边界
      const pattern = wholeWord.value ? `\\b${escapedWord}\\b` : escapedWord;

      try {
        const regex = new RegExp(pattern, flags);
        result = result.replace(regex, (match) => `<span class="highlight-match">${escapeHtml(match)}</span>`);
      } catch (e) {
        // 如果创建正则表达式失败，继续处理下一个词
        console.warn('无法为词语创建正则表达式:', word, e);
      }
    });

    return result;
  } catch (error) {
    console.warn('普通文本高亮失败:', error);
    return text;
  }
};

// HTML 转义函数，防止 XSS
const escapeHtml = (text) => {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
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
      host: settingsStore.everythingSettings.host,
      port: settingsStore.everythingSettings.port,
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
  selectedRowIndex.value = -1; // 重置选中行

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
      host: settingsStore.everythingSettings.host,
      port: settingsStore.everythingSettings.port,
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
      // 自动选中第一行（如果有结果）
      if (results.value.length > 0) {
        selectedRowIndex.value = 0;
        // 延迟聚焦，确保表格已经渲染
        nextTick(() => {
          if (resultsContainer.value) {
            resultsContainer.value.focus();
          }
        });
      }
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

// 处理行点击 - 选中文件
const handleRowClick = (row, column, event) => {
  // 查找当前行在结果数组中的索引
  const index = results.value.findIndex(item =>
    item.name === row.name && item.path === row.path
  );
  if (index >= 0) {
    selectedRowIndex.value = index;
    // 确保容器获得焦点，以便接收键盘事件
    nextTick(() => {
      if (resultsContainer.value) {
        resultsContainer.value.focus();
      }
    });

    // 如果启用了预览模式，单击时自动预览
    if (previewEnabled.value && row.type === 'file') {
      previewFile(row);
    }
  }
};

// 获取行的 CSS 类名 - 用于高亮选中行
const getRowClassName = ({ rowIndex }) => {
  return rowIndex === selectedRowIndex.value ? 'selected-row' : '';
};

// 处理键盘事件
const handleKeyDown = (event) => {
  if (results.value.length === 0) return;

  switch (event.code) {
    case 'ArrowUp':
      event.preventDefault();
      if (selectedRowIndex.value > 0) {
        selectedRowIndex.value--;
      } else {
        selectedRowIndex.value = results.value.length - 1;
      }
      scrollToSelectedRow();
      break;
    case 'ArrowDown':
      event.preventDefault();
      if (selectedRowIndex.value < results.value.length - 1) {
        selectedRowIndex.value++;
      } else {
        selectedRowIndex.value = 0;
      }
      scrollToSelectedRow();
      break;
    case 'Space':
      event.preventDefault();
      if (selectedRowIndex.value >= 0 && selectedRowIndex.value < results.value.length) {
        const selectedRow = results.value[selectedRowIndex.value];
        // 只预览文件，不预览文件夹
        if (selectedRow.type === 'file') {
          if (previewEnabled.value) {
            // 预览模式下：切换预览面板显示/隐藏
            if (showPreview.value &&
              previewFilePath.value === getFullFilePath(selectedRow.path, selectedRow.name)) {
              showPreview.value = false; // 如果当前文件已经在预览，则关闭预览
            } else {
              previewFile(selectedRow); // 否则预览选中的文件
            }
          } else {
            // 非预览模式下：打开对话框
            previewFile(selectedRow);
          }
        }
      }
      break;
    case 'Enter':
      event.preventDefault();
      if (selectedRowIndex.value >= 0 && selectedRowIndex.value < results.value.length) {
        const selectedRow = results.value[selectedRowIndex.value];
        openFileDefault(getFullFilePath(selectedRow.path, selectedRow.name), selectedRow.type);
      }
      break;
  }
};

// 滚动到选中行
const scrollToSelectedRow = () => {
  nextTick(() => {
    if (tableRef.value && selectedRowIndex.value >= 0) {
      // 使用 Element Plus 表格的方法滚动到指定行
      const table = tableRef.value;
      const rows = table.$el.querySelectorAll('.el-table__body tbody tr');
      const selectedRow = rows[selectedRowIndex.value];
      if (selectedRow) {
        selectedRow.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
      }
    }
  });
};

// 聚焦结果容器
const focusResultsContainer = () => {
  if (resultsContainer.value) {
    resultsContainer.value.focus();
  }
};

// 预览文件
const previewFile = (row) => {
  previewFilePath.value = getFullFilePath(row.path, row.name);
  previewFileName.value = row.name;
  showPreview.value = true;
};

// 处理预览切换
const handlePreviewToggle = (enabled) => {
  previewEnabled.value = enabled;
  globalSettingsStore.setPreviewEnabled(enabled); // 保存到store
  if (!enabled) {
    showPreview.value = false; // 关闭预览模式时隐藏预览面板
  }
};

// 关闭预览
const closePreview = () => {
  showPreview.value = false;
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

  // 搜索条件变化时立即搜索（这会触发重新高亮）
  currentPage.value = 1;
  handleSearch().then(() => {
    applyColumnWidths();
  });
});

// 监听高级选项显示状态
watch(showAdvancedOptions, (newValue) => {
  settingsStore.setSearchSetting('showAdvancedOptions', newValue);
});

// 监听外观设置变化，实时预览
watch([appearanceForm, fontSizeNumber], () => {
  applyTableStyles();
}, { deep: true });

// 应用表格样式
const applyTableStyles = () => {
  const styleId = 'dynamic-table-styles';
  let existingStyle = document.getElementById(styleId);

  if (existingStyle) {
    existingStyle.remove();
  }

  const style = document.createElement('style');
  style.id = styleId;

  // 获取当前设置值
  const currentFontFamily = showAppearanceSettings.value ?
    appearanceForm.value.tableFontFamily :
    settingsStore.appearanceSettings.tableFontFamily;

  const currentFontSize = showAppearanceSettings.value ?
    fontSizeNumber.value + 'px' :
    settingsStore.appearanceSettings.tableFontSize;

  style.textContent = `
    /* 表格整体样式 */
    .search-results .el-table,
    .search-results .el-table table,
    .search-results .el-table tbody,
    .search-results .el-table thead {
      font-family: ${currentFontFamily} !important;
      font-size: ${currentFontSize} !important;
    }
    
    /* 表格单元格样式 */
    .search-results .el-table .el-table__cell,
    .search-results .el-table .el-table__cell .cell,
    .search-results .el-table th.el-table__cell,
    .search-results .el-table td.el-table__cell {
      font-family: ${currentFontFamily} !important;
      font-size: ${currentFontSize} !important;
    }
    
    /* 表头样式 */
    .search-results .el-table .el-table__header-wrapper .el-table__header .el-table__cell,
    .search-results .el-table .el-table__header-wrapper .el-table__header .el-table__cell .cell,
    .search-results .el-table .el-table__header .el-table__cell,
    .search-results .el-table .el-table__header .cell {
      font-family: ${currentFontFamily} !important;
      font-size: ${currentFontSize} !important;
    }
    
    /* 表体样式 */
    .search-results .el-table .el-table__body-wrapper .el-table__body .el-table__cell,
    .search-results .el-table .el-table__body-wrapper .el-table__body .el-table__cell .cell,
    .search-results .el-table .el-table__body .el-table__cell,
    .search-results .el-table .el-table__body .cell {
      font-family: ${currentFontFamily} !important;
      font-size: ${currentFontSize} !important;
    }
    
    /* 单元格内容样式 */
    .search-results .el-table .cell,
    .search-results .el-table .cell span,
    .search-results .el-table .cell div,
    .search-results .el-table td .cell,
    .search-results .el-table th .cell {
      font-family: ${currentFontFamily} !important;
      font-size: ${currentFontSize} !important;
    }
    
    /* 排序指示器样式 */
    .search-results .el-table .sortable-header,
    .search-results .el-table .sortable-header span {
      font-family: ${currentFontFamily} !important;
      font-size: ${currentFontSize} !important;
    }
    
    /* 标签样式 */
    .search-results .el-table .el-tag {
      font-family: ${currentFontFamily} !important;
      font-size: ${currentFontSize} !important;
    }
  `;

  document.head.appendChild(style);
};

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

// 保存Everything设置
const saveEverythingSettings = () => {
  settingsStore.setEverythingSettings({
    host: everythingForm.value.host,
    port: everythingForm.value.port
  });
  showEverythingSettings.value = false;
  ElMessage.success('Everything设置保存成功');
};

// 重置Everything设置
const resetEverythingSettings = () => {
  everythingForm.value = {
    host: 'localhost',
    port: 8080
  };
};

// 保存界面设置
const saveAppearanceSettings = () => {
  settingsStore.setAppearanceSettings({
    tableFontFamily: appearanceForm.value.tableFontFamily,
    tableFontSize: fontSizeNumber.value + 'px'
  });

  // 保存代码主题设置
  globalSettingsStore.setPreviewCodeTheme(codeThemeSelection.value);

  showAppearanceSettings.value = false;
  ElMessage.success('界面设置保存成功');
};

// 重置界面设置
const resetAppearanceSettings = () => {
  appearanceForm.value = {
    tableFontFamily: 'inherit',
    tableFontSize: '14px'
  };
  fontSizeNumber.value = 14;
  codeThemeSelection.value = 'auto'; // 重置代码主题
};

// 选择文件夹
const selectFolder = async () => {
  try {
    const result = await open({
      directory: true,
      multiple: false,
      title: '选择文件夹'
    });

    if (result) {
      // 在搜索框中添加选择的文件夹路径和一个空格，路径用双引号包围
      const folderPath = `"${result}"`;
      if (searchQuery.value.trim()) {
        searchQuery.value = folderPath + ' ' + searchQuery.value;
      } else {
        searchQuery.value = folderPath + ' ';
      }
      ElMessage.success('已添加文件夹路径到搜索条件');
    }
  } catch (error) {
    ElMessage.error('选择文件夹失败');
    console.error('选择文件夹失败:', error);
  }
};

// 处理设置下拉菜单命令
const handleSettingsCommand = (command) => {
  switch (command) {
    case 'exportSettings':
      exportSettings();
      break;
    case 'importSettings':
      importSettings();
      break;
    case 'appearance':
      showAppearanceSettings.value = true;
      break;
    case 'everything':
      showEverythingSettings.value = true;
      break;
    case 'columns':
      showColumnSettings.value = true;
      break;
    default:
      break;
  }
};

// 导出设置到 JSON 文件
const exportSettings = async () => {
  try {
    // 收集所有 store 的状态
    const allSettings = {
      // 全局设置
      globalSettings: {
        exportPath: globalSettingsStore.exportPath,
        autoOpenFolder: globalSettingsStore.autoOpenFolder,
        theme: globalSettingsStore.theme,
        collapsedKeys: globalSettingsStore.collapsedKeys,
        previewSettings: globalSettingsStore.previewSettings
      },

      // 文件搜索设置
      fileSearchSettings: {
        columnVisibility: settingsStore.columnVisibility,
        columnWidths: settingsStore.columnWidths,
        columnOrder: settingsStore.columnOrder,
        searchSettings: settingsStore.searchSettings,
        paginationSettings: settingsStore.paginationSettings,
        layoutSettings: settingsStore.layoutSettings,
        everythingSettings: settingsStore.everythingSettings,
        appearanceSettings: settingsStore.appearanceSettings
      },

      // 文件类型设置
      fileTypesSettings: {
        fileTypes: fileTypesStore.fileTypes,
        specialFilters: fileTypesStore.specialFilters
      },

      // 导出时间戳
      exportTime: new Date().toISOString(),
      version: '1.0.0'
    };

    // 获取桌面路径并生成文件名
    const desktopPath = await invoke('get_desktop_path');
    const timestamp = new Date().toISOString().replace(/[:.]/g, '-').slice(0, 19);
    const fileName = `my-tools-settings-${timestamp}.json`;
    const filePath = `${desktopPath}\\${fileName}`;

    // 写入文件
    await invoke('write_file', {
      path: filePath,
      content: JSON.stringify(allSettings, null, 2)
    });

    ElMessage.success(`设置已导出到: ${fileName}`);

    // 自动打开文件所在目录
    try {
      await openPath(desktopPath);
    } catch (error) {
      console.warn('自动打开目录失败:', error);
    }

  } catch (error) {
    console.error('导出设置失败:', error);
    ElMessage.error(`导出设置失败: ${error.message || '未知错误'}`);
  }
};

// 从 JSON 文件导入设置
const importSettings = async () => {
  try {
    const result = await open({
      multiple: false,
      filters: [
        {
          name: 'JSON 文件',
          extensions: ['json']
        }
      ],
      title: '选择设置文件'
    });

    if (!result) {
      return; // 用户取消了选择
    }

    // 读取文件内容
    const content = await invoke('read_file', { path: result });

    try {
      const importedSettings = JSON.parse(content);

      // 验证文件格式
      if (!importedSettings.version || !importedSettings.exportTime) {
        throw new Error('不是有效的设置文件格式');
      }

      // 确认导入
      await ElMessageBox.confirm(
        `确定要导入设置文件吗？这将覆盖当前所有设置。\n\n导出时间: ${new Date(importedSettings.exportTime).toLocaleString()}\n版本: ${importedSettings.version}`,
        '确认导入设置',
        {
          confirmButtonText: '确定导入',
          cancelButtonText: '取消',
          type: 'warning'
        }
      );

      // 导入全局设置
      if (importedSettings.globalSettings) {
        const gs = importedSettings.globalSettings;
        if (gs.exportPath !== undefined) globalSettingsStore.setExportPath(gs.exportPath);
        if (gs.autoOpenFolder !== undefined) globalSettingsStore.setAutoOpenFolder(gs.autoOpenFolder);
        if (gs.theme !== undefined) globalSettingsStore.setTheme(gs.theme);
        if (gs.collapsedKeys !== undefined) globalSettingsStore.setCollapsedKeys(gs.collapsedKeys);

        // 导入预览设置
        if (gs.previewSettings !== undefined) {
          // 如果是旧版本的预览设置格式，需要转换
          if (gs.previewSettings.panelWidth !== undefined && !gs.previewSettings.image) {
            // 旧版本格式，转换为新格式
            const oldSettings = gs.previewSettings;
            globalSettingsStore.previewSettings = {
              enabled: oldSettings.enabled || false,
              codeTheme: oldSettings.codeTheme || 'auto',
              fontSize: oldSettings.fontSize || '13px',
              fontFamily: oldSettings.fontFamily || 'Consolas, Monaco, "Courier New", monospace',
              wordWrap: oldSettings.wordWrap || false,
              image: {
                panelWidth: oldSettings.panelWidth || 400,
                panelHeight: oldSettings.panelHeight || 300,
                widthRatio: oldSettings.widthRatio || 1 / 3,
                heightRatio: oldSettings.imageModeHeight || 0.5
              },
              text: {
                panelWidth: oldSettings.panelWidth || 500,
                panelHeight: oldSettings.panelHeight || 400,
                widthRatio: oldSettings.widthRatio || 1 / 3,
                heightRatio: oldSettings.documentModeHeight || 0.8
              },
              pdf: {
                panelWidth: oldSettings.panelWidth || 600,
                panelHeight: oldSettings.panelHeight || 800,
                widthRatio: oldSettings.widthRatio || 0.4,
                heightRatio: oldSettings.documentModeHeight || 0.9
              }
            };
          } else {
            // 新版本格式，直接赋值
            globalSettingsStore.previewSettings = gs.previewSettings;
          }
        }
      }

      // 导入文件搜索设置
      if (importedSettings.fileSearchSettings) {
        const fss = importedSettings.fileSearchSettings;

        // 导入列设置
        if (fss.columnVisibility) {
          Object.keys(fss.columnVisibility).forEach(key => {
            settingsStore.setColumnVisibility(key, fss.columnVisibility[key]);
          });
        }

        if (fss.columnWidths) {
          Object.keys(fss.columnWidths).forEach(key => {
            settingsStore.setColumnWidth(key, fss.columnWidths[key]);
          });
        }

        if (fss.columnOrder) {
          settingsStore.setColumnOrder(fss.columnOrder);
        }

        // 导入搜索设置
        if (fss.searchSettings) {
          settingsStore.setSearchSettings(fss.searchSettings);
          // 更新本地响应式变量
          matchCase.value = fss.searchSettings.matchCase || false;
          matchPath.value = fss.searchSettings.matchPath || false;
          wholeWord.value = fss.searchSettings.wholeWord || false;
          useRegex.value = fss.searchSettings.useRegex || false;
          showAdvancedOptions.value = fss.searchSettings.showAdvancedOptions || false;
        }

        // 导入布局设置
        if (fss.layoutSettings) {
          settingsStore.setLayoutSizes(
            fss.layoutSettings.searchInputWidth || 400,
            fss.layoutSettings.filterWidth || 600
          );
          // 更新本地变量
          searchInputWidth.value = fss.layoutSettings.searchInputWidth || 400;
          filterWidth.value = fss.layoutSettings.filterWidth || 600;
        }

        // 导入服务设置
        if (fss.everythingSettings) {
          settingsStore.setEverythingSettings(fss.everythingSettings);
          // 更新表单数据
          everythingForm.value = { ...fss.everythingSettings };
        }

        // 导入外观设置
        if (fss.appearanceSettings) {
          settingsStore.setAppearanceSettings(fss.appearanceSettings);
          // 更新表单数据
          appearanceForm.value = { ...fss.appearanceSettings };
          fontSizeNumber.value = parseInt(fss.appearanceSettings.tableFontSize) || 14;
          // 应用外观设置
          applyTableStyles();
        }
      }

      // 导入文件类型设置
      if (importedSettings.fileTypesSettings) {
        const fts = importedSettings.fileTypesSettings;

        if (fts.fileTypes) {
          fileTypesStore.setFileTypes(fts.fileTypes);
        }

        if (fts.specialFilters) {
          fileTypesStore.setSpecialFilters(fts.specialFilters);
        }
      }

      ElMessage.success('设置导入成功！');

      // 重新应用所有设置
      await nextTick();
      applyColumnWidths();

    } catch (parseError) {
      throw new Error(`解析设置文件失败: ${parseError.message}`);
    }

  } catch (error) {
    if (error.message === 'cancelled') {
      return; // 用户取消了导入
    }
    console.error('导入设置失败:', error);
    ElMessage.error(`导入设置失败: ${error.message || '未知错误'}`);
  }
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

  // 应用保存的表格样式
  applyTableStyles();

  // 页面加载时自动执行一次空关键字搜索初始化结果列表
  handleSearch();

  // 自动聚焦到搜索框
  setTimeout(() => {
    searchInputRef.value?.focus();
  }, 100);
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

<style lang="scss" src="../assets/styles/flie-search.scss" scoped />