<template>
  <div class="markdown-viewer prose prose-slate max-w-none">
    <div v-html="renderedContent" ref="contentRef"></div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, onMounted } from "vue";
import { marked } from "marked";
import * as shiki from "shiki";

interface Props {
	content: string;
}

const props = defineProps<Props>();
const contentRef = ref<HTMLElement | null>(null);

// 配置 marked 解析器
const renderer = new marked.Renderer();

// 生成唯一标识符
let codeBlockId = 0;

// 代码高亮函数
async function highlightCode(code: string, language: string) {
	try {
		const highlighter = await shiki.getHighlighter({ theme: "github-dark" });
		return highlighter.codeToHtml(code, { lang: language });
	} catch (error) {
		console.error("Error highlighting code:", error);
		return `<pre><code class="language-${language}">${code}</code></pre>`;
	}
}

// 自定义代码块渲染
renderer.code = function (code: string, language: string | undefined) {
	if (!language) {
		return `<pre><code>${code}</code></pre>`;
	}

	// 解析平台信息，格式：language:platform
	const parts = language.split(":");
	const lang = parts[0];
	const platform = parts[1] || "default";

	// 生成唯一 ID
	const id = `code-block-${codeBlockId++}`;

	// 确保 code 是字符串
	const codeString = typeof code === "string" ? code : String(code);

	// 使用普通代码块，后续通过异步方式添加高亮
	// 因为 renderer.code 是同步函数，无法直接使用异步的 shiki API
	if (platform !== "default") {
		return `
      <div class="code-platform-container" data-code-block-id="${id}">
        <div class="code-platform-switcher mb-4">
          <div class="flex flex-wrap gap-2">
            <button 
              class="px-4 py-2 rounded-lg text-sm font-medium transition-all duration-300 bg-white/10 text-white border border-white/10 active"
              data-platform="unity"
              data-code-block-id="${id}"
            >
              Unity
            </button>
            <button 
              class="px-4 py-2 rounded-lg text-sm font-medium transition-all duration-300 text-slate-400 hover:text-white hover:bg-white/5 border border-transparent"
              data-platform="cocos"
              data-code-block-id="${id}"
            >
              Cocos
            </button>
          </div>
        </div>
        <pre><code class="language-${lang}" data-code="${codeString}" data-language="${lang}" data-code-block-id="${id}">${codeString}</code></pre>
      </div>
    `;
	}

	return `<pre><code class="language-${lang}" data-code="${codeString}" data-language="${lang}">${codeString}</code></pre>`;
};

// 自定义超链接渲染，修复超链接问题
renderer.link = function (href: string, title: string | null, text: string) {
	// 处理相对路径
	let processedHref = href;
	if (href && !href.startsWith("http://") && !href.startsWith("https://")) {
		// 假设相对路径是指向文档的
		processedHref = `/document${href}`;
	}

	const titleAttr = title ? ` title="${title}"` : "";
	return `<a href="${processedHref}"${titleAttr} class="text-primary-600 hover:text-primary-700 underline">${text}</a>`;
};

marked.use({
	renderer,
});

const renderedContent = computed(() => {
	// 确保 content 是字符串
	const contentString =
		typeof props.content === "string" ? props.content : String(props.content);
	const content = contentString.replace(/^---\n[\s\S]*?\n---/, "");
	// 使用 marked.parse 的正确方法获取字符串
	return marked.parse(content).toString();
});

// 平台切换逻辑
function setupPlatformSwitching() {
	if (!contentRef.value) return;

	// 为所有平台切换按钮添加点击事件
	const switchButtons = contentRef.value.querySelectorAll(
		".code-platform-switcher button",
	);
	switchButtons.forEach((button) => {
		button.addEventListener("click", (e) => {
			const target = e.currentTarget as HTMLElement;
			const platform = target.dataset.platform;
			const codeBlockId = target.dataset.codeBlockId;

			if (platform && codeBlockId) {
				// 更新按钮状态
				const container = contentRef.value?.querySelector(
					`[data-code-block-id="${codeBlockId}"]`,
				);
				if (container) {
					const buttons = container.querySelectorAll(
						".code-platform-switcher button",
					);
					buttons.forEach((btn) => {
						btn.classList.remove(
							"active",
							"bg-white/10",
							"text-white",
							"border",
							"border-white/10",
						);
						btn.classList.add(
							"text-slate-400",
							"hover:text-white",
							"hover:bg-white/5",
							"border-transparent",
						);
					});

					target.classList.add(
						"active",
						"bg-white/10",
						"text-white",
						"border",
						"border-white/10",
					);
					target.classList.remove(
						"text-slate-400",
						"hover:text-white",
						"hover:bg-white/5",
						"border-transparent",
					);

					// 这里可以添加代码切换逻辑
					// 实际项目中，可能需要从服务器或本地存储获取不同平台的代码
					console.log(`Switching to ${platform} for code block ${codeBlockId}`);
				}
			}
		});
	});
}

// 代码高亮逻辑
async function setupCodeHighlighting() {
	if (!contentRef.value) return;

	try {
		// 直接使用 shiki 的 codeToHtml 函数，这是更简单的方式
		const codeBlocks = contentRef.value.querySelectorAll("pre code");
		codeBlocks.forEach(async (codeElement) => {
			// 优先使用 data-code 属性中的代码内容
			const code = codeElement.dataset.code || codeElement.textContent || "";
			const language =
				codeElement.dataset.language ||
				codeElement.className.replace("language-", "") ||
				"text";

			try {
				// 使用 shiki 的 codeToHtml 函数直接生成高亮代码
				const highlightedCode = await shiki.codeToHtml(code, {
					lang: language,
					theme: "github-dark",
				});
				const preElement = codeElement.parentElement;
				if (preElement) {
					preElement.innerHTML = highlightedCode;
				}
			} catch (error) {
				console.error("Error highlighting code:", error);
			}
		});
	} catch (error) {
		console.error("Error setting up code highlighting:", error);
	}
}

onMounted(async () => {
	setupPlatformSwitching();
	await setupCodeHighlighting();
});
</script>

<style scoped>
.markdown-viewer {
  @apply text-gray-700;
}

.markdown-viewer :deep(h1) {
  @apply text-3xl font-bold mb-6 mt-8 text-gray-900;
}

.markdown-viewer :deep(h2) {
  @apply text-2xl font-semibold mb-4 mt-6 text-gray-900 border-b-2 border-primary-500/20 pb-2;
}

.markdown-viewer :deep(h3) {
  @apply text-xl font-semibold mb-3 mt-5 text-gray-900;
}

.markdown-viewer :deep(p) {
  @apply mb-4 leading-relaxed;
}

.markdown-viewer :deep(ul),
.markdown-viewer :deep(ol) {
  @apply mb-4 pl-6;
}

.markdown-viewer :deep(li) {
  @apply mb-2;
}

.markdown-viewer :deep(code) {
  @apply bg-gray-100 px-1.5 py-0.5 rounded text-sm font-mono text-primary-600;
}

.markdown-viewer :deep(pre) {
  @apply bg-gray-900 text-gray-100 p-4 rounded-lg mb-4 overflow-x-auto;
}

.markdown-viewer :deep(pre code) {
  @apply bg-transparent px-0 py-0 text-gray-100;
}

.markdown-viewer :deep(a) {
  @apply text-primary-600 hover:text-primary-700 underline;
}

.markdown-viewer :deep(blockquote) {
  @apply border-l-4 border-primary-500 pl-4 italic text-gray-600 mb-4 bg-primary-500/5 py-2;
}

.markdown-viewer :deep(table) {
  @apply w-full border-collapse mb-4;
}

.markdown-viewer :deep(th),
.markdown-viewer :deep(td) {
  @apply border border-gray-300 px-4 py-2;
}

.markdown-viewer :deep(th) {
  @apply bg-gray-100 font-semibold text-gray-800;
}

/* 代码平台切换组件样式 */
.markdown-viewer :deep(.code-platform-container) {
  @apply mb-6;
}

.markdown-viewer :deep(.code-platform-switcher) {
  @apply mb-4;
}

.markdown-viewer :deep(.code-platform-switcher button) {
  @apply px-4 py-2 rounded-lg text-sm font-medium transition-all duration-300;
}

.markdown-viewer :deep(.code-platform-switcher button:hover) {
  @apply text-white hover:bg-white/5;
}

.markdown-viewer :deep(.code-platform-switcher button.active) {
  @apply bg-white/10 text-white border border-white/10;
}
</style>
