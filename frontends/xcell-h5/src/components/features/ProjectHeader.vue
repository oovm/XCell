<template>
  <div class="project-header">
    <h1>项目设置</h1>
  </div>
</template>

<script setup lang="ts">
</script>

<style scoped>
.project-header {
  padding: 20px 24px;
  border-bottom: 1px solid #e4e7ed;
  background: #ffffff;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  animation: fadeIn 0.5s ease;
}

.project-header h1 {
  margin: 0;
  font-size: 24px;
  font-weight: 600;
  color: #303133;
}

/* 动画效果 */
@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>