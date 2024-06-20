<template>
  <component :is="currentLayout">
    <router-view :key="$route.path" />
  </component>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import EditorLayout from "./layouts/EditorLayout.vue";
import SetupLayout from "./layouts/SetupLayout.vue";

const route = useRoute();
const router = useRouter();
const isSetupWindow = ref(false);

// 获取当前窗口的标签
const getWindowLabel = () => {
  if (typeof window !== 'undefined' && (window as any).__TAURI__) {
    return (window as any).__TAURI__.window.label;
  }
  return '';
};

onMounted(() => {
  const windowLabel = getWindowLabel();
  
  // 如果是 setup 窗口，保持在根路径
  if (windowLabel === 'setup') {
    isSetupWindow.value = true;
    if (route.path !== '/') {
      router.push('/');
    }
    return;
  }

  // 编辑器窗口：从 URL 查询参数获取项目路径，或者从 localStorage 获取
  const queryParams = new URLSearchParams(window.location.search);
  let projectPath = queryParams.get('path');
  
  if (projectPath) {
    // 如果 URL 中有项目路径，保存到 localStorage
    localStorage.setItem('projectPath', projectPath);
  } else {
    // 如果没有，从 localStorage 获取
    projectPath = localStorage.getItem('projectPath');
  }
  
  // 如果没有项目路径，重定向到根路径
  if (!projectPath && route.path !== '/') {
    router.push('/');
  }
});

// 根据路由和窗口类型决定当前布局
const currentLayout = computed(() => {
  // 如果是 setup 窗口，总是使用 SetupLayout
  if (isSetupWindow.value) {
    return SetupLayout;
  }
  
  // 根路径使用 SetupLayout，其他路径使用 EditorLayout
  if (route.path === '/') {
    return SetupLayout;
  }
  return EditorLayout;
});
</script>

<style scoped>
/* 全局样式已在布局组件中定义 */
</style>
