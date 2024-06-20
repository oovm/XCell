<template>
  <div class="test-navigation">
    <h3>测试导航</h3>
    <div>
      <button @click="navigate('/project')">项目</button>
      <button @click="navigate('/table')">表格</button>
      <button @click="navigate('/edit')">编辑</button>
      <button @click="navigate('/artifact')">产物</button>
      <button @click="navigate('/chat')">聊天</button>
      <button @click="navigate('/setting')">设置</button>
    </div>
    <div>
      <p>当前路由: {{ route.path }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRoute, useRouter } from "vue-router";

const route = useRoute();
const router = useRouter();

const navigate = (path: string) => {
  console.log('Testing navigation to:', path);
  router.push(path);
  console.log('Navigation to:', path);
};
</script>

<style scoped>
.test-navigation {
  padding: 20px;
  background: #f0f0f0;
  margin: 20px;
  border-radius: 8px;
}

test-navigation button {
  margin: 5px;
  padding: 10px;
  border: 1px solid #ccc;
  border-radius: 4px;
  cursor: pointer;
}

test-navigation button:hover {
  background: #e0e0e0;
}
</style>