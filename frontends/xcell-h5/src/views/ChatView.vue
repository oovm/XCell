<template>
  <div class="chat-view-content">
    <div class="empty-state">
      <el-icon class="empty-icon"><Message /></el-icon>
      <p>聊天视图</p>
      <p class="empty-subtitle">在这里与 AI 助手交流</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Message } from "@element-plus/icons-vue";
</script>

<style scoped>
.empty-state {
  text-align: center;
}

.empty-icon {
  font-size: 48px;
  opacity: 0.5;
  margin-bottom: 16px;
  color: #1890ff;
}

.empty-subtitle {
  font-size: 14px;
  opacity: 0.7;
  margin-top: 8px;
}
</style>