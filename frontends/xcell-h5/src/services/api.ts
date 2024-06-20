import ErrorHandler from '../utils/errorHandler';

// 检查是否在 Tauri 环境中
const isTauri = typeof window !== 'undefined' && (window as any).__TAURI__ !== undefined;

// 动态导入 Tauri API
let readDir: any = null;
let readTextFile: any = null;
let writeTextFile: any = null;
let createDir: any = null;
let removeFile: any = null;
let join: any = null;
let dirname: any = null;

if (isTauri) {
  // 动态导入 Tauri API
  // 注意：在实际使用中，需要确保 Tauri API 的导入路径正确
  try {
    // 尝试导入 Tauri API
    import('@tauri-apps/api').then((tauri: any) => {
      // 使用类型断言来避免 TypeScript 错误
      if (tauri.fs && tauri.path) {
        readDir = tauri.fs.readDir;
        readTextFile = tauri.fs.readTextFile;
        writeTextFile = tauri.fs.writeTextFile;
        createDir = tauri.fs.createDir;
        removeFile = tauri.fs.removeFile;
        join = tauri.path.join;
        dirname = tauri.path.dirname;
      } else {
        console.warn('Tauri API does not have fs or path properties');
      }
    });
  } catch (error) {
    console.error('Failed to import Tauri API:', error);
  }
}

// 表格数据类型
export interface TableData {
  id: string;
  name: string;
  type: 'table' | 'class' | 'enum' | 'language';
  path: string;
  draft: boolean;
  createdAt: string;
  package?: string;
}

// 表格详情类型
export interface TableDetail {
  id: string;
  name: string;
  columns: {
    prop: string;
    label: string;
    width?: string | number;
    editable?: boolean;
  }[];
  data: {
    id: string;
    [key: string]: string | number | boolean | undefined;
  }[];
}

// API 服务类
class ApiService {
  /**
   * 获取表格列表
   * @returns 表格列表
   */
  async getTableList(): Promise<TableData[]> {
    try {
      // 从本地存储获取项目路径
      const projectPath = localStorage.getItem('projectPath');
      if (!projectPath) {
        return [];
      }

      // 检查是否在 Tauri 环境中
      if (isTauri && readDir && join) {
        // 读取项目目录下的表格文件
        const tablesDir = await join(projectPath, 'tables');
        const classesDir = await join(projectPath, 'classes');
        const enumsDir = await join(projectPath, 'enums');
        const languagesDir = await join(projectPath, 'languages');

        const tableList: TableData[] = [];

        // 读取表格目录
        try {
          const tables = await readDir(tablesDir);
          for (const file of tables) {
            if (file.name.endsWith('.xlsx') || file.name.endsWith('.csv')) {
              tableList.push({
                id: file.name,
                name: file.name.replace(/\.[^/.]+$/, ''),
                type: 'table',
                path: await join('tables', file.name),
                draft: false,
                createdAt: file.modifiedAt?.toISOString() || new Date().toISOString()
              });
            }
          }
        } catch (error) {
          // 目录不存在，忽略错误
        }

        // 读取类目录
        try {
          const classes = await readDir(classesDir);
          for (const file of classes) {
            if (file.name.endsWith('.xlsx') || file.name.endsWith('.csv')) {
              tableList.push({
                id: file.name,
                name: file.name.replace(/\.[^/.]+$/, ''),
                type: 'class',
                path: await join('classes', file.name),
                draft: false,
                createdAt: file.modifiedAt?.toISOString() || new Date().toISOString()
              });
            }
          }
        } catch (error) {
          // 目录不存在，忽略错误
        }

        // 读取枚举目录
        try {
          const enums = await readDir(enumsDir);
          for (const file of enums) {
            if (file.name.endsWith('.xlsx') || file.name.endsWith('.csv')) {
              tableList.push({
                id: file.name,
                name: file.name.replace(/\.[^/.]+$/, ''),
                type: 'enum',
                path: await join('enums', file.name),
                draft: false,
                createdAt: file.modifiedAt?.toISOString() || new Date().toISOString()
              });
            }
          }
        } catch (error) {
          // 目录不存在，忽略错误
        }

        // 读取语言目录
        try {
          const languages = await readDir(languagesDir);
          for (const file of languages) {
            if (file.name.endsWith('.xlsx') || file.name.endsWith('.csv')) {
              tableList.push({
                id: file.name,
                name: file.name.replace(/\.[^/.]+$/, ''),
                type: 'language',
                path: await join('languages', file.name),
                draft: false,
                createdAt: file.modifiedAt?.toISOString() || new Date().toISOString()
              });
            }
          }
        } catch (error) {
          // 目录不存在，忽略错误
        }

        return tableList;
      } else {
        // 非 Tauri 环境，返回模拟数据
        return [
          {
            id: "1",
            name: "ExampleTable",
            type: "table",
            path: "tables/ExampleTable.xlsx",
            draft: false,
            createdAt: "2024-01-01 10:00:00"
          },
          {
            id: "2",
            name: "UserClass",
            type: "class",
            path: "classes/UserClass.xlsx",
            draft: false,
            createdAt: "2024-01-02 14:30:00"
          },
          {
            id: "3",
            name: "StatusEnum",
            type: "enum",
            path: "enums/StatusEnum.xlsx",
            draft: true,
            createdAt: "2024-01-03 09:15:00"
          },
          {
            id: "4",
            name: "LanguagePack",
            type: "language",
            path: "languages/LanguagePack.xlsx",
            draft: false,
            createdAt: "2024-01-04 11:45:00"
          },
          {
            id: "5",
            name: "ProductTable",
            type: "table",
            path: "tables/ProductTable.xlsx",
            draft: false,
            createdAt: "2024-01-05 16:20:00"
          }
        ];
      }
    } catch (error) {
      console.error('获取表格列表失败:', error);
      ErrorHandler.handleError(error, '获取表格列表失败');
      throw error;
    }
  }

  /**
   * 获取表格详情
   * @param id 表格 ID
   * @returns 表格详情
   */
  async getTableDetail(id: string): Promise<TableDetail> {
    try {
      // 从本地存储获取项目路径
      const projectPath = localStorage.getItem('projectPath');
      if (!projectPath) {
        throw new Error('项目路径未设置');
      }

      // 检查是否在 Tauri 环境中
      if (isTauri && readTextFile && join) {
        // 构建表格文件路径
        let tablePath: string;
        if (id.includes('/')) {
          tablePath = await join(projectPath, id);
        } else {
          // 尝试在不同目录中查找
          const possiblePaths = [
            await join(projectPath, 'tables', id),
            await join(projectPath, 'classes', id),
            await join(projectPath, 'enums', id),
            await join(projectPath, 'languages', id)
          ];

          // 找到第一个存在的文件
          let found = false;
          for (const path of possiblePaths) {
            try {
              // 检查文件是否存在
              await readTextFile(path);
              tablePath = path;
              found = true;
              break;
            } catch {
              // 文件不存在，继续尝试
            }
          }

          if (!found) {
            throw new Error('表格文件不存在');
          }
        }

        // 这里应该实现读取表格文件的逻辑
        // 暂时返回示例数据
        return {
          id,
          name: id.replace(/\.[^/.]+$/, ''),
          columns: [
            { prop: 'id', label: 'ID', width: 100, editable: false },
            { prop: 'name', label: '名称', width: 200, editable: true },
            { prop: 'value', label: '值', width: 300, editable: true }
          ],
          data: [
            { id: '1', name: 'Item 1', value: 'Value 1' },
            { id: '2', name: 'Item 2', value: 'Value 2' },
            { id: '3', name: 'Item 3', value: 'Value 3' }
          ]
        };
      } else {
        // 非 Tauri 环境，返回模拟数据
        return {
          id,
          name: id.replace(/\.[^/.]+$/, ''),
          columns: [
            { prop: 'id', label: 'ID', width: 100, editable: false },
            { prop: 'name', label: '名称', width: 200, editable: true },
            { prop: 'age', label: '年龄', width: 100, editable: true },
            { prop: 'email', label: '邮箱', width: 300, editable: true }
          ],
          data: [
            { id: '1', name: '张三', age: 25, email: 'zhangsan@example.com' },
            { id: '2', name: '李四', age: 30, email: 'lisi@example.com' },
            { id: '3', name: '王五', age: 35, email: 'wangwu@example.com' }
          ]
        };
      }
    } catch (error) {
      console.error('获取表格详情失败:', error);
      ErrorHandler.handleError(error, '获取表格详情失败');
      throw error;
    }
  }

  /**
   * 保存表格数据
   * @param id 表格 ID
   * @param data 表格数据
   * @returns 保存结果
   */
  async saveTableData(id: string, data: any[]): Promise<boolean> {
    try {
      // 从本地存储获取项目路径
      const projectPath = localStorage.getItem('projectPath');
      if (!projectPath) {
        throw new Error('项目路径未设置');
      }

      // 检查是否在 Tauri 环境中
      if (isTauri && writeTextFile && join && dirname && createDir) {
        // 构建表格文件路径
        let tablePath: string;
        if (id.includes('/')) {
          tablePath = await join(projectPath, id);
        } else {
          // 默认保存到 tables 目录
          tablePath = await join(projectPath, 'tables', id);
        }

        // 确保目录存在
        const tableDir = await dirname(tablePath);
        try {
          await createDir(tableDir, { recursive: true });
        } catch (error) {
          // 目录已存在，忽略错误
        }

        // 这里应该实现保存表格文件的逻辑
        // 暂时将数据保存为 JSON 文件
        const jsonData = JSON.stringify(data, null, 2);
        await writeTextFile(tablePath.replace(/\.[^/.]+$/, '.json'), jsonData);

        ErrorHandler.showSuccess('表格保存成功');
        return true;
      } else {
        // 非 Tauri 环境，模拟保存
        console.log('模拟保存表格数据:', id, data);
        ErrorHandler.showSuccess('表格保存成功');
        return true;
      }
    } catch (error) {
      console.error('保存表格数据失败:', error);
      ErrorHandler.handleError(error, '保存表格数据失败');
      throw error;
    }
  }

  /**
   * 创建新表格
   * @param table 表格信息
   * @returns 创建结果
   */
  async createTable(table: Omit<TableData, 'id' | 'createdAt'>): Promise<TableData> {
    try {
      // 从本地存储获取项目路径
      const projectPath = localStorage.getItem('projectPath');
      if (!projectPath) {
        throw new Error('项目路径未设置');
      }

      // 检查是否在 Tauri 环境中
      if (isTauri && writeTextFile && join && dirname && createDir) {
        // 构建表格文件路径
        const tablePath = await join(projectPath, table.path);

        // 确保目录存在
        const tableDir = await dirname(tablePath);
        try {
          await createDir(tableDir, { recursive: true });
        } catch (error) {
          // 目录已存在，忽略错误
        }

        // 创建空的表格文件
        await writeTextFile(tablePath, '');

        // 返回创建的表格信息
        const result: TableData = {
          ...table,
          id: table.path,
          createdAt: new Date().toISOString()
        };

        ErrorHandler.showSuccess('表格创建成功');
        return result;
      } else {
        // 非 Tauri 环境，模拟创建
        console.log('模拟创建表格:', table);
        const result: TableData = {
          ...table,
          id: table.path,
          createdAt: new Date().toISOString()
        };
        ErrorHandler.showSuccess('表格创建成功');
        return result;
      }
    } catch (error) {
      console.error('创建表格失败:', error);
      ErrorHandler.handleError(error, '创建表格失败');
      throw error;
    }
  }

  /**
   * 更新表格信息
   * @param id 表格 ID
   * @param table 表格信息
   * @returns 更新结果
   */
  async updateTable(id: string, table: Partial<TableData>): Promise<TableData> {
    try {
      // 从本地存储获取项目路径
      const projectPath = localStorage.getItem('projectPath');
      if (!projectPath) {
        throw new Error('项目路径未设置');
      }

      // 检查是否在 Tauri 环境中
      if (isTauri && readTextFile && writeTextFile && join && dirname && createDir && removeFile) {
        // 构建旧表格文件路径
        let oldPath: string = '';
        if (id.includes('/')) {
          oldPath = await join(projectPath, id);
        } else {
          // 尝试在不同目录中查找
          const possiblePaths = [
            await join(projectPath, 'tables', id),
            await join(projectPath, 'classes', id),
            await join(projectPath, 'enums', id),
            await join(projectPath, 'languages', id)
          ];

          // 找到第一个存在的文件
          let found = false;
          for (const path of possiblePaths) {
            try {
              // 检查文件是否存在
              await readTextFile(path);
              oldPath = path;
              found = true;
              break;
            } catch {
              // 文件不存在，继续尝试
            }
          }

          if (!found) {
            throw new Error('表格文件不存在');
          }
        }

        // 如果路径改变，需要重命名文件
        if (table.path && table.path !== id) {
          const newPath = await join(projectPath, table.path);
          
          // 确保新目录存在
          const newDir = await dirname(newPath);
          try {
            await createDir(newDir, { recursive: true });
          } catch (error) {
            // 目录已存在，忽略错误
          }

          // 读取旧文件内容
          const content = await readTextFile(oldPath);
          // 写入新文件
          await writeTextFile(newPath, content);
          // 删除旧文件
          await removeFile(oldPath);
        }

        // 返回更新后的表格信息
        const result: TableData = {
          id: table.path || id,
          name: table.name || 'Unknown',
          type: table.type || 'table',
          path: table.path || id,
          draft: table.draft || false,
          createdAt: table.createdAt || new Date().toISOString()
        };

        ErrorHandler.showSuccess('表格更新成功');
        return result;
      } else {
        // 非 Tauri 环境，模拟更新
        console.log('模拟更新表格:', id, table);
        const result: TableData = {
          id: table.path || id,
          name: table.name || 'Unknown',
          type: table.type || 'table',
          path: table.path || id,
          draft: table.draft || false,
          createdAt: table.createdAt || new Date().toISOString()
        };
        ErrorHandler.showSuccess('表格更新成功');
        return result;
      }
    } catch (error) {
      console.error('更新表格失败:', error);
      ErrorHandler.handleError(error, '更新表格失败');
      throw error;
    }
  }

  /**
   * 删除表格
   * @param id 表格 ID
   * @returns 删除结果
   */
  async deleteTable(id: string): Promise<boolean> {
    try {
      // 从本地存储获取项目路径
      const projectPath = localStorage.getItem('projectPath');
      if (!projectPath) {
        throw new Error('项目路径未设置');
      }

      // 检查是否在 Tauri 环境中
      if (isTauri && removeFile && readTextFile && join) {
        // 构建表格文件路径
        let tablePath: string = '';
        if (id.includes('/')) {
          tablePath = await join(projectPath, id);
        } else {
          // 尝试在不同目录中查找
          const possiblePaths = [
            await join(projectPath, 'tables', id),
            await join(projectPath, 'classes', id),
            await join(projectPath, 'enums', id),
            await join(projectPath, 'languages', id)
          ];

          // 找到第一个存在的文件
          let found = false;
          for (const path of possiblePaths) {
            try {
              // 检查文件是否存在
              await readTextFile(path);
              tablePath = path;
              found = true;
              break;
            } catch {
              // 文件不存在，继续尝试
            }
          }

          if (!found) {
            throw new Error('表格文件不存在');
          }
        }

        // 删除表格文件
        await removeFile(tablePath);

        ErrorHandler.showSuccess('表格删除成功');
        return true;
      } else {
        // 非 Tauri 环境，模拟删除
        console.log('模拟删除表格:', id);
        ErrorHandler.showSuccess('表格删除成功');
        return true;
      }
    } catch (error) {
      console.error('删除表格失败:', error);
      ErrorHandler.handleError(error, '删除表格失败');
      throw error;
    }
  }

  /**
   * 导入表格
   * @param file 文件对象
   * @returns 导入结果
   */
  async importTable(file: File): Promise<TableData> {
    try {
      // 从本地存储获取项目路径
      const projectPath = localStorage.getItem('projectPath');
      if (!projectPath) {
        throw new Error('项目路径未设置');
      }

      // 检查是否在 Tauri 环境中
      if (isTauri && writeTextFile && join && dirname && createDir) {
        // 构建表格文件路径
        const tablePath = await join(projectPath, 'tables', file.name);

        // 确保目录存在
        const tableDir = await dirname(tablePath);
        try {
          await createDir(tableDir, { recursive: true });
        } catch (error) {
          // 目录已存在，忽略错误
        }

        // 读取文件内容
        const reader = new FileReader();
        const fileContent = await new Promise<ArrayBuffer>((resolve, reject) => {
          reader.onload = () => resolve(reader.result as ArrayBuffer);
          reader.onerror = reject;
          reader.readAsArrayBuffer(file);
        });

        // 这里应该实现保存文件的逻辑
        // 暂时将文件保存为文本文件
        const textContent = new TextDecoder().decode(fileContent);
        await writeTextFile(tablePath, textContent);

        // 返回导入的表格信息
        const result: TableData = {
          id: `tables/${file.name}`,
          name: file.name.replace(/\.[^/.]+$/, ''),
          type: 'table',
          path: `tables/${file.name}`,
          draft: false,
          createdAt: new Date().toISOString()
        };

        ErrorHandler.showSuccess('表格导入成功');
        return result;
      } else {
        // 非 Tauri 环境，模拟导入
        console.log('模拟导入表格:', file.name);
        const result: TableData = {
          id: `tables/${file.name}`,
          name: file.name.replace(/\.[^/.]+$/, ''),
          type: 'table',
          path: `tables/${file.name}`,
          draft: false,
          createdAt: new Date().toISOString()
        };
        ErrorHandler.showSuccess('表格导入成功');
        return result;
      }
    } catch (error) {
      console.error('导入表格失败:', error);
      ErrorHandler.handleError(error, '导入表格失败');
      throw error;
    }
  }

  /**
   * 导出表格
   * @param id 表格 ID
   * @returns 导出结果
   */
  async exportTable(id: string): Promise<Blob> {
    try {
      // 从本地存储获取项目路径
      const projectPath = localStorage.getItem('projectPath');
      if (!projectPath) {
        throw new Error('项目路径未设置');
      }

      // 构建表格文件路径
      let tablePath: string = '';
      if (id.includes('/')) {
        tablePath = await join(projectPath, id);
      } else {
        // 尝试在不同目录中查找
        const possiblePaths = [
          await join(projectPath, 'tables', id),
          await join(projectPath, 'classes', id),
          await join(projectPath, 'enums', id),
          await join(projectPath, 'languages', id)
        ];

        // 找到第一个存在的文件
        let found = false;
        for (const path of possiblePaths) {
          try {
            // 检查文件是否存在
            await readTextFile(path);
            tablePath = path;
            found = true;
            break;
          } catch {
            // 文件不存在，继续尝试
          }
        }

        if (!found) {
          throw new Error('表格文件不存在');
        }
      }

      // 读取表格文件内容
      const content = await readTextFile(tablePath);

      // 将内容转换为 Blob
      const blob = new Blob([content], { type: 'application/json' });

      ErrorHandler.showSuccess('表格导出成功');
      return blob;
    } catch (error) {
      console.error('导出表格失败:', error);
      ErrorHandler.handleError(error, '导出表格失败');
      throw error;
    }
  }
}

export const apiService = new ApiService();