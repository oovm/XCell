<template>
  <div class="project-container">
    <ProjectHeader />
    <div class="project-content">
      <div class="project-info-section">
        <h2>项目信息</h2>
        <el-card class="project-info-card" shadow="hover">
          <el-form label-width="150px" class="project-form">
            <el-form-item label="版本号">
              <el-input v-model="projectConfig.version" placeholder="输入版本号" class="form-input" />
            </el-form-item>
            <el-form-item label="包含的 Excel 路径">
              <el-input v-model="projectConfig.include" placeholder="输入包含的 Excel 路径" class="form-input" />
            </el-form-item>
            <el-form-item label="排除的 Excel 模式">
              <el-input v-model="projectConfig.exclude" placeholder="输入排除的 Excel 模式" class="form-input" />
            </el-form-item>
            <el-form-item label="行列排序模式">
              <el-select v-model="projectConfig.line" placeholder="选择行列排序模式" class="form-select">
                <el-option label="按行" value="row" />
                <el-option label="按列" value="column" />
              </el-select>
            </el-form-item>
            
            <el-form-item label="类型解析模式">
              <el-select v-model="projectConfig.typing" placeholder="选择类型解析模式" class="form-select">
                <el-option label="严格" value="strict" />
                <el-option label="宽松" value="relaxed" />
              </el-select>
            </el-form-item>
            
            <el-form-item label="合表模式">
              <el-select v-model="projectConfig.merge" placeholder="选择合表模式" class="form-select">
                <el-option label="按目录" value="directory" />
                <el-option label="按前缀" value="prefix" />
              </el-select>
            </el-form-item>
            
            <el-form-item class="form-buttons">
              <el-button type="success" class="generate-btn" @click="handleGenerate">一键生成</el-button>
            </el-form-item>
          </el-form>
        </el-card>
      </div>
      
      <!-- 表头控制卡片 -->
      <div class="header-control-section">
        <h2>表头控制</h2>
        <el-card class="header-control-card" shadow="hover">
          <el-form label-width="150px" class="header-control-form">
            <!-- 表头含义设置 -->
            <el-form-item label="第一行表头">
              <el-select v-model="projectConfig.headers.firstRow" placeholder="选择第一行表头含义" class="form-select">
                <el-option label="字段" value="字段" />
                <el-option label="类型" value="类型" />
                <el-option label="注释" value="注释" />
                <el-option label="忽略" value="忽略" />
              </el-select>
            </el-form-item>
            <el-form-item label="第二行表头">
              <el-select v-model="projectConfig.headers.secondRow" placeholder="选择第二行表头含义" class="form-select">
                <el-option label="字段" value="字段" />
                <el-option label="类型" value="类型" />
                <el-option label="注释" value="注释" />
                <el-option label="忽略" value="忽略" />
              </el-select>
            </el-form-item>
            <el-form-item label="第三行表头">
              <el-select v-model="projectConfig.headers.thirdRow" placeholder="选择第三行表头含义" class="form-select">
                <el-option label="字段" value="字段" />
                <el-option label="类型" value="类型" />
                <el-option label="注释" value="注释" />
                <el-option label="忽略" value="忽略" />
              </el-select>
            </el-form-item>
            
            <!-- 起始内容偏移量 -->
            <el-form-item label="起始内容偏移">
              <div class="offset-controls">
                <el-form-item label="X (列)" class="offset-item">
                  <el-input-number v-model="projectConfig.offset.x" :min="0" class="form-input" />
                </el-form-item>
                <el-form-item label="Y (行)" class="offset-item">
                  <el-input-number v-model="projectConfig.offset.y" :min="0" class="form-input" />
                </el-form-item>
              </div>
            </el-form-item>
          </el-form>
        </el-card>
      </div>
      
      <!-- 生成日志控制台 -->
      <div class="console-section" v-if="showConsole">
        <h2>生成日志</h2>
        <el-card class="console-card" shadow="hover">
          <div class="console-content">
            <div v-for="(log, index) in logs" :key="index" :class="['log-item', log.type]">
              <span class="log-time">{{ log.time }}</span>
              <span class="log-content">{{ log.content }}</span>
            </div>
          </div>
          <div class="console-actions">
            <el-button type="default" size="small" @click="clearLogs">清空日志</el-button>
          </div>
        </el-card>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import ProjectHeader from "../components/features/ProjectHeader.vue";

// 项目配置数据结构
const projectConfig = ref({
  version: '1.0.0',
  include: 'excel/**/*.xlsx',
  exclude: '**/temp/**',
  line: 'row',
  typing: 'strict',
  merge: 'directory',
  // 表头控制
  headers: {
    firstRow: '字段',
    secondRow: '类型',
    thirdRow: '注释'
  },
  // 起始内容偏移量
  offset: {
    x: 0, // 列偏移
    y: 3  // 行偏移，默认跳过前3行表头
  }
});

// 控制台相关状态
const showConsole = ref(false);
interface LogItem {
  time: string;
  content: string;
  type: string;
}
const logs = ref<LogItem[]>([]);

// 生成日志
function addLog(content: string, type: string = 'info') {
  const now = new Date();
  const time = now.toLocaleTimeString();
  logs.value.push({ time, content, type });
}

// 清空日志
function clearLogs() {
  logs.value = [];
}

// 一键生成处理
function handleGenerate() {
  showConsole.value = true;
  clearLogs();
  
  // 模拟生成过程
  addLog('开始生成代码...', 'info');
  
  // 模拟生成步骤
  setTimeout(() => {
    addLog('正在解析配置...', 'info');
  }, 500);
  
  setTimeout(() => {
    addLog('正在处理 Excel 文件...', 'info');
  }, 1000);
  
  setTimeout(() => {
    addLog('正在生成 C# 代码...', 'info');
  }, 1500);
  
  setTimeout(() => {
    addLog('正在生成 Lua 代码...', 'info');
  }, 2000);
  
  setTimeout(() => {
    addLog('生成完成！', 'success');
  }, 2500);
}
</script>

<style scoped>
.project-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background: #f5f7fa;
}

.project-content {
  flex: 1;
  overflow: auto;
  padding: 20px;
}

.project-info-section h2 {
  margin: 0 0 24px 0;
  font-size: 22px;
  font-weight: 600;
  color: #303133;
}

.project-info-card {
  border-radius: 12px;
  transition: all 0.3s ease;
}

.project-info-card:hover {
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
}

.project-form {
  padding: 8px 0;
}

.form-input,
.form-select {
  width: 100%;
  max-width: 400px;
  transition: all 0.3s ease;
}

.form-input:focus,
.form-select:focus {
  box-shadow: 0 0 0 2px rgba(64, 158, 255, 0.2);
}

.form-buttons {
  display: flex;
  gap: 12px;
  margin-top: 24px;
}

.save-btn,
.generate-btn {
  border-radius: 6px;
  transition: all 0.3s ease;
  font-weight: 500;
  padding: 8px 20px;
}

.save-btn:hover,
.generate-btn:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.save-btn {
  background: #409EFF;
  border-color: #409EFF;
}

.save-btn:hover {
  box-shadow: 0 4px 12px rgba(64, 158, 255, 0.3);
}

.generate-btn {
  background: #67C23A;
  border-color: #67C23A;
}

.generate-btn:hover {
  box-shadow: 0 4px 12px rgba(103, 194, 58, 0.3);
}

/* 表头控制卡片样式 */
.header-control-section {
  margin-top: 24px;
}

.header-control-section h2 {
  margin: 0 0 24px 0;
  font-size: 22px;
  font-weight: 600;
  color: #303133;
}

.header-control-card {
  border-radius: 12px;
  transition: all 0.3s ease;
}

.header-control-card:hover {
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
}

.header-control-form {
  padding: 8px 0;
}

/* 偏移量控制样式 */
.offset-controls {
  margin-left: -150px;
  padding-left: 150px;
}

.offset-item {
  margin-bottom: 12px;
}

.offset-item:last-child {
  margin-bottom: 0;
}

/* 控制台样式 */
.console-section {
  margin-top: 24px;
}

.console-section h2 {
  margin: 0 0 24px 0;
  font-size: 22px;
  font-weight: 600;
  color: #303133;
}

.console-card {
  border-radius: 12px;
  transition: all 0.3s ease;
}

.console-card:hover {
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
}

.console-content {
  max-height: 300px;
  overflow-y: auto;
  padding: 12px;
  background: #f5f7fa;
  border-radius: 8px;
  margin-bottom: 12px;
}

.log-item {
  display: flex;
  margin-bottom: 8px;
  padding: 4px 0;
  font-size: 14px;
}

.log-time {
  color: #909399;
  margin-right: 12px;
  min-width: 80px;
}

.log-content {
  flex: 1;
}

.log-item.info .log-content {
  color: #303133;
}

.log-item.success .log-content {
  color: #67C23A;
}

.log-item.error .log-content {
  color: #F56C6C;
}

.log-item.warn .log-content {
  color: #E6A23C;
}

.console-actions {
  display: flex;
  justify-content: flex-end;
  padding-top: 12px;
  border-top: 1px solid #EBEEF5;
}



/* 滚动条样式 */
.project-content::-webkit-scrollbar {
  width: 6px;
  height: 6px;
}

.project-content::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 3px;
}

.project-content::-webkit-scrollbar-thumb {
  background: #c1c1c1;
  border-radius: 3px;
}

.project-content::-webkit-scrollbar-thumb:hover {
  background: #a8a8a8;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .project-content {
    padding: 16px;
  }
  
  .project-info-section h2 {
    font-size: 20px;
  }
  
  .form-input,
  .form-select {
    max-width: 100%;
  }
  
  .form-buttons {
    flex-direction: column;
  }
  
  .save-btn,
  .generate-btn {
    width: 100%;
  }
}
</style>