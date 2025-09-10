<template>
  <div class="file-icon" :style="{ width: size + 'px', height: size + 'px' }">
    <img 
      v-if="iconData" 
      :src="iconData" 
      :alt="fileName"
      :style="{ width: size + 'px', height: size + 'px' }"
      @error="handleIconError"
    />
    <div 
      v-else 
      class="default-icon"
      :style="{ width: size + 'px', height: size + 'px', fontSize: (size * 0.6) + 'px' }"
    >
      <el-icon>
        <component :is="getDefaultIcon()" />
      </el-icon>
    </div>
  </div>
</template>

<script setup>
import { ref, watch, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { Document, Folder, Picture, VideoPlay, Microphone, Collection } from '@element-plus/icons-vue';

const props = defineProps({
  filePath: {
    type: String,
    required: true
  },
  fileName: {
    type: String,
    required: true
  },
  fileType: {
    type: String,
    default: 'file'
  },
  size: {
    type: Number,
    default: 16
  }
});

const iconData = ref('');
const isLoading = ref(false);

// 图标缓存
const iconCache = new Map();

// 获取默认图标
const getDefaultIcon = () => {
  if (props.fileType === 'folder') {
    return Folder;
  }
  
  const extension = props.fileName.split('.').pop()?.toLowerCase();
  
  switch (extension) {
    case 'jpg':
    case 'jpeg':
    case 'png':
    case 'gif':
    case 'bmp':
    case 'svg':
    case 'webp':
      return Picture;
    case 'mp4':
    case 'avi':
    case 'mkv':
    case 'mov':
    case 'wmv':
    case 'flv':
      return VideoPlay;
    case 'mp3':
    case 'wav':
    case 'flac':
    case 'aac':
    case 'ogg':
      return Microphone;
    case 'zip':
    case 'rar':
    case '7z':
    case 'tar':
    case 'gz':
      return Collection;
    default:
      return Document;
  }
};

// 加载文件图标
const loadIcon = async () => {
  if (isLoading.value) return;
  
  const cacheKey = props.fileType === 'folder' ? 'folder' : props.fileName.split('.').pop()?.toLowerCase() || 'unknown';
  
  // 检查缓存
  if (iconCache.has(cacheKey)) {
    iconData.value = iconCache.get(cacheKey);
    return;
  }
  
  isLoading.value = true;
  
  try {
    const icon = await invoke('get_file_icon', { filePath: props.filePath });
    iconData.value = icon;
    
    // 缓存图标
    iconCache.set(cacheKey, icon);
  } catch (error) {
    console.log('无法获取文件图标:', error);
    // 使用默认图标
    iconData.value = '';
  } finally {
    isLoading.value = false;
  }
};

// 处理图标加载错误
const handleIconError = () => {
  iconData.value = '';
};

// 监听文件路径变化
watch([() => props.filePath, () => props.fileName], () => {
  loadIcon();
}, { immediate: true });

onMounted(() => {
  loadIcon();
});
</script>

<style scoped>
.file-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.file-icon img {
  object-fit: contain;
  border-radius: 2px;
}

.default-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--el-text-color-regular);
}

.default-icon .el-icon {
  width: 100%;
  height: 100%;
}
</style>