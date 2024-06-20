<template>
  <div class="file-tree-container">
    <div class="panel-header">
      <h3>表格</h3>
    </div>
    <div class="file-tree">
      <el-tree
        :data="fileTreeData"
        :props="treeProps"
        node-key="id"
        default-expand-all
        @node-click="handleNodeClick"
        class="xcell-file-tree"
      >
        <template #default="{ data }">
          <div class="tree-node-content">
            <el-icon v-if="data.type === 'folder'" class="node-icon"><Folder /></el-icon>
            <el-icon v-else-if="data.type === 'table'" class="node-icon table-icon"><Document /></el-icon>
            <span class="node-label">{{ data.name }}</span>
          </div>
        </template>
      </el-tree>
    </div>
    
    <div class="panel-section">
      <div class="section-title">最近打开</div>
      <div class="history-list">
        <div 
          v-for="item in projectHistory" 
          :key="item.id"
          class="history-item"
          @click="openHistoryProject(item.path)"
        >
          <div class="history-item-info">
            <div class="history-item-name">{{ item.name }}</div>
            <div class="history-item-time">
              <el-icon class="time-icon"><Clock /></el-icon>
              {{ formatTime(item.lastOpened) }}
            </div>
          </div>
        </div>
        <div v-if="projectHistory.length === 0" class="no-history">
          暂无历史记录
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Clock, Document, Folder } from "@element-plus/icons-vue";
import { onMounted, ref } from "vue";

// 定义事件
const emit = defineEmits(["node-click", "open-project"]);

// 文件树数据结构
interface FileTreeNode {
	id: string;
	name: string;
	type: "folder" | "file" | "table";
	path: string;
	children?: FileTreeNode[];
}

// 项目历史记录类型定义
interface ProjectHistory {
	id: string;
	name: string;
	path: string;
	lastOpened: string;
}

// 文件树数据
const fileTreeData = ref<FileTreeNode[]>([
	{
		id: "1",
		name: "表格",
		type: "folder",
		path: "/tables",
		children: [
			{
				id: "2",
				name: "users.xcell",
				type: "table",
				path: "/tables/users.xcell",
			},
			{
				id: "3",
				name: "products.xcell",
				type: "table",
				path: "/tables/products.xcell",
			},
			{
				id: "4",
				name: "orders.xcell",
				type: "table",
				path: "/tables/orders.xcell",
			},
		],
	},
]);

// 树节点属性配置
const treeProps = {
	children: "children",
	label: "name",
};

// 历史记录数据
const projectHistory = ref<ProjectHistory[]>([]);

// 加载历史记录
const loadHistory = () => {
	const savedHistory = localStorage.getItem("xcell-project-history");
	if (savedHistory) {
		projectHistory.value = JSON.parse(savedHistory);
		// 按最后打开时间排序
		projectHistory.value.sort(
			(a, b) =>
				new Date(b.lastOpened).getTime() - new Date(a.lastOpened).getTime(),
		);
	}
};

// 处理树节点点击
const handleNodeClick = (data: FileTreeNode) => {
	emit("node-click", data);
};

// 打开历史项目
const openHistoryProject = (path: string) => {
	emit("open-project", path);
};

// 格式化时间
const formatTime = (timeString: string) => {
	const date = new Date(timeString);
	return date.toLocaleString();
};

// 组件挂载时加载历史记录
onMounted(() => {
	loadHistory();
});
</script>

<style scoped>
.file-tree-container {
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding: 16px;
}

.panel-header {
  padding-bottom: 8px;
  border-bottom: 1px solid #3e3e42;
}

.panel-header h3 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: #d4d4d4;
  text-transform: uppercase;
}

.file-tree {
  flex: 1;
  overflow: auto;
}

.panel-section {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.section-title {
  font-size: 12px;
  font-weight: 600;
  color: #858585;
  margin-bottom: 8px;
  text-transform: uppercase;
}

.tree-node-content {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 2px 0;
}

.node-icon {
  font-size: 16px;
  color: #6a9955;
}

.table-icon {
  color: #4ec9b0;
}

.node-label {
  font-size: 13px;
  color: #d4d4d4;
  flex: 1;
}

.history-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-height: 200px;
  overflow-y: auto;
}

.history-item {
  padding: 8px;
  border-radius: 4px;
  background: #2d2d30;
  cursor: pointer;
  transition: background 0.2s;
}

.history-item:hover {
  background: rgba(255, 255, 255, 0.1);
}

.history-item-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.history-item-name {
  font-size: 13px;
  font-weight: 500;
  color: #d4d4d4;
}

.history-item-time {
  font-size: 11px;
  color: #6a9955;
  display: flex;
  align-items: center;
  gap: 4px;
}

.time-icon {
  font-size: 12px;
}

.no-history {
  font-size: 12px;
  color: #858585;
  padding: 8px;
  text-align: center;
}

/* 滚动条样式 */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: #1e1e1e;
}

::-webkit-scrollbar-thumb {
  background: #424242;
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: #4e4e4e;
}
</style>