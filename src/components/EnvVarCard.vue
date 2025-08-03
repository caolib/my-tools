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
        <div class="var-value">{{ envVar.value }}</div>
    </div>
</template>

<script setup>
import { Edit, Delete } from '@element-plus/icons-vue'

defineProps({
    envVar: {
        type: Object,
        required: true
    }
})

defineEmits(['edit', 'delete'])
</script>

<style lang="scss" scoped>
@use '../assets/styles/variables.scss' as *;

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
