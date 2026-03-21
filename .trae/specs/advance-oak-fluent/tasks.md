# 推进 Oak Fluent 开发 - 实现计划

## [x] Task 1: 实现基本的 FTL 解析功能
- **Priority**: P0
- **Depends On**: None
- **Description**:
  - 实现 parse 函数，支持基本的 FTL 语法
  - 支持简单的消息定义和文本内容
  - 实现基本的错误处理
- **Acceptance Criteria Addressed**: AC-1, AC-5
- **Test Requirements**:
  - `programmatic` TR-1.1: 能够解析简单的 FTL 字符串
  - `programmatic` TR-1.2: 能够处理无效的 FTL 语法并返回错误信息
- **Notes**: 参考 Fluent 规范，实现核心解析逻辑

## [x] Task 2: 实现变量和消息引用支持
- **Priority**: P0
- **Depends On**: Task 1
- **Description**:
  - 支持 FTL 中的变量引用（如 `{ $name }`）
  - 支持消息引用（如 `{ message-id }`）
  - 确保解析器能够正确处理这些语法
- **Acceptance Criteria Addressed**: AC-1, AC-3
- **Test Requirements**:
  - `programmatic` TR-2.1: 能够解析包含变量的 FTL 消息
  - `programmatic` TR-2.2: 能够解析包含消息引用的 FTL 消息
- **Notes**: 注意处理嵌套引用的情况

## [x] Task 3: 实现复数形式和性别形式支持
- **Priority**: P0
- **Depends On**: Task 2
- **Description**:
  - 支持 FTL 中的复数形式语法（如 `{ $count -> [one] ... *[other] ... }`）
  - 支持性别形式语法
  - 实现选择表达式的解析和处理
- **Acceptance Criteria Addressed**: AC-1, AC-4
- **Test Requirements**:
  - `programmatic` TR-3.1: 能够解析包含复数形式的 FTL 消息
  - `programmatic` TR-3.2: 能够解析包含性别形式的 FTL 消息
- **Notes**: 参考 Fluent 规范中的复数规则

## [x] Task 4: 完善翻译器功能
- **Priority**: P0
- **Depends On**: Task 3
- **Description**:
  - 完善 Translator 结构体的功能
  - 确保能够正确处理变量替换
  - 确保能够正确处理复数形式和性别形式
  - 实现消息属性的支持
- **Acceptance Criteria Addressed**: AC-2, AC-3, AC-4
- **Test Requirements**:
  - `programmatic` TR-4.1: 能够正确翻译基本消息
  - `programmatic` TR-4.2: 能够正确翻译带参数的消息
  - `programmatic` TR-4.3: 能够正确翻译复数形式的消息
- **Notes**: 确保翻译器与解析器的输出格式兼容

## [x] Task 5: 编写测试用例
- **Priority**: P1
- **Depends On**: Task 4
- **Description**:
  - 为解析器编写单元测试
  - 为翻译器编写单元测试
  - 编写集成测试，测试完整的翻译流程
- **Acceptance Criteria Addressed**: AC-1, AC-2, AC-3, AC-4, AC-5
- **Test Requirements**:
  - `programmatic` TR-5.1: 所有测试通过
  - `human-judgment` TR-5.2: 测试覆盖全面，包括边界情况
- **Notes**: 测试用例应覆盖各种 FTL 语法和翻译场景

## [/] Task 6: 优化性能和错误处理
- **Priority**: P1
- **Depends On**: Task 5
- **Description**:
  - 优化解析器的性能
  - 改进错误处理，提供更清晰的错误信息
  - 优化翻译器的性能，特别是缓存机制
- **Acceptance Criteria Addressed**: NFR-1, NFR-4
- **Test Requirements**:
  - `programmatic` TR-6.1: 解析性能不劣于 fluent-bundle
  - `human-judgment` TR-6.2: 错误信息清晰易懂
- **Notes**: 可以使用性能基准测试来验证性能改进

## [ ] Task 7: 编写文档
- **Priority**: P2
- **Depends On**: Task 6
- **Description**:
  - 为解析器编写文档
  - 为翻译器编写文档
  - 提供使用示例和最佳实践
- **Acceptance Criteria Addressed**: NFR-3
- **Test Requirements**:
  - `human-judgment` TR-7.1: 文档完整清晰
  - `human-judgment` TR-7.2: 示例代码能够正常运行
- **Notes**: 文档应包括 API 参考和使用指南