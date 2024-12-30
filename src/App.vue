<template>
  <div class="app">
    <nav>
      <router-link to="/">パスワード一覧</router-link> |
      <router-link to="/add">新規登録</router-link>
    </nav>
    <router-view></router-view>
  </div>
</template>

<style>
.app {
  padding: 20px;
}

nav {
  margin-bottom: 20px;
}

nav a {
  margin-right: 10px;
}
</style>
