import { defineStore } from 'pinia'
import defaultCommitTypes from '@/assets/config/commit.json'

export const useCommitTypesStore = defineStore('commitTypes', {
    state: () => ({
        commitTypes: [...defaultCommitTypes]
    }),

    getters: {
        // 获取所有提交类型
        allCommitTypes: (state) => state.commitTypes,

        // 根据 value 获取提交类型
        getCommitTypeByValue: (state) => (value) => {
            return state.commitTypes.find(type => type.value === value)
        },

        // 检查提交类型是否存在
        hasCommitType: (state) => (value) => {
            return state.commitTypes.some(type => type.value === value)
        }
    },

    actions: {
        // 添加自定义提交类型
        addCommitType(value, label, icon) {
            // 检查是否已存在
            if (this.hasCommitType(value)) {
                return { success: false, message: '该提交类型已存在' }
            }

            this.commitTypes.push({
                value,
                label,
                icon,
                isCustom: true
            })

            return { success: true, message: '添加成功' }
        },

        // 更新提交类型
        updateCommitType(value, label, icon) {
            const index = this.commitTypes.findIndex(type => type.value === value)
            if (index === -1) {
                return { success: false, message: '提交类型不存在' }
            }

            this.commitTypes[index] = {
                ...this.commitTypes[index],
                label,
                icon
            }

            return { success: true, message: '更新成功' }
        },

        // 删除自定义提交类型
        deleteCommitType(value) {
            const index = this.commitTypes.findIndex(type => type.value === value)
            if (index === -1) {
                return { success: false, message: '提交类型不存在' }
            }

            // 只能删除自定义类型
            if (!this.commitTypes[index].isCustom) {
                return { success: false, message: '不能删除默认提交类型' }
            }

            this.commitTypes.splice(index, 1)
            return { success: true, message: '删除成功' }
        },

        // 重置为默认提交类型
        resetToDefault() {
            this.commitTypes = [...defaultCommitTypes]
        }
    },

    persist: {
        storage: localStorage,
        key: 'wem-commit-types'
    }
})
