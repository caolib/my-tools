<template>
  <div class="custom-titlebar" data-tauri-drag-region>
    <div class="titlebar-content" data-tauri-drag-region>
      <!-- logo+搜索框区域 -->
      <div class="logo-search-group">
        <img src="/icon.png" alt="App Logo" class="app-logo" draggable="false" style="cursor: pointer"
          @click="openProjectRepo" />
        <div class="titlebar-search">
          <el-input v-model="searchText" placeholder="搜索变量名/值/全部" clearable size="small" style="width: 320px"
            @input="onSearchInput">
            <template #append>
              <el-select v-model="searchType" size="small" style="width: 80px" @change="onSearchInput">
                <el-option label="全部" value="all" />
                <el-option label="变量名" value="name" />
                <el-option label="变量值" value="value" />
              </el-select>
            </template>
          </el-input>
        </div>
      </div>

      <!-- 中间操作区域 -->
      <div class="titlebar-actions">
        <!-- 导入导出按钮 -->
        <el-dropdown @command="handleImportExport" trigger="click">
          <el-button size="small" :icon="Setting">
            配置
            <el-icon>
              <ArrowDown />
            </el-icon>
          </el-button>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item command="settings">
                <el-icon>
                  <Setting />
                </el-icon>
                设置
              </el-dropdown-item>
              <el-dropdown-item command="export">
                <el-icon>
                  <Download />
                </el-icon>
                导出配置
              </el-dropdown-item>
              <el-dropdown-item command="import">
                <el-icon>
                  <Upload />
                </el-icon>
                导入配置
              </el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>

        <!-- 主题切换 -->
        <el-button @click="toggleTheme($event)" :icon="isDark ? Sunny : Moon" circle class="theme-btn" />

        <!-- 刷新按钮 -->
        <el-button @click="$emit('refresh')" size="small" :icon="Refresh" :loading="loading">
          刷新
        </el-button>

        <!-- 管理员权限状态 -->
        <el-tag v-if="!isAdmin" type="warning" style="cursor: pointer" round @click="emit('requestAdminPrivileges')">
          没有以管理员身份运行
        </el-tag>
        <el-tag v-else type="success">
          <el-icon>
            <Check />
          </el-icon>
          管理员模式
        </el-tag>
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
</template>

<script setup>
import {
  Setting,
  Moon,
  Sunny,
  Warning,
  Check,
  Refresh,
  Search,
  FullScreen,
  SemiSelect,
  ArrowDown,
  Download,
  Upload,
} from "@element-plus/icons-vue";
import { ref, onMounted, computed, nextTick } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { openUrl } from "@tauri-apps/plugin-opener";
import { useSettingsStore } from "@/stores/settings";

const appWindow = getCurrentWindow();
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
  isAdmin: Boolean,
  loading: Boolean,
});

const searchText = ref("");
const searchType = ref("all");

const emit = defineEmits([
  "toggleTheme",
  "requestAdminPrivileges",
  "refresh",
  "search",
  "export",
  "import",
  "openSettings",
]);

const handleImportExport = (command) => {
  if (command === "settings") {
    emit("openSettings");
  } else {
    emit(command);
  }
};

const onSearchInput = () => {
  emit("search", { text: searchText.value, type: searchType.value });
};

const openProjectRepo = () => {
  openUrl("https://github.com/caolib/my-tools");
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

  .titlebar-search {
    -webkit-app-region: no-drag;
  }

  .titlebar-content {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 100%;
    padding: 0 var(--spacing-sm);

    // logo+搜索框组合
    .logo-search-group {
      display: flex;
      align-items: center;
      gap: 12px;
      margin-right: 16px;

      .app-logo {
        width: 28px;
        height: 28px;
        border-radius: 6px;
        box-shadow: 0 1px 4px rgba(0, 0, 0, 0.08);
        user-select: none;
      }

      .titlebar-search {
        margin-right: 0;
      }
    }

    .titlebar-actions {
      @include flex-start;
      gap: var(--spacing-sm);

      .theme-btn {
        width: 28px;
        height: 28px;
        border: none;
        background: transparent;

        &:hover {
          background: var(--el-fill-color-light);
        }
      }

      .refresh-btn {
        width: 28px;
        height: 28px;
        border: none;
        background: transparent;

        &:hover {
          background: var(--el-fill-color-light);
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
  .titlebar-actions,
  .window-controls {
    -webkit-app-region: no-drag;
  }
}
</style>
