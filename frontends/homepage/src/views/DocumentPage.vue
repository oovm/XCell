<template>
  <div class="flex flex-col lg:flex-row min-h-[calc(100vh-120px)]">
    <aside class="lg:w-80 flex-shrink-0 border-r border-slate-200 bg-slate-50">
      <div class="sticky top-20 p-6 h-[calc(100vh-5rem)] overflow-y-auto">
        <h2 class="text-xl font-bold text-slate-800 mb-6">文档导航</h2>
        <nav v-if="docTree.length > 0">
          <div v-for="node in docTree" :key="node.id" class="mb-1">
            <DocTreeNode :node="node" :current-path="currentDocPath" @select="selectDoc" />
          </div>
        </nav>
        <div v-else class="text-slate-600 text-sm">
          加载文档中...
        </div>
      </div>
    </aside>

    <main class="flex-1">
      <div class="p-8 lg:p-12 min-h-[calc(100vh-120px)]">
        <div v-if="currentDoc">
          <h2 class="text-3xl font-bold mb-6 text-slate-800">{{ currentDoc.title }}</h2>
          <div class="prose prose-slate max-w-none">
            <MarkdownViewer :content="currentDocContent" />
          </div>
        </div>
        <div v-else-if="errorMessage" class="text-red-600 p-4 bg-red-50 rounded-lg">
          <h3 class="text-xl font-bold mb-2">错误信息</h3>
          <pre>{{ errorMessage }}</pre>
        </div>
        <div v-else class="text-slate-600 text-center py-16">
          <p class="text-lg">请从左侧选择文档查看</p>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { useRouter } from "vue-router";
import { loadDocs, getDocContent, type DocNode } from "@/docs";
import MarkdownViewer from "@/components/MarkdownViewer.vue";
import DocTreeNode from "@/components/DocTreeNode.vue";

const router = useRouter();
const docTree = ref<DocNode[]>([]);
const currentDocPath = ref<string>("");
const currentDoc = ref<DocNode | null>(null);
const currentDocContent = ref<string>("");
const currentLanguage = ref<string>(
	localStorage.getItem("language") || "zh-hans",
);
const errorMessage = ref<string>("");

async function init() {
	try {
		console.log("Initializing docs...");
		console.log("Current language:", currentLanguage.value);

		// 直接尝试加载 /document 路径的内容
		console.log("Trying to load /document content directly...");
		const directContent = await getDocContent(
			"/document",
			currentLanguage.value,
		);
		console.log("Direct content length:", directContent.length);
		console.log("Direct content:", directContent);

		// 尝试直接读取 readme.md 文件
		const testFilePath = `./documentation/${currentLanguage.value}/readme.md`;
		console.log(`Trying to read file directly: ${testFilePath}`);

		docTree.value = await loadDocs(currentLanguage.value);
		console.log("Loaded docTree:", docTree.value);
		if (docTree.value.length > 0) {
			console.log("First node:", docTree.value[0]);
			const firstDoc = findFirstDoc(docTree.value[0]);
			console.log("First doc found:", firstDoc);
			if (firstDoc) {
				selectDoc(firstDoc);
			} else {
				console.log("No first doc found");
				errorMessage.value = "No first doc found";
			}
		} else {
			console.log("Doc tree is empty");
			errorMessage.value = "Doc tree is empty";
		}
	} catch (error) {
		console.error("Error initializing docs:", error);
		errorMessage.value = `Error: ${error instanceof Error ? error.message : String(error)}`;
	}
}

function findFirstDoc(node: DocNode): DocNode | null {
	if (!node.isDirectory && !node.children?.length) {
		return node;
	}
	if (node.children && node.children.length > 0) {
		return findFirstDoc(node.children[0]);
	}
	return null;
}

async function selectDoc(node: DocNode) {
	console.log("=== selectDoc called ===");
	console.log(`Node:`, node);
	currentDocPath.value = node.path;
	currentDoc.value = node;
	console.log(
		`Calling getDocContent with path: ${node.path}, language: ${currentLanguage.value}`,
	);
	currentDocContent.value = await getDocContent(
		node.path,
		currentLanguage.value,
	);
	console.log(`Got content length: ${currentDocContent.value.length}`);
	// 更新路由
	router.push(node.path);
}

// 监听 localStorage 中的语言变化
function checkLanguageChange() {
	const storedLanguage = localStorage.getItem("language") || "zh-hans";
	if (storedLanguage !== currentLanguage.value) {
		currentLanguage.value = storedLanguage;
		init();
	}
}

// 每秒钟检查一次语言变化
const languageCheckInterval = setInterval(checkLanguageChange, 1000);

onMounted(() => {
	init();
});

// 组件卸载时清除定时器
onUnmounted(() => {
	clearInterval(languageCheckInterval);
});
</script>