import { createRouter, createWebHistory } from 'vue-router'
import OperatorsView from '@/views/OperatorsView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'operators',
      component: OperatorsView,
    },
  ],
})

export default router