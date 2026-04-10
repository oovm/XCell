import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { apiService, type TableData, type TableDetail } from "../services/api";

/** 表格状态 Store */
export const useTableStore = defineStore("table", () => {
  /** 表格列表 */
  const tableList = ref<TableData[]>([]);

  /** 当前选中的表格 ID */
  const currentTableId = ref<string>("");

  /** 当前表格详情 */
  const currentTableDetail = ref<TableDetail | null>(null);

  /** 加载状态 */
  const isLoading = ref<boolean>(false);

  /** 搜索过滤文本 */
  const filterText = ref<string>("");

  /** 过滤后的表格列表 */
  const filteredTableList = computed(() => {
    if (!filterText.value) {
      return tableList.value;
    }
    const search = filterText.value.toLowerCase();
    return tableList.value.filter(
      (item) =>
        item.name.toLowerCase().includes(search) ||
        item.type.toLowerCase().includes(search) ||
        item.path.toLowerCase().includes(search)
    );
  });

  /** 当前选中的表格数据 */
  const currentTable = computed(
    () => tableList.value.find((t) => t.id === currentTableId.value) || null
  );

  /** 加载表格列表 */
  async function loadTableList(): Promise<void> {
    try {
      isLoading.value = true;
      tableList.value = await apiService.getTableList();
    } catch (error) {
      console.error("加载表格列表失败:", error);
      tableList.value = [];
    } finally {
      isLoading.value = false;
    }
  }

  /** 加载表格详情 */
  async function loadTableDetail(id: string): Promise<void> {
    try {
      isLoading.value = true;
      currentTableId.value = id;
      currentTableDetail.value = await apiService.getTableDetail(id);
    } catch (error) {
      console.error("加载表格详情失败:", error);
      currentTableDetail.value = null;
    } finally {
      isLoading.value = false;
    }
  }

  /** 删除表格 */
  async function deleteTable(id: string): Promise<boolean> {
    try {
      await apiService.deleteTable(id);
      const index = tableList.value.findIndex((t) => t.id === id);
      if (index !== -1) {
        tableList.value.splice(index, 1);
      }
      if (currentTableId.value === id) {
        currentTableId.value = "";
        currentTableDetail.value = null;
      }
      return true;
    } catch (error) {
      console.error("删除表格失败:", error);
      return false;
    }
  }

  /** 更新表格 */
  async function updateTable(
    id: string,
    data: Partial<TableData>
  ): Promise<boolean> {
    try {
      const updated = await apiService.updateTable(id, data);
      const index = tableList.value.findIndex((t) => t.id === id);
      if (index !== -1) {
        tableList.value[index] = updated;
      }
      return true;
    } catch (error) {
      console.error("更新表格失败:", error);
      return false;
    }
  }

  /** 清除当前表格 */
  function clearCurrentTable(): void {
    currentTableId.value = "";
    currentTableDetail.value = null;
  }

  /** 设置过滤文本 */
  function setFilterText(text: string): void {
    filterText.value = text;
  }

  return {
    tableList,
    currentTableId,
    currentTableDetail,
    isLoading,
    filterText,
    filteredTableList,
    currentTable,
    loadTableList,
    loadTableDetail,
    deleteTable,
    updateTable,
    clearCurrentTable,
    setFilterText,
  };
});
