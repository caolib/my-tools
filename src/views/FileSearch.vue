<template>
  <div class="file-search-container">
    <div class="search-header">
      <h2>文件搜索</h2>
      <div class="search-form">
        <el-input v-model="searchQuery" placeholder="输入搜索关键字（空白显示所有文件）" @keyup.enter="handleSearch" class="search-input"
          size="large" clearable>
          <template #append>
            <el-button type="primary" @click="handleSearch" :loading="loading">
              <el-icon>
                <Search />
              </el-icon>
            </el-button>
          </template>
        </el-input>
      </div>

      <div class="search-options">
        <el-row :gutter="20">
          <el-col :span="6">
            <el-form-item label="排序方式">
              <el-select v-model="sortBy" size="small">
                <el-option label="名称" value="name" />
                <el-option label="路径" value="path" />
                <el-option label="修改日期" value="date_modified" />
                <el-option label="大小" value="size" />
              </el-select>
            </el-form-item>
          </el-col>
          <el-col :span="6">
            <el-form-item label="排序顺序">
              <el-select v-model="sortOrder" size="small">
                <el-option label="升序" :value="1" />
                <el-option label="降序" :value="0" />
              </el-select>
            </el-form-item>
          </el-col>
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
        </el-row>
        <el-row :gutter="20">
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
            提示：双击文件名打开文件，双击路径打开所在文件夹
          </el-text>
        </div>

        <el-table :data="results" style="width: 100%" :max-height="600" stripe>
          <el-table-column prop="name" label="文件名" min-width="200">
            <template #default="{ row }">
              <span 
                @dblclick="openFileDefault(getFullFilePath(row.path, row.name), row.file_type || row.type)"
                style="cursor: pointer; color: var(--el-color-primary);"
                :title="'双击' + ((row.file_type || row.type) === 'folder' ? '打开文件夹' : '打开文件')"
              >
                {{ row.name }}
              </span>
            </template>
          </el-table-column>
          <el-table-column prop="file_type" label="类型" width="80" align="center">
            <template #default="{ row }">
              <el-tag :type="(row.file_type || row.type) === 'folder' ? 'success' : 'info'" size="small">
                {{ (row.file_type || row.type) === 'folder' ? '文件夹' : '文件' }}
              </el-tag>
            </template>
          </el-table-column>
          <el-table-column prop="path" label="路径" min-width="300" show-overflow-tooltip>
            <template #default="{ row }">
              <span 
                @dblclick="openFileDefault(row.path, 'folder')"
                style="cursor: pointer; color: var(--el-color-success);"
                title="双击打开所在文件夹"
              >
                {{ row.path }}
              </span>
            </template>
          </el-table-column>
          <el-table-column prop="size" label="大小" width="100" align="right">
            <template #default="{ row }">
              {{ formatFileSize(row.size) }}
            </template>
          </el-table-column>
          <el-table-column prop="date_modified" label="修改时间" width="180">
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
</template>

<script setup>
import { ref, computed, watch, onMounted } from "vue";
import { ElMessage } from "element-plus";
import { Search } from "@element-plus/icons-vue";
import { invoke } from "@tauri-apps/api/core";
import { openPath } from "@tauri-apps/plugin-opener";

// 状态管理
const searchQuery = ref("");
const loading = ref(false);
const error = ref(false);
const results = ref([]);
const totalResults = ref(0);
const hasSearched = ref(false);

// 分页和排序
const currentPage = ref(1);
const pageSize = ref(20);
const sortBy = ref("name");
const sortOrder = ref(0); // 0: 降序, 1: 升序
const matchCase = ref(false);
const matchPath = ref(false);
const wholeWord = ref(false);
const useRegex = ref(false);

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
  if (!sizeStr) return "0 B";
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

// 搜索文件
const handleSearch = async () => {
  // 允许空关键字搜索
  const searchTerm = searchQuery.value.trim() || '*'; // 空关键字时使用 * 表示全部

  loading.value = true;
  error.value = false;
  hasSearched.value = true;

  try {
    const result = await invoke('search_everything', {
      search: searchTerm,
      offset: offset.value,
      count: pageSize.value,
      sort: sortBy.value,
      ascending: sortOrder.value === 1,
      case: matchCase.value,
      wholeword: wholeWord.value,
      path: matchPath.value,
      regex: useRegex.value
    });

    results.value = result.results || [];
    totalResults.value = result.total_results || 0;
    
    // 调试信息：打印第一个结果的结构
    if (results.value.length > 0) {
      console.log('第一个搜索结果的结构:', results.value[0]);
      console.log('第一个结果的 file_type 字段:', results.value[0].file_type);
      console.log('第一个结果的 type 字段:', results.value[0].type);
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
  if (hasSearched.value && searchQuery.value.trim()) {
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
  max-width: 1200px;
  margin: 0 auto;
}

.search-header {
  margin-bottom: 30px;
}

.search-header h2 {
  margin-bottom: 20px;
  color: var(--el-text-color-primary);
}

.search-form {
  margin-bottom: 20px;
}

.search-input {
  max-width: 600px;
}

.search-options {
  background: var(--el-fill-color-light);
  padding: 15px;
  border-radius: 8px;
  margin-top: 15px;
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

  .search-options {
    padding: 10px;
  }

  .results-container {
    padding: 10px;
  }
}
</style>