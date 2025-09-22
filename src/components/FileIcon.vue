<template>
  <div class="file-icon" :style="{ width: size + 'px', height: size + 'px' }">
    <img :src="getIconSrc()" :alt="fileName" :style="{ width: size + 'px', height: size + 'px' }"
      @error="handleIconError" />
  </div>
</template>

<script setup>
import { computed } from 'vue';

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

// 获取图标路径
const getIconSrc = () => {
  // 检查是否是特定的应用程序
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

  // 对于其他文件，使用默认图标
  return '/icon.png';
};

// 处理图标加载错误
const handleIconError = (event) => {
  // 如果图标加载失败，使用默认图标
  event.target.src = '/icon.png';
};
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
</style>