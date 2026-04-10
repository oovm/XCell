# XCell 依赖替换 - 实施计划

## [ ] 任务 1: 移除根目录 Cargo.toml 中的 serde_json 依赖
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 从根目录 Cargo.toml 文件中移除 serde_json 依赖
  - 确保 oak-json 依赖配置正确
- **Acceptance Criteria Addressed**: AC-1
- **Test Requirements**:
  - `programmatic` TR-1.1: 根目录 Cargo.toml 文件中不再包含 serde_json 依赖
  - `programmatic` TR-1.2: 根目录 Cargo.toml 文件中包含 oak-json 依赖
- **Notes**: 根目录 Cargo.toml 中已经包含 oak-json 依赖，只需要移除 serde_json

## [ ] 任务 2: 移除子项目中的 serde_json 依赖
- **Priority**: P0
- **Depends On**: 任务 1
- **Description**: 
  - 从所有子项目的 Cargo.toml 文件中移除 serde_json 依赖
  - 确保所有子项目都使用 oak-json 作为 JSON 解析库
- **Acceptance Criteria Addressed**: AC-1
- **Test Requirements**:
  - `programmatic` TR-2.1: 所有子项目的 Cargo.toml 文件中不再包含 serde_json 依赖
  - `programmatic` TR-2.2: 所有子项目都正确配置了 oak-json 依赖
- **Notes**: 需要检查 backends 目录下的所有子项目

## [ ] 任务 3: 移除子项目中的 toml 依赖
- **Priority**: P0
- **Depends On**: 任务 1
- **Description**: 
  - 从所有子项目的 Cargo.toml 文件中移除 toml 依赖
  - 确保所有子项目都使用 oak-toml 作为 TOML 解析库
- **Acceptance Criteria Addressed**: AC-1
- **Test Requirements**:
  - `programmatic` TR-3.1: 所有子项目的 Cargo.toml 文件中不再包含 toml 依赖
  - `programmatic` TR-3.2: 所有子项目都正确配置了 oak-toml 依赖
- **Notes**: 需要检查 backends 目录下的所有子项目

## [ ] 任务 4: 修改代码中的 serde_json 使用方式
- **Priority**: P0
- **Depends On**: 任务 2
- **Description**: 
  - 修改所有使用 serde_json 的代码，替换为 oak-json 的使用方式
  - 确保代码在替换后能够正常编译和运行
- **Acceptance Criteria Addressed**: AC-1, AC-2
- **Test Requirements**:
  - `programmatic` TR-4.1: 所有代码文件中不再使用 serde_json 库
  - `programmatic` TR-4.2: 所有代码文件都使用 oak-json 库
- **Notes**: 需要搜索并替换所有使用 serde_json 的代码

## [ ] 任务 5: 修改代码中的 toml 使用方式
- **Priority**: P0
- **Depends On**: 任务 3
- **Description**: 
  - 修改所有使用 toml 的代码，替换为 oak-toml 的使用方式
  - 确保代码在替换后能够正常编译和运行
- **Acceptance Criteria Addressed**: AC-1, AC-2
- **Test Requirements**:
  - `programmatic` TR-5.1: 所有代码文件中不再使用 toml 库
  - `programmatic` TR-5.2: 所有代码文件都使用 oak-toml 库
- **Notes**: 需要搜索并替换所有使用 toml 的代码

## [ ] 任务 6: 构建项目
- **Priority**: P0
- **Depends On**: 任务 4, 任务 5
- **Description**: 
  - 执行 cargo build 命令构建项目
  - 确保项目能够成功构建，无编译错误
- **Acceptance Criteria Addressed**: AC-2
- **Test Requirements**:
  - `programmatic` TR-6.1: cargo build 命令执行成功，无编译错误
- **Notes**: 构建整个项目，确保所有依赖都正确解析

## [ ] 任务 7: 运行测试
- **Priority**: P0
- **Depends On**: 任务 6
- **Description**: 
  - 执行 cargo test 命令运行所有测试
  - 确保所有测试都通过
- **Acceptance Criteria Addressed**: AC-3
- **Test Requirements**:
  - `programmatic` TR-7.1: cargo test 命令执行成功，所有测试通过
- **Notes**: 运行整个项目的测试套件，确保功能正常

## [ ] 任务 8: 验证功能
- **Priority**: P1
- **Depends On**: 任务 7
- **Description**: 
  - 运行项目的主要功能，验证功能是否与替换前保持一致
  - 确保所有核心功能都能正常工作
- **Acceptance Criteria Addressed**: AC-4
- **Test Requirements**:
  - `human-judgment` TR-8.1: 项目的主要功能正常运行
  - `human-judgment` TR-8.2: 功能与替换前保持一致
- **Notes**: 手动验证项目的核心功能