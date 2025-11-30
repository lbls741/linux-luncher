<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const isLoading = ref(true);
const showInitScreen = ref(false);

onMounted(async () => {
  try {
    const needsInit = await invoke<boolean>('check_config_and_initialize');
    showInitScreen.value = needsInit;
  } catch (e) {
    console.error("Failed to check config:", e);
    // On error, default to the main screen to avoid getting stuck.
    showInitScreen.value = false;
  } finally {
    isLoading.value = false;
  }
});

function enterMainApp() {
  showInitScreen.value = false;
}

async function handlePrepare() {
  try {
    await invoke('get_prepare');
    alert('Preparation complete!');
  } catch (e) {
    console.error("Failed to prepare:", e);
    alert(`Preparation failed: ${e}`);
  }
}
</script>

<template>
  <div v-if="isLoading" class="container">
    <p>Loading...</p>
  </div>

  <div v-else-if="showInitScreen" class="container">
    <h1>Welcome to Luncher</h1>
    <p>It looks like this is your first time running the application.</p>
    <p>A configuration file has been created for you.</p>
    <button @click="enterMainApp" style="margin-top: 20px;">Enter Main Application</button>
  </div>

  <main v-else class="container">
    <h1>Linux Luncher</h1>
    <p>请点击下方的按钮来准备您的运行环境。</p>
    <button @click="handlePrepare" style="margin-top: 10px;">Prepare Environment</button>
  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}
</style>
