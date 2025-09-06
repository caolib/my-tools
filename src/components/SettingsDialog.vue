<template>
  <el-dialog v-model="visible" title="设置" :close-on-click-modal="false" @closed="handleClosed">
    <el-form label-position="left">
      <el-form-item label="默认导出路径">
        <div class="path-input-group">
          <el-input v-model="settingsStore.exportPath" placeholder="请选择默认导出路径" readonly class="path-input">
            <template #prefix>
              <el-icon>
                <Folder />
              </el-icon>
            </template>
          </el-input>
          <el-button @click="selectExportPath" :icon="FolderOpened" type="primary">
            选择
          </el-button>
        </div>
      </el-form-item>

      <el-form-item label="导出后自动打开配置文件所在目录">
        <el-switch v-model="settingsStore.autoOpenFolder" />
      </el-form-item>
    </el-form>

    <template #footer>
      <span class="dialog-footer">
        <el-button @click="handleReset">恢复默认</el-button>
        <el-button @click="visible = false">取消</el-button>
        <el-button type="primary" @click="handleSave" :loading="saving">
          保存
        </el-button>
      </span>
    </template>
  </el-dialog>
</template>

<script setup>
import { ref, watch, computed } from "vue";
import { ElMessage } from "element-plus";
import { Folder, FolderOpened } from "@element-plus/icons-vue";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { useSettingsStore } from "@/stores/settings";

const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(["update:modelValue", "saved"]);

const settingsStore = useSettingsStore();
const saving = ref(false);

const visible = computed({
  get: () => props.modelValue,
  set: (val) => emit("update:modelValue", val),
});

// 默认设置
const DEFAULT_SETTINGS = {
  exportPath: "",
  autoOpenFolder: true,
};

// 选择导出路径
const selectExportPath = async () => {
  try {
    const documentsDir = await invoke("get_documents_dir");
    const selected = await open({
      title: "选择默认导出路径",
      defaultPath: settingsStore.exportPath || documentsDir,
      directory: true,
      multiple: false,
    });

    if (selected) {
      settingsStore.exportPath = selected;
    }
  } catch (error) {
    ElMessage.error("选择路径失败: " + error);
  }
};

// 保存设置
const handleSave = async () => {
  saving.value = true;

  try {
    // 使用$patch保存设置到Pinia store，持久化插件会自动保存到localStorage
    settingsStore.$patch({
      exportPath: settingsStore.exportPath,
      autoOpenFolder: settingsStore.autoOpenFolder,
    });
    ElMessage.success({
      message: "设置已保存",
    });
    emit("saved");
    visible.value = false;
  } catch (error) {
    ElMessage.error("保存设置失败: " + error);
  } finally {
    saving.value = false;
  }
};

// 监听可见性变化
watch(visible, (newVal) => {
  if (newVal) {
    // Pinia持久化插件会自动加载设置，无需手动调用
  }
});
</script>

<style scoped>
.path-input-group {
  display: flex;
  gap: 8px;
  align-items: center;
}

.path-input {
  flex: 1;
}

.path-tip,
.switch-tip {
  margin-top: 4px;
  line-height: 1.4;
}

.dialog-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
</style>