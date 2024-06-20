XCell 配置表管理工具
==================

## 项目架构

XCell 项目由以下几个核心模块组成，按照依赖关系从底层到上层排列：

1. **xcell-types**：核心类型定义库，包含所有数据类型的定义和处理逻辑
2. **xcell-provider**：表格提供者库，负责表格数据的读取和写入
3. **xcell-core**：核心分析库，负责表格数据的解析、验证和分析
4. **xcell-generator**：代码生成器，负责生成各种格式的代码和数据文件
5. **xcell**：命令行工具，提供用户交互接口
6. **xcell-macros**：宏定义库，提供各种宏定义

## 模块职责

- **xcell-types**：定义基础数据类型，如整数、小数、字符串、布尔值、向量等，以及它们的序列化和反序列化逻辑
- **xcell-provider**：提供表格数据的读取和写入功能，支持Excel、CSV等格式
- **xcell-core**：解析表格数据，验证数据完整性，分析数据关系，生成中间数据结构
- **xcell-generator**：根据中间数据结构生成各种格式的代码和数据文件，如C#、TypeScript、JSON等
- **xcell**：提供命令行接口，接收用户命令，调用其他模块的功能
- **xcell-macros**：提供宏定义，简化代码编写

## 命令行使用

```yaml
命令: xcell.exe [OPTIONS] [COMMAND]

指令:
  check:  检查配置表, 但不导出任何文件
  clear:  清除数据库与缓存

选项:
      --workspace <WORKSPACE>  手动设置工作目录, 不输入表示当前目录
  -w, --watch                  启用监听模式, 当有文件修改时只更新对应文件
      --disable-xml            强制关闭 xml 生成
      --disable-json           强制关闭 json 生成
  -h, --help                   显示帮助
  -V, --version                显示版本
```