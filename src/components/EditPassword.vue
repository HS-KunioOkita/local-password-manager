<template>
  <div class="edit-password">
    <h2>パスワード編集</h2>
    <div v-if="loading">Loading...</div>
    <form v-else @submit.prevent="updatePassword">
      <div class="form-group">
        <label>名前:</label>
        <input v-model="entry.name" placeholder="名前" required>
      </div>
      <div class="form-group">
        <label>ログインID:</label>
        <input v-model="entry.login_id" placeholder="ログインID" required>
      </div>
      <div class="form-group">
        <label>パスワード:</label>
        <div class="password-input-group">
          <input 
            v-model="entry.password" 
            :type="showPassword ? 'text' : 'password'" 
            placeholder="パスワード" 
            required
          >
          <button 
            type="button" 
            class="toggle-password" 
            @click="togglePassword"
          >
            {{ showPassword ? '非表示' : '表示' }}
          </button>
        </div>
      </div>
      <div class="form-group">
        <label>URL:</label>
        <input v-model="entry.url" placeholder="URL (オプション)">
      </div>
      <div class="button-group">
        <button type="submit">更新</button>
        <button type="button" @click="router.push('/')">キャンセル</button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useRouter, useRoute } from 'vue-router'

const router = useRouter()
const route = useRoute()
const loading = ref(true)
const showPassword = ref(false)
const entry = ref({
  id: null,
  name: '',
  login_id: '',
  password: '',
  url: ''
})

const togglePassword = () => {
  showPassword.value = !showPassword.value
}

onMounted(async () => {
  const passwordId = parseInt(route.params.id as string)
  const password = await invoke('get_password', { id: passwordId })
  entry.value = password
  loading.value = false
})

const updatePassword = async () => {
  await invoke('update_password', { entry: entry.value })
  router.push('/')
}
</script>

<style scoped>
.edit-password {
  max-width: 500px;
  margin: 0 auto;
  padding: 20px;
}

.form-group {
  margin-bottom: 15px;
}

label {
  display: block;
  margin-bottom: 5px;
  font-weight: bold;
}

input {
  width: 100%;
  padding: 8px;
  border: 1px solid #ddd;
  border-radius: 4px;
}

.button-group {
  display: flex;
  gap: 10px;
  justify-content: flex-end;
  margin-top: 20px;
}

button {
  padding: 8px 16px;
  border-radius: 4px;
  cursor: pointer;
}

button[type="submit"] {
  background-color: #4CAF50;
  color: white;
  border: none;
}

button[type="button"] {
  background-color: #f44336;
  color: white;
  border: none;
}

/* 既存のスタイルはそのままに、以下を追加 */
.password-input-group {
  display: flex;
  gap: 8px;
}

.password-input-group input {
  flex: 1;
}

.toggle-password {
  padding: 8px 12px;
  background-color: #666;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.toggle-password:hover {
  background-color: #777;
}
</style>
