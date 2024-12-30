import { createRouter, createWebHistory } from 'vue-router'
import SearchPasswords from '../components/SearchPasswords.vue'
import AddPassword from '../components/AddPassword.vue'

const routes = [
  {
    path: '/',
    name: 'search',
    component: SearchPasswords
  },
  {
    path: '/add',
    name: 'add',
    component: AddPassword
  }
]

export const router = createRouter({
  history: createWebHistory(),
  routes
})
