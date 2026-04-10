import { invoke } from '@tauri-apps/api/core';

/** 表格数据类型 */
export interface TableData {
  /** 唯一标识 */
  id: string;
  /** 表格名称 */
  name: string;
  /** 表格类型 */
  type: 'table' | 'class' | 'enum' | 'language';
  /** 文件路径 */
  path: string;
  /** 是否为草稿 */
  draft: boolean;
  /** 创建时间 */
  createdAt: string;
  /** 所属包名 */
  package?: string;
}

/** 表格列定义 */
export interface TableColumn {
  /** 列属性名 */
  prop: string;
  /** 列显示标签 */
  label: string;
  /** 列宽度 */
  width?: string | number;
  /** 是否可编辑 */
  editable?: boolean;
}

/** 表格详情 */
export interface TableDetail {
  /** 唯一标识 */
  id: string;
  /** 表格名称 */
  name: string;
  /** 列定义列表 */
  columns: TableColumn[];
  /** 行数据列表 */
  data: Record<string, unknown>[];
}

/** 验证结果 */
export interface ValidationResult {
  /** 是否通过验证 */
  valid: boolean;
  /** 错误信息列表 */
  errors: string[];
}

/** 检查是否在 Tauri 环境中 */
const isTauri: boolean = typeof window !== 'undefined' && (window as any).__TAURI__ !== undefined;

/** 确保 Tauri 环境可用，否则抛出错误 */
function requireTauri(): void {
  if (!isTauri) {
    throw new Error('此应用需要 Tauri 环境');
  }
}

/** API 服务类，封装所有与 Tauri 后端的通信 */
class ApiService {
  /** 获取表格列表 */
  async getTableList(): Promise<TableData[]> {
    requireTauri();
    try {
      return await invoke<TableData[]>('get_table_list');
    } catch (error) {
      throw new Error(`获取表格列表失败: ${error}`);
    }
  }

  /** 获取表格详情 */
  async getTableDetail(id: string): Promise<TableDetail> {
    requireTauri();
    try {
      return await invoke<TableDetail>('get_table_detail', { tableId: id });
    } catch (error) {
      throw new Error(`获取表格详情失败: ${error}`);
    }
  }

  /** 读取表格文件 */
  async readTable(filePath: string): Promise<TableDetail> {
    requireTauri();
    try {
      return await invoke<TableDetail>('read_table', { filePath });
    } catch (error) {
      throw new Error(`读取表格失败: ${error}`);
    }
  }

  /** 保存表格数据 */
  async saveTable(filePath: string, data: unknown): Promise<boolean> {
    requireTauri();
    try {
      return await invoke<boolean>('save_table', { filePath, data });
    } catch (error) {
      throw new Error(`保存表格失败: ${error}`);
    }
  }

  /** 验证表格数据 */
  async validateTable(filePath: string): Promise<ValidationResult> {
    requireTauri();
    try {
      return await invoke<ValidationResult>('validate_table', { filePath });
    } catch (error) {
      throw new Error(`验证表格失败: ${error}`);
    }
  }

  /** 生成代码 */
  async generateCode(workspacePath: string, target: string): Promise<boolean> {
    requireTauri();
    try {
      return await invoke<boolean>('generate_code', { workspacePath, target });
    } catch (error) {
      throw new Error(`生成代码失败: ${error}`);
    }
  }

  /** 创建表格 */
  async createTable(table: Omit<TableData, 'id' | 'createdAt'>): Promise<TableData> {
    requireTauri();
    try {
      return await invoke<TableData>('create_table', { table });
    } catch (error) {
      throw new Error(`创建表格失败: ${error}`);
    }
  }

  /** 更新表格 */
  async updateTable(id: string, table: Partial<TableData>): Promise<TableData> {
    requireTauri();
    try {
      return await invoke<TableData>('update_table', { tableId: id, table });
    } catch (error) {
      throw new Error(`更新表格失败: ${error}`);
    }
  }

  /** 删除表格 */
  async deleteTable(id: string): Promise<boolean> {
    requireTauri();
    try {
      return await invoke<boolean>('delete_table', { tableId: id });
    } catch (error) {
      throw new Error(`删除表格失败: ${error}`);
    }
  }

  /** 导入表格 */
  async importTable(file: File): Promise<TableData> {
    requireTauri();
    try {
      const content = await file.text();
      return await invoke<TableData>('import_table', { fileName: file.name, fileContent: content });
    } catch (error) {
      throw new Error(`导入表格失败: ${error}`);
    }
  }

  /** 导出表格 */
  async exportTable(id: string): Promise<string> {
    requireTauri();
    try {
      return await invoke<string>('export_table', { tableId: id });
    } catch (error) {
      throw new Error(`导出表格失败: ${error}`);
    }
  }

  /** 打开项目选择对话框 */
  async openProjectDialog(): Promise<string> {
    requireTauri();
    try {
      return await invoke<string>('open_project_dialog');
    } catch (error) {
      throw new Error(`打开项目对话框失败: ${error}`);
    }
  }

  /** 打开编辑器窗口 */
  async openEditorWindow(projectPath: string): Promise<void> {
    requireTauri();
    try {
      await invoke<void>('open_editor_window', { projectPath });
    } catch (error) {
      throw new Error(`打开编辑器窗口失败: ${error}`);
    }
  }
}

/** API 服务单例 */
export const apiService = new ApiService();
