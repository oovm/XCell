<template>
  <div class="file-view">
    <div class="file-view-header">
      <h2 class="file-view-title">文件浏览器</h2>
      <span v-if="projectStore.projectPath" class="file-view-path">{{ projectStore.projectPath }}</span>
    </div>

    <div class="file-view-search">
      <el-input
        v-model="searchText"
        placeholder="搜索文件名..."
        clearable
        prefix-icon="Search"
        class="search-input"
      />
    </div>

    <div class="file-view-tree-wrapper">
      <el-tree
        v-if="fileTreeData.length > 0"
        ref="treeRef"
        :data="fileTreeData"
        :props="treeProps"
        node-key="fullPath"
        default-expand-all
        :expand-on-click-node="false"
        :filter-node-method="filterNode"
        class="file-tree"
        @node-click="handleNodeClick"
      >
        <template #default="{ data }">
          <div class="tree-node-content">
            <el-icon v-if="data.isDir" class="node-icon folder-icon"><Folder /></el-icon>
            <el-icon v-else class="node-icon file-icon"><Document /></el-icon>
            <span class="node-label" v-html="highlightText(data.label, searchText)" />
          </div>
        </template>
      </el-tree>

      <div v-else class="empty-state">
        <el-icon class="empty-icon"><FolderOpened /></el-icon>
        <p class="empty-text">暂无文件</p>
        <p class="empty-subtitle">请先打开项目以查看文件结构</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from "vue";
import { Document, Folder, FolderOpened } from "@element-plus/icons-vue";
import type ElTree from "element-plus/es/components/tree/index";
import { useProjectStore } from "../stores/project";
import { useTableStore } from "../stores/table";
import type { TableData } from "../services/api";

/** 文件树节点 */
interface FileTreeNode {
  /** 节点标签（文件名或目录名） */
  label: string;
  /** 是否为目录 */
  isDir: boolean;
  /** 子节点 */
  children?: FileTreeNode[];
  /** 完整路径 */
  fullPath: string;
}

/** 类型到目录信息的映射 */
interface TypeDirInfo {
  /** 目录标签 */
  label: string;
  /** 目录路径 */
  dirPath: string;
}

const projectStore = useProjectStore();
const tableStore = useTableStore();

/** 搜索文本 */
const searchText = ref<string>("");

/** el-tree 组件引用 */
const treeRef = ref<InstanceType<typeof ElTree> | null>(null);

/** 类型到目录的映射配置 */
const typeDirMap: Record<string, TypeDirInfo> = {
  table: { label: "tables", dirPath: "/tables" },
  class: { label: "classes", dirPath: "/classes" },
  enum: { label: "enums", dirPath: "/enums" },
  language: { label: "languages", dirPath: "/languages" },
};

/** el-tree 属性配置 */
const treeProps = {
  children: "children",
  label: "label",
};

/** 根据表格列表构建文件树 */
const fileTreeData = computed<FileTreeNode[]>(() => {
  const tables = tableStore.tableList;
  if (tables.length === 0) {
    return [];
  }

  const grouped: Record<string, TableData[]> = {};
  for (const table of tables) {
    const type = table.type || "table";
    if (!grouped[type]) {
      grouped[type] = [];
    }
    grouped[type].push(table);
  }

  const result: FileTreeNode[] = [];
  const typeOrder: string[] = ["table", "class", "enum", "language"];

  for (const type of typeOrder) {
    const items = grouped[type];
    if (!items || items.length === 0) {
      continue;
    }

    const dirInfo = typeDirMap[type] || { label: type, dirPath: `/${type}` };

    const dirNode: FileTreeNode = {
      label: dirInfo.label,
      isDir: true,
      fullPath: dirInfo.dirPath,
      children: items.map((item) => ({
        label: item.name,
        isDir: false,
        fullPath: item.path,
      })),
    };

    result.push(dirNode);
  }

  for (const [type, items] of Object.entries(grouped)) {
    if (typeOrder.includes(type)) {
      continue;
    }

    const dirNode: FileTreeNode = {
      label: type,
      isDir: true,
      fullPath: `/${type}`,
      children: items.map((item) => ({
        label: item.name,
        isDir: false,
        fullPath: item.path,
      })),
    };

    result.push(dirNode);
  }

  return result;
});

/** 过滤树节点 */
function filterNode(value: string, data: FileTreeNode): boolean {
  if (!value) {
    return true;
  }
  return data.label.toLowerCase().includes(value.toLowerCase());
}

/** 高亮匹配文本 */
function highlightText(text: string, query: string): string {
  if (!query) {
    return text;
  }

  const escapedQuery = query.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const regex = new RegExp(`(${escapedQuery})`, "gi");
  return text.replace(regex, '<mark class="search-highlight">$1</mark>');
}

/** 处理树节点点击 */
function handleNodeClick(data: FileTreeNode): void {
  if (!data.isDir) {
    console.log("点击文件:", data.fullPath);
  }
}

/** 监听搜索文本变化，过滤树 */
watch(searchText, (value) => {
  treeRef.value?.filter(value);
});

/** 组件挂载时加载表格列表 */
onMounted(() => {
  tableStore.loadTableList();
});
</script>

<style scoped>
.file-view {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;
  background: #f5f7fa;
}

.file-view-header {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 20px 24px;
  background: #ffffff;
  border-bottom: 1px solid #e0e0e0;
}

.file-view-title {
  font-size: 24px;
  font-weight: 600;
  color: #333;
  margin: 0;
}

.file-view-path {
  font-size: 12px;
  color: #999;
  word-break: break-all;
}

.file-view-search {
  padding: 12px 24px;
  background: #ffffff;
  border-bottom: 1px solid #e0e0e0;
}

.search-input {
  width: 100%;
}

.file-view-tree-wrapper {
  flex: 1;
  overflow: auto;
  background: #ffffff;
  padding: 8px 0;
}

.file-tree {
  background: transparent;
}

.file-tree :deep(.el-tree-node__content) {
  height: 32px;
  padding-right: 8px;
}

.file-tree :deep(.el-tree-node__content:hover) {
  background-color: #f5f7fa;
}

.file-tree :deep(.el-tree-node.is-current > .el-tree-node__content) {
  background-color: #ecf5ff;
}

.tree-node-content {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 1;
  overflow: hidden;
}

.node-icon {
  font-size: 16px;
  flex-shrink: 0;
}

.folder-icon {
  color: #e6a23c;
}

.file-icon {
  color: #409eff;
}

.node-label {
  font-size: 13px;
  color: #333;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex: 1;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 24px;
  text-align: center;
}

.empty-icon {
  font-size: 48px;
  color: #c0c4cc;
  margin-bottom: 16px;
}

.empty-text {
  font-size: 16px;
  color: #909399;
  margin: 0 0 4px;
}

.empty-subtitle {
  font-size: 13px;
  color: #c0c4cc;
  margin: 0;
}
</style>

<style>
.search-highlight {
  background-color: #fef08a;
  color: inherit;
  padding: 0 1px;
  border-radius: 2px;
}
</style>
