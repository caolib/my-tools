import { createRouter, createWebHashHistory } from 'vue-router'

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
        component: () => import('../views/FileSearch.vue')
    }
]

const router = createRouter({
    history: createWebHashHistory(),
    routes
})

export default router
