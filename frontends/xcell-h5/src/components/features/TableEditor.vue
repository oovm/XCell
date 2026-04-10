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
          <el-button size="small" @click="undo" :disabled="undoStack.length === 0">
            <el-icon><RefreshLeft /></el-icon> 撤销
          </el-button>
          <el-button size="small" @click="redo" :disabled="redoStack.length === 0">
            <el-icon><RefreshRight /></el-icon> 重做
          </el-button>
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
                @focus="onCellFocus(row, column)"
                @change="editTableCell(row, column)"
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
import { ref, onMounted, onUnmounted } from "vue";
import { ArrowRight, Document, Loading, RefreshLeft, RefreshRight } from "@element-plus/icons-vue";
import { apiService } from "../../services/api";

/** 定义属性 */
const props = defineProps({
	tableData: {
		type: Object,
		default: null,
	},
});

/** 定义事件 */
const emit = defineEmits(["save", "cell-select"]);

/** 加载状态 */
const loading = ref(false);

/** 表格数据结构 */
interface TableData {
	id: string;
	[key: string]: string | number | boolean | undefined;
}

/** 表格列定义 */
interface TableColumn {
	prop: string;
	label: string;
	width?: string | number;
	editable?: boolean;
}

/** 单元格属性数据结构 */
interface CellProperties {
	position: string;
	value: string | number | boolean | undefined;
	type: string;
	format: string;
	validation: string;
}

/** 编辑操作记录 */
interface EditAction {
	/** 操作类型 */
	type: 'edit' | 'add-row' | 'delete-row';
	/** 行索引 */
	rowIndex: number;
	/** 列属性名 */
	columnProp?: string;
	/** 旧值 */
	oldValue?: unknown;
	/** 新值 */
	newValue?: unknown;
	/** 被删除的行数据（用于 delete-row 撤销） */
	deletedRow?: Record<string, unknown>;
}

/** 撤销栈 */
const undoStack = ref<EditAction[]>([]);

/** 重做栈 */
const redoStack = ref<EditAction[]>([]);

/** 编辑前的单元格值 */
const beforeEditValue = ref<unknown>(undefined);

/** 处理ID跳转 */
const handleIdClick = (id: string) => {
	console.log("Navigating to ID:", id);
	alert(`跳转到ID ${id} 的原始定义`);
};

/** 保存表格数据 */
const saveTableData = async () => {
	if (props.tableData) {
		try {
			loading.value = true;
			await apiService.saveTable(props.tableData.id, props.tableData.data);
			emit("save", props.tableData.data);
		} catch (error) {
			console.error('保存表格数据失败:', error);
		} finally {
			loading.value = false;
		}
	}
};

/** 记录编辑前的单元格值 */
const onCellFocus = (row: TableData, column: TableColumn) => {
	beforeEditValue.value = row[column.prop];
};

/** 添加表格行 */
const addTableRow = () => {
	if (props.tableData) {
		const rowIndex = props.tableData.data.length;
		const newRow: Record<string, unknown> = {
			id: (rowIndex + 1).toString(),
			name: "",
			age: "",
			email: "",
		};
		props.tableData.data.push(newRow as TableData);
		undoStack.value.push({
			type: 'add-row',
			rowIndex,
			newValue: newRow,
		});
		redoStack.value = [];
	}
};

/** 删除表格行 */
const deleteTableRow = (index: number) => {
	if (props.tableData) {
		const deletedRow = { ...props.tableData.data[index] } as Record<string, unknown>;
		props.tableData.data.splice(index, 1);
		undoStack.value.push({
			type: 'delete-row',
			rowIndex: index,
			deletedRow,
		});
		redoStack.value = [];
	}
};

/** 编辑表格单元格 */
const editTableCell = (row: TableData, column: TableColumn) => {
	const oldValue = beforeEditValue.value;
	const newValue = row[column.prop];
	if (oldValue === newValue) return;

	const rowIndex = props.tableData.data.indexOf(row);
	if (rowIndex === -1) return;

	undoStack.value.push({
		type: 'edit',
		rowIndex,
		columnProp: column.prop,
		oldValue,
		newValue,
	});
	redoStack.value = [];
};

/** 撤销操作 */
const undo = () => {
	const action = undoStack.value.pop();
	if (!action || !props.tableData) return;

	if (action.type === 'edit') {
		const row = props.tableData.data[action.rowIndex];
		if (row && action.columnProp) {
			row[action.columnProp] = action.oldValue as string | number | boolean | undefined;
		}
	} else if (action.type === 'add-row') {
		props.tableData.data.splice(action.rowIndex, 1);
	} else if (action.type === 'delete-row') {
		props.tableData.data.splice(action.rowIndex, 0, action.deletedRow as TableData);
	}

	redoStack.value.push(action);
};

/** 重做操作 */
const redo = () => {
	const action = redoStack.value.pop();
	if (!action || !props.tableData) return;

	if (action.type === 'edit') {
		const row = props.tableData.data[action.rowIndex];
		if (row && action.columnProp) {
			row[action.columnProp] = action.newValue as string | number | boolean | undefined;
		}
	} else if (action.type === 'add-row') {
		props.tableData.data.splice(action.rowIndex, 0, action.newValue as TableData);
	} else if (action.type === 'delete-row') {
		props.tableData.data.splice(action.rowIndex, 1);
	}

	undoStack.value.push(action);
};

/** 键盘快捷键处理 */
const handleKeydown = (event: KeyboardEvent) => {
	if (event.ctrlKey && event.key === 'z') {
		event.preventDefault();
		undo();
	} else if (event.ctrlKey && event.key === 'y') {
		event.preventDefault();
		redo();
	}
};

/** 选择单元格 */
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

onMounted(() => {
	document.addEventListener('keydown', handleKeydown);
});

onUnmounted(() => {
	document.removeEventListener('keydown', handleKeydown);
});
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
