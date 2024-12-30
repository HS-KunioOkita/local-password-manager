<template>
    <div class="search-passwords">
      <input v-model="searchQuery" placeholder="検索..." class="search-input">
      
      <table class="password-table">
        <thead>
          <tr>
            <th>名前</th>
            <th>ログインID</th>
            <th>パスワード</th>
            <th>URL</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="password in filteredPasswords" :key="password.id">
            <td>{{ password.name }}</td>
            <td>{{ password.login_id }}</td>
            <td>
            {{ visiblePasswordId === password.id ? password.password : '*******' }}
            </td>
            <td>{{ password.url || '-' }}</td>
            <td>
              <button @click="showPassword(password)">表示</button>
              <button @click="copyPassword(password)">コピー</button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </template>
  <script setup lang="ts">
  import { ref, computed, onMounted } from 'vue'
  import { invoke } from '@tauri-apps/api/core'
  
  const passwords = ref([])
  const searchQuery = ref('')
  const visiblePasswordId = ref<number | null>(null)
  
  const filteredPasswords = computed(() => {
    return passwords.value.filter(password => 
      password.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      password.login_id.toLowerCase().includes(searchQuery.value.toLowerCase())
    )
  })
  
  const showPassword = (password: any) => {
    if (visiblePasswordId.value === password.id) {
      visiblePasswordId.value = null
    } else {
      visiblePasswordId.value = password.id
    }
  }
  
  const copyPassword = async (password: any) => {
    await navigator.clipboard.writeText(password.password)
  }
  
  const loadPasswords = async () => {
    passwords.value = await invoke('get_passwords')
  }
  
  onMounted(() => {
    loadPasswords()
  })
  </script>

<style scoped>
  .password-table {
    width: 100%;
    border-collapse: collapse;
    margin-top: 20px;
  }
  
  .password-table th,
  .password-table td {
    border: 1px solid #ddd;
    padding: 8px;
    text-align: left;
  }
  
  .password-table th {
    background-color: #f4f4f4;
  }
  
  .password-table tr:nth-child(even) {
    background-color: #f9f9f9;
  }
  
  .password-table tr:hover {
    background-color: #f5f5f5;
  }
  
  button {
    margin: 0 4px;
    padding: 4px 8px;
  }
  
  .search-input {
    width: 100%;
    padding: 8px;
    margin-bottom: 16px;
    border: 1px solid #ddd;
    border-radius: 4px;
  }
  </style>
  