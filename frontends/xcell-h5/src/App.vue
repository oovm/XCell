<template>
  <component :is="currentLayout">
    <router-view :key="$route.path" />
  </component>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useProjectStore } from "./stores/project";
import EditorLayout from "./layouts/EditorLayout.vue";
import SetupLayout from "./layouts/SetupLayout.vue";

const route = useRoute();
const router = useRouter();
const projectStore = useProjectStore();
const isSetupWindow = ref(false);

/** 获取当前窗口的标签 */
const getWindowLabel = () => {
  if (typeof window !== 'undefined' && (window as any).__TAURI__) {
    return (window as any).__TAURI__.window.label;
  }
  return '';
};

onMounted(() => {
  const windowLabel = getWindowLabel();

  if (windowLabel === 'setup') {
    isSetupWindow.value = true;
    if (route.path !== '/') {
      router.push('/');
    }
    return;
  }

  const queryParams = new URLSearchParams(window.location.search);
  const pathFromUrl = queryParams.get('path');

  if (pathFromUrl) {
    projectStore.setProjectPath(pathFromUrl);
  } else {
    projectStore.restoreProjectPath();
  }

  if (!projectStore.projectPath && route.path !== '/') {
    router.push('/');
  }
});

/** 根据路由和窗口类型决定当前布局 */
const currentLayout = computed(() => {
  if (isSetupWindow.value) {
    return SetupLayout;
  }

  if (route.path === '/') {
    return SetupLayout;
  }
  return EditorLayout;
});
</script>

<style scoped>
/* 全局样式已在布局组件中定义 */
</style>
