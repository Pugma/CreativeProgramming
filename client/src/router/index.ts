import { createRouter, createWebHistory } from 'vue-router'
import TopPage from '@/views/TopPage.vue'

const router = createRouter({
  routes: [
    {
      path: '/',
      name: 'top',
      component: TopPage
    },
    {
      path: '/me',
      name: 'MyPage',
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import('@/views/MyPage.vue')
    },
    {
      path: '/group/:groupId',
      name: 'GroupPage',
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import('@/views/GroupPage.vue')
    },
    {
      path: '/:pathMatch(.*)*',
      name: 'Not found',
      component: () => import('@/views/NotFound.vue')
    }
  ],
  history: createWebHistory(import.meta.env.BASE_URL)
})

export default router
