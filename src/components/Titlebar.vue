<template>
  <div class="custom-titlebar" data-tauri-drag-region>
    <div class="titlebar-content" data-tauri-drag-region>
      <!-- 左侧区域：logo + 导航 -->
      <div class="left-section">
        <div class="logo-group">
          <img src="/icon.png" alt="App Logo" class="app-logo" draggable="false" style="cursor: pointer"
            @click="openProjectRepo" />
        </div>

        <!-- 导航项 -->
        <div class="nav-items" data-tauri-drag-region="false">
          <button class="nav-item" :class="{ active: $route.name === 'EnvVarManager' }" @click="navigateTo('/env')">
            环境变量
          </button>
          <button class="nav-item" :class="{ active: $route.path === '/file-search' || $route.path === '/' }"
            @click="navigateTo('/file-search')">
            文件搜索
          </button>
          <button class="nav-item" :class="{ active: $route.name === 'Projects' }" @click="navigateTo('/projects')">
            项目
          </button>
        </div>
      </div>

      <!-- 右侧区域：主题切换 + 窗口控制 -->
      <div class="right-section">
        <!-- 主题切换 -->
        <div class="theme-controls" data-tauri-drag-region="false">
          <el-button @click="toggleTheme($event)" :icon="isDark ? Sunny : Moon" circle class="theme-btn" />
        </div>

        <!-- 窗口控制按钮 -->
        <div class="window-controls" data-tauri-drag-region="false">
          <button class="title-bar-button" @click="minimizeWindow" title="最小化">
            <svg width="12" height="12" viewBox="0 0 12 12">
              <rect x="2" y="5" width="8" height="2" fill="currentColor" />
            </svg>
          </button>
          <button class="title-bar-button" @click="toggleMaximize" :title="isMaximized ? '还原' : '最大化'">
            <svg width="12" height="12" viewBox="0 0 12 12" v-if="!isMaximized">
              <rect x="2" y="2" width="8" height="8" fill="none" stroke="currentColor" stroke-width="1" />
            </svg>
            <svg width="12" height="12" viewBox="0 0 12 12" v-else>
              <path d="M4 1 L10 1 L10 4 L8 4 L8 3 L4 3 Z" fill="none" stroke="currentColor" stroke-width="1" />
              <path d="M8 4 L10 4 L10 7 L8 7 Z" fill="none" stroke="currentColor" stroke-width="1" />
              <rect x="2" y="3" width="6" height="6" fill="none" stroke="currentColor" stroke-width="1" />
            </svg>
          </button>
          <button class="title-bar-button close" @click="closeWindow" title="关闭">
            <svg width="12" height="12" viewBox="0 0 12 12">
              <path d="M2 2 L10 10 M10 2 L2 10" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
            </svg>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import {
  Moon,
  Sunny,
} from "@element-plus/icons-vue";
import { ref, onMounted, computed, nextTick } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { openUrl } from "@tauri-apps/plugin-opener";
import { useRouter } from "vue-router";
import { useSettingsStore } from "@/stores/settings";

const appWindow = getCurrentWindow();
const router = useRouter();
const isMaximized = ref(false);
const settingsStore = useSettingsStore();

const updateMaximized = async () => {
  isMaximized.value = await appWindow.isMaximized();
};

const minimizeWindow = () => {
  appWindow.minimize();
};

const toggleMaximize = async () => {
  await appWindow.toggleMaximize();
  updateMaximized();
};

const closeWindow = () => {
  appWindow.close();
};

// 使用computed属性从store获取主题状态
const isDark = computed(() => settingsStore.theme === "dark");

// 初始化主题
onMounted(() => {
  updateMaximized();
  appWindow.onResized(() => updateMaximized());
  // 主题已在 main.js 中初始化，这里不需要重复设置
});

// 修改主题切换逻辑
const toggleTheme = async (event) => {
  const enableTransitions = () =>
    'startViewTransition' in document &&
    window.matchMedia('(prefers-reduced-motion: no-preference)').matches;

  const newTheme = isDark.value ? "light" : "dark";

  if (!enableTransitions()) {
    settingsStore.setTheme(newTheme);
    return;
  }

  const x = event.clientX;
  const y = event.clientY;
  const clipPath = [
    `circle(0px at ${x}px ${y}px)`,
    `circle(${Math.hypot(
      Math.max(x, innerWidth - x),
      Math.max(y, innerHeight - y)
    )}px at ${x}px ${y}px)`
  ];

  await document.startViewTransition(async () => {
    settingsStore.setTheme(newTheme);
    await nextTick();
  }).ready;

  document.documentElement.animate(
    { clipPath: clipPath },
    {
      duration: 300,
      easing: 'ease-in',
      pseudoElement: `::view-transition-new(root)`
    }
  );
};

const props = defineProps({
});

const emit = defineEmits([
  "toggleTheme",
]);

const openProjectRepo = () => {
  openUrl("https://github.com/caolib/my-tools");
};

// 导航功能
const navigateTo = (path) => {
  router.push(path);
};
</script>

<style lang="scss" scoped>
@use "../assets/styles/variables.scss" as *;

.custom-titlebar {
  height: 40px;
  background: var(--el-bg-color);
  border-bottom: 1px solid var(--el-border-color-lighter);
  user-select: none;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 1000;

  .titlebar-content {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 100%;
    padding: 0 var(--spacing-sm);

    // 左侧区域
    .left-section {
      display: flex;
      align-items: center;
      gap: var(--spacing-md);

      // logo组合
      .logo-group {
        display: flex;
        align-items: center;
        gap: 12px;

        .app-logo {
          width: 28px;
          height: 28px;
          border-radius: 6px;
          box-shadow: 0 1px 4px rgba(0, 0, 0, 0.08);
          user-select: none;
        }

        .app-title {
          font-size: 14px;
          font-weight: 500;
          color: var(--el-text-color-primary);
        }
      }

      // 导航项
      .nav-items {
        display: flex;
        align-items: center;
        gap: var(--spacing-xs);

        .nav-item {
          padding: 6px 12px;
          border: none;
          background: transparent;
          border-radius: 6px;
          font-size: 13px;
          font-weight: 500;
          color: var(--el-text-color-secondary);
          cursor: pointer;
          transition: all 0.2s ease;
          user-select: none;

          &:hover {
            background: var(--el-fill-color-light);
            color: var(--el-text-color-primary);
          }

          &.active {
            background: var(--el-color-primary-light-9);
            color: var(--el-color-primary);
          }
        }
      }
    }

    // 右侧区域
    .right-section {
      display: flex;
      align-items: center;
      gap: var(--spacing-xs);

      .theme-controls {
        display: flex;
        align-items: center;

        .theme-btn {
          width: 28px;
          height: 28px;
          border: none;
          background: transparent;

          &:hover {
            background: var(--el-fill-color-light);
          }
        }
      }
    }

    .window-controls {
      display: flex;
      align-items: center;
      gap: 4px;
      height: 100%;

      .title-bar-button {
        width: 32px;
        height: 32px;
        border: none;
        background: transparent;
        border-radius: 6px;
        display: flex;
        align-items: center;
        justify-content: center;
        color: var(--el-text-color-secondary);
        transition: background 0.2s, color 0.2s;
        margin: 0 1px;

        &:hover {
          background: var(--el-fill-color-light);
          color: var(--el-text-color-primary);
        }

        &.close:hover {
          background: var(--el-color-danger);
          color: #fff;
        }
      }
    }
  }

  // 防止拖拽区域影响按钮点击
  .nav-items,
  .theme-controls,
  .window-controls {
    -webkit-app-region: no-drag;
  }
}
</style>
