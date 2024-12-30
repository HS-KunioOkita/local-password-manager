<template>
  <div class="search-passwords">
    <input v-model="searchQuery" placeholder="検索..." class="search-input">
    
    <div class="password-list">
      <div v-for="password in filteredPasswords" :key="password.id" class="password-item">
        <h3>{{ password.name }}</h3>
        <p>ログインID: {{ password.login_id }}</p>
        <p>パスワード: ******* </p>
        <p v-if="password.url">URL: {{ password.url }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const passwords = ref([])
const searchQuery = ref('')

const filteredPasswords = computed(() => {
  return passwords.value.filter(password => 
    password.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
    password.login_id.toLowerCase().includes(searchQuery.value.toLowerCase())
  )
})

const loadPasswords = async () => {
  passwords.value = await invoke('get_passwords')
}

onMounted(() => {
  loadPasswords()
})
</script>
