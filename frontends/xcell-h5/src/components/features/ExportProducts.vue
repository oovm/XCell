<template>
  <div class="export-products">
    <div class="panel-section">
      <div class="section-title">导出产物</div>
      <div class="product-list">
        <div 
          v-for="product in exportProducts" 
          :key="product.id"
          class="export-product-item"
          @click="openExportLocation(product.path)"
        >
          <div class="export-product-info">
            <div class="export-product-name">{{ product.name }}</div>
            <div class="export-product-meta">
              <span class="export-product-type">{{ product.type }}</span>
              <span class="export-product-time">{{ formatTime(product.created) }}</span>
            </div>
          </div>
        </div>
        <div v-if="exportProducts.length === 0" class="no-exports">
          暂无导出产物
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

// 定义事件
const emit = defineEmits(["open-location"]);

// 导出产物数据结构
interface ExportProduct {
	id: string;
	name: string;
	path: string;
	type: string;
	created: string;
}

// 导出产物列表
const exportProducts = ref<ExportProduct[]>([
	{
		id: "1",
		name: "users.json",
		path: "/dist/users.json",
		type: "JSON",
		created: new Date().toISOString(),
	},
	{
		id: "2",
		name: "products.ts",
		path: "/dist/products.ts",
		type: "TypeScript",
		created: new Date().toISOString(),
	},
	{
		id: "3",
		name: "data.sql",
		path: "/dist/data.sql",
		type: "SQL",
		created: new Date().toISOString(),
	},
]);

// 打开导出产物所在位置
const openExportLocation = (path: string) => {
	emit("open-location", path);
};

// 格式化时间
const formatTime = (timeString: string) => {
	const date = new Date(timeString);
	return date.toLocaleString();
};
</script>

<style scoped>
.export-products {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.panel-section {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.section-title {
  font-size: 12px;
  font-weight: 600;
  color: #858585;
  margin-bottom: 8px;
  text-transform: uppercase;
}

.product-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-height: 300px;
  overflow-y: auto;
}

.export-product-item {
  padding: 8px;
  border-radius: 4px;
  background: #2d2d30;
  cursor: pointer;
  transition: background 0.2s;
}

.export-product-item:hover {
  background: rgba(255, 255, 255, 0.1);
}

.export-product-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.export-product-name {
  font-size: 13px;
  font-weight: 500;
  color: #d4d4d4;
}

.export-product-meta {
  display: flex;
  gap: 8px;
  font-size: 11px;
  color: #858585;
}

.export-product-type {
  background: rgba(78, 201, 176, 0.2);
  padding: 0 6px;
  border-radius: 10px;
}

.export-product-time {
  display: flex;
  align-items: center;
  gap: 4px;
}

.no-exports {
  font-size: 12px;
  color: #858585;
  padding: 16px;
  text-align: center;
  background: #2d2d30;
  border-radius: 4px;
}

/* 滚动条样式 */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: #1e1e1e;
}

::-webkit-scrollbar-thumb {
  background: #424242;
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: #4e4e4e;
}
</style>