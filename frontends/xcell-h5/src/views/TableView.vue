<template>
  <div class="table-view">
    <!-- 页面标题 -->
    <div class="page-header">
      <h1 class="page-title">表格管理</h1>
      <div class="header-actions">
        <el-upload
          class="upload-button"
          :auto-upload="false"
          :on-change="handleFileUpload"
          accept=".xlsx,.xls,.toml"
          :show-file-list="false"
        >
          <el-button type="primary" :icon="Upload">
            导入表格
          </el-button>
        </el-upload>
        <el-button type="primary" class="create-button" @click="handleCreateTable">
          <el-icon><Plus /></el-icon>
          创建表格
        </el-button>
      </div>
    </div>
    
    <!-- 过滤器区域 -->
    <div class="filter-section">
      <el-input
        v-model="filterText"
        placeholder="搜索表格"
        prefix-icon="Search"
        class="filter-input"
        @keyup.enter="handleSearch"
      />
      <el-button type="primary" class="search-button" @click="handleSearch">搜索</el-button>
    </div>
    
    <!-- 表格列表 -->
    <div class="table-container">
      <el-table
        v-loading="loading"
        element-loading-text="加载中..."
        :data="filteredTableData"
        style="width: 100%"
        class="table-list"
        :header-cell-style="{ backgroundColor: '#f8f9fa' }"
        :row-style="{ height: '60px' }"
      >
        <el-table-column label="表格" min-width="300">
          <template #default="scope">
            <div class="table-info">
              <div class="table-name">{{ scope.row.name }}</div>
              <div class="table-path">{{ scope.row.path }}</div>
            </div>
          </template>
        </el-table-column>
        <el-table-column prop="type" width="140">
          <template #header>
            <div class="column-header-with-help">
              <span>表格类型</span>
              <el-tooltip content="表格类型决定了数据的处理方式" placement="top">
                <el-button size="small" circle @click="navigateToDocs('table-type')" class="help-button">
                  <el-icon><QuestionFilled /></el-icon>
                </el-button>
              </el-tooltip>
            </div>
          </template>
          <template #default="scope">
            <div class="tag-container">
              <el-tag :type="getTypeTagType(scope.row.type)" class="type-tag">{{ scope.row.type }}</el-tag>
            </div>
          </template>
        </el-table-column>
        <el-table-column prop="package" width="140">
          <template #header>
            <div class="column-header-with-help">
              <span>分包</span>
              <el-tooltip content="每个包拥有单独的单例和加载逻辑" placement="top">
                <el-button size="small" circle @click="navigateToDocs('package')" class="help-button">
                  <el-icon><QuestionFilled /></el-icon>
                </el-button>
              </el-tooltip>
            </div>
          </template>
          <template #default="scope">
            <div class="package-container">
              <el-select v-model="scope.row.package" size="small" @change="handlePackageChange(scope.row)">
                <el-option label="主包" value="DataTable" />
                <el-option label="分包1" value="package1" />
                <el-option label="分包2" value="package2" />
                <el-option label="分包3" value="package3" />
              </el-select>
            </div>
          </template>
        </el-table-column>
        <el-table-column prop="draft" width="120">
          <template #header>
            <div class="column-header-with-help">
              <span>草稿</span>
              <el-tooltip content="草稿状态的表格不会被导出" placement="top">
                <el-button size="small" circle @click="navigateToDocs('draft-status')" class="help-button">
                  <el-icon><QuestionFilled /></el-icon>
                </el-button>
              </el-tooltip>
            </div>
          </template>
          <template #default="scope">
            <el-switch v-model="scope.row.draft" @change="handleDraftChange(scope.row)" />
          </template>
        </el-table-column>
        <el-table-column label="修改时间" width="160">
          <template #default="scope">
            {{ formatRelativeTime(scope.row.createdAt) }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="300" fixed="right">
          <template #default="scope">
            <div class="action-buttons">
              <el-button size="default" circle @click="handleOpen(scope.row)" title="打开" :style="{ backgroundColor: '#4CAF50', borderColor: '#4CAF50', color: '#ffffff' }">
                <el-icon><Folder /></el-icon>
              </el-button>
              <el-button size="default" circle @click="openEditDialog(scope.row)" title="编辑" :style="{ backgroundColor: '#2196F3', borderColor: '#2196F3', color: '#ffffff' }">
                <el-icon><Edit /></el-icon>
              </el-button>
              <el-button size="default" circle @click="handleExportTable(scope.row)" title="导出" :style="{ backgroundColor: '#FF9800', borderColor: '#FF9800', color: '#ffffff' }">
                <el-icon><Download /></el-icon>
              </el-button>
              <el-button size="default" circle type="danger" @click="handleDelete(scope.row)" title="删除">
                <el-icon><Delete /></el-icon>
              </el-button>
            </div>
          </template>
        </el-table-column>
      </el-table>
    </div>
    
    <!-- 分页区域 -->
    <div class="pagination-section">
      <div class="pagination-info">
        共 {{ total }} 个表格
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
    
    <!-- 编辑元属性弹窗 -->
    <el-dialog
      v-model="dialogVisible"
      title="编辑表格属性"
      width="500px"
      :close-on-click-modal="false"
    >
      <el-form :model="editForm" label-width="100px" class="edit-form">
        <el-form-item label="表格名" required>
          <el-input v-model="editForm.name" placeholder="输入表格名" />
        </el-form-item>
        <el-form-item label="表格类型" required>
          <el-select v-model="editForm.type" style="width: 100%">
            <el-option label="table" value="table" />
            <el-option label="class" value="class" />
            <el-option label="enum" value="enum" />
            <el-option label="language" value="language" />
          </el-select>
        </el-form-item>
        <el-form-item label="分包" required>
          <el-select v-model="editForm.package" style="width: 100%">
            <el-option label="主包" value="DataTable" />
            <el-option label="分包1" value="package1" />
            <el-option label="分包2" value="package2" />
            <el-option label="分包3" value="package3" />
          </el-select>
        </el-form-item>
        <el-form-item label="路径" required>
          <el-input v-model="editForm.path" placeholder="输入项目相对路径" />
        </el-form-item>
        <el-form-item label="草稿">
          <el-switch v-model="editForm.draft" />
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="dialogVisible = false">取消</el-button>
          <el-button type="primary" @click="saveEdit">保存</el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useRouter } from "vue-router";
import { Folder, Edit, Delete, Plus, QuestionFilled, Loading, Upload, Download } from "@element-plus/icons-vue";
import { apiService, TableData } from "../services/api";

const router = useRouter();

// 过滤器
const filterText = ref("");

// 分页
const currentPage = ref(1);
const pageSize = ref(10);
const total = ref(0);

// 编辑弹窗
const dialogVisible = ref(false);
const editForm = ref<TableData>({
  id: "",
  name: "",
  type: "table",
  package: "DataTable",
  path: "",
  draft: false,
  createdAt: ""
});
const currentEditRow = ref<TableData | null>(null);

// 加载状态
const loading = ref(false);

// 表格数据
const tableData = ref<TableData[]>([]);

// 获取表格类型对应的标签类型
const getTypeTagType = (type: string): string => {
  switch (type) {
    case 'table': return 'primary';
    case 'class': return 'success';
    case 'enum': return 'warning';
    case 'language': return 'info';
    default: return '';
  }
};

// 加载表格数据
const loadTableData = async () => {
  try {
    loading.value = true;
    const data = await apiService.getTableList();
    // 为每个表格添加package字段，默认为DataTable
    tableData.value = data.map((item: TableData) => ({
      ...item,
      package: item.package || 'DataTable'
    }));
  } catch (error) {
    console.error('加载表格数据失败:', error);
  } finally {
    loading.value = false;
  }
};

// 组件挂载时加载数据
onMounted(() => {
  loadTableData();
});

// 计算过滤后的数据
const filteredTableData = computed(() => {
  let filtered = tableData.value;
  
  // 应用搜索过滤
  if (filterText.value) {
    const searchText = filterText.value.toLowerCase();
    filtered = filtered.filter(item => 
      item.name.toLowerCase().includes(searchText) ||
      item.type.toLowerCase().includes(searchText) ||
      item.path.toLowerCase().includes(searchText) ||
      (item.draft ? 'draft' : 'published').includes(searchText)
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

// 编辑表格
const handleEdit = (row: TableData) => {
  // 导航到编辑页面，传递表格ID
  router.push(`/edit?id=${row.id}`);
};

// 删除表格
const handleDelete = async (row: TableData) => {
  try {
    // 这里可以添加删除确认逻辑
    const confirm = window.confirm(`确定要删除表格 ${row.name} 吗？`);
    if (!confirm) return;
    
    loading.value = true;
    await apiService.deleteTable(row.id);
    // 从本地数据中删除
    const index = tableData.value.findIndex(item => item.id === row.id);
    if (index !== -1) {
      tableData.value.splice(index, 1);
    }
  } catch (error) {
    console.error('删除表格失败:', error);
  } finally {
    loading.value = false;
  }
};

// 打开编辑弹窗
const openEditDialog = (row: TableData) => {
  // 保存当前编辑的行
  currentEditRow.value = row;
  // 复制数据到编辑表单
  editForm.value = { 
    ...row,
    package: row.package || 'DataTable' // 确保分包字段存在，默认为主包
  };
  // 显示弹窗
  dialogVisible.value = true;
};

// 保存编辑
const saveEdit = async () => {
  if (currentEditRow.value) {
    try {
      loading.value = true;
      await apiService.updateTable(currentEditRow.value.id, editForm.value);
      // 更新本地数据
      Object.assign(currentEditRow.value, editForm.value);
      // 关闭弹窗
      dialogVisible.value = false;
    } catch (error) {
      console.error('保存表格失败:', error);
    } finally {
      loading.value = false;
    }
  }
};

// 处理草稿状态变更
const handleDraftChange = async (row: TableData) => {
  try {
    await apiService.updateTable(row.id, { draft: row.draft });
  } catch (error) {
    console.error('更新草稿状态失败:', error);
    // 恢复原始状态
    row.draft = !row.draft;
  }
};

// 处理分包变更
const handlePackageChange = async (row: TableData) => {
  try {
    await apiService.updateTable(row.id, { package: row.package });
  } catch (error) {
    console.error('更新分包失败:', error);
    // 恢复原始状态
    // 这里可以添加恢复逻辑
  }
};

// 处理打开表格
const handleOpen = (row: TableData) => {
  // 直接打开文件
  console.log("打开文件:", row.path);
  // 这里可以添加打开文件的逻辑，例如使用系统默认应用打开文件
  // 由于是前端环境，实际打开文件可能需要后端支持
  alert(`打开文件: ${row.path}`);
};

// 处理创建表格
const handleCreateTable = () => {
  // 导航到编辑页面，不传递表格ID表示创建新表格
  router.push("/edit");
};

// 处理文件上传
const handleFileUpload = async (file: any) => {
  try {
    loading.value = true;
    const table = await apiService.importTable(file.raw);
    tableData.value.push(table);
  } catch (error) {
    console.error('导入表格失败:', error);
  } finally {
    loading.value = false;
  }
};

// 处理表格导出
const handleExportTable = async (row: TableData) => {
  try {
    loading.value = true;
    const blob = await apiService.exportTable(row.id);
    // 创建下载链接
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${row.name}.json`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  } catch (error) {
    console.error('导出表格失败:', error);
  } finally {
    loading.value = false;
  }
};

// 导航到文档
const navigateToDocs = (topic: string) => {
  // 这里可以根据不同的主题导航到不同的文档页面
  console.log("导航到文档:", topic);
  // 例如：window.open(`/docs#${topic}`, '_blank');
  alert(`导航到文档: ${topic}`);
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
.table-view {
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
  background-color: #4CAF50 !important;
  border-color: #4CAF50 !important;
  font-size: 14px;
  padding: 8px 16px;
}

.create-button:hover {
  background-color: #45a049 !important;
  border-color: #45a049 !important;
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
  background-color: #4CAF50 !important;
  border-color: #4CAF50 !important;
  height: 40px;
  padding: 0 20px;
}

.search-button:hover {
  background-color: #45a049 !important;
  border-color: #45a049 !important;
}

/* 表格容器 */
.table-container {
  flex: 1;
  background: #ffffff;
  overflow: hidden;
  width: 100%;
}

/* 表格样式 */
.table-list {
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

/* 表格信息 */
.table-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.table-name {
  font-weight: 500;
  color: #333;
  font-size: 14px;
}

.table-path {
  font-size: 12px;
  color: #666;
}

/* 标签容器 */
.tag-container {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100%;
  height: 100%;
}

/* 分包容器 */
.package-container {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100%;
  height: 100%;
}

.package-container .el-select {
  width: 100%;
  max-width: 100px;
}

/* 类型标签 */
.type-tag {
  border-radius: 4px;
  padding: 2px 8px;
  font-size: 12px;
  min-width: 60px;
  text-align: center;
}

/* 带帮助图标的列标题 */
.column-header-with-help {
  display: flex;
  align-items: center;
  gap: 4px;
}

/* 帮助按钮 */
.help-button {
  width: 20px;
  height: 20px;
  padding: 0;
  font-size: 12px;
  min-width: 20px;
}

.help-button .el-icon {
  font-size: 12px;
}

/* 操作按钮 */
.action-buttons {
  display: flex;
  gap: 8px;
  align-items: center;
  padding: 4px;
  justify-content: center;
}

:deep(.el-button--primary) {
  background-color: #4CAF50 !important;
  border-color: #4CAF50 !important;
}

:deep(.el-button--primary:hover) {
  background-color: #45a049 !important;
  border-color: #45a049 !important;
}

:deep(.el-button--danger) {
  background-color: #dc3545 !important;
  border-color: #dc3545 !important;
}

:deep(.el-button--danger:hover) {
  background-color: #c82333 !important;
  border-color: #c82333 !important;
}

/* 开关样式 */
:deep(.el-switch__core:after) {
  background-color: #ffffff !important;
}

:deep(.el-switch.is-checked .el-switch__core) {
  background-color: #4CAF50 !important;
  border-color: #4CAF50 !important;
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
  color: #4CAF50 !important;
}

:deep(.el-pagination.is-background .el-pager li.active) {
  background-color: #4CAF50 !important;
  color: #ffffff !important;
}

:deep(.el-pagination__button:hover) {
  color: #4CAF50 !important;
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
  .table-view {
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