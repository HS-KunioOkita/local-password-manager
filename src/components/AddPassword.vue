<template>
  <div class="add-password">
    <h2>新規パスワード登録</h2>
    <form @submit.prevent="savePassword">
      <input v-model="entry.name" placeholder="名前" required>
      <input v-model="entry.login_id" placeholder="ログインID" required>
      <input v-model="entry.password" type="password" placeholder="パスワード" required>
      <input v-model="entry.url" placeholder="URL (オプション)">
      <button type="submit">保存</button>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useRouter } from 'vue-router'

const router = useRouter()
const entry = ref({
  name: '',
  login_id: '',
  password: '',
  url: ''
})

const savePassword = async () => {
  await invoke('save_password', { entry: entry.value })
  router.push('/') // 保存後にトップページに戻る
}
</script>
