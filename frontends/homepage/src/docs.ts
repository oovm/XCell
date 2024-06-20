export interface DocNode {
	id: string;
	title: string;
	path: string;
	isDirectory: boolean;
	children?: DocNode[];
	parentId?: string;
	order?: number;
}

export interface DocMetadata {
	title?: string;
	order?: number;
}

import { docsModules } from "virtual:docs";

// 多语言文档配置
export interface LanguageConfig {
	code: string;
	name: string;
}

// 文档节点配置
interface DocNodeConfig {
	id: string;
	path: string;
	title: Record<string, string>; // 不同语言的标题
	filePath: string; // 相对于语言目录的路径
	order?: number;
}

// 支持的语言
const languages: LanguageConfig[] = [
	{ code: "zh-hans", name: "简体中文" },
	{ code: "en", name: "English" },
];

// 文档节点配置接口
interface DocNodeConfig {
	id: string;
	path: string;
	title: Record<string, string>;
	filePath: string;
	order?: number;
	children?: DocNodeConfig[];
}

// 通用文档结构 - 完全手动构建，控制顺序和层级
const docNodes: DocNodeConfig[] = [
	{
		id: "index",
		path: "/document",
		title: {
			"zh-hans": "首页",
			en: "Home",
		},
		filePath: "readme.md",
		order: 1,
	},
	{
		id: "overview",
		path: "/document/overview",
		title: {
			"zh-hans": "概览",
			en: "Overview",
		},
		filePath: "overview/index.md",
		order: 2,
		children: [
			{
				id: "overview-features",
				path: "/document/overview/features",
				title: {
					"zh-hans": "功能特性",
					en: "Features",
				},
				filePath: "overview/features.md",
				order: 1,
			},
		],
	},
	{
		id: "concepts",
		path: "/document/concepts",
		title: {
			"zh-hans": "概念",
			en: "Concepts",
		},
		filePath: "concepts/index.md",
		order: 3,
		children: [
			{
				id: "concepts-class",
				path: "/document/concepts/class",
				title: {
					"zh-hans": "类",
					en: "Class",
				},
				filePath: "concepts/class.md",
				order: 1,
			},
			{
				id: "concepts-table",
				path: "/document/concepts/table",
				title: {
					"zh-hans": "表格",
					en: "Table",
				},
				filePath: "concepts/table.md",
				order: 2,
			},
			{
				id: "concepts-enumerate",
				path: "/document/concepts/enumerate",
				title: {
					"zh-hans": "枚举",
					en: "Enumerate",
				},
				filePath: "concepts/enumerate.md",
				order: 3,
			},
			{
				id: "concepts-language",
				path: "/document/concepts/language",
				title: {
					"zh-hans": "语言",
					en: "Language",
				},
				filePath: "concepts/language.md",
				order: 4,
			},
			{
				id: "concepts-merge",
				path: "/document/concepts/merge",
				title: {
					"zh-hans": "合并",
					en: "Merge",
				},
				filePath: "concepts/merge.md",
				order: 5,
			},
			{
				id: "concepts-config",
				path: "/document/concepts/config",
				title: {
					"zh-hans": "配置",
					en: "Config",
				},
				filePath: "concepts/config.md",
				order: 6,
			},
		],
	},
	{
		id: "tutorials",
		path: "/document/tutorials",
		title: {
			"zh-hans": "教程",
			en: "Tutorials",
		},
		filePath: "tutorials/index.md",
		order: 4,
		children: [
			{
				id: "tutorials-getting-started",
				path: "/document/tutorials/getting-started",
				title: {
					"zh-hans": "快速开始",
					en: "Getting Started",
				},
				filePath: "tutorials/getting-started.md",
				order: 1,
			},
			{
				id: "tutorials-use-cases",
				path: "/document/tutorials/use-cases",
				title: {
					"zh-hans": "使用案例",
					en: "Use Cases",
				},
				filePath: "tutorials/use-cases/index.md",
				order: 2,
				children: [
					{
						id: "tutorials-unity-integration",
						path: "/document/tutorials/use-cases/unity-integration",
						title: {
							"zh-hans": "Unity 集成",
							en: "Unity Integration",
						},
						filePath: "tutorials/use-cases/unity-integration.md",
						order: 1,
					},
				],
			},
		],
	},
	{
		id: "advanced",
		path: "/document/advanced",
		title: {
			"zh-hans": "高级功能",
			en: "Advanced",
		},
		filePath: "advanced/index.md",
		order: 5,
		children: [
			{
				id: "advanced-advanced-features",
				path: "/document/advanced/advanced-features",
				title: {
					"zh-hans": "高级特性",
					en: "Advanced Features",
				},
				filePath: "advanced/advanced-features.md",
				order: 1,
			},
			{
				id: "advanced-type-system",
				path: "/document/advanced/type-system",
				title: {
					"zh-hans": "类型系统",
					en: "Type System",
				},
				filePath: "advanced/type-system.md",
				order: 2,
			},
			{
				id: "advanced-meta-data",
				path: "/document/advanced/meta-data",
				title: {
					"zh-hans": "元数据",
					en: "Meta Data",
				},
				filePath: "advanced/meta-data.md",
				order: 3,
			},
			{
				id: "advanced-extensibility",
				path: "/document/advanced/extensibility",
				title: {
					"zh-hans": "可扩展性",
					en: "Extensibility",
				},
				filePath: "advanced/extensibility.md",
				order: 4,
			},
		],
	},
];

// 递归转换文档节点配置
function transformDocNode(
	node: DocNodeConfig,
	language: string = "zh-hans",
): DocNode {
	const transformedNode: DocNode = {
		id: node.id,
		title: node.title[language] || node.title["zh-hans"],
		path: node.path,
		isDirectory: !!node.children && node.children.length > 0,
		order: node.order,
		children: [],
	};

	if (node.children && node.children.length > 0) {
		transformedNode.children = node.children
			.map((child) => transformDocNode(child, language))
			.sort((a, b) => (a.order || 999) - (b.order || 999));
	}

	return transformedNode;
}

// 获取文档节点配置
function getDocNodes(language: string = "zh-hans"): Array<{
	id: string;
	path: string;
	title: string;
	filePath: string;
	order?: number;
}> {
	// 扁平化文档节点配置，用于 getDocContent 函数
	const flattenedNodes: Array<{
		id: string;
		path: string;
		title: string;
		filePath: string;
		order?: number;
	}> = [];

	function flattenNode(node: DocNodeConfig) {
		const filePath = `./documentation/${language}/${node.filePath}`;
		console.log(
			`Flattening node ${node.id}: path=${node.path}, filePath=${filePath}`,
		);
		flattenedNodes.push({
			id: node.id,
			path: node.path,
			title: node.title[language] || node.title["zh-hans"],
			filePath: filePath,
			order: node.order,
		});

		if (node.children) {
			node.children.forEach((child) => flattenNode(child));
		}
	}

	docNodes.forEach((node) => flattenNode(node));
	console.log(`Flattened nodes:`, flattenedNodes);
	return flattenedNodes;
}

// 读取文件内容
function readFileContent(filePath: string): string {
	try {
		// 从虚拟模块中读取文档内容
		// 转换路径格式，移除 ./ 前缀和 documentation/ 前缀
		const normalizedPath = filePath
			.replace(/^\.\//, "")
			.replace(/^documentation\//, "");
		console.log(`Reading file from virtual module: ${normalizedPath}`);
		console.log(
			`Available keys in docsModules:`,
			Object.keys(docsModules).filter((key) => key.includes("readme")),
		);
		const content = docsModules[normalizedPath];
		if (content) {
			console.log(`Found content for ${normalizedPath}`);
			return content;
		}
		throw new Error(`File not found in virtual module: ${normalizedPath}`);
	} catch (error) {
		console.error(`Error reading file ${filePath}:`, error);
		return `# 错误

无法加载文件: ${filePath}`;
	}
}

console.log(
	"Language configs:",
	languages.map((config) => config.code),
);

function buildDocTree(docs: DocNode[]): DocNode[] {
	const nodeMap: Record<string, DocNode> = {};

	// 定义一级节点 ID 列表
	const rootNodeIds = ["overview", "concepts", "tutorials", "advanced"];

	// 首先创建所有节点，并初始化 children 数组
	docs.forEach((doc) => {
		const node = { ...doc, children: [] };
		nodeMap[doc.id] = node;
	});

	const root: DocNode[] = [];

	// 构建树结构
	docs.forEach((doc) => {
		const node = nodeMap[doc.id];

		// 检查是否是根节点：index 节点、一级节点或非目录节点
		if (
			node.id === "index" ||
			rootNodeIds.includes(node.id) ||
			!node.isDirectory
		) {
			// 根级节点，直接添加到根节点
			if (!root.some((n) => n.id === node.id)) {
				root.push(node);
			}
		} else {
			// 子级节点，添加到对应目录节点
			// 从 ID 中提取父目录 ID
			const parentId = node.id.split("-")[0];
			const parentNode = nodeMap[parentId];

			if (parentNode && parentNode.children) {
				// 确保不会重复添加
				if (!parentNode.children.some((n) => n.id === node.id)) {
					parentNode.children.push(node);
				}
			}
		}
	});

	const sortNodes = (nodes: DocNode[]): DocNode[] => {
		return nodes
			.sort((a, b) => {
				// 按顺序排序
				return (a.order || 999) - (b.order || 999);
			})
			.map((node) => ({
				...node,
				children: node.children ? sortNodes(node.children) : undefined,
			}));
	};

	return sortNodes(root);
}

export async function loadDocs(
	language: string = "zh-hans",
): Promise<DocNode[]> {
	// 直接从配置构建文档树
	const docTree = docNodes
		.map((node) => transformDocNode(node, language))
		.sort((a, b) => (a.order || 999) - (b.order || 999));

	console.log(`Built doc tree for ${language}:`, docTree);
	return docTree;
}

export async function getDocContent(
	path: string,
	language: string = "zh-hans",
): Promise<string> {
	console.log(`=== getDocContent called ===`);
	console.log(`Path: ${path}`);
	console.log(`Language: ${language}`);

	const docNodes = getDocNodes(language);
	console.log(`=== Doc nodes found: ${docNodes.length} ===`);
	docNodes.forEach((node) => {
		console.log(`  - ${node.path} -> ${node.filePath}`);
	});

	const docNode = docNodes.find((node) => node.path === path);
	if (docNode) {
		console.log(`=== Found doc node ===`);
		console.log(`  ID: ${docNode.id}`);
		console.log(`  Path: ${docNode.path}`);
		console.log(`  FilePath: ${docNode.filePath}`);
		const content = readFileContent(docNode.filePath);
		console.log(`  Content length: ${content.length}`);
		return content;
	}

	// 特殊处理根路径 /document
	if (path === "/document") {
		console.log(`=== Handling root document path ===`);
		const rootDocNode = docNodes.find((node) => node.path === "/document");
		if (rootDocNode) {
			console.log(`=== Found root doc node ===`);
			console.log(`  ID: ${rootDocNode.id}`);
			console.log(`  Path: ${rootDocNode.path}`);
			console.log(`  FilePath: ${rootDocNode.filePath}`);
			const content = readFileContent(rootDocNode.filePath);
			console.log(`  Content length: ${content.length}`);
			return content;
		} else {
			console.log(`=== Root doc node not found ===`);
		}
	}

	console.log(`=== Doc node not found for path: ${path} ===`);
	return "";
}

// 获取支持的语言列表
export function getSupportedLanguages(): { code: string; name: string }[] {
	return languages.map((config) => ({ code: config.code, name: config.name }));
}

// 测试函数，验证 readme.md 文件是否可以被正确读取
export async function testReadmeFile() {
	console.log("Testing readme.md file...");
	const testPath = "/document";
	const testLanguage = "zh-hans";

	console.log(`Testing path: ${testPath}, language: ${testLanguage}`);

	// 测试 getDocNodes
	const docNodes = getDocNodes(testLanguage);
	console.log(`Doc nodes:`, docNodes);

	// 测试 getDocContent
	const content = await getDocContent(testPath, testLanguage);
	console.log(`Content length: ${content.length}`);
	console.log(`Content preview: ${content.substring(0, 100)}...`);

	// 直接测试 readFileContent
	const testFilePath = `./documentation/${testLanguage}/readme.md`;
	const directContent = readFileContent(testFilePath);
	console.log(`Direct content length: ${directContent.length}`);
	console.log(`Direct content preview: ${directContent.substring(0, 100)}...`);
}

// 立即执行测试
testReadmeFile().catch(console.error);
