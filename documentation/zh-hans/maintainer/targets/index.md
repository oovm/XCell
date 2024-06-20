# Targets 目标平台

XCell 支持多种目标平台，包括前端和后端。本章节详细介绍各个平台的集成方式和类型映射。

## 前端平台

前端平台主要面向游戏引擎和前端框架，提供代码生成和数据加载功能。

### 游戏引擎

- [Unity](unity.md) - Unity 引擎集成
- [Cocos](cocos.md) - Cocos 引擎集成
- [Unreal Engine](unreal.md) - Unreal Engine 集成
- [Godot](godot.md) - Godot 引擎集成
- [XLua](xlua.md) - XLua 脚本集成

### 前端框架

- [React](react.md) - React 框架集成
- [Vue](vue.md) - Vue 框架集成
- [Angular](angular.md) - Angular 框架集成
- [TypeScript/JavaScript](typescript.md) - TypeScript/JavaScript 集成

## 后端平台

后端平台主要面向服务器端和数据存储，提供数据持久化和服务端集成。

### 数据库

- [MySQL](mysql.md) - MySQL 数据库集成
- [PostgreSQL](postgresql.md) - PostgreSQL 数据库集成
- [SQLite](sqlite.md) - SQLite 数据库集成
- [SQL](sql.md) - 通用 SQL 数据库集成

### 服务端框架

- [Node.js](nodejs.md) - Node.js 集成
- [Python](python.md) - Python 集成
- [Java](java.md) - Java 集成
- [Go](go.md) - Go 语言集成

### 数据格式

- [JSON](json.md) - JSON 数据格式
- [XML](xml.md) - XML 数据格式
- [Protobuf](protobuf.md) - Protobuf 数据格式

## 跨平台支持

XCell 设计为跨平台的，支持在不同平台间共享配置数据，确保数据一致性和开发效率。

### 平台间数据共享

- 统一的数据模型
- 跨平台类型映射
- 标准化的配置格式

### 最佳实践

- 根据目标平台选择合适的数据格式
- 利用合表规则管理跨平台数据
- 定期同步各平台的数据结构
