<template>
  <div class="chat-view">
    <div class="chat-header">
      <h2 class="chat-title">聊天</h2>
    </div>

    <el-scrollbar ref="scrollbarRef" class="chat-messages">
      <div class="messages-container">
        <div
          v-for="msg in messages"
          :key="msg.id"
          :class="['message-row', msg.role === 'user' ? 'message-row--user' : 'message-row--assistant']"
        >
          <div v-if="msg.role === 'assistant'" class="avatar avatar--assistant">
            <el-icon :size="20"><Monitor /></el-icon>
          </div>

          <div :class="['message-bubble', `message-bubble--${msg.role}`]">
            <div
              v-if="msg.role === 'assistant'"
              class="message-content message-content--markdown"
              v-html="renderMarkdown(msg.content)"
            />
            <div v-else class="message-content">
              {{ msg.content }}
            </div>
            <div class="message-time">{{ formatTime(msg.timestamp) }}</div>
          </div>

          <div v-if="msg.role === 'user'" class="avatar avatar--user">
            <el-icon :size="20"><User /></el-icon>
          </div>
        </div>

        <div v-if="isTyping" class="message-row message-row--assistant">
          <div class="avatar avatar--assistant">
            <el-icon :size="20"><Monitor /></el-icon>
          </div>
          <div class="message-bubble message-bubble--assistant">
            <div class="typing-indicator">
              <span class="typing-dot" />
              <span class="typing-dot" />
              <span class="typing-dot" />
            </div>
          </div>
        </div>
      </div>
    </el-scrollbar>

    <div class="chat-input-area">
      <el-input
        v-model="inputText"
        type="textarea"
        :autosize="{ minRows: 1, maxRows: 4 }"
        placeholder="输入消息..."
        resize="none"
        class="chat-input"
        @keydown.enter.exact="handleEnterKey"
      />
      <el-button
        type="primary"
        :icon="Promotion"
        :disabled="!inputText.trim() || isTyping"
        class="send-button"
        @click="sendMessage"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { Monitor, Promotion, User } from "@element-plus/icons-vue";
import { nextTick, onMounted, ref } from "vue";

/** 聊天消息 */
interface ChatMessage {
  /** 消息 ID */
  id: string;
  /** 消息角色 */
  role: "user" | "assistant";
  /** 消息内容 */
  content: string;
  /** 发送时间 */
  timestamp: Date;
}

/** 消息列表 */
const messages = ref<ChatMessage[]>([]);

/** 输入框文本 */
const inputText = ref("");

/** AI 正在输入标记 */
const isTyping = ref(false);

/** 滚动条引用 */
const scrollbarRef = ref<InstanceType<typeof import("element-plus")["ElScrollbar"]>>();

/** 生成唯一 ID */
const generateId = (): string => {
  return Date.now().toString(36) + Math.random().toString(36).substring(2, 8);
};

/** 格式化时间 */
const formatTime = (date: Date): string => {
  const hours = date.getHours().toString().padStart(2, "0");
  const minutes = date.getMinutes().toString().padStart(2, "0");
  return `${hours}:${minutes}`;
};

/** 简易 Markdown 转 HTML */
const renderMarkdown = (text: string): string => {
  let html = text;

  html = html.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");

  html = html.replace(/^### (.+)$/gm, "<h3>$1</h3>");
  html = html.replace(/^## (.+)$/gm, "<h2>$1</h2>");
  html = html.replace(/^# (.+)$/gm, "<h1>$1</h1>");

  html = html.replace(/\*\*(.+?)\*\*/g, "<strong>$1</strong>");
  html = html.replace(/\*(.+?)\*/g, "<em>$1</em>");
  html = html.replace(/`([^`]+)`/g, "<code>$1</code>");

  html = html.replace(/^[-*] (.+)$/gm, "<li>$1</li>");
  html = html.replace(/(<li>.*<\/li>\n?)+/g, (match) => `<ul>${match}</ul>`);

  html = html.replace(/^(\d+)\. (.+)$/gm, "<li>$2</li>");

  html = html.replace(/\n{2,}/g, "</p><p>");
  html = html.replace(/\n/g, "<br>");
  html = `<p>${html}</p>`;

  html = html.replace(/<p><\/p>/g, "");
  html = html.replace(/<p>(<h[1-3]>)/g, "$1");
  html = html.replace(/(<\/h[1-3]>)<\/p>/g, "$1");
  html = html.replace(/<p>(<ul>)/g, "$1");
  html = html.replace(/(<\/ul>)<\/p>/g, "$1");

  return html;
};

/** 滚动到底部 */
const scrollToBottom = async () => {
  await nextTick();
  if (scrollbarRef.value) {
    scrollbarRef.value.setScrollTop(999999);
  }
};

/** 模拟 AI 回复 */
const simulateAIReply = (userMessage: string): string => {
  return `收到你的消息：**${userMessage}**\n\n我是 XCell AI 助手，目前处于演示模式。正式版将支持以下功能：\n\n- 表格数据智能分析\n- 配置文件自动生成\n- 数据校验与错误检测\n\n请期待后续更新！`;
};

/** 发送消息 */
const sendMessage = async () => {
  const text = inputText.value.trim();
  if (!text || isTyping.value) return;

  const userMsg: ChatMessage = {
    id: generateId(),
    role: "user",
    content: text,
    timestamp: new Date(),
  };
  messages.value.push(userMsg);
  inputText.value = "";
  await scrollToBottom();

  isTyping.value = true;
  await scrollToBottom();

  await new Promise((resolve) => setTimeout(resolve, 600 + Math.random() * 800));

  const aiMsg: ChatMessage = {
    id: generateId(),
    role: "assistant",
    content: simulateAIReply(text),
    timestamp: new Date(),
  };
  messages.value.push(aiMsg);
  isTyping.value = false;
  await scrollToBottom();
};

/** 处理 Enter 键发送 */
const handleEnterKey = (e: KeyboardEvent) => {
  if (e.shiftKey) return;
  e.preventDefault();
  sendMessage();
};

/** 初始化欢迎消息 */
onMounted(() => {
  messages.value.push({
    id: generateId(),
    role: "assistant",
    content: "你好！我是 **XCell AI 助手** 🎉\n\n我可以帮助你处理表格数据、生成配置、检查数据格式等。请随时向我提问！",
    timestamp: new Date(),
  });
  scrollToBottom();
});
</script>

<style scoped>
.chat-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  background-color: #f5f7fa;
}

.chat-header {
  display: flex;
  align-items: center;
  padding: 16px 24px;
  background-color: #ffffff;
  border-bottom: 1px solid #e4e7ed;
  flex-shrink: 0;
}

.chat-title {
  font-size: 18px;
  font-weight: 600;
  color: #303133;
  margin: 0;
}

.chat-messages {
  flex: 1;
  overflow: hidden;
}

.messages-container {
  padding: 20px 24px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.message-row {
  display: flex;
  align-items: flex-start;
  gap: 12px;
}

.message-row--user {
  justify-content: flex-end;
}

.message-row--assistant {
  justify-content: flex-start;
}

.avatar {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.avatar--assistant {
  background-color: #409EFF;
  color: #ffffff;
}

.avatar--user {
  background-color: #67c23a;
  color: #ffffff;
}

.message-bubble {
  max-width: 70%;
  padding: 12px 16px;
  border-radius: 12px;
  line-height: 1.6;
  word-break: break-word;
}

.message-bubble--assistant {
  background-color: #ffffff;
  color: #303133;
  border-top-left-radius: 4px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.06);
}

.message-bubble--user {
  background-color: #409EFF;
  color: #ffffff;
  border-top-right-radius: 4px;
}

.message-content {
  font-size: 14px;
}

.message-content--markdown {
  line-height: 1.7;
}

.message-content--markdown :deep(h1) {
  font-size: 18px;
  font-weight: 700;
  margin: 8px 0 4px;
}

.message-content--markdown :deep(h2) {
  font-size: 16px;
  font-weight: 600;
  margin: 8px 0 4px;
}

.message-content--markdown :deep(h3) {
  font-size: 15px;
  font-weight: 600;
  margin: 6px 0 4px;
}

.message-content--markdown :deep(strong) {
  font-weight: 600;
}

.message-content--markdown :deep(code) {
  background-color: #f0f2f5;
  padding: 2px 6px;
  border-radius: 3px;
  font-size: 13px;
  font-family: "Cascadia Code", "Fira Code", Consolas, monospace;
}

.message-content--markdown :deep(ul) {
  padding-left: 20px;
  margin: 4px 0;
}

.message-content--markdown :deep(li) {
  margin: 2px 0;
}

.message-time {
  font-size: 11px;
  margin-top: 6px;
  opacity: 0.5;
}

.message-bubble--user .message-time {
  text-align: right;
  color: rgba(255, 255, 255, 0.8);
}

.message-bubble--assistant .message-time {
  color: #909399;
}

.typing-indicator {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 0;
}

.typing-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background-color: #c0c4cc;
  animation: typing-bounce 1.4s infinite ease-in-out both;
}

.typing-dot:nth-child(1) {
  animation-delay: 0s;
}

.typing-dot:nth-child(2) {
  animation-delay: 0.2s;
}

.typing-dot:nth-child(3) {
  animation-delay: 0.4s;
}

@keyframes typing-bounce {
  0%,
  80%,
  100% {
    transform: scale(0.6);
    opacity: 0.4;
  }
  40% {
    transform: scale(1);
    opacity: 1;
  }
}

.chat-input-area {
  display: flex;
  align-items: flex-end;
  gap: 12px;
  padding: 16px 24px;
  background-color: #ffffff;
  border-top: 1px solid #e4e7ed;
  flex-shrink: 0;
}

.chat-input {
  flex: 1;
}

.chat-input :deep(.el-textarea__inner) {
  border-radius: 8px;
  padding: 10px 14px;
  font-size: 14px;
  line-height: 1.5;
  border-color: #dcdfe6;
}

.chat-input :deep(.el-textarea__inner:focus) {
  border-color: #409EFF;
  box-shadow: 0 0 0 2px rgba(64, 158, 255, 0.1);
}

.send-button {
  height: 40px;
  width: 40px;
  border-radius: 8px;
  flex-shrink: 0;
}

.send-button :deep(.el-icon) {
  font-size: 18px;
}
</style>
