<template>
  <div class="env-manager">
    <!-- 自定义标题栏 -->
    <Titlebar :isAdmin="isAdmin" :loading="loading" @requestAdminPrivileges="requestAdminPrivileges"
      @refresh="loadEnvVars" @search="onSearch" @export="exportEnvVars" @import="importEnvVars" />

    <!-- 主内容区域 -->
    <div class="main-content">
      <el-collapse v-model="activeCollapse" class="collapse-container">
        <!-- 系统变量 -->
        <el-collapse-item name="system" class="collapse-item">
          <template #title>
            <div class="collapse-header">
              <div class="header-info">
                <el-icon>
                  <Monitor />
                </el-icon>
                <div class="header-text">
                  <h2 class="section-title">系统环境变量</h2>
                  <p class="section-desc">{{ systemVars.length }} 个变量</p>
                </div>
              </div>
              <div class="header-actions">
                <el-button type="primary" text @click.stop="showAddSystemDialog" class="add-btn" :disabled="!isAdmin">
                  <el-icon>
                    <Plus />
                  </el-icon>
                  添加
                </el-button>
              </div>
            </div>
          </template>
          <div class="vars-container">
            <EnvVarCard v-for="row in filteredSystemVars" :key="row.name" :env-var="row" :is-admin="isAdmin"
              :disable-edit="!isAdmin || row.name === 'Path' && !isAdmin" @edit="(row) => editVar(row, 'system')"
              @delete="(row) => deleteVar(row, 'system')" />
          </div>
        </el-collapse-item>

        <!-- 用户变量 -->
        <el-collapse-item name="user" class="collapse-item">
          <template #title>
            <div class="collapse-header">
              <div class="header-info">
                <el-icon class="user-icon">
                  <User />
                </el-icon>
                <div class="header-text">
                  <h2 class="section-title">用户环境变量</h2>
                  <p class="section-desc">{{ userVars.length }} 个变量</p>
                </div>
              </div>
              <div class="header-actions">
                <el-button type="primary" @click.stop="showAddUserDialog" class="add-btn" text>
                  <el-icon>
                    <Plus />
                  </el-icon>
                  添加
                </el-button>
              </div>
            </div>
          </template>
          <div class="vars-container">
            <EnvVarCard v-for="row in filteredUserVars" :key="row.name" :env-var="row" :is-admin="true"
              :disable-edit="false" @edit="(row) => editVar(row, 'user')" @delete="(row) => deleteVar(row, 'user')" />
          </div>
        </el-collapse-item>
      </el-collapse>

      <!-- 添加/编辑对话框 -->
      <el-dialog v-model="showAddDialog" title="编辑环境变量" width="500px" :close-on-click-modal="false"
        class="custom-dialog">
        <el-form :model="newVarForm" label-width="0" class="form-container">
          <el-form-item required>
            <el-input v-model="newVarForm.name" placeholder="变量名" size="large" />
          </el-form-item>
          <el-form-item required>
            <el-input v-model="newVarForm.value" placeholder="变量值" type="textarea" :rows="4" size="large" />
          </el-form-item>
          <el-alert v-if="newVarForm.scope === 'system'" title="注意：修改系统环境变量需要管理员权限" type="warning" :closable="false"
            show-icon />
        </el-form>
        <template #footer>
          <div class="dialog-footer">
            <el-button @click="cancelEdit" size="large">取消</el-button>
            <el-button @click="addVar" :loading="submitting" size="large">{{ isEditing ? '更新' : '添加'
            }}</el-button>
          </div>
        </template>
      </el-dialog>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, computed, watch } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import {
  Plus,
  Refresh,
  Monitor,
  User,
  Edit,
  Delete,
  Moon,
  Sunny
} from '@element-plus/icons-vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import EnvVarCard from './EnvVarCard.vue'
import Titlebar from './Titlebar.vue'

const systemVars = ref([])
const userVars = ref([])
const searchText = ref('')
const searchType = ref('all')
// 过滤后的变量，并将 Path 变量置顶
function sortPathFirst(vars) {
  const idx = vars.findIndex(v => v.name === 'Path')
  if (idx === -1) return vars
  return [vars[idx], ...vars.slice(0, idx), ...vars.slice(idx + 1)]
}

const filteredSystemVars = computed(() => {
  let arr = []
  if (!searchText.value) {
    arr = systemVars.value
  } else if (searchType.value === 'all') {
    arr = systemVars.value.filter(v => v.name.toLowerCase().includes(searchText.value.toLowerCase()) || v.value.toLowerCase().includes(searchText.value.toLowerCase()))
  } else if (searchType.value === 'name') {
    arr = systemVars.value.filter(v => v.name.toLowerCase().includes(searchText.value.toLowerCase()))
  } else if (searchType.value === 'value') {
    arr = systemVars.value.filter(v => v.value.toLowerCase().includes(searchText.value.toLowerCase()))
  } else {
    arr = systemVars.value
  }
  return sortPathFirst(arr)
})

const filteredUserVars = computed(() => {
  let arr = []
  if (!searchText.value) {
    arr = userVars.value
  } else if (searchType.value === 'all') {
    arr = userVars.value.filter(v => v.name.toLowerCase().includes(searchText.value.toLowerCase()) || v.value.toLowerCase().includes(searchText.value.toLowerCase()))
  } else if (searchType.value === 'name') {
    arr = userVars.value.filter(v => v.name.toLowerCase().includes(searchText.value.toLowerCase()))
  } else if (searchType.value === 'value') {
    arr = userVars.value.filter(v => v.value.toLowerCase().includes(searchText.value.toLowerCase()))
  } else {
    arr = userVars.value
  }
  return sortPathFirst(arr)
})

// 处理搜索事件
const onSearch = ({ text, type }) => {
  searchText.value = text
  searchType.value = type
  // 搜索时自动展开所有面板
  activeCollapse.value = ['system', 'user']
}
const showAddDialog = ref(false)
const loading = ref(false)
const submitting = ref(false)
const COLLAPSE_KEY = 'wem_env_collapse'
// 默认全部折叠
const activeCollapse = ref([])
const isAdmin = ref(false) // 管理员权限状态

const newVarForm = ref({
  name: '',
  value: '',
  scope: 'user'
})

const editMode = ref(false)

// 显示添加系统变量对话框
const showAddSystemDialog = () => {
  newVarForm.value = { name: '', value: '', scope: 'system' }
  editMode.value = false
  showAddDialog.value = true
}

// 显示添加用户变量对话框
const showAddUserDialog = () => {
  newVarForm.value = { name: '', value: '', scope: 'user' }
  editMode.value = false
  showAddDialog.value = true
}




// 检测管理员权限
const checkAdminPrivileges = async () => {
  try {
    // 调用后端检测管理员权限
    const result = await invoke('check_admin_privileges')
    isAdmin.value = result
  } catch (error) {
    isAdmin.value = false
    console.log('检测管理员权限失败:', error)
  }
}

// 请求管理员权限
const requestAdminPrivileges = async () => {
  try {
    await ElMessageBox.confirm(
      '应用将以管理员身份重新启动，是否继续？',
      '管理员权限确认',
      {
        type: 'warning',
        confirmButtonText: '确定',
        cancelButtonText: '取消'
      }
    )

    // 调用后端以管理员身份重启（已集成窗口状态保存）
    await invoke('request_admin_privileges')
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error(`请求管理员权限失败: ${error}`)
      console.error('Error requesting admin privileges:', error)
    }
  }
}

// 仅在编辑时为 true
const isEditing = computed(() => editMode.value)

// 加载真实的环境变量
const loadEnvVars = async () => {
  loading.value = true
  try {
    const result = await invoke('get_env_vars')
    systemVars.value = result.system_vars || []
    userVars.value = result.user_vars || []
    ElMessage({
      message: `成功加载 ${systemVars.value.length + userVars.value.length} 个环境变量`,
      grouping: true,
      offset: 40,
      duration: 1000,
      type: 'success',
    })
  } catch (error) {
    ElMessage.error(`获取环境变量失败: ${error}`)
    console.error('Error loading env vars:', error)
  } finally {
    loading.value = false
  }
}

const editVar = (row, scope) => {
  newVarForm.value = {
    ...row,
    scope: scope
  }
  editMode.value = true
  showAddDialog.value = true
}

const cancelEdit = () => {
  showAddDialog.value = false
  newVarForm.value = { name: '', value: '', scope: 'user' }
  editMode.value = false
}

const deleteVar = async (row, scope) => {
  try {
    const isSystem = scope === 'system'
    await invoke('delete_env_var', {
      name: row.name,
      isSystem
    })

    ElMessage.success(`变量 "${row.name}" 删除成功`)
    await loadEnvVars() // 重新加载环境变量
  } catch (error) {
    ElMessage.error(`删除失败: ${error}`)
    console.error('Error deleting env var:', error)
  }
}

const addVar = async () => {
  if (!newVarForm.value.name || !newVarForm.value.value) {
    ElMessage.warning('请填写完整的变量信息')
    return
  }

  // 检查变量是否已存在
  const targetVars = newVarForm.value.scope === 'system' ? systemVars.value : userVars.value
  const existingVar = targetVars.find(v => v.name === newVarForm.value.name)

  if (existingVar && !isEditing.value) {
    try {
      await ElMessageBox.confirm(
        `变量 "${newVarForm.value.name}" 已存在，是否覆盖？`,
        '变量已存在',
        {
          type: 'warning',
          confirmButtonText: '覆盖',
          cancelButtonText: '取消'
        }
      )
    } catch (error) {
      if (error === 'cancel') return
    }
  }

  submitting.value = true
  try {
    // 调用 Tauri 后端 API 添加变量
    await invoke('set_env_var', {
      name: newVarForm.value.name,
      value: newVarForm.value.value,
      isSystem: newVarForm.value.scope === 'system'
    })

    const action = isEditing.value ? '更新' : '添加'
    ElMessage.success(`变量 "${newVarForm.value.name}" ${action}成功`)
    showAddDialog.value = false
    newVarForm.value = { name: '', value: '', scope: 'user' }
    await loadEnvVars() // 重新加载环境变量
  } catch (error) {
    const action = isEditing.value ? '更新' : '添加'
    ElMessage.error(`${action}失败: ${error}`)
    console.error('Error adding env var:', error)
  } finally {
    submitting.value = false
  }
}


// 恢复折叠状态
onMounted(() => {
  const saved = localStorage.getItem(COLLAPSE_KEY)
  if (saved) {
    try {
      const arr = JSON.parse(saved)
      if (Array.isArray(arr)) activeCollapse.value = arr
    } catch { }
  }
  checkAdminPrivileges()
  loadEnvVars()
})

// 监听折叠状态变化并保存
watch(activeCollapse, (val) => {
  localStorage.setItem(COLLAPSE_KEY, JSON.stringify(val))
}, { deep: true })

// 导出环境变量配置
const exportEnvVars = async () => {
  try {
    const filePath = await invoke('export_env_vars')
    ElMessage.success(`配置已导出到: ${filePath}`)

    // 自动在文件管理器中显示导出的文件
    try {
      await invoke('reveal_in_explorer', { filePath })
    } catch (error) {
      console.warn('无法打开文件管理器:', error)
    }
  } catch (error) {
    ElMessage.error(`导出失败: ${error}`)
    console.error('Export error:', error)
  }
}

// 导入环境变量配置
const importEnvVars = async () => {
  try {
    // 打开文件选择对话框
    const selected = await open({
      title: '选择环境变量配置文件',
      filters: [{
        name: 'JSON 配置文件',
        extensions: ['json']
      }],
      multiple: false
    })

    if (!selected) {
      return // 用户取消选择
    }

    // 确认导入
    await ElMessageBox.confirm(
      '导入配置会覆盖已存在的同名环境变量，是否继续？',
      '确认导入',
      {
        type: 'warning',
        confirmButtonText: '确定导入',
        cancelButtonText: '取消'
      }
    )

    // 调用后端导入
    const result = await invoke('import_env_vars', { filePath: selected })
    ElMessage.success(result)

    // 重新加载环境变量
    await loadEnvVars()
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error(`导入失败: ${error}`)
      console.error('Import error:', error)
    }
  }
}
</script>

<style lang="scss" scoped>
@use '../assets/styles/variables.scss' as *;

.env-manager {
  min-height: 100vh;
  background: linear-gradient(135deg, var(--el-bg-color-page) 0%, var(--el-fill-color-lighter) 100%);

  .main-content {
    padding: 50px var(--spacing-md) var(--spacing-md);

    .collapse-container {
      .collapse-item {
        .collapse-header {
          @include flex-between;
          padding: var(--spacing-lg);
          width: 100%;

          .header-info {
            @include flex-start;
            gap: var(--spacing-md);

            .header-text {
              .section-title {
                margin: 0 0 var(--spacing-xs) 0;
                font-size: var(--font-size-large);
                font-weight: var(--font-weight-primary);
                color: var(--el-text-color-primary);
              }

              .section-desc {
                margin: 0;
                font-size: var(--font-size-small);
                color: var(--el-text-color-secondary);
              }
            }
          }

          .header-actions {
            @include flex-start;
            gap: var(--spacing-sm);

            .add-btn {
              @include button-style;
            }
          }
        }

        .vars-container {
          padding: 0 var(--spacing-lg) var(--spacing-lg);
        }
      }
    }

    .form-container {
      .el-form-item {
        margin-bottom: var(--spacing-lg);
      }
    }

    .dialog-footer {
      @include flex-between;

      .el-button {
        @include button-style;
      }
    }
  }

  // 响应式设计
  @include respond-to('xs') {
    .main-content {
      padding: 50px var(--spacing-md) var(--spacing-md);

      .collapse-item .collapse-header {
        flex-direction: column;
        align-items: flex-start;
        gap: var(--spacing-md);

        .header-actions {
          align-self: flex-end;
        }
      }
    }
  }
}
</style>
