<template>
  <div class="edit-view">
    <!-- 页面标题 -->
    <div class="page-header">
      <h1 class="page-title">{{ isCreating ? '创建表格' : '编辑表格' }}</h1>
      <el-button type="primary" @click="handleSave">
        <el-icon><Check /></el-icon>
        保存
      </el-button>
    </div>
    
    <!-- 表格编辑器 -->
    <div class="editor-container">
      <el-alert
        v-if="!tableData"
        :title="isCreating ? '创建新表格' : '加载表格中...'"
        type="info"
        :closable="false"
        show-icon
      />
      
      <TableEditor
        v-else
        :table-data="tableData"
        @save="handleTableSave"
        @cell-select="handleCellSelect"
      />
    </div>
    
    <!-- 单元格属性面板 -->
    <div class="properties-panel">
      <h3>单元格属性</h3>
      <div v-if="selectedCell" class="cell-properties">
        <el-form :model="selectedCell" label-width="80px">
          <el-form-item label="位置">
            <el-input v-model="selectedCell.position" disabled />
          </el-form-item>
          <el-form-item label="值">
            <el-input v-model="selectedCell.value" />
          </el-form-item>
          <el-form-item label="类型">
            <el-input v-model="selectedCell.type" disabled />
          </el-form-item>
          <el-form-item label="格式">
            <el-select v-model="selectedCell.format">
              <el-option label="文本" value="文本" />
              <el-option label="数字" value="数字" />
              <el-option label="布尔值" value="布尔值" />
            </el-select>
          </el-form-item>
          <el-form-item label="验证">
            <el-input v-model="selectedCell.validation" />
          </el-form-item>
        </el-form>
      </div>
      <div v-else class="no-selection">
        <el-icon class="empty-icon"><InfoFilled /></el-icon>
        <p>选择一个单元格查看属性</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { useRoute, useRouter } from "vue-router";
import { Check, InfoFilled } from "@element-plus/icons-vue";
import TableEditor from "../components/features/TableEditor.vue";
import { apiService, TableDetail } from "../services/api";

const route = useRoute();
const router = useRouter();

// 表格 ID
const tableId = computed(() => route.query.id as string | undefined);

// 是否为创建模式
const isCreating = computed(() => !tableId.value);

// 表格数据
const tableData = ref<TableDetail | null>(null);

// 选中的单元格
const selectedCell = ref<any>(null);

// 加载表格数据
const loadTableData = async () => {
  if (tableId.value) {
    try {
      const data = await apiService.getTableDetail(tableId.value);
      tableData.value = data;
    } catch (error) {
      console.error('加载表格数据失败:', error);
    }
  } else {
    // 创建新表格
    tableData.value = {
      id: Date.now().toString(),
      name: '新表格',
      columns: [
        { prop: 'id', label: 'ID', width: 100, editable: false },
        { prop: 'name', label: '名称', width: 200, editable: true },
        { prop: 'value', label: '值', width: 300, editable: true }
      ],
      data: []
    };
  }
};

// 处理表格保存
const handleTableSave = async (data: any[]) => {
  try {
    if (tableData.value) {
      await apiService.saveTableData(tableData.value.id, data);
      // 保存成功后跳转到表格列表
      router.push('/table');
    }
  } catch (error) {
    console.error('保存表格失败:', error);
  }
};

// 处理保存按钮点击
const handleSave = () => {
  // 触发表格保存
  // 这里可以添加额外的保存逻辑
};

// 处理单元格选择
const handleCellSelect = (cellProps: any) => {
  selectedCell.value = cellProps;
};

// 组件挂载时加载数据
onMounted(() => {
  loadTableData();
});
</script>

<style scoped>
/* 整体布局 */
.edit-view {
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

/* 编辑器容器 */
.editor-container {
  flex: 1;
  padding: 24px;
  background: #ffffff;
  margin: 0 24px;
  border-radius: 8px;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
  overflow: hidden;
}

/* 属性面板 */
.properties-panel {
  width: 300px;
  padding: 24px;
  background: #ffffff;
  border-left: 1px solid #e0e0e0;
  overflow-y: auto;
}

.properties-panel h3 {
  font-size: 16px;
  font-weight: 600;
  color: #333;
  margin: 0 0 20px 0;
  padding-bottom: 10px;
  border-bottom: 1px solid #e0e0e0;
}

/* 单元格属性 */
.cell-properties {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

/* 无选择状态 */
.no-selection {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 200px;
  color: #858585;
  gap: 12px;
}

.empty-icon {
  font-size: 48px;
  opacity: 0.5;
}

/* 响应式设计 */
@media (max-width: 1200px) {
  .properties-panel {
    width: 250px;
  }
}

@media (max-width: 768px) {
  .edit-view {
    flex-direction: column;
  }
  
  .editor-container {
    margin: 0 16px;
    padding: 16px;
  }
  
  .properties-panel {
    width: 100%;
    border-left: none;
    border-top: 1px solid #e0e0e0;
  }
}
</style>