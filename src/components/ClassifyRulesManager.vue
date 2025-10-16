<template>
    <el-dialog v-model="dialogVisible" title="自动分类规则管理" width="80vw">
        <div class="classify-rules-manager">
            <el-tabs v-model="activeTab">
                <el-tab-pane v-for="type in commitTypes" :key="type.value" :name="type.value">
                    <template #label>
                        <span>{{ type.icon }} {{ type.value }}</span>
                    </template>
                    <div class="rule-editor">
                        <el-form label-width="100px">
                            <el-form-item label="以...开始">
                                <el-select v-model="rules[type.value].startsWith" multiple filterable allow-create
                                    default-first-option placeholder="输入关键词后按回车添加" style="width: 100%">
                                    <el-option v-for="item in rules[type.value].startsWith" :key="item" :label="item"
                                        :value="item" />
                                </el-select>
                                <div class="help-text">当描述以这些关键词开始时，自动分类为此类型</div>
                            </el-form-item>

                            <el-form-item label="包含...">
                                <el-select v-model="rules[type.value].contains" multiple filterable allow-create
                                    default-first-option placeholder="输入关键词后按回车添加" style="width: 100%">
                                    <el-option v-for="item in rules[type.value].contains" :key="item" :label="item"
                                        :value="item" />
                                </el-select>
                                <div class="help-text">当描述包含这些关键词时，自动分类为此类型</div>
                            </el-form-item>

                            <el-form-item label="以...结尾">
                                <el-select v-model="rules[type.value].endsWith" multiple filterable allow-create
                                    default-first-option placeholder="输入关键词后按回车添加" style="width: 100%">
                                    <el-option v-for="item in rules[type.value].endsWith" :key="item" :label="item"
                                        :value="item" />
                                </el-select>
                                <div class="help-text">当描述以这些关键词结尾时，自动分类为此类型</div>
                            </el-form-item>
                        </el-form>
                    </div>
                </el-tab-pane>
            </el-tabs>
        </div>

        <template #footer>
            <div style="display: flex; justify-content: space-between; width: 100%;">
                <el-button @click="handleReset">重置为默认</el-button>
                <div>
                    <el-button @click="dialogVisible = false">取消</el-button>
                    <el-button type="primary" @click="handleSave">保存</el-button>
                </div>
            </div>
        </template>
    </el-dialog>
</template>

<script setup>
import { ref, computed, watch } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useSettingsStore } from '@/stores/settings'
import { useCommitTypesStore } from '@/stores/commitTypes'

const settingsStore = useSettingsStore()
const commitTypesStore = useCommitTypesStore()
const dialogVisible = ref(false)
const activeTab = ref('feat')

const commitTypes = computed(() => commitTypesStore.allCommitTypes)

// 规则数据
const rules = ref({})

// 初始化规则数据
const initRules = () => {
    const savedRules = settingsStore.commitGenerator.classifyRules
    rules.value = {}

    commitTypes.value.forEach(type => {
        if (savedRules[type.value]) {
            rules.value[type.value] = {
                startsWith: [...savedRules[type.value].startsWith],
                contains: [...savedRules[type.value].contains],
                endsWith: [...savedRules[type.value].endsWith]
            }
        } else {
            rules.value[type.value] = {
                startsWith: [],
                contains: [],
                endsWith: []
            }
        }
    })
}

// 显示对话框
const showDialog = () => {
    initRules()
    dialogVisible.value = true
    activeTab.value = commitTypes.value[0]?.value || 'feat'
}

// 保存规则
const handleSave = () => {
    settingsStore.setClassifyRules(rules.value)
    ElMessage.success('自动分类规则已保存')
    dialogVisible.value = false
}

// 重置为默认规则
const handleReset = () => {
    ElMessageBox.confirm(
        '确定要重置为默认自动分类规则吗？这将删除所有自定义规则。',
        '确认重置',
        {
            confirmButtonText: '确定',
            cancelButtonText: '取消',
            type: 'warning'
        }
    ).then(() => {
        settingsStore.resetClassifyRules()
        initRules()
        ElMessage.success('已重置为默认规则')
    }).catch(() => {
        // 取消重置
    })
}

// 监听提交类型变化，更新规则
watch(() => commitTypes.value, () => {
    if (dialogVisible.value) {
        initRules()
    }
}, { deep: true })

defineExpose({
    showDialog
})
</script>

<style scoped>
.classify-rules-manager {
    min-height: 400px;
}

.rule-editor {
    padding: 10px;
}

.help-text {
    font-size: 12px;
    color: var(--el-text-color-secondary);
    margin-top: 5px;
}
</style>
