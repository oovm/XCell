<template>
  <div class="editor-layout">
    <!-- 顶部区域：活动栏 + 内容 -->
    <div class="top-area">
      <!-- 左侧活动栏 -->
      <ActivityBar />
      
      <!-- 内容区域 -->
      <main class="content-area">
        <slot></slot>
      </main>
    </div>
    
    <!-- 底部状态栏 -->
    <StatusBar />
  </div>
</template>

<script setup lang="ts">
import ActivityBar from "../components/ActivityBar.vue";
import StatusBar from "../components/StatusBar.vue";
</script>

<style>
/* 全局样式重置 */
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: 'PingFang SC', 'Microsoft YaHei', Arial, sans-serif;
  font-size: 14px;
  line-height: 1.5;
  color: #333333;
  background-color: #f5f7fa;
}
</style>

<style scoped>
.editor-layout {
  display: flex;
  flex-direction: column;
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  background-color: #f5f7fa;
}

/* 顶部区域 */
.top-area {
  flex: 1;
  display: flex;
  overflow: hidden;
  background-color: #ffffff;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
}

/* 内容区域 */
.content-area {
  flex: 1;
  display: flex;
  overflow: auto;
  background: #f5f7fa;
  padding: 0;
}
</style>