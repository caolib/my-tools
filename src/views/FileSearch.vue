<template>
  <div class="file-search-container">
    <div class="search-header">
      <div class="search-form">
        <!-- 搜索框和文件类型筛选 - 同一行 -->
        <div class="search-row">
          <el-input v-model="searchQuery" placeholder="搜索关键字" @keyup.enter="handleSearch" class="search-input"
            size="large" clearable>
          </el-input>
          
          <!-- 文件类型筛选 - 移到搜索框右边 -->
          <div class="file-type-filters" @click="showFileTypesManager = true">
            <el-radio-group v-model="selectedFileType" @click.stop>
              <el-radio label="">所有文件</el-radio>
              <el-radio
                v-for="(typeConfig, key) in fileTypesStore.fileTypes"
                :key="key"
                :label="key"
              >
                {{ typeConfig.name }}
              </el-radio>
              <el-radio label="file">仅文件</el-radio>
              <el-radio label="folder">仅文件夹</el-radio>
            </el-radio-group>
          </div>
        </div>
        
        <!-- 高级选项折叠按钮 -->
        <div class="advanced-options-toggle">
          <el-button 
            type="primary" 
            link 
            @click="showAdvancedOptions = !showAdvancedOptions"
            class="toggle-btn"
          >
            <span>搜索选项</span>
            <el-icon :class="{ 'rotate-180': showAdvancedOptions }">
              <ArrowDown />
            </el-icon>
          </el-button>
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

    <div class="search-results">
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
          <el-text type="info" size="small">
            提示：双击文件名打开文件，双击路径打开所在文件夹，点击列头进行排序
          </el-text>
        </div>

        <el-table :data="results" style="width: 100%" :max-height="600" stripe>
          <el-table-column prop="name" min-width="200">
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
              <span 
                @dblclick="openFileDefault(getFullFilePath(row.path, row.name), row.type)"
                style="cursor: pointer;"
                :title="'双击' + (row.type === 'folder' ? '打开文件夹' : '打开文件')"
              >
                {{ row.name }}
              </span>
            </template>
          </el-table-column>
          <el-table-column prop="type" label="类型" width="80" align="center">
            <template #default="{ row }">
              <el-tag :type="row.type === 'folder' ? 'success' : 'info'" size="small">
                {{ row.type === 'folder' ? '文件夹' : '文件' }}
              </el-tag>
            </template>
          </el-table-column>
          <el-table-column prop="path" min-width="300" show-overflow-tooltip>
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
              <span 
                @dblclick="openFileDefault(row.path, 'folder')"
                style="cursor: pointer;"
                title="双击打开所在文件夹"
              >
                {{ row.path }}
              </span>
            </template>
          </el-table-column>
          <el-table-column prop="size" width="100" align="right">
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
              {{ formatFileSize(row.size) }}
            </template>
          </el-table-column>
          <el-table-column prop="date_modified" width="180">
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
              {{ formatDate(row.date_modified) }}
            </template>
          </el-table-column>
        </el-table>

        <div class="pagination-container">
          <el-pagination v-model:current-page="currentPage" v-model:page-size="pageSize" :total="totalResults"
            :page-sizes="[20, 50, 100]" layout="total, sizes, prev, pager, next" @size-change="handleSizeChange"
            @current-change="handleCurrentChange" />
        </div>
      </div>
    </div>
  </div>
  
  <!-- 文件类型管理对话框 -->
  <FileTypesManager v-model:visible="showFileTypesManager" />
</template>

<script setup>
import { ref, computed, watch, onMounted } from "vue";
import { ElMessage } from "element-plus";
import { Search, ArrowDown } from "@element-plus/icons-vue";
import { invoke } from "@tauri-apps/api/core";
import { openPath } from "@tauri-apps/plugin-opener";
import { useFileTypesStore } from "@/stores/fileTypes";
import FileTypesManager from "@/components/FileTypesManager.vue";

// 状态管理
const searchQuery = ref("");
const loading = ref(false);
const error = ref(false);
const results = ref([]);
const totalResults = ref(0);
const hasSearched = ref(false);

// 文件类型筛选
const selectedFileType = ref(''); // 使用单个值而不是数组
const fileTypesStore = useFileTypesStore();
const showFileTypesManager = ref(false);

// 分页和排序
const currentPage = ref(1);
const pageSize = ref(20);
const sortBy = ref(""); // 空字符串表示无排序
const sortOrder = ref(1); // 1: 升序, 0: 降序
const matchCase = ref(false);
const matchPath = ref(false);
const wholeWord = ref(false);
const useRegex = ref(false);
const showAdvancedOptions = ref(false);

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

// 搜索文件
const handleSearch = async () => {
  // 允许空关键字搜索
  let searchTerm = searchQuery.value.trim() || '*'; // 空关键字时使用 * 表示全部
  
  // 添加文件类型筛选条件
  const fileTypeFilter = buildFileTypeFilter();
  if (fileTypeFilter) {
    searchTerm = fileTypeFilter + ' ' + searchTerm;
  }

  loading.value = true;
  error.value = false;
  hasSearched.value = true;

  try {
    const searchParams = {
      search: searchTerm,
      offset: offset.value,
      count: pageSize.value,
      case: matchCase.value,
      wholeword: wholeWord.value,
      path: matchPath.value,
      regex: useRegex.value
    };
    
    // 只有在有排序字段时才添加排序参数
    if (sortBy.value) {
      searchParams.sort = sortBy.value;
      searchParams.ascending = sortOrder.value === 1;
      console.log('添加排序参数:', sortBy.value, sortOrder.value === 1 ? '升序' : '降序');
    } else {
      console.log('无排序参数');
    }

    console.log('搜索参数:', searchParams);

    const result = await invoke('search_everything', searchParams);

    results.value = result.results || [];
    totalResults.value = result.totalResults || result.total_results || 0;
    
    console.log('搜索结果统计:', {
      totalResults: result.totalResults,
      total_results: result.total_results,
      最终总数: totalResults.value,
      结果数量: results.value.length
    });
    
    // 调试信息：打印第一个结果的结构
    if (results.value.length > 0) {
      console.log('第一个搜索结果的结构:', results.value[0]);
      console.log('第一个结果的所有字段:', Object.keys(results.value[0]));
    }

    if (results.value.length === 0) {
      ElMessage.info("未找到相关文件");
    }
  } catch (err) {
    console.error("搜索失败:", err);
    error.value = true;
    ElMessage.error("搜索失败，请检查Everything服务是否运行");
  } finally {
    loading.value = false;
  }
};

// 处理列头排序点击
const handleSort = (column) => {
  console.log('点击排序列:', column, '当前排序:', sortBy.value, '当前顺序:', sortOrder.value);
  
  if (sortBy.value === column) {
    // 同一列：升序 → 降序 → 无排序
    if (sortOrder.value === 1) {
      sortOrder.value = 0; // 切换到降序
      console.log('切换到降序');
    } else {
      // 取消排序
      sortBy.value = "";
      sortOrder.value = 1;
      console.log('取消排序');
    }
  } else {
    // 不同列：设置为当前列的升序
    sortBy.value = column;
    sortOrder.value = 1;
    console.log('新排序列:', column, '升序');
  }
  
  console.log('排序后状态:', sortBy.value, sortOrder.value);
};

// 双击文件名或路径：使用默认方式打开文件或文件夹
const openFileDefault = async (filePath, fileType) => {
  try {
    console.log('打开文件:', filePath, '类型:', fileType);
    
    // 统一使用 openPath 打开文件和文件夹
    try {
      console.log('尝试使用 openPath 打开:', filePath);
      await openPath(filePath);
      console.log('openPath 成功');
    } catch (openerError) {
      console.warn('openPath 失败，尝试系统默认方式:', openerError);
      // 如果 Opener 插件失败，使用系统 shell 命令打开
      await invoke("shell_open", { path: filePath });
    }
    
    ElMessage.success(`已打开${fileType === 'folder' ? '文件夹' : '文件'}`);
  } catch (err) {
    console.error("打开文件失败:", err);
    ElMessage.error(`无法打开${fileType === 'folder' ? '文件夹' : '文件'}: ${err.message || err}`);
  }
};

// 分页变化处理
const handleSizeChange = (newSize) => {
  pageSize.value = newSize;
  currentPage.value = 1;
  if (hasSearched.value) {
    handleSearch();
  }
};

const handleCurrentChange = (newPage) => {
  currentPage.value = newPage;
  if (hasSearched.value) {
    handleSearch();
  }
};

// 监听排序和筛选选项变化
watch([sortBy, sortOrder, matchCase, matchPath, wholeWord, useRegex], () => {
  if (hasSearched.value) {
    currentPage.value = 1;
    handleSearch();
  }
});

// 监听文件类型筛选变化
watch(selectedFileType, () => {
  if (hasSearched.value) {
    currentPage.value = 1;
    handleSearch();
  }
});

// 监听搜索关键字变化，实时搜索（防抖）
let searchTimeout = null;
watch(searchQuery, (newValue) => {
  // 清除上一次的定时器
  if (searchTimeout) {
    clearTimeout(searchTimeout);
  }
  
  // 设置 300ms 防抖延迟，支持空关键字搜索
  searchTimeout = setTimeout(() => {
    currentPage.value = 1;
    handleSearch();
  }, 300);
});

// 页面加载时自动搜索
onMounted(() => {
  console.log('页面加载完成，自动搜索');
  handleSearch(); // 空关键字搜索，显示所有文件
});
</script>

<style scoped>
.file-search-container {
  padding: 20px;
  padding-top: 60px;
  margin: 0 auto;
}

.search-row {
  display: flex;
  gap: 20px;
  align-items: flex-start;
  flex-wrap: wrap;
}

.search-input {
  flex: 1;
  min-width: 300px;
  max-width: 500px;
}


.file-type-filters {
    background: var(--el-fill-color-lighter);
    padding: 4px 16px;
    border-radius: 6px;
    border: 1px solid var(--el-border-color-light);
    min-width: 300px;
    flex-shrink: 0;
    cursor: pointer;
    transition: all 0.3s ease;
}

.file-type-filters:hover {
  background: var(--el-fill-color-light);
  border-color: var(--el-color-primary-light-5);
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.advanced-options-toggle {
  margin-top: 5px;
}

.toggle-btn {
  display: flex;
  align-items: center;
  gap: 5px;
  font-size: 14px;
  padding: 8px 0;
}

.toggle-btn .el-icon {
  transition: transform 0.3s ease;
}

.toggle-btn .el-icon.rotate-180 {
  transform: rotate(180deg);
}


.filter-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--el-text-color-primary);
}

.file-type-filters :deep(.el-radio-group) {
  display: flex;
  flex-wrap: wrap;
  gap: 16px;
}

.file-type-filters :deep(.el-radio) {
  margin-right: 0;
  white-space: nowrap;
}

.file-type-filters :deep(.el-radio__label) {
  font-size: 13px;
  color: var(--el-text-color-regular);
}

.file-type-filters :deep(.el-radio__input.is-checked .el-radio__inner) {
  background-color: var(--el-color-primary);
  border-color: var(--el-color-primary);
}

.search-options {
    background: var(--el-fill-color-light);
    padding: 0px 10px;
    border-radius: 8px;
    margin-top: 10px;
}

.search-results {
  min-height: 400px;
}

.loading-container {
  padding: 40px;
}

.error-container,
.empty-container {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 400px;
}

.results-container {
  background: var(--el-bg-color);
  border-radius: 8px;
  padding: 20px;
  box-shadow: var(--el-box-shadow-light);
}

.results-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  padding-bottom: 10px;
  border-bottom: 1px solid var(--el-border-color-light);
}

.pagination-container {
  margin-top: 20px;
  display: flex;
  justify-content: center;
}

/* 排序指示器样式 */
.sortable-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  cursor: pointer;
  user-select: none;
  transition: color 0.2s;
}

.sortable-header:hover {
  color: var(--el-color-primary);
}

.sortable-header.active {
  color: var(--el-color-primary);
  font-weight: 600;
}

.sort-indicator {
  display: flex;
  flex-direction: column;
  margin-left: 4px;
  line-height: 1;
}

.sort-indicator i {
  font-size: 10px;
  color: var(--el-text-color-placeholder);
  transition: color 0.2s;
}

.sort-indicator i.active {
  color: var(--el-color-primary);
}

.el-icon-caret-top,
.el-icon-caret-bottom {
  width: 0;
  height: 0;
  border-style: solid;
}

.el-icon-caret-top {
  border-left: 4px solid transparent;
  border-right: 4px solid transparent;
  border-bottom: 6px solid currentColor;
  margin-bottom: 1px;
}

.el-icon-caret-bottom {
  border-left: 4px solid transparent;
  border-right: 4px solid transparent;
  border-top: 6px solid currentColor;
}

:deep(.el-table) {
  --el-table-header-bg-color: var(--el-fill-color-light);
}

:deep(.el-table__row:hover) {
  background-color: var(--el-fill-color-lighter);
}

@media (max-width: 768px) {
  .file-search-container {
    padding: 10px;
  }

  .search-row {
    flex-direction: column;
    gap: 15px;
  }

  .search-input {
    max-width: 100%;
  }

  .file-type-filters {
    min-width: auto;
    width: 100%;
  }

  .search-options {
    padding: 10px;
  }

  .results-container {
    padding: 10px;
  }
}
</style>