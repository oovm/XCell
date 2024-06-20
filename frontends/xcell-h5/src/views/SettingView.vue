<template>
  <div class="setting-view">
    <!-- 左侧导航 -->
    <div class="setting-sidebar">
      <div class="sidebar-title">设置</div>
      <el-menu
        :default-active="activeSubmenu"
        class="setting-menu"
        @select="handleMenuSelect"
      >
        <el-menu-item index="/setting/basic">
          <el-icon><Setting /></el-icon>
          <span>基本配置</span>
        </el-menu-item>
        <el-menu-item index="/setting/project">
          <el-icon><Folder /></el-icon>
          <span>项目配置</span>
        </el-menu-item>
        <el-menu-item index="/setting/mcp">
          <el-icon><Operation /></el-icon>
          <span>MCP 配置</span>
        </el-menu-item>
        <el-menu-item index="/setting/advanced">
          <el-icon><View /></el-icon>
          <span>高级配置</span>
        </el-menu-item>
      </el-menu>
    </div>
    
    <!-- 右侧内容 -->
    <div class="setting-content">
      <!-- 基本配置 -->
      <div v-if="activeSubmenu === '/setting/basic'" class="setting-section">
        <h1 class="page-title">基本配置</h1>
        <el-card class="setting-card">
          <el-form :model="config" label-width="120px">
            <el-form-item label="版本号">
              <el-input v-model="config.version" placeholder="输入版本号" />
            </el-form-item>
            <el-form-item label="包含文件">
              <el-input v-model="config.include" placeholder="输入包含的 Excel 文件路径模式" />
            </el-form-item>
            <el-form-item label="排除文件">
              <el-input v-model="config.exclude" placeholder="输入排除的 Excel 文件路径模式" />
            </el-form-item>
          </el-form>
        </el-card>
      </div>
      
      <!-- 项目配置 -->
      <div v-if="activeSubmenu === '/setting/project'" class="setting-section">
        <h1 class="page-title">项目配置</h1>
        <el-card class="setting-card">
          <!-- 行列配置 -->
          <el-form :model="config.project.line" label-width="120px">
            <el-form-item label="字段名行">
              <el-input-number v-model="config.project.line.field" :min="1" />
            </el-form-item>
            <el-form-item label="数据类型行">
              <el-input-number v-model="config.project.line.type" :min="1" />
            </el-form-item>
            <el-form-item label="注释行">
              <el-input-number v-model="config.project.line.comment" :min="1" />
            </el-form-item>
            <el-form-item label="数据起始行">
              <el-input-number v-model="config.project.line.data" :min="1" />
            </el-form-item>
          </el-form>
          
          <!-- 类型解析配置 -->
          <el-form :model="config.project.type" label-width="120px">
            <el-form-item label="布尔类型 - 接受值">
              <el-tag v-for="(value, index) in config.project.type.bool.accept" :key="index" closable @close="removeAcceptValue(index)">{{ value }}</el-tag>
              <el-input v-model="newAcceptValue" placeholder="添加接受值" style="width: 200px; margin-left: 10px;" />
              <el-button type="primary" @click="addAcceptValue">添加</el-button>
            </el-form-item>
            <el-form-item label="布尔类型 - 拒绝值">
              <el-tag v-for="(value, index) in config.project.type.bool.reject" :key="index" closable @close="removeRejectValue(index)">{{ value }}</el-tag>
              <el-input v-model="newRejectValue" placeholder="添加拒绝值" style="width: 200px; margin-left: 10px;" />
              <el-button type="primary" @click="addRejectValue">添加</el-button>
            </el-form-item>
          </el-form>
        </el-card>
      </div>
      
      <!-- MCP 配置 -->
      <div v-if="activeSubmenu === '/setting/mcp'" class="setting-section">
        <h1 class="page-title">MCP 配置</h1>
        <el-card class="setting-card">
          <el-form :model="config.mcp" label-width="120px">
            <el-form-item label="MCP 服务器地址">
              <el-input v-model="config.mcp.server" placeholder="输入 MCP 服务器地址" />
            </el-form-item>
            <el-form-item label="API 密钥">
              <el-input v-model="config.mcp.apiKey" placeholder="输入 API 密钥" type="password" />
            </el-form-item>
            <el-form-item label="超时时间">
              <el-input-number v-model="config.mcp.timeout" :min="1" :max="60" />
              <span style="margin-left: 8px;">秒</span>
            </el-form-item>
          </el-form>
        </el-card>
      </div>
      
      <!-- 高级配置 -->
      <div v-if="activeSubmenu === '/setting/advanced'" class="setting-section">
        <h1 class="page-title">高级配置</h1>
        <el-card class="setting-card">
          <el-form :model="config.advanced" label-width="120px">
            <el-form-item label="日志级别">
              <el-select v-model="config.advanced.logLevel">
                <el-option label="debug" value="debug" />
                <el-option label="info" value="info" />
                <el-option label="warn" value="warn" />
                <el-option label="error" value="error" />
              </el-select>
            </el-form-item>
            <el-form-item label="缓存大小">
              <el-input-number v-model="config.advanced.cacheSize" :min="1" :max="1000" />
              <span style="margin-left: 8px;">MB</span>
            </el-form-item>
          </el-form>
        </el-card>
      </div>
      
      <!-- 操作按钮 -->
      <div class="action-buttons">
        <el-button type="primary" @click="saveConfig">保存配置</el-button>
        <el-button @click="resetConfig">重置默认值</el-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed } from "vue";
import { Setting, Folder, Operation, View } from "@element-plus/icons-vue";
import { useRoute, useRouter } from "vue-router";

const route = useRoute();
const router = useRouter();

// 新的接受值和拒绝值
const newAcceptValue = ref("");
const newRejectValue = ref("");

// 当前活动的子菜单
const activeSubmenu = computed(() => {
  const path = route.path;
  if (path.startsWith("/setting/")) {
    return path;
  }
  return "/setting/basic";
});

// 处理菜单选择
const handleMenuSelect = (key: string) => {
  router.push(key);
};

// 配置数据
const config = reactive({
  version: "0.0.0",
  include: "*.xlsx",
  exclude: "",
  project: {
    line: {
      field: 1,
      type: 2,
      comment: 3,
      data: 4
    },
    type: {
      bool: {
        accept: ["true", "√"],
        reject: ["false", "x"]
      }
    }
  },
  mcp: {
    server: "",
    apiKey: "",
    timeout: 30
  },
  advanced: {
    logLevel: "info",
    cacheSize: 100
  }
});

// 添加接受值
const addAcceptValue = () => {
  if (newAcceptValue.value) {
    config.project.type.bool.accept.push(newAcceptValue.value);
    newAcceptValue.value = "";
  }
};

// 移除接受值
const removeAcceptValue = (index: number) => {
  config.project.type.bool.accept.splice(index, 1);
};

// 添加拒绝值
const addRejectValue = () => {
  if (newRejectValue.value) {
    config.project.type.bool.reject.push(newRejectValue.value);
    newRejectValue.value = "";
  }
};

// 移除拒绝值
const removeRejectValue = (index: number) => {
  config.project.type.bool.reject.splice(index, 1);
};

// 保存配置
const saveConfig = () => {
  // 这里可以添加保存配置的逻辑
  console.log("保存配置:", config);
  // 显示保存成功的消息
  alert("配置保存成功！");
};

// 重置默认值
const resetConfig = () => {
  // 重置为默认值
  Object.assign(config, {
    version: "0.0.0",
    include: "*.xlsx",
    exclude: "",
    project: {
      line: {
        field: 1,
        type: 2,
        comment: 3,
        data: 4
      },
      type: {
        bool: {
          accept: ["true", "√"],
          reject: ["false", "x"]
        }
      }
    },
    mcp: {
      server: "",
      apiKey: "",
      timeout: 30
    },
    advanced: {
      logLevel: "info",
      cacheSize: 100
    }
  });
};
</script>

<style scoped>
.setting-view {
  display: flex;
  height: 100%;
}

/* 左侧导航栏 */
.setting-sidebar {
  width: 240px;
  background: #f8f9fa;
  border-right: 1px solid #e0e0e0;
  display: flex;
  flex-direction: column;
  padding-top: 20px;
}

.sidebar-title {
  font-size: 18px;
  font-weight: 600;
  color: #333;
  padding: 0 20px 20px;
  border-bottom: 1px solid #e0e0e0;
}

.setting-menu {
  flex: 1;
  border-right: none;
}

.setting-menu .el-menu-item {
  height: 48px;
  line-height: 48px;
  margin: 0 12px;
  border-radius: 6px;
}

.setting-menu .el-menu-item:hover {
  background: rgba(64, 158, 255, 0.1);
}

.setting-menu .el-menu-item.is-active {
  background: #409EFF;
  color: #fff;
}

.setting-menu .el-menu-item .el-icon {
  margin-right: 12px;
  font-size: 18px;
}

/* 右侧内容区域 */
.setting-content {
  flex: 1;
  padding: 20px;
  height: 100%;
  overflow-y: auto;
}

.setting-section {
  margin-bottom: 30px;
}

.page-title {
  font-size: 20px;
  font-weight: 600;
  margin-bottom: 20px;
  color: #333;
}

.setting-card {
  margin-bottom: 20px;
  border-radius: 4px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

.card-header {
  font-size: 16px;
  font-weight: 500;
  color: #333;
}

.action-buttons {
  margin-top: 20px;
  display: flex;
  gap: 12px;
}

/* 调整标签样式 */
:deep(.el-tag) {
  margin-right: 8px;
  margin-bottom: 8px;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .setting-view {
    flex-direction: column;
  }
  
  .setting-sidebar {
    width: 100%;
    height: auto;
    border-right: none;
    border-bottom: 1px solid #e0e0e0;
  }
  
  .setting-menu {
    display: flex;
    overflow-x: auto;
  }
  
  .setting-menu .el-menu-item {
    white-space: nowrap;
    margin: 0 4px;
  }
}
</style>