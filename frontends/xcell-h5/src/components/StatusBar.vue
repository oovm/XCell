<template>
  <footer class="status-bar">
    <div class="status-item">XCell Editor</div>
    <div class="status-right">
      <div class="status-item">Ready</div>
      <div class="mcp-indicator" @click="navigateToMcp" title="MCP Settings">
        <div class="mcp-light"></div>
        <span class="mcp-text">MCP</span>
      </div>
    </div>
  </footer>
</template>

<script setup lang="ts">
import { useRouter } from "vue-router";

const router = useRouter();

// 导航到 MCP 设置页面
const navigateToMcp = () => {
  router.push("/setting/mcp");
};
</script>

<style scoped>
.status-bar {
  height: 24px;
  background: #f8f9fa;
  border-top: 1px solid #e0e0e0;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 16px;
  font-size: 12px;
  color: #666666;
}

.status-right {
  display: flex;
  align-items: center;
  gap: 12px;
}

.status-item {
  padding: 0 8px;
}

/* MCP 指示器 */
.mcp-indicator {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 0 8px;
  cursor: pointer;
  border-radius: 4px;
  transition: background-color 0.2s;
}

.mcp-indicator:hover {
  background-color: rgba(0, 0, 0, 0.05);
}

/* MCP 呼吸灯 */
.mcp-light {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background-color: #409EFF;
  animation: breathe 2s infinite ease-in-out;
}

/* 呼吸动画 */
@keyframes breathe {
  0%, 100% {
    opacity: 0.3;
    transform: scale(0.8);
  }
  50% {
    opacity: 1;
    transform: scale(1);
  }
}

.mcp-text {
  font-size: 12px;
  color: #666666;
}
</style>