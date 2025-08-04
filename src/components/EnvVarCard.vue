<template>
    <div class="env-var-card">
        <div class="card-header">
            <span class="var-name">{{ envVar.name }}</span>
            <div class="card-actions">
                <el-button @click="$emit('edit', envVar)" size="small" :icon="Edit" text round>编辑</el-button>
                <el-popconfirm title="确定要删除该变量吗？" confirm-button-text="确定" cancel-button-text="取消"
                    @confirm="$emit('delete', envVar)">
                    <template #reference>
                        <el-button size="small" :icon="Delete" round text type="danger">删除</el-button>
                    </template>
                </el-popconfirm>
            </div>
        </div>
        <div class="var-value">
            <template v-if="envVar.name === 'Path'">
                <div v-if="!editingPath" class="path-list-clickable" @click="startEditPath" style="cursor:pointer;">
                    <ul class="path-list">
                        <li v-for="(item, idx) in pathList" :key="idx">{{ item }}</li>
                    </ul>
                </div>
                <div v-else>
                    <div class="path-edit-container">
                        <div style="display: flex; flex-direction: column; gap: 8px; margin-bottom: 12px;">
                            <div style="display: flex; gap: 8px; align-items: center;">
                                <el-button size="small" type="primary" @click="savePath">保存</el-button>
                                <el-button size="small" @click="cancelEditPath">取消</el-button>
                            </div>
                            <el-alert v-if="isDirty" title="有未保存的更改，请点击保存" type="warning" show-icon
                                style="margin-bottom: 8px;" />
                        </div>
                        <div v-for="(item, index) in editList" :key="`item-${index}`" class="path-edit-item">
                            <!-- 输入框区域 -->
                            <div class="input-wrapper">
                                <el-input v-model="editList[index]" class="path-input" placeholder="输入路径..."
                                    size="small" />
                            </div>
                            <!-- 操作菜单 -->
                            <el-dropdown @command="handleCommand" trigger="click">
                                <el-button size="small" text class="menu-btn">
                                    <el-icon>
                                        <MoreFilled />
                                    </el-icon>
                                </el-button>
                                <template #dropdown>
                                    <el-dropdown-menu>
                                        <el-dropdown-item :command="`up-${index}`" :disabled="index === 0">
                                            <el-icon>
                                                <ArrowUp />
                                            </el-icon>
                                            上移
                                        </el-dropdown-item>
                                        <el-dropdown-item :command="`down-${index}`"
                                            :disabled="index === editList.length - 1">
                                            <el-icon>
                                                <ArrowDown />
                                            </el-icon>
                                            下移
                                        </el-dropdown-item>
                                        <el-dropdown-item :command="`move-to-top-${index}`" :disabled="index === 0"
                                            divided>
                                            <el-icon>
                                                <Top />
                                            </el-icon>
                                            移到顶部
                                        </el-dropdown-item>
                                        <el-dropdown-item :command="`move-to-bottom-${index}`"
                                            :disabled="index === editList.length - 1">
                                            <el-icon>
                                                <Bottom />
                                            </el-icon>
                                            移到底部
                                        </el-dropdown-item>
                                        <el-dropdown-item :command="`insert-above-${index}`" divided>
                                            <el-icon>
                                                <Plus />
                                            </el-icon>
                                            在上面插入
                                        </el-dropdown-item>
                                        <el-dropdown-item :command="`insert-below-${index}`">
                                            <el-icon>
                                                <Plus />
                                            </el-icon>
                                            在下面插入
                                        </el-dropdown-item>
                                        <el-dropdown-item :command="`delete-${index}`" divided>
                                            <el-icon>
                                                <Delete />
                                            </el-icon>
                                            删除
                                        </el-dropdown-item>
                                    </el-dropdown-menu>
                                </template>
                            </el-dropdown>
                        </div>
                    </div>
                </div>
            </template>
            <template v-else>
                {{ envVar.value }}
            </template>
        </div>
    </div>
</template>

<script setup>
import { Edit, Delete, MoreFilled, ArrowUp, ArrowDown, Plus, Top, Bottom } from '@element-plus/icons-vue'
import { computed, ref, watch } from 'vue'
import { ElMessage, ElAlert } from 'element-plus'

const props = defineProps({
    envVar: {
        type: Object,
        required: true
    }
})
const emit = defineEmits(['edit', 'delete'])

const pathList = computed(() => {
    if (props.envVar.name === 'Path') {
        return props.envVar.value.split(';').filter(Boolean)
    }
    return []
})

// Path 编辑相关
const editingPath = ref(false)
const editList = ref([])

// 跟踪编辑内容是否有变动
const isDirty = ref(false)

// 处理下拉菜单命令
const handleCommand = (command) => {
    console.log('🎯 收到菜单命令:', command)

    if (command.startsWith('up-')) {
        const index = parseInt(command.replace('up-', ''))
        moveUp(index)
    } else if (command.startsWith('down-')) {
        const index = parseInt(command.replace('down-', ''))
        moveDown(index)
    } else if (command.startsWith('move-to-top-')) {
        const index = parseInt(command.replace('move-to-top-', ''))
        moveToTop(index)
    } else if (command.startsWith('move-to-bottom-')) {
        const index = parseInt(command.replace('move-to-bottom-', ''))
        moveToBottom(index)
    } else if (command.startsWith('insert-above-')) {
        const index = parseInt(command.replace('insert-above-', ''))
        insertAbove(index)
    } else if (command.startsWith('insert-below-')) {
        const index = parseInt(command.replace('insert-below-', ''))
        insertBelow(index)
    } else if (command.startsWith('delete-')) {
        const index = parseInt(command.replace('delete-', ''))
        removePathItem(index)
    }
}

// 上移
const moveUp = (index) => {
    if (index > 0) {
        const item = editList.value[index]
        editList.value.splice(index, 1)
        editList.value.splice(index - 1, 0, item)
        console.log('⬆️ 上移项目:', index)
    }
}

// 下移
const moveDown = (index) => {
    if (index < editList.value.length - 1) {
        const item = editList.value[index]
        editList.value.splice(index, 1)
        editList.value.splice(index + 1, 0, item)
        console.log('⬇️ 下移项目:', index)
    }
}

// 移到顶部
const moveToTop = (index) => {
    if (index > 0) {
        const item = editList.value[index]
        editList.value.splice(index, 1)
        editList.value.unshift(item)
        console.log('🔝 移到顶部:', index)
    }
}

// 移到底部
const moveToBottom = (index) => {
    if (index < editList.value.length - 1) {
        const item = editList.value[index]
        editList.value.splice(index, 1)
        editList.value.push(item)
        console.log('🔽 移到底部:', index)
    }
}

// 在上面插入
const insertAbove = (index) => {
    editList.value.splice(index, 0, '')
    console.log('⬆️➕ 在上面插入:', index)
}

// 在下面插入
const insertBelow = (index) => {
    editList.value.splice(index + 1, 0, '')
    console.log('⬇️➕ 在下面插入:', index)
    isDirty.value = true
}

// 移除Path项目
const removePathItem = (index) => {
    editList.value.splice(index, 1)
    console.log('🗑️ 删除项目:', index)
    isDirty.value = true
}

function startEditPath() {
    editList.value = [...pathList.value]
    editingPath.value = true
    console.log('📝 开始编辑Path:', editList.value)
    isDirty.value = false
}

function cancelEditPath() {
    editingPath.value = false
    editList.value = []
    console.log('❌ 取消编辑Path')
    isDirty.value = false
}

function addPath() {
    editList.value.push('')
    console.log('➕ 添加新路径')
    isDirty.value = true
}

async function savePath() {
    const newValue = editList.value.filter(Boolean).join(';')
    emit('edit', { ...props.envVar, value: newValue })
    editingPath.value = false
    console.log('💾 保存Path:', newValue)
    isDirty.value = false
}

// 如果外部 Path 变化，自动退出编辑
watch(() => props.envVar.value, () => {
    if (editingPath.value) {
        editingPath.value = false
        console.log('🔄 外部Path变化，退出编辑模式')
        isDirty.value = false
    }
})

// 监听 editList 变化，判断是否有未保存更改
watch(editList, (newVal) => {
    if (!editingPath.value) return
    // 只要内容和原始 pathList 不一致就提示
    isDirty.value = newVal.join(';') !== pathList.value.join(';')
}, { deep: true })
</script>

<style lang="scss" scoped>
@use '../assets/styles/variables.scss' as *;

.path-list-clickable {
    transition: background 0.2s;
    border-radius: 6px;

    &:hover {
        background: rgba(79, 70, 229, 0.06); // 淡淡的紫色悬浮
    }
}

// Path变量分行样式
.path-list {
    padding-left: 1.2em;
    margin: 0;
    max-height: 250px;
    overflow-y: auto;

    li {
        font-size: var(--font-size-small);
        color: var(--el-text-color-regular);
        word-break: break-all;
        line-height: 1.5;
        margin-bottom: 2px;
        display: flex;
        align-items: center;
        gap: 8px;
        padding: 4px;
        border-radius: 4px;
        transition: all 0.2s;
    }
}



.path-edit-item {
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 4px;

    .input-wrapper {
        flex: 1;
    }

    .menu-btn {
        color: #6b7280;
        padding: 4px;

        &:hover {
            color: #4f46e5;
            background: rgba(79, 70, 229, 0.1);
        }
    }
}

:deep(.el-input .el-input__wrapper) {
    border: none !important;
}

.env-var-card {
    @include card-style;
    padding: var(--spacing-md);
    margin-bottom: var(--spacing-sm);

    .card-header {
        @include flex-between;
        margin-bottom: var(--spacing-sm);

        .var-name {
            font-weight: var(--font-weight-primary);
            color: var(--el-text-color-primary);
            font-size: var(--font-size-base);
            word-break: break-word;
            min-width: 120px;
            flex-shrink: 0;
        }

        .card-actions {
            display: flex;
            gap: var(--spacing-xs);
            flex-shrink: 0;
        }
    }

    .var-value {
        color: var(--el-text-color-regular);
        font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
        font-size: var(--font-size-small);
        word-break: break-all;
        white-space: pre-line;
        background-color: var(--el-fill-color-extra-light);
        padding: var(--spacing-sm) var(--spacing-md);
        border-radius: var(--border-radius-small);
        border: 1px solid var(--el-border-color-extra-light);
        margin-left: var(--spacing-xs);
        line-height: 1.4;
    }

    // 响应式设计
    @include respond-to('xs') {
        .card-header {
            flex-direction: column;
            align-items: flex-start;
            gap: var(--spacing-sm);

            .var-name {
                min-width: auto;
            }

            .card-actions {
                align-self: flex-end;
            }
        }

        .var-value {
            margin-left: 0;
        }
    }
}
</style>