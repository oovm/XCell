<template>
  <div class="mcp-setting-view-content">
    <div class="empty-state">
      <el-icon class="empty-icon"><Setting /></el-icon>
      <p>MCP 设置</p>
      <p class="empty-subtitle">在这里配置 MCP 相关设置</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Setting } from "@element-plus/icons-vue";
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