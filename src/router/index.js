import { createRouter, createWebHashHistory } from 'vue-router'
import { useSettingsStore } from '@/stores/settings'

const routes = [
    {
        path: '/env',
        name: 'EnvVarManager',
        component: () => import('../views/EnvVarManager.vue')
    },
    {
        path: '/file-search',
        name: 'FileSearch',
        component: () => import('../views/FileSearch.vue')
    },
    {
        path: '/',
        name: 'Home',
        redirect: () => {
            // 根据设置中保存的当前路由来决定默认页面
            const settingsStore = useSettingsStore()
            const currentRoute = settingsStore.currentRoute || 'FileSearch'

            if (currentRoute === 'EnvVarManager') {
                return '/env'
            } else {
                return '/file-search'
            }
        }
    }
]

const router = createRouter({
    history: createWebHashHistory(),
    routes
})

// 添加路由守卫来保存当前路由状态
router.afterEach((to) => {
    const settingsStore = useSettingsStore()
    if (to.name && to.name !== 'Home') {
        settingsStore.setCurrentRoute(to.name)
    }
})

export default router
