<template>
  <aside class="activity-bar">
    <div 
      class="activity-item" 
      :class="{ active: activeView === 'setup' }"
      @click="router.push('/')"
      title="选择项目"
    >
      <el-icon><SwitchButton /></el-icon>
    </div>
    <div 
      class="activity-item" 
      :class="{ active: activeView === 'project' }"
      @click="router.push('/project')"
      title="项目属性"
    >
      <el-icon><Folder /></el-icon>
    </div>
    <div 
      class="activity-item" 
      :class="{ active: activeView === 'table' }"
      @click="router.push('/table')"
      title="表格属性"
    >
      <el-icon><Document /></el-icon>
    </div>
    <div 
      class="activity-item" 
      :class="{ active: activeView === 'edit' }"
      @click="router.push('/edit')"
      title="表格编辑"
    >
      <el-icon><Edit /></el-icon>
    </div>
    <div 
      class="activity-item" 
      :class="{ active: activeView === 'group' }"
      @click="router.push('/group')"
      title="分包管理"
    >
      <el-icon><Operation /></el-icon>
    </div>
    <div 
      class="activity-item" 
      :class="{ active: activeView === 'artifact' }"
      @click="router.push('/artifact')"
      title="产物视图"
    >
      <el-icon><View /></el-icon>
    </div>
    <div 
      class="activity-item" 
      :class="{ active: activeView === 'chat' }"
      @click="router.push('/chat')"
      title="聊天视图"
    >
      <el-icon><Message /></el-icon>
    </div>
    <div 
      class="activity-item bottom" 
      :class="{ active: activeView === 'setting' }"
      @click="router.push('/setting')"
      title="设置"
    >
      <el-icon><Setting /></el-icon>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { Document, Edit, Folder, View, Message, Setting, Operation, SwitchButton } from "@element-plus/icons-vue";
import { computed } from "vue";
import { useRoute, useRouter } from "vue-router";

const route = useRoute();
const router = useRouter();

// 当前活动视图
const activeView = computed(() => {
	const path = route.path;
	if (path === "/") return "setup";
	if (path === "/project") return "project";
	if (path === "/table") return "table";
	if (path === "/edit") return "edit";
	if (path === "/group") return "group";
	if (path === "/artifact") return "artifact";
	if (path === "/chat") return "chat";
	if (path === "/setting" || path.startsWith("/setting/")) return "setting";
	return "project";
});
</script>

<style scoped>
.activity-bar {
  width: 64px;
  background: #f8f9fa;
  border-right: 1px solid #e0e0e0;
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 12px 0;
  gap: 12px;
  position: relative;
  z-index: 100;
}

.activity-item {
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 6px;
  cursor: pointer;
  color: #666666;
  transition: all 0.2s;
  user-select: none;
  position: relative;
  z-index: 10;
}

.activity-item .icon-wrapper {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
}

.activity-item .el-icon {
  font-size: 20px;
}

.activity-item:hover {
  background: rgba(0, 0, 0, 0.05);
  color: #333333;
}

.activity-item.active {
  background: #409EFF;
  color: #ffffff;
}

.activity-item.bottom {
  margin-top: auto;
}
</style>