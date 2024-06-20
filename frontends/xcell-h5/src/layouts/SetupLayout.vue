<template>
  <div class="setup-layout">
    <div class="setup-container">
      <div class="setup-header">
        <h1>XCell</h1>
        <p>数据表格管理工具</p>
      </div>
      
      <div class="setup-content">
        <!-- 项目拖入区域 -->
        <div 
          class="drop-area"
          :class="{ 'drag-over': isDragOver }"
          @dragover.prevent="handleDragOver"
          @dragleave.prevent="handleDragLeave"
          @drop.prevent="handleDrop"
        >
          <div class="drop-icon">📁</div>
          <h2>拖入项目文件夹</h2>
          <p>或</p>
          <button class="open-button" @click="openProject">
            打开已有项目
          </button>
        </div>
      </div>
      
      <div class="setup-footer">
        <p>© 2026 XCell. 保留所有权利。</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const isDragOver = ref(false);

// 处理拖入悬停
const handleDragOver = (event: DragEvent) => {
  isDragOver.value = true;
  event.dataTransfer!.dropEffect = 'copy';
};

// 处理拖入离开
const handleDragLeave = () => {
  isDragOver.value = false;
};

// 打开编辑器窗口
const openEditorWindow = async (projectPath: string) => {
  try {
    console.log('调用 open_editor_window 命令，参数:', projectPath);
    await invoke('open_editor_window', { projectPath });
    console.log('open_editor_window 命令调用成功');
  } catch (error) {
            console.error('打开编辑器窗口失败:', error);
            console.error('错误信息: ' + (error as any).message);
        }
};

// 处理项目拖入
const handleDrop = async (event: DragEvent) => {
  isDragOver.value = false;
  
  // 兼容旧版浏览器
  const files = event.dataTransfer?.files;
  if (files && files.length > 0) {
    // 这里我们假设在 Tauri 环境中，文件路径可以通过特殊方式获取
    // 实际实现中可能需要使用 Tauri 的 API 来处理文件拖入
    console.log('拖入文件:', files[0].name);
    
    // 这里应该获取真实的拖入路径，暂时用模拟值
    const projectPath = '/path/to/project';
    
    // 打开编辑器窗口（不保存到 localStorage，避免影响选择器窗口）
    await openEditorWindow(projectPath);
  }
};

// 打开已有项目
const openProject = async () => {
  try {
    console.log('开始打开项目...');
    
    console.log('调用 open_project_dialog 命令');
    const projectPath = await invoke('open_project_dialog');
    console.log('open_project_dialog 命令返回结果:', projectPath);
    
    if (projectPath && projectPath !== 'User cancelled') {
      console.log('选择的项目路径:', projectPath);
      
      // 打开编辑器窗口（不保存到 localStorage，避免影响选择器窗口）
      await openEditorWindow(projectPath as string);
    } else {
      console.log('用户取消了选择');
    }
  } catch (error) {
      console.error('打开项目失败:', error);
      // 忽略用户取消的错误
      if ((error as any).message !== 'User cancelled') {
        console.error('打开项目失败: ' + (error as any).message);
      }
    }
};
</script>

<style scoped>
.setup-layout {
  display: flex;
  flex-direction: column;
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  background-color: #f5f7fa;
}

.setup-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 2rem;
  max-width: 800px;
  margin: 0 auto;
  width: 100%;
}

.setup-header {
  text-align: center;
  margin-bottom: 3rem;
}

.setup-header h1 {
  font-size: 3rem;
  font-weight: bold;
  color: #333;
  margin-bottom: 0.5rem;
}

.setup-header p {
  font-size: 1.2rem;
  color: #666;
}

.setup-content {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
}

.drop-area {
  width: 100%;
  max-width: 600px;
  padding: 4rem 2rem;
  border: 2px dashed #ccc;
  border-radius: 12px;
  text-align: center;
  transition: all 0.3s ease;
  background-color: #fff;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
}

.drop-area:hover {
  border-color: #409eff;
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.12);
}

.drop-area.drag-over {
  border-color: #67c23a;
  background-color: rgba(103, 194, 58, 0.05);
  box-shadow: 0 8px 24px rgba(103, 194, 58, 0.2);
  transform: scale(1.02);
}

.drop-icon {
  font-size: 4rem;
  margin-bottom: 1.5rem;
}

.drop-area h2 {
  font-size: 1.5rem;
  font-weight: 600;
  color: #333;
  margin-bottom: 1rem;
}

.drop-area p {
  color: #666;
  margin: 1rem 0;
}

.open-button {
  margin-top: 1rem;
  padding: 0.8rem 2rem;
  font-size: 1rem;
  font-weight: 500;
  color: #fff;
  background-color: #409eff;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.3s ease;
}

.open-button:hover {
  background-color: #66b1ff;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(64, 158, 255, 0.3);
}

.setup-footer {
  margin-top: 3rem;
  text-align: center;
  color: #999;
  font-size: 0.9rem;
}
</style>