/// XCell Runtime
/// TypeScript runtime utilities for XCell

import { instantiate, type Root } from "../lib/xcell_wasi.js";
import * as preview2Shim from "@bytecodealliance/preview2-shim";

export class XCell {
  private static _root: Root | null = null;
  private static _initPromise: Promise<void> | null = null;
  private static _vmId: number | null = null;

  /**
   * 初始化 XCell
   * @param wasmPath WASM 模块路径
   */
  static async init(wasmPath?: string): Promise<void> {
    if (this._root) return;
    if (this._initPromise) return this._initPromise;

    this._initPromise = (async () => {
      console.log('Initializing XCell...');
      
      // 加载 WASM 模块
      const modules = new Map<string, WebAssembly.Module>();
      
      // 这里需要根据实际生成的 WASM 文件进行调整
      // 暂时使用一个简化的加载方式
      try {
        // 假设 WASM 文件在 lib 目录下
        const wasmUrl = "../lib/xcell_wasi.wasm";
        const res = await fetch(wasmUrl);
        const buf = await res.arrayBuffer();
        const mod = await WebAssembly.compile(buf);
        modules.set("xcell_wasi.wasm", mod);
        
        console.log('WASM module loaded successfully');
        
        // 实例化 WASI 模块
        this._root = await instantiate(
          (path: string) => modules.get(path)!, 
          preview2Shim as any
        );
        
        console.log('WASI module instantiated successfully');
        
        // 创建 VM 实例
        this._vmId = await this._wasi.createVm();
        console.log('VM instance created successfully');
      } catch (error) {
        console.error('Failed to initialize XCell:', error);
        throw error;
      }
    })();
    
    await this._initPromise;
  }

  /**
   * 获取 XCell WASI 接口
   */
  private static get _wasi() {
    if (!this._root) {
      throw new Error('XCell not initialized.');
    }
    return this._root.xcellWasi;
  }

  /**
   * 处理表格数据
   * @param data 表格数据
   * @returns 处理后的结果
   */
  static async processTable(data: any): Promise<any> {
    await this.init();
    const dataStr = JSON.stringify(data);
    const result = await this._wasi.processData(this._vmId, dataStr);
    return JSON.parse(result);
  }

  /**
   * 验证表格数据
   * @param data 表格数据
   * @returns 验证结果
   */
  static async validateTable(data: any): Promise<boolean> {
    await this.init();
    const dataStr = JSON.stringify(data);
    await this._wasi.validateTable(dataStr);
    return true;
  }

  /**
   * 测试 WASI 模块
   * @returns 测试结果
   */
  static async testWasi(): Promise<string> {
    await this.init();
    return this._wasi.hello();
  }

  /**
   * 处理表格文件
   * @param source 源文件路径
   * @param output 输出文件路径
   */
  static async processTableFile(source?: string, output?: string): Promise<void> {
    await this.init();
    await this._wasi.processTable(source, output);
  }

  /**
   * 验证表格文件
   * @param source 源文件路径
   */
  static async validateTableFile(source?: string): Promise<void> {
    await this.init();
    await this._wasi.validateTable(source);
  }
}

export default XCell;