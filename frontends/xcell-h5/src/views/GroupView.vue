<template>
  <div class="group-view">
    <!-- 页面标题 -->
    <div class="page-header">
      <h1 class="page-title">分包管理</h1>
      <el-button type="primary" class="create-button" @click="handleCreateGroup">
        <el-icon><Plus /></el-icon>
        创建分包
      </el-button>
    </div>
    
    <!-- 过滤器区域 -->
    <div class="filter-section">
      <el-input
        v-model="filterText"
        placeholder="搜索分包"
        prefix-icon="Search"
        class="filter-input"
        @keyup.enter="handleSearch"
      />
      <el-button type="primary" class="search-button" @click="handleSearch">搜索</el-button>
    </div>
    
    <!-- 分包列表 -->
    <div class="group-container">
      <el-table
        v-loading="loading"
        element-loading-text="加载中..."
        :data="filteredGroupData"
        style="width: 100%"
        class="group-list"
        :header-cell-style="{ backgroundColor: '#f8f9fa' }"
        :row-style="{ height: '60px' }"
      >
        <el-table-column label="分包名称" min-width="200">
          <template #default="scope">
            <div class="group-info">
              <div class="group-name">{{ scope.row.name }}</div>
              <div class="group-desc">{{ scope.row.description || '无描述' }}</div>
            </div>
          </template>
        </el-table-column>
        <el-table-column prop="id" label="分包ID" width="120">
          <template #default="scope">
            <code>{{ scope.row.id }}</code>
          </template>
        </el-table-column>
        <el-table-column label="包含表格" width="100">
          <template #default="scope">
            <el-tag size="small" type="info">{{ scope.row.tableCount || 0 }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column label="创建时间" width="160">
          <template #default="scope">
            {{ formatRelativeTime(scope.row.createdAt) }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="200" fixed="right">
          <template #default="scope">
            <div class="action-buttons">
              <el-button size="default" circle @click="openEditDialog(scope.row)" title="编辑" :style="{ backgroundColor: '#409EFF', borderColor: '#409EFF', color: '#ffffff' }">
                <el-icon><Edit /></el-icon>
              </el-button>
              <el-tooltip :content="scope.row.id === 'DataTable' ? '主包必须加载' : '删除分包'" placement="top">
                <el-button 
                  size="default" 
                  circle 
                  :disabled="scope.row.id === 'DataTable'" 
                  @click="handleDelete(scope.row)" 
                  :style="scope.row.id === 'DataTable' ? { backgroundColor: '#C0C4CC', borderColor: '#C0C4CC', color: '#ffffff', cursor: 'not-allowed' } : { backgroundColor: '#F56C6C', borderColor: '#F56C6C', color: '#ffffff' }"
                >
                  <el-icon><Delete /></el-icon>
                </el-button>
              </el-tooltip>
            </div>
          </template>
        </el-table-column>
      </el-table>
    </div>
    
    <!-- 分页区域 -->
    <div class="pagination-section">
      <div class="pagination-info">
        共 {{ total }} 个分包
      </div>
      <el-pagination
        v-model:current-page="currentPage"
        v-model:page-size="pageSize"
        :page-sizes="[10, 20, 50, 100]"
        layout="sizes, prev, pager, next"
        :total="total"
        @size-change="handleSizeChange"
        @current-change="handleCurrentChange"
        class="pagination-control"
      />
    </div>
    
    <!-- 编辑弹窗 -->
    <el-dialog
      v-model="dialogVisible"
      :title="isCreating ? '创建分包' : '编辑分包'"
      width="500px"
      :close-on-click-modal="false"
    >
      <el-form :model="editForm" label-width="100px" class="edit-form">
        <el-form-item label="分包名称" required>
          <el-input v-model="editForm.name" placeholder="输入分包名称" />
        </el-form-item>
        <el-form-item label="分包ID" required>
          <el-input v-model="editForm.id" placeholder="输入分包ID" />
        </el-form-item>
        <el-form-item label="描述">
          <el-input v-model="editForm.description" placeholder="输入分包描述" type="textarea" :rows="3" />
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="dialogVisible = false">取消</el-button>
          <el-button type="primary" @click="saveEdit">{{ isCreating ? '创建' : '保存' }}</el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useRouter } from "vue-router";
import { Edit, Delete, Plus, Search } from "@element-plus/icons-vue";

// 定义分包数据类型
interface GroupData {
  id: string;
  name: string;
  description?: string;
  tableCount?: number;
  createdAt: string;
}

const router = useRouter();

// 过滤器
const filterText = ref("");

// 分页
const currentPage = ref(1);
const pageSize = ref(10);
const total = ref(0);

// 编辑弹窗
const dialogVisible = ref(false);
const isCreating = ref(false);
const editForm = ref<GroupData>({
  id: "",
  name: "",
  description: "",
  tableCount: 0,
  createdAt: new Date().toISOString()
});
const currentEditRow = ref<GroupData | null>(null);

// 加载状态
const loading = ref(false);

// 分包数据
const groupData = ref<GroupData[]>([]);

// 模拟分包数据
const mockGroupData: GroupData[] = [
  {
    id: "DataTable",
    name: "主包",
    description: "默认主包，包含主要表格数据",
    tableCount: 15,
    createdAt: new Date().toISOString()
  },
  {
    id: "package1",
    name: "分包1",
    description: "第一个分包",
    tableCount: 8,
    createdAt: new Date(Date.now() - 86400000).toISOString()
  },
  {
    id: "package2",
    name: "分包2",
    description: "第二个分包",
    tableCount: 5,
    createdAt: new Date(Date.now() - 172800000).toISOString()
  },
  {
    id: "package3",
    name: "分包3",
    description: "第三个分包",
    tableCount: 3,
    createdAt: new Date(Date.now() - 259200000).toISOString()
  }
];

// 加载分包数据
const loadGroupData = async () => {
  try {
    loading.value = true;
    // 模拟API调用
    await new Promise(resolve => setTimeout(resolve, 500));
    groupData.value = mockGroupData;
  } catch (error) {
    console.error('加载分包数据失败:', error);
  } finally {
    loading.value = false;
  }
};

// 组件挂载时加载数据
onMounted(() => {
  loadGroupData();
});

// 计算过滤后的数据
const filteredGroupData = computed(() => {
  let filtered = groupData.value;
  
  // 应用搜索过滤
  if (filterText.value) {
    const searchText = filterText.value.toLowerCase();
    filtered = filtered.filter(item => 
      item.name.toLowerCase().includes(searchText) ||
      item.id.toLowerCase().includes(searchText) ||
      (item.description && item.description.toLowerCase().includes(searchText))
    );
  }
  
  // 更新总数
  total.value = filtered.length;
  
  // 应用分页
  const start = (currentPage.value - 1) * pageSize.value;
  const end = start + pageSize.value;
  return filtered.slice(start, end);
});

// 搜索处理
const handleSearch = () => {
  currentPage.value = 1; // 重置到第一页
};

// 分页大小变化
const handleSizeChange = (size: number) => {
  pageSize.value = size;
  currentPage.value = 1; // 重置到第一页
};

// 当前页变化
const handleCurrentChange = (current: number) => {
  currentPage.value = current;
};

// 打开编辑弹窗
const openEditDialog = (row: GroupData) => {
  // 保存当前编辑的行
  currentEditRow.value = row;
  // 复制数据到编辑表单
  editForm.value = { ...row };
  // 设置为编辑模式
  isCreating.value = false;
  // 显示弹窗
  dialogVisible.value = true;
};

// 打开创建弹窗
const handleCreateGroup = () => {
  // 重置编辑表单
  editForm.value = {
    id: "",
    name: "",
    description: "",
    tableCount: 0,
    createdAt: new Date().toISOString()
  };
  // 设置为创建模式
  isCreating.value = true;
  // 显示弹窗
  dialogVisible.value = true;
};

// 保存编辑
const saveEdit = async () => {
  try {
    loading.value = true;
    // 模拟API调用
    await new Promise(resolve => setTimeout(resolve, 500));
    
    if (isCreating.value) {
      // 创建新分包
      groupData.value.push({
        ...editForm.value,
        tableCount: 0,
        createdAt: new Date().toISOString()
      });
    } else if (currentEditRow.value) {
      // 更新现有分包
      Object.assign(currentEditRow.value, editForm.value);
    }
    
    // 关闭弹窗
    dialogVisible.value = false;
  } catch (error) {
    console.error('保存分包失败:', error);
  } finally {
    loading.value = false;
  }
};

// 删除分包
const handleDelete = async (row: GroupData) => {
  // 主包不能删除
  if (row.id === 'DataTable') {
    alert('主包不能删除');
    return;
  }
  
  try {
    // 这里可以添加删除确认逻辑
    const confirm = window.confirm(`确定要删除分包 ${row.name} 吗？`);
    if (!confirm) return;
    
    loading.value = true;
    // 模拟API调用
    await new Promise(resolve => setTimeout(resolve, 500));
    
    // 从本地数据中删除
    const index = groupData.value.findIndex(item => item.id === row.id);
    if (index !== -1) {
      groupData.value.splice(index, 1);
    }
  } catch (error) {
    console.error('删除分包失败:', error);
  } finally {
    loading.value = false;
  }
};



// 格式化相对时间
const formatRelativeTime = (dateString: string): string => {
  const date = new Date(dateString);
  const now = new Date();
  const diffInSeconds = Math.floor((now.getTime() - date.getTime()) / 1000);
  
  if (diffInSeconds < 60) {
    return `${diffInSeconds}秒前`;
  } else if (diffInSeconds < 3600) {
    const minutes = Math.floor(diffInSeconds / 60);
    return `${minutes}分钟前`;
  } else if (diffInSeconds < 86400) {
    const hours = Math.floor(diffInSeconds / 3600);
    return `${hours}小时前`;
  } else {
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    return `${year}-${month}-${day}`;
  }
};
</script>

<style scoped>
/* 整体布局 */
.group-view {
  padding: 0;
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 0;
  background: #f5f7fa;
}

/* 页面标题 */
.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  background: #ffffff;
  border-bottom: 1px solid #e0e0e0;
}

.page-title {
  font-size: 24px;
  font-weight: 600;
  color: #333;
  margin: 0;
}

.create-button {
  background-color: #409EFF !important;
  border-color: #409EFF !important;
  font-size: 14px;
  padding: 8px 16px;
}

.create-button:hover {
  background-color: #66b1ff !important;
  border-color: #66b1ff !important;
}

/* 过滤器区域 */
.filter-section {
  display: flex;
  gap: 12px;
  align-items: center;
  padding: 16px 24px;
  background: #f8f9fa;
  border-bottom: 1px solid #e0e0e0;
}

.filter-input {
  flex: 1;
  max-width: 400px;
  height: 40px;
  border-radius: 4px;
}

.search-button {
  background-color: #409EFF !important;
  border-color: #409EFF !important;
  height: 40px;
  padding: 0 20px;
}

.search-button:hover {
  background-color: #66b1ff !important;
  border-color: #66b1ff !important;
}

/* 分包容器 */
.group-container {
  flex: 1;
  background: #ffffff;
  overflow: hidden;
  width: 100%;
}

/* 表格样式 */
.group-list {
  width: 100%;
  height: 100%;
  min-width: 100%;
}

:deep(.el-table) {
  width: 100%;
  height: 100%;
  overflow: hidden;
}

:deep(.el-table th) {
  background-color: #f8f9fa;
  font-weight: 600;
  color: #333;
  border-bottom: 1px solid #e0e0e0;
  padding: 12px 16px;
}

:deep(.el-table tr:hover > td) {
  background-color: #f0f9ff !important;
}

:deep(.el-table td) {
  border-bottom: 1px solid #f0f0f0;
  padding: 12px 16px;
}

/* 分包信息 */
.group-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.group-name {
  font-weight: 500;
  color: #333;
  font-size: 14px;
}

.group-desc {
  font-size: 12px;
  color: #666;
}

/* 操作按钮 */
.action-buttons {
  display: flex;
  gap: 12px;
  align-items: center;
  padding: 4px;
  justify-content: center;
}

:deep(.el-button--primary) {
  background-color: #409EFF !important;
  border-color: #409EFF !important;
}

:deep(.el-button--primary:hover) {
  background-color: #66b1ff !important;
  border-color: #66b1ff !important;
}

:deep(.el-button--danger) {
  background-color: #F56C6C !important;
  border-color: #F56C6C !important;
}

:deep(.el-button--danger:hover) {
  background-color: #f78989 !important;
  border-color: #f78989 !important;
}

/* 分页区域 */
.pagination-section {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 24px;
  background: #f8f9fa;
  border-top: 1px solid #e0e0e0;
}

.pagination-info {
  font-size: 14px;
  color: #666;
}

.pagination-control {
  display: flex;
  align-items: center;
}

/* 分页样式 */
:deep(.el-pagination.is-background .el-pager li:not(.disabled):hover) {
  color: #409EFF !important;
}

:deep(.el-pagination.is-background .el-pager li.active) {
  background-color: #409EFF !important;
  color: #ffffff !important;
}

:deep(.el-pagination__button:hover) {
  color: #409EFF !important;
}

/* 编辑表单 */
.edit-form {
  margin-top: 16px;
}

:deep(.el-form-item__label) {
  font-weight: 500;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .group-view {
    padding: 16px;
  }
  
  .page-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }
  
  .filter-section {
    flex-direction: column;
    align-items: stretch;
  }
  
  .filter-input {
    max-width: none;
  }
  
  .pagination-section {
    flex-direction: column;
    align-items: stretch;
    gap: 12px;
  }
  
  .pagination-control {
    justify-content: center;
  }
}
</style>