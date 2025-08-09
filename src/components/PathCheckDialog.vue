<template>
    <el-dialog v-model="visible" :title="title" width="70vw" :before-close="handleClose" class="path-check-dialog">
        <div class="dialog-content">
            <!-- 统计信息 -->
            <div v-if="stats" class="stats-section">
                <p class="section-title">{{ statsTitle }}：</p>
                <div class="stats-tags">
                    <el-tag v-for="stat in stats" :key="stat.label" :type="stat.type" size="small">
                        {{ stat.label }}: {{ stat.count }}个
                    </el-tag>
                </div>
            </div>

            <!-- 路径列表 -->
            <div class="paths-section">
                <p class="section-title">{{ pathsTitle }}：</p>
                <div class="paths-container">
                    <div v-for="(item, index) in pathItems" :key="index" class="path-item">
                        <el-icon v-if="item.icon" class="path-icon" :class="item.iconClass">
                            <component :is="item.icon" />
                        </el-icon>
                        <div class="path-content">
                            <span v-if="item.clickable" class="path-text clickable" @click="handlePathClick(item.path)"
                                :title="item.path">
                                {{ item.path }}
                            </span>
                            <span v-else class="path-text" :title="item.path">
                                {{ item.path }}
                            </span>
                        </div>
                        <el-tag v-if="item.tag" :type="item.tag.type" size="small">
                            {{ item.tag.text }}
                        </el-tag>
                    </div>
                </div>
            </div>

            <!-- 确认信息 -->
            <p class="confirm-text">{{ confirmText }}</p>
        </div>

        <template #footer>
            <div class="dialog-footer">
                <el-button @click="handleCancel">取消</el-button>
                <el-button type="primary" @click="handleConfirm">确定</el-button>
            </div>
        </template>
    </el-dialog>
</template>

<script setup>
import { ref, watch } from 'vue'
import { ElMessage } from 'element-plus'
import { Warning } from '@element-plus/icons-vue'
import { openPath } from '@tauri-apps/plugin-opener'

const props = defineProps({
    modelValue: {
        type: Boolean,
        default: false
    },
    title: {
        type: String,
        required: true
    },
    statsTitle: {
        type: String,
        default: ''
    },
    stats: {
        type: Array,
        default: () => []
    },
    pathsTitle: {
        type: String,
        required: true
    },
    pathItems: {
        type: Array,
        required: true
    },
    confirmText: {
        type: String,
        required: true
    }
})

const emit = defineEmits(['update:modelValue', 'confirm', 'cancel'])

const visible = ref(false)

watch(() => props.modelValue, (val) => {
    visible.value = val
})

watch(visible, (val) => {
    emit('update:modelValue', val)
})

const handleClose = () => {
    visible.value = false
}

const handleCancel = () => {
    emit('cancel')
    visible.value = false
}

const handleConfirm = () => {
    emit('confirm')
    visible.value = false
}

const handlePathClick = async (path) => {
    try {
        console.log('尝试打开路径:', path)
        await openPath(path)
        ElMessage.success(`已在文件管理器中打开: ${path}`)
    } catch (error) {
        console.error('打开路径失败:', error)
        ElMessage.error(`无法打开路径: ${path}`)
    }
}
</script>

<style lang="scss" scoped src="../assets/styles/path-check-dialog.scss"></style>
