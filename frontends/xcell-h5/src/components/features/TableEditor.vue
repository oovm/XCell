<template>
  <div class="table-editor">
    <div v-if="!tableData" class="empty-state">
      <el-icon class="empty-icon"><Document /></el-icon>
      <p>选择一个表格文件开始编辑</p>
    </div>
    
    <div v-else class="table-content">
      <div class="table-header">
        <h3>{{ tableData.name }}</h3>
        <div class="table-actions">
          <el-button type="primary" size="small" @click="saveTableData" :loading="loading">保存</el-button>
          <el-button size="small" @click="addTableRow">添加行</el-button>
        </div>
      </div>
      <el-table
        :data="tableData.data"
        style="width: 100%"
        border
        class="xcell-table"
      >
        <el-table-column
          v-for="column in tableData.columns"
          :key="column.prop"
          :prop="column.prop"
          :label="column.label"
          :width="column.width"
        >
          <template #default="{ row, column }">
            <div 
              v-if="column.prop === 'id'" 
              class="id-cell" 
              @click="handleIdClick(row[column.prop]); selectCell(row, column)"
            >
              <span class="id-link">{{ row[column.prop] }}</span>
              <el-icon class="id-icon"><ArrowRight /></el-icon>
            </div>
            <div 
              v-else-if="column.editable"
              @click="selectCell(row, column)"
            >
              <el-input
                v-model="row[column.prop]"
                size="small"
                @change="editTableCell(row, column, row[column.prop])"
              />
            </div>
            <div 
              v-else
              @click="selectCell(row, column)"
            >
              {{ row[column.prop] }}
            </div>
          </template>
        </el-table-column>
        <el-table-column label="操作" width="100">
          <template #default="{ $index }">
            <el-button
              type="danger"
              size="small"
              @click="deleteTableRow($index)"
            >
              删除
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { ArrowRight, Document, Loading } from "@element-plus/icons-vue";
import { apiService } from "../../services/api";

// 定义属性
const props = defineProps({
	tableData: {
		type: Object,
		default: null,
	},
});

// 定义事件
const emit = defineEmits(["save", "cell-select"]);

// 加载状态
const loading = ref(false);

// 表格数据结构
interface TableData {
	id: string;
	[key: string]: string | number | boolean | undefined;
}

// 表格列定义
interface TableColumn {
	prop: string;
	label: string;
	width?: string | number;
	editable?: boolean;
}

// 单元格属性数据结构
interface CellProperties {
	position: string;
	value: string | number | boolean | undefined;
	type: string;
	format: string;
	validation: string;
}

// 处理ID跳转
const handleIdClick = (id: string) => {
	console.log("Navigating to ID:", id);
	// 这里需要实现跳转到原始定义的逻辑
	alert(`跳转到ID ${id} 的原始定义`);
};

// 保存表格数据
const saveTableData = async () => {
	if (props.tableData) {
		try {
			loading.value = true;
			await apiService.saveTableData(props.tableData.id, props.tableData.data);
			emit("save", props.tableData.data);
		} catch (error) {
			console.error('保存表格数据失败:', error);
		} finally {
			loading.value = false;
		}
	}
};

// 添加表格行
const addTableRow = () => {
	if (props.tableData) {
		const newId = (props.tableData.data.length + 1).toString();
		props.tableData.data.push({
			id: newId,
			name: "",
			age: "",
			email: "",
		});
	}
};

// 删除表格行
const deleteTableRow = (index: number) => {
	if (props.tableData) {
		props.tableData.data.splice(index, 1);
	}
};

// 编辑表格单元格
const editTableCell = (
	row: TableData,
	column: TableColumn,
	value: string | number | boolean | undefined,
) => {
	row[column.prop] = value;
};

// 选择单元格
const selectCell = (row: TableData, column: TableColumn) => {
	const cellProps: CellProperties = {
		position: `${column.label}${row.id}`,
		value: row[column.prop],
		type: typeof row[column.prop],
		format: column.prop === "age" ? "数字" : "文本",
		validation: "无",
	};
	emit("cell-select", cellProps);
};
</script>

<style scoped>
.table-editor {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 16px;
  color: #858585;
}

.empty-icon {
  font-size: 48px;
  opacity: 0.5;
}

.table-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.table-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-bottom: 8px;
  border-bottom: 1px solid #3e3e42;
}

.table-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: #d4d4d4;
}

.table-actions {
  display: flex;
  gap: 8px;
}

.xcell-table {
  --el-table-bg-color: #252526;
  --el-table-header-bg-color: #2d2d30;
  --el-table-header-text-color: #d4d4d4;
  --el-table-body-text-color: #d4d4d4;
  --el-table-border-color: #3e3e42;
  --el-table-row-hover-bg-color: rgba(255, 255, 255, 0.1);
  flex: 1;
  overflow: auto;
}

.xcell-table .el-input__wrapper {
  --el-input-bg-color: #1e1e1e;
  --el-input-border-color: #3e3e42;
  --el-input-text-color: #d4d4d4;
}

/* ID单元格样式 */
.id-cell {
  display: flex;
  align-items: center;
  gap: 4px;
  cursor: pointer;
  color: #4ec9b0;
  padding: 4px;
  border-radius: 4px;
  transition: background 0.2s;
}

.id-cell:hover {
  background: rgba(78, 201, 176, 0.2);
}

.id-link {
  font-weight: 500;
}

.id-icon {
  font-size: 12px;
  opacity: 0.7;
}
</style>