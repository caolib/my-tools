import { createRouter, createWebHashHistory } from 'vue-router'
import EnvVarManager from '../views/EnvVarManager.vue'

const routes = [
    {
        path: '/',
        name: 'EnvVarManager',
        component: EnvVarManager
    }
]

const router = createRouter({
    history: createWebHashHistory(),
    routes
})

export default router
