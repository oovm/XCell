<template>
  <div class="artifact-view">
    <!-- 页面标题 -->
    <div class="page-header">
      <h2>产物视图</h2>
      <div class="header-buttons">
        <el-button type="primary" @click="generateCurrent" :icon="Refresh" class="generate-btn">
          立即生成
        </el-button>
        <el-button type="primary" @click="generateAll" :icon="Refresh" class="generate-btn">
          全部生成
        </el-button>
      </div>
    </div>
    
    <!-- 主要内容区域 -->
    <div class="main-content">
      <!-- 左侧产物类型导航 -->
      <div class="artifact-nav">
        <div 
          class="nav-item" 
          :class="{ active: selectedType === 'unity' }"
          @click="selectedType = 'unity'"
        >
          <el-icon class="nav-icon"><Box /></el-icon>
          <span>Unity</span>
        </div>
        <div 
          class="nav-item" 
          :class="{ active: selectedType === 'cocos' }"
          @click="selectedType = 'cocos'"
        >
          <el-icon class="nav-icon"><Box /></el-icon>
          <span>Cocos</span>
        </div>
        <div 
          class="nav-item" 
          :class="{ active: selectedType === 'json' }"
          @click="selectedType = 'json'"
        >
          <el-icon class="nav-icon"><Document /></el-icon>
          <span>JSON</span>
        </div>
        <div 
          class="nav-item" 
          :class="{ active: selectedType === 'sql' }"
          @click="selectedType = 'sql'"
        >
          <el-icon class="nav-icon"><Document /></el-icon>
          <span>SQL</span>
        </div>
        <div 
          class="nav-item" 
          :class="{ active: selectedType === 'csv' }"
          @click="selectedType = 'csv'"
        >
          <el-icon class="nav-icon"><Document /></el-icon>
          <span>CSV</span>
        </div>
        <div class="nav-item add-item" @click="addNewArtifactType">
          <el-icon class="nav-icon"><Plus /></el-icon>
          <span>添加</span>
        </div>
      </div>
      
      <!-- 右侧详细信息 -->
      <div class="artifact-details">
        <!-- 产物标题和启用开关 -->
        <div class="detail-header">
          <h3 
            class="editable-title" 
            contenteditable="true" 
            @blur="updateTitle"
            @keydown.enter.prevent
          >{{ artifactName }}</h3>
          <div class="enable-switch">
            <span>启用</span>
            <el-switch v-model="isEnabled" active-color="#409EFF" />
          </div>
        </div>
        
        <!-- 产物信息区域 -->
        <div class="artifact-info-section">
          <el-form :model="artifactInfo" label-width="100px" class="artifact-form">
            <!-- 基本信息 -->
            <el-row :gutter="20">
              <el-col :span="12">
                <el-form-item label="产物类型">
                  <div class="readonly-field">{{ artifactInfo.type }}</div>
                </el-form-item>
              </el-col>
              <el-col :span="12">
                <el-form-item label="生成时间">
                  <div class="readonly-field">{{ artifactInfo.time }}</div>
                </el-form-item>
              </el-col>
            </el-row>
            
            <!-- 文件路径 -->
            <el-form-item label="文件路径">
              <el-input v-model="artifactPath" class="path-input" placeholder="输入文件路径" />
              <div class="path-buttons">
                <el-button type="primary" size="small" @click="selectPath" class="path-btn">
                  <el-icon><FolderOpened /></el-icon>
                  选择
                </el-button>
                <el-button size="small" @click="openPath" class="path-btn">
                  <el-icon><Link /></el-icon>
                  打开
                </el-button>
                <el-button size="small" @click="clearPath" class="path-btn">
                  <el-icon><Delete /></el-icon>
                  清除
                </el-button>
              </div>
            </el-form-item>
            
            <el-row :gutter="20">
              <el-col :span="12">
                <el-form-item label="文件大小">
                  <div class="readonly-field">{{ artifactInfo.size }}</div>
                </el-form-item>
              </el-col>
            </el-row>
            
            <!-- 产物类型特定配置 -->
            <el-divider content-position="left">配置选项</el-divider>
            
            <!-- JSON 配置 -->
            <template v-if="selectedType === 'json'">
              <el-row :gutter="20">
                <el-col :span="12">
                  <el-form-item label="缩进空格">
                    <el-input-number v-model="artifactConfig.indent" :min="0" :max="8" />
                  </el-form-item>
                </el-col>
                <el-col :span="12">
                  <el-form-item label="美化输出">
                    <el-switch v-model="artifactConfig.pretty" />
                  </el-form-item>
                </el-col>
              </el-row>
            </template>
            
            <!-- SQL 配置 -->
            <template v-if="selectedType === 'sql'">
              <el-form-item label="数据库类型">
                <el-select v-model="artifactConfig.dbType" placeholder="选择数据库类型" style="width: 100%">
                  <el-option label="MySQL" value="mysql" />
                  <el-option label="PostgreSQL" value="postgresql" />
                  <el-option label="SQLite" value="sqlite" />
                </el-select>
              </el-form-item>
            </template>
            
            <!-- CSV 配置 -->
            <template v-if="selectedType === 'csv'">
              <el-row :gutter="20">
                <el-col :span="12">
                  <el-form-item label="BOM 兼容">
                    <div class="form-item-content">
                      <el-tooltip content="若不开启，Excel 加载将是乱码" placement="top">
                        <el-button size="small" circle class="help-button">
                          <el-icon><QuestionFilled /></el-icon>
                        </el-button>
                      </el-tooltip>
                      <el-switch v-model="artifactConfig.bom" />
                    </div>
                  </el-form-item>
                </el-col>
                <el-col :span="12">
                  <el-form-item label="移除类型">
                    <div class="form-item-content">
                      <el-tooltip content="只保留 field name 作为表头" placement="top">
                        <el-button size="small" circle class="help-button">
                          <el-icon><QuestionFilled /></el-icon>
                        </el-button>
                      </el-tooltip>
                      <el-switch v-model="artifactConfig.removeType" />
                    </div>
                  </el-form-item>
                </el-col>
              </el-row>
              <el-form-item label="分隔符">
                <el-select v-model="artifactConfig.delimiter" placeholder="选择分隔符" style="width: 100%">
                  <el-option label="," value="," />
                  <el-option label=";" value=";" />
                  <el-option label="Tab" value="\t" />
                </el-select>
              </el-form-item>
            </template>
          </el-form>
        </div>
        
        <!-- 分包配置卡片 -->
        <div class="package-config-card">
          <div class="package-config-header">
            <h4 class="package-config-title">分包</h4>
            <div class="toggle-container">
              <span class="toggle-label">{{ isInverseSelection ? '反选' : '正选' }}</span>
              <el-switch v-model="isInverseSelection" />
            </div>
          </div>
          <div class="package-select-content">
            <el-select v-model="selectedPackages" multiple placeholder="选择分包" style="width: 100%">
              <el-option label="主包" value="DataTable" />
              <el-option label="分包1" value="package1" />
              <el-option label="分包2" value="package2" />
              <el-option label="分包3" value="package3" />
            </el-select>
          </div>
        </div>
        
        <!-- 加载器配置卡片 -->
        <div class="package-config-card">
          <div class="package-config-header">
            <h4 class="package-config-title">加载器</h4>
          </div>
          <div class="package-select-content">
            <el-select v-model="loaderType" placeholder="选择加载器类型" style="width: 100%">
              <el-option label="同步加载" value="sync" />
              <el-option label="异步加载" value="async" />
              <el-option label="延迟加载" value="lazy" />
            </el-select>
          </div>
        </div>
        
        <!-- 储存格式区域 -->
        <div class="storage-format-section">
          <div class="storage-format-header">
            <span class="storage-format-title">储存格式</span>
            <el-select v-model="storageFormat" placeholder="选择格式" style="width: 120px">
              <el-option label="JSON" value="json" />
              <el-option label="CSV" value="csv" />
              <el-option label="Binary" value="binary" />
            </el-select>
          </div>
        </div>
        
        <!-- 操作按钮 -->
        <div class="action-buttons">
          <el-button type="primary" class="action-btn" :icon="FolderOpened">
            打开
          </el-button>
          <el-button class="action-btn" :icon="View">
            查看
          </el-button>
          <el-button type="warning" class="action-btn" :icon="Delete">
            清除
          </el-button>
          <el-button type="danger" class="action-btn" :icon="Delete">
            删除
          </el-button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { Download, Document, Box, FolderOpened, View, Delete, Plus, Refresh, DocumentCopy, Link, QuestionFilled } from "@element-plus/icons-vue";

// 当前选中的产物类型
const selectedType = ref('unity');

// 选中的分包（支持多选）
const selectedPackages = ref<string[]>(['DataTable']);

// 是否反选模式
const isInverseSelection = ref(false);

// 产物路径
const artifactPath = ref('');

// 是否启用
const isEnabled = ref(true);

// 产物名称
const artifactName = ref('Unity 产物');

// 产物信息
const artifactInfo = ref({
  type: 'Unity',
  time: '',
  size: '12.5 KB'
});

// 产物配置
const artifactConfig = ref({
  indent: 2,
  pretty: true,
  dbType: 'mysql',
  bom: false,
  delimiter: ',',
  removeType: false
});

// 储存格式
const storageFormat = ref('json');

// 加载器类型
const loaderType = ref('sync');

// 格式化时间
const formatTime = (timeString: string) => {
	const date = new Date(timeString);
	return date.toLocaleString();
};

// 监听选中类型变化，更新默认路径和产物信息
watch(selectedType, (newType) => {
  if (newType === 'unity') {
    artifactName.value = 'Unity 产物';
    artifactPath.value = '/dist/unity/Artifacts.cs';
    artifactInfo.value = {
      type: 'Unity',
      time: formatTime(new Date().toISOString()),
      size: '12.5 KB'
    };
  } else if (newType === 'cocos') {
    artifactName.value = 'Cocos 产物';
    artifactPath.value = '/dist/cocos/Artifacts.ts';
    artifactInfo.value = {
      type: 'Cocos',
      time: formatTime(new Date().toISOString()),
      size: '8.2 KB'
    };
  } else if (newType === 'json') {
    artifactName.value = 'JSON 产物';
    artifactPath.value = '/dist/data.json';
    artifactInfo.value = {
      type: 'JSON',
      time: formatTime(new Date().toISOString()),
      size: '5.7 KB'
    };
  } else if (newType === 'sql') {
    artifactName.value = 'SQL 产物';
    artifactPath.value = '/dist/data.sql';
    artifactInfo.value = {
      type: 'SQL',
      time: formatTime(new Date().toISOString()),
      size: '3.1 KB'
    };
  } else if (newType === 'csv') {
    artifactName.value = 'CSV 产物';
    artifactPath.value = '/dist/data.csv';
    artifactInfo.value = {
      type: 'CSV',
      time: formatTime(new Date().toISOString()),
      size: '4.2 KB'
    };
    // 重置 CSV 特定配置
    artifactConfig.value.removeType = false;
  }
}, { immediate: true });

// 选择路径
const selectPath = () => {
  console.log('选择路径');
  // 这里可以添加打开文件选择器的逻辑
  // 模拟选择路径
  artifactPath.value = '/dist/custom/path/' + (selectedType.value === 'unity' ? 'Artifacts.cs' : 
                                           selectedType.value === 'cocos' ? 'Artifacts.ts' : 
                                           selectedType.value === 'json' ? 'data.json' : 'data.sql');
};

// 打开路径
const openPath = () => {
  console.log('打开路径:', artifactPath.value);
  // 这里可以添加打开路径的逻辑
};

// 清除路径
const clearPath = () => {
  artifactPath.value = '';
  console.log('路径已清除');
};

// 复制预览内容
const copyPreview = () => {
  navigator.clipboard.writeText(filePreview.value)
    .then(() => {
      console.log('预览内容已复制到剪贴板');
      // 这里可以添加复制成功的提示
    })
    .catch(err => {
      console.error('复制失败:', err);
    });
};

// 添加新产物类型
const addNewArtifactType = () => {
  console.log('添加新产物类型');
  // 这里可以添加添加新产物类型的逻辑
};

// 立即生成（只生成当前打开的产物）
const generateCurrent = () => {
  // 根据反选模式计算要生成的包
  let packagesToGenerate: string[];
  const allPackages = ['DataTable', 'package1', 'package2', 'package3'];
  
  if (isInverseSelection.value) {
    // 反选模式：生成不在selectedPackages中的包
    packagesToGenerate = allPackages.filter(pkg => !selectedPackages.value.includes(pkg));
  } else {
    // 正选模式：生成selectedPackages中的包
    packagesToGenerate = [...selectedPackages.value];
  }
  
  console.log('立即生成产物（当前类型）:', selectedType.value);
  console.log('生成产物的包:', packagesToGenerate);
  // 这里可以添加生成逻辑
};

// 全部生成
const generateAll = () => {
  // 所有可用的包
  const allPackages = ['DataTable', 'package1', 'package2', 'package3'];
  
  // 根据反选模式计算要生成的包
  let packagesToGenerate: string[];
  if (isInverseSelection.value) {
    // 反选模式：生成不在selectedPackages中的包
    packagesToGenerate = allPackages.filter(pkg => !selectedPackages.value.includes(pkg));
  } else {
    // 正选模式：生成selectedPackages中的包
    packagesToGenerate = [...selectedPackages.value];
  }
  
  console.log('生成产物的包:', packagesToGenerate);
  // 这里可以添加生成逻辑
};

// 更新标题
const updateTitle = (event: Event) => {
  const target = event.target as HTMLElement;
  artifactName.value = target.textContent || '';
  console.log('标题已更新:', artifactName.value);
  // 这里可以添加保存标题的逻辑
};

// 文件预览内容
const filePreview = computed(() => {
  if (selectedType.value === 'unity') {
    return `// Unity Artifacts.cs
public class Artifacts {
    public static class Config {
        public const string Version = "1.0.0";
    }
}`;
  } else if (selectedType.value === 'cocos') {
    return `// Cocos Artifacts.ts
export class Artifacts {
    static Config = {
        Version: "1.0.0"
    };
}`;
  } else if (selectedType.value === 'json') {
    return `{
  "version": "1.0.0",
  "data": []
}`;
  } else if (selectedType.value === 'sql') {
    return `-- SQL data.sql
CREATE TABLE IF NOT EXISTS artifacts (
    id INT PRIMARY KEY,
    name VARCHAR(255)
);`;
  } else if (selectedType.value === 'csv') {
    return `id,name,value
1,Item1,100
2,Item2,200
3,Item3,300`;
  } else {
    return '';
  }
});
</script>

<style scoped>
.artifact-view {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: #f5f7fa;
  padding: 20px;
  margin: 0;
  overflow: hidden;
}

/* 页面标题 */
.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
  padding-bottom: 16px;
  border-bottom: 1px solid #e4e7ed;
}

.page-header h2 {
  margin: 0;
  font-size: 22px;
  font-weight: 600;
  color: #303133;
}

.header-buttons {
  display: flex;
  gap: 12px;
  align-items: center;
}

.generate-btn {
  border-radius: 6px;
  transition: all 0.3s ease;
}

.generate-btn:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(64, 158, 255, 0.3);
}

/* 主要内容区域 */
.main-content {
  flex: 1;
  display: flex;
  gap: 20px;
  overflow: hidden;
  min-height: 0;
}

/* 左侧产物类型导航 */
.artifact-nav {
  width: 220px;
  background: #ffffff;
  border-right: 1px solid #e4e7ed;
  padding: 12px;
  overflow-y: auto;
  transition: all 0.3s ease;
}

.nav-item {
  display: flex;
  align-items: center;
  padding: 14px 16px;
  margin-bottom: 6px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.3s ease;
  gap: 10px;
  font-size: 14px;
  color: #606266;
  background: #ffffff;
  border: 1px solid transparent;
}

.nav-item:hover {
  background: #ecf5ff;
  color: #409EFF;
  border-color: #d9ecff;
  transform: translateX(4px);
}

.nav-item.active {
  background: #409EFF;
  color: #ffffff;
  border-color: #409EFF;
  box-shadow: 0 2px 8px rgba(64, 158, 255, 0.3);
}

.nav-icon {
  font-size: 16px;
}

.add-item {
  border: 2px dashed #dcdfe6;
  color: #909399;
  justify-content: center;
  margin-top: 12px;
  background: #fafafa;
}

.add-item:hover {
  border-color: #409EFF;
  background: #ecf5ff;
  color: #409EFF;
}

/* 右侧详细信息 */
.artifact-details {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 20px;
  padding: 20px;
  background: #ffffff;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

/* 产物标题和启用开关 */
.detail-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 0;
  border-bottom: 1px solid #e4e7ed;
  gap: 20px;
}

.editable-title {
  flex: 1;
  max-width: 400px;
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: #303133;
  padding: 8px 12px;
  border: 1px solid transparent;
  border-radius: 8px;
  transition: all 0.3s ease;
  outline: none;
}

.editable-title:hover {
  border-color: #dcdfe6;
  background: #f8f9fa;
}

.editable-title:focus {
  border-color: #409EFF;
  background: #ffffff;
  box-shadow: 0 0 0 2px rgba(64, 158, 255, 0.2);
}

.enable-switch {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 14px;
  color: #606266;
  font-weight: 500;
}

/* 产物信息区域 */
.artifact-info-section {
  background: #ffffff;
  padding: 24px 0;
  flex: 1;
  min-height: 400px;
}

.artifact-form {
  padding: 0;
}

.readonly-field {
  padding: 8px 12px;
  background: #f8f9fa;
  border: 1px solid #e4e7ed;
  border-radius: 4px;
  color: #606266;
  font-size: 14px;
  min-height: 32px;
  display: flex;
  align-items: center;
}

.path-input {
  border-color: #dcdfe6;
  transition: all 0.3s ease;
}

.path-input:focus {
  border-color: #409EFF;
  box-shadow: 0 0 0 2px rgba(64, 158, 255, 0.2);
}

/* 分包选择容器 */
.package-select-container {
  display: flex;
  align-items: center;
  gap: 12px;
}

/* 切换容器 */
.toggle-container {
  display: flex;
  align-items: center;
  gap: 8px;
  white-space: nowrap;
}

/* 切换标签 */
.toggle-label {
  font-size: 12px;
  color: #606266;
  font-weight: 500;
}

.path-buttons {
  margin-top: 16px;
  display: flex;
  gap: 10px;
}

.path-btn {
  border-radius: 4px;
  transition: all 0.3s ease;
}

.path-btn:hover {
  transform: translateY(-1px);
}

/* 带帮助图标的开关 */
.switch-with-help {
  display: flex;
  align-items: center;
  gap: 8px;
}

/* 表单项目内容 */
.form-item-content {
  display: flex;
  align-items: center;
  gap: 8px;
}

/* 带帮助图标的标签 */
.label-with-help {
  display: flex;
  align-items: center;
  gap: 4px;
  width: 100%;
  padding-left: 100px;
}

.label-with-help span {
  font-size: 14px;
  color: #606266;
  font-weight: 500;
}

/* 帮助按钮 */
.help-button {
  width: 24px;
  height: 24px;
  padding: 0;
  font-size: 14px;
  min-width: 24px;
}

.help-button .el-icon {
  font-size: 14px;
}

/* 分包配置卡片 */
.package-config-card {
  background: #ffffff;
  border: 1px solid #e4e7ed;
  border-radius: 8px;
  padding: 16px;
  margin-bottom: 20px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
  transition: all 0.3s ease;
}

.package-config-card:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.package-config-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
  padding-bottom: 8px;
  border-bottom: 1px solid #e4e7ed;
}

.package-config-title {
  font-size: 14px;
  font-weight: 600;
  color: #303133;
  margin: 0;
}

.package-select-content {
  padding-top: 8px;
}

/* 储存格式区域 */
.storage-format-section {
  background: #ffffff;
  border: 1px solid #e4e7ed;
  border-radius: 8px;
  padding: 16px;
  margin-bottom: 20px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
  transition: all 0.3s ease;
}

.storage-format-section:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.storage-format-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
}

.storage-format-title {
  font-size: 14px;
  font-weight: 600;
  color: #303133;
  margin: 0;
}

/* 操作按钮 */
.action-buttons {
  display: flex;
  gap: 12px;
  padding: 20px 0;
  border-top: 1px solid #e4e7ed;
  margin-top: auto;
}

.action-btn {
  flex: 1;
  max-width: 120px;
  border-radius: 6px;
  transition: all 0.3s ease;
  font-weight: 500;
}

.action-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.action-btn:first-child {
  flex: 2;
  max-width: 160px;
}

/* 响应式设计 */
@media (max-width: 1024px) {
  .artifact-view {
    padding: 16px;
  }
  
  .main-content {
    flex-direction: column;
  }
  
  .artifact-nav {
    width: 100%;
    flex-direction: row;
    overflow-x: auto;
    padding: 12px;
  }
  
  .nav-item {
    margin-bottom: 0;
    margin-right: 10px;
    min-width: 120px;
    justify-content: center;
  }
  
  .nav-item:hover {
    transform: translateY(-2px);
    translateX: 0;
  }
  
  .detail-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 16px;
  }
  
  .path-buttons {
    flex-wrap: wrap;
  }
  
  .action-buttons {
    flex-wrap: wrap;
  }
  
  .action-btn {
    flex: 1;
    min-width: 100px;
    max-width: none;
  }
  
  .action-btn:first-child {
    flex: 1;
    max-width: none;
  }
}



/* 滚动条样式 */
.artifact-nav::-webkit-scrollbar,
.artifact-details::-webkit-scrollbar,
.preview-content::-webkit-scrollbar {
  width: 6px;
  height: 6px;
}

.artifact-nav::-webkit-scrollbar-track,
.artifact-details::-webkit-scrollbar-track,
.preview-content::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 3px;
}

.artifact-nav::-webkit-scrollbar-thumb,
.artifact-details::-webkit-scrollbar-thumb,
.preview-content::-webkit-scrollbar-thumb {
  background: #c1c1c1;
  border-radius: 3px;
}

.artifact-nav::-webkit-scrollbar-thumb:hover,
.artifact-details::-webkit-scrollbar-thumb:hover,
.preview-content::-webkit-scrollbar-thumb:hover {
  background: #a8a8a8;
}
</style>