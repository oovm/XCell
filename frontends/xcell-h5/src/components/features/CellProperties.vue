<template>
  <div class="cell-properties">
    <div class="panel-section">
      <div class="section-title">单元格属性</div>
      <div v-if="selectedCell" class="property-list">
        <div class="property-item">
          <div class="property-label">位置</div>
          <div class="property-value">{{ selectedCell.position }}</div>
        </div>
        <div class="property-item">
          <div class="property-label">值</div>
          <div class="property-value">{{ selectedCell.value }}</div>
        </div>
        <div class="property-item">
          <div class="property-label">类型</div>
          <div class="property-value">{{ selectedCell.type }}</div>
        </div>
        <div class="property-item">
          <div class="property-label">格式</div>
          <div class="property-value">{{ selectedCell.format }}</div>
        </div>
        <div class="property-item">
          <div class="property-label">验证</div>
          <div class="property-value">{{ selectedCell.validation }}</div>
        </div>
      </div>
      <div v-else class="no-selection">
        选择一个单元格查看属性
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps({
	selectedCell: {
		type: Object,
		default: null,
	},
});
</script>

<style scoped>
.cell-properties {
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

.property-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.property-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px;
  background: #2d2d30;
  border-radius: 4px;
}

.property-label {
  font-size: 13px;
  color: #858585;
}

.property-value {
  font-size: 13px;
  color: #d4d4d4;
  font-weight: 500;
}

.no-selection {
  font-size: 12px;
  color: #858585;
  padding: 16px;
  text-align: center;
  background: #2d2d30;
  border-radius: 4px;
}
</style>