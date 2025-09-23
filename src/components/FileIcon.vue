<template>
  <div class="file-icon" :style="{ width: size + 'px', height: size + 'px' }">
    <img v-if="iconData" :src="iconData" :alt="fileName" :style="{ width: size + 'px', height: size + 'px' }"
      @error="handleIconError" />
    <div v-else class="default-icon"
      :style="{ width: size + 'px', height: size + 'px', fontSize: (size * 0.6) + 'px' }">
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
// 失败的路径缓存，避免重复尝试
const failedPathsCache = new Set();

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

// 检查是否应该使用静态图标（只针对项目界面的编辑器图标）
const shouldUseStaticIcon = () => {
  const lowerFileName = props.fileName.toLowerCase();
  return lowerFileName === 'code.exe' || lowerFileName === 'trae.exe' || lowerFileName === 'qoder.exe' || lowerFileName === 'idea64.exe' || lowerFileName === 'idea.exe';
};

// 获取静态图标路径
const getStaticIconSrc = () => {
  const lowerFileName = props.fileName.toLowerCase();

  if (lowerFileName === 'code.exe') {
    return '/code.png';
  }

  if (lowerFileName === 'trae.exe') {
    return '/trae.png';
  }

  if (lowerFileName === 'qoder.exe') {
    return '/qoder.png';
  }

  if (lowerFileName === 'idea64.exe' || lowerFileName === 'idea.exe') {
    return '/idea.png';
  }

  return '/icon.png';
};

// 加载文件图标
const loadIcon = async () => {
  if (isLoading.value) return;

  // 对于编辑器exe文件，直接使用静态图标
  if (shouldUseStaticIcon()) {
    iconData.value = getStaticIconSrc();
    return;
  }

  // 如果路径之前已经失败过，直接使用默认图标
  if (failedPathsCache.has(props.filePath)) {
    iconData.value = '';
    return;
  }

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
    // 将失败的路径加入失败缓存
    failedPathsCache.add(props.filePath);

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