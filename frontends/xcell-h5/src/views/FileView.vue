<template>
  <div class="file-view-content">
    <div class="empty-state">
      <el-icon class="empty-icon"><Folder /></el-icon>
      <p>文件视图</p>
      <p class="empty-subtitle">在这里搜索和管理项目文件</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Folder } from "@element-plus/icons-vue";
</script>

<style scoped>
.empty-state {
  text-align: center;
}

.empty-icon {
  font-size: 48px;
  opacity: 0.5;
  margin-bottom: 16px;
  color: #17a2b8;
}

.empty-subtitle {
  font-size: 14px;
  opacity: 0.7;
  margin-top: 8px;
}
</style>