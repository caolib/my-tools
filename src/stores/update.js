import { defineStore } from 'pinia'

export const useUpdateStore = defineStore('update', {
    state: () => ({
        hasUpdate: false,
        updateInfo: null,
        checking: false,
        downloading: false,
        downloadProgress: 0,
        error: null
    }),

    actions: {
        setHasUpdate(value) {
            this.hasUpdate = value
        },

        setUpdateInfo(info) {
            this.updateInfo = info
        },

        setChecking(value) {
            this.checking = value
        },

        setDownloading(value) {
            this.downloading = value
        },

        setDownloadProgress(progress) {
            this.downloadProgress = progress
        },

        setError(error) {
            this.error = error
        },

        reset() {
            this.hasUpdate = false
            this.updateInfo = null
            this.checking = false
            this.downloading = false
            this.downloadProgress = 0
            this.error = null
        }
    },

    persist: false // 更新状态不需要持久化
})
