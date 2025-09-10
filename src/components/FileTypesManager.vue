<template>
  <el-dialog
    v-model="visible"
    width="800px"
    :close-on-click-modal="false"
  >
    <div class="file-types-manager">
      <!-- 现有文件类型列表 -->
      <div class="existing-types">
        <h3>文件类型配置</h3>
        <el-table :data="fileTypeList" style="width: 100%" max-height="300">
          <el-table-column prop="key" label="键名" width="120">
            <template #default="{ row }">
              <el-text type="info" size="small">{{ row.key }}</el-text>
            </template>
          </el-table-column>
          <el-table-column prop="name" label="类型名称" width="120">
            <template #default="{ row, $index }">
              <el-input
                v-if="editingIndex === $index"
                v-model="editingData.name"
                size="small"
              />
              <span v-else>{{ row.name }}</span>
            </template>
          </el-table-column>
          <el-table-column label="显示" width="80" align="center">
            <template #default="{ row }">
              <el-switch
                :model-value="row.isVisible !== false"
                @change="(val) => fileTypesStore.updateFileTypeVisibility(row.key, val)"
                size="small"
              />
            </template>
          </el-table-column>
          <el-table-column prop="extensions" label="文件后缀" min-width="300">
            <template #default="{ row, $index }">
              <div v-if="editingIndex === $index" class="extensions-edit">
                <el-tag
                  v-for="(ext, extIndex) in editingData.extensions"
                  :key="extIndex"
                  closable
                  @close="removeExtension(extIndex)"
                  class="extension-tag"
                >
                  {{ ext }}
                </el-tag>
                <el-input
                  v-if="showNewExtInput"
                  ref="newExtInputRef"
                  v-model="newExtension"
                  size="small"
                  @keyup.enter="addExtension"
                  @blur="addExtension"
                  class="new-ext-input"
                />
                <el-button
                  v-else
                  size="small"
                  @click="showAddExtension"
                  type="primary"
                  link
                >
                  + 添加后缀
                </el-button>
              </div>
              <div v-else class="extensions-display">
                <el-tag
                  v-for="ext in row.extensions"
                  :key="ext"
                  size="small"
                  class="extension-tag"
                >
                  {{ ext }}
                </el-tag>
              </div>
            </template>
          </el-table-column>
          <el-table-column label="操作" width="120">
            <template #default="{ row, $index }">
              <div v-if="editingIndex === $index">
                <el-button size="small" type="primary" @click="saveEdit">保存</el-button>
                <el-button size="small" @click="cancelEdit">取消</el-button>
              </div>
              <div v-else>
                <el-button size="small" @click="startEdit($index, row)">编辑</el-button>
                <el-button
                  v-if="row.isCustom"
                  size="small"
                  type="danger"
                  @click="deleteFileType($index, row.key)"
                >
                  删除
                </el-button>
              </div>
            </template>
          </el-table-column>
        </el-table>
      </div>
      
      <!-- 特殊筛选选项 -->
      <div class="special-filters">
        <h3>特殊筛选选项</h3>
        <el-table :data="specialFilterList" style="width: 100%" max-height="150">
          <el-table-column prop="key" label="键名" width="120">
            <template #default="{ row }">
              <el-text type="info" size="small">{{ row.key }}</el-text>
            </template>
          </el-table-column>
          <el-table-column prop="name" label="类型名称" width="120">
            <template #default="{ row }">
              <span>{{ row.name }}</span>
            </template>
          </el-table-column>
          <el-table-column label="显示" width="80" align="center">
            <template #default="{ row }">
              <el-switch
                :model-value="row.isVisible !== false"
                @change="(val) => fileTypesStore.updateSpecialFilterVisibility(row.key, val)"
                size="small"
              />
            </template>
          </el-table-column>
        </el-table>
      </div>

      <!-- 添加自定义类型 -->
      <div class="add-custom-type">
        <h3>添加自定义类型</h3>
        <el-form :model="newFileType" label-width="80px">
          <el-form-item label="类型键名">
            <el-input
              v-model="newFileType.key"
              placeholder="如: code, design 等（英文，唯一）"
            />
          </el-form-item>
          <el-form-item label="类型名称">
            <el-input
              v-model="newFileType.name"
              placeholder="如: 代码文件, 设计文件 等"
            />
          </el-form-item>
          <el-form-item label="文件后缀">
            <div class="new-type-extensions">
              <el-tag
                v-for="(ext, index) in newFileType.extensions"
                :key="index"
                closable
                @close="removeNewTypeExtension(index)"
                class="extension-tag"
              >
                {{ ext }}
              </el-tag>
              <el-input
                v-if="showNewTypeExtInput"
                ref="newTypeExtInputRef"
                v-model="newTypeExtension"
                size="small"
                @keyup.enter="addNewTypeExtension"
                @blur="addNewTypeExtension"
                class="new-ext-input"
              />
              <el-button
                v-else
                size="small"
                @click="showAddNewTypeExtension"
                type="primary"
                link
              >
                + 添加后缀
              </el-button>
            </div>
          </el-form-item>
          <el-form-item>
            <el-button type="primary" @click="addCustomType">添加类型</el-button>
            <el-button @click="resetNewFileType">重置</el-button>
          </el-form-item>
        </el-form>
      </div>
    </div>

    <template #footer>
      <!-- <el-button @click="debugCurrentConfig">查看当前配置</el-button> -->
      <el-button @click="visible = false">关闭</el-button>
      <el-button type="danger" @click="resetToDefault">重置为默认</el-button>
    </template>
  </el-dialog>
</template>

<script setup>
import { ref, computed, watch, nextTick } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useFileTypesStore } from '@/stores/fileTypes'

const visible = defineModel('visible', { default: false })

const fileTypesStore = useFileTypesStore()

// 编辑相关
const editingIndex = ref(-1)
const editingData = ref({ name: '', extensions: [] })
const showNewExtInput = ref(false)
const newExtension = ref('')
const newExtInputRef = ref()

// 新增自定义类型
const newFileType = ref({
  key: '',
  name: '',
  extensions: []
})
const showNewTypeExtInput = ref(false)
const newTypeExtension = ref('')
const newTypeExtInputRef = ref()

// 文件类型列表（转换为数组形式便于表格显示）
const fileTypeList = computed(() => {
  return Object.entries(fileTypesStore.fileTypes).map(([key, value]) => ({
    key,
    ...value
  }))
})

// 特殊筛选项列表
const specialFilterList = computed(() => {
  return Object.entries(fileTypesStore.specialFilters).map(([key, value]) => ({
    key,
    ...value
  }))
})

// 开始编辑
const startEdit = (index, row) => {
  editingIndex.value = index
  editingData.value = {
    name: row.name,
    extensions: [...row.extensions]
  }
}

// 取消编辑
const cancelEdit = () => {
  editingIndex.value = -1
  editingData.value = { name: '', extensions: [] }
  showNewExtInput.value = false
  newExtension.value = ''
}

// 保存编辑
const saveEdit = () => {
  const row = fileTypeList.value[editingIndex.value]
  if (!editingData.value.name.trim()) {
    ElMessage.error('类型名称不能为空')
    return
  }
  if (editingData.value.extensions.length === 0) {
    ElMessage.error('至少需要一个文件后缀')
    return
  }
  
  fileTypesStore.updateFileType(
    row.key,
    editingData.value.name,
    editingData.value.extensions
  )
  cancelEdit()
  ElMessage.success('保存成功')
}

// 显示添加后缀输入框
const showAddExtension = () => {
  showNewExtInput.value = true
  nextTick(() => {
    newExtInputRef.value?.focus()
  })
}

// 添加后缀
const addExtension = () => {
  const ext = newExtension.value.trim().toLowerCase()
  if (ext && !editingData.value.extensions.includes(ext)) {
    editingData.value.extensions.push(ext)
  }
  newExtension.value = ''
  showNewExtInput.value = false
}

// 移除后缀
const removeExtension = (index) => {
  editingData.value.extensions.splice(index, 1)
}

// 显示添加新类型后缀输入框
const showAddNewTypeExtension = () => {
  showNewTypeExtInput.value = true
  nextTick(() => {
    newTypeExtInputRef.value?.focus()
  })
}

// 添加新类型后缀
const addNewTypeExtension = () => {
  const ext = newTypeExtension.value.trim().toLowerCase()
  if (ext && !newFileType.value.extensions.includes(ext)) {
    newFileType.value.extensions.push(ext)
  }
  newTypeExtension.value = ''
  showNewTypeExtInput.value = false
}

// 移除新类型后缀
const removeNewTypeExtension = (index) => {
  newFileType.value.extensions.splice(index, 1)
}

// 添加自定义类型
const addCustomType = () => {
  const { key, name, extensions } = newFileType.value
  
  if (!key.trim()) {
    ElMessage.error('类型键名不能为空')
    return
  }
  if (!name.trim()) {
    ElMessage.error('类型名称不能为空')
    return
  }
  if (extensions.length === 0) {
    ElMessage.error('至少需要一个文件后缀')
    return
  }
  if (!/^[a-zA-Z][a-zA-Z0-9_]*$/.test(key)) {
    ElMessage.error('类型键名只能包含字母、数字和下划线，且以字母开头')
    return
  }
  
  const success = fileTypesStore.addCustomFileType(key.trim(), name.trim(), extensions)
  if (success) {
    ElMessage.success('添加成功')
    resetNewFileType()
  } else {
    ElMessage.error('类型键名已存在')
  }
}

// 重置新文件类型表单
const resetNewFileType = () => {
  newFileType.value = {
    key: '',
    name: '',
    extensions: []
  }
  showNewTypeExtInput.value = false
  newTypeExtension.value = ''
}

// 删除自定义文件类型
const deleteFileType = async (index, key) => {
  try {
    await ElMessageBox.confirm('确定要删除这个自定义文件类型吗？', '确认删除', {
      type: 'warning'
    })
    
    const success = fileTypesStore.deleteCustomFileType(key)
    if (success) {
      ElMessage.success('删除成功')
    } else {
      ElMessage.error('无法删除系统预设类型')
    }
  } catch {
    // 用户取消删除
  }
}

// 重置为默认
const resetToDefault = async () => {
  try {
    await ElMessageBox.confirm('确定要重置所有文件类型配置为默认设置吗？这将删除所有自定义类型。', '确认重置', {
      type: 'warning'
    })
    
    fileTypesStore.resetToDefault()
    ElMessage.success('已重置为默认配置')
  } catch {
    // 用户取消重置
  }
}

// 调试功能：查看当前配置
// const debugCurrentConfig = () => {
//   console.log('当前文件类型配置:', fileTypesStore.fileTypes)
//   console.log('localStorage 中的配置:', localStorage.getItem('wem-file-types'))
//   console.log('音频类型配置:', fileTypesStore.fileTypes.audio)
//   ElMessage.info('请查看控制台输出')
// }

// 监听对话框关闭，重置编辑状态
watch(visible, (newVal) => {
  if (!newVal) {
    cancelEdit()
    resetNewFileType()
  }
})
</script>

<style scoped>
.file-types-manager {
  max-height: 600px;
  overflow-y: auto;
}

.existing-types {
  margin-bottom: 30px;
}

.existing-types h3,
.add-custom-type h3 {
  margin-bottom: 15px;
  color: var(--el-text-color-primary);
  font-size: 16px;
  font-weight: 600;
}

.extensions-edit,
.extensions-display,
.new-type-extensions {
  display: flex;
  flex-wrap: wrap;
  gap: 5px;
  align-items: center;
}

.extension-tag {
  margin-right: 0;
}

.new-ext-input {
  width: 80px;
}

.add-custom-type {
  border-top: 1px solid var(--el-border-color-light);
  padding-top: 20px;
}
</style>