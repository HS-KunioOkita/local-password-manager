import { createRouter, createWebHistory } from 'vue-router'
import SearchPasswords from '../components/SearchPasswords.vue'
import AddPassword from '../components/AddPassword.vue'
import EditPassword from '../components/EditPassword.vue'

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
  },
  {
    path: '/edit/:id',
    name: 'edit',
    component: EditPassword
  }
]
export const router = createRouter({
  history: createWebHistory(),
  routes
})
