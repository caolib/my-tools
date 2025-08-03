<template>
  <div class="env-manager">
    <!-- 自定义标题栏 -->
    <Titlebar :isAdmin="isAdmin" :loading="loading" @requestAdminPrivileges="requestAdminPrivileges"
      @refresh="loadEnvVars" @search="onSearch" />

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
                <el-button type="primary" text @click.stop="showAddSystemDialog" class="add-btn">
                  <el-icon>
                    <Plus />
                  </el-icon>
                  添加
                </el-button>
              </div>
            </div>
          </template>
          <div class="vars-container">
            <EnvVarCard v-for="row in filteredSystemVars" :key="row.name" :env-var="row" @edit="editVar"
              @delete="deleteVar" />
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
            <EnvVarCard v-for="row in filteredUserVars" :key="row.name" :env-var="row" @edit="editVar"
              @delete="deleteVar" />
          </div>
        </el-collapse-item>
      </el-collapse>

      <!-- 添加/编辑对话框 -->
      <el-dialog v-model="showAddDialog" :title="dialogTitle" width="500px" :close-on-click-modal="false"
        class="custom-dialog">
        <el-form :model="newVarForm" label-width="0" class="form-container">
          <el-form-item required>
            <el-input v-model="newVarForm.name" placeholder="变量名" size="large" />
          </el-form-item>
          <el-form-item required>
            <el-input v-model="newVarForm.value" placeholder="变量值" type="textarea" :rows="4" size="large"
              show-word-limit />
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
import { ref, onMounted, computed } from 'vue'
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
import EnvVarCard from './EnvVarCard.vue'
import Titlebar from './Titlebar.vue'

const systemVars = ref([])
const userVars = ref([])
const searchText = ref('')
const searchType = ref('all')
// 过滤后的变量
const filteredSystemVars = computed(() => {
  if (!searchText.value) return systemVars.value
  if (searchType.value === 'all') {
    return systemVars.value.filter(v => v.name.toLowerCase().includes(searchText.value.toLowerCase()) || v.value.toLowerCase().includes(searchText.value.toLowerCase()))
  } else if (searchType.value === 'name') {
    return systemVars.value.filter(v => v.name.toLowerCase().includes(searchText.value.toLowerCase()))
  } else if (searchType.value === 'value') {
    return systemVars.value.filter(v => v.value.toLowerCase().includes(searchText.value.toLowerCase()))
  }
  return systemVars.value
})

const filteredUserVars = computed(() => {
  if (!searchText.value) return userVars.value
  if (searchType.value === 'all') {
    return userVars.value.filter(v => v.name.toLowerCase().includes(searchText.value.toLowerCase()) || v.value.toLowerCase().includes(searchText.value.toLowerCase()))
  } else if (searchType.value === 'name') {
    return userVars.value.filter(v => v.name.toLowerCase().includes(searchText.value.toLowerCase()))
  } else if (searchType.value === 'value') {
    return userVars.value.filter(v => v.value.toLowerCase().includes(searchText.value.toLowerCase()))
  }
  return userVars.value
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
const activeCollapse = ref(['system']) // 默认只展开系统变量，折叠用户变量
const isAdmin = ref(false) // 管理员权限状态

const newVarForm = ref({
  name: '',
  value: '',
  scope: 'user'
})
const editMode = ref(false)

// 计算对话框标题
const dialogTitle = computed(() => {
  if (isEditing.value) {
    return newVarForm.value.scope === 'system' ? '编辑系统环境变量' : '编辑用户环境变量'
  } else {
    return newVarForm.value.scope === 'system' ? '添加系统环境变量' : '添加用户环境变量'
  }
})

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
      '应用将以管理员身份重新启动。是否继续？',
      '管理员权限确认',
      {
        type: 'warning',
        confirmButtonText: '确定',
        cancelButtonText: '取消'
      }
    )

    // 调用后端以管理员身份重启
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

const editVar = (row) => {
  newVarForm.value = {
    ...row,
    scope: systemVars.value.includes(row) ? 'system' : 'user'
  }
  editMode.value = true
  showAddDialog.value = true
}

const cancelEdit = () => {
  showAddDialog.value = false
  newVarForm.value = { name: '', value: '', scope: 'user' }
  editMode.value = false
}

const deleteVar = async (row) => {
  try {
    // 直接删除，无需二次确认，Popconfirm 已确认
    const isSystem = systemVars.value.includes(row)
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

onMounted(() => {
  checkAdminPrivileges()
  loadEnvVars()
})
</script>

<style lang="scss" scoped>
@use '../assets/styles/variables.scss' as *;

.env-manager {
  min-height: 100vh;
  background: linear-gradient(135deg, var(--el-bg-color-page) 0%, var(--el-fill-color-lighter) 100%);

  .main-content {
    padding: 60px var(--spacing-md) var(--spacing-md);

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
      padding: 60px var(--spacing-md) var(--spacing-md);

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
