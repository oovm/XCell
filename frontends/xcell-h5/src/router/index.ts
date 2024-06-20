import { createRouter, createWebHistory } from "vue-router";

const routes = [
	{
		path: "/",
		name: "SetupView",
		// 根路径使用 SetupLayout
		component: { template: '<div></div>' },
	},
	{
		path: "/project",
		name: "ProjectView",
		component: () => import("../views/ProjectView.vue"),
	},
	{
		path: "/table",
		name: "TableView",
		component: () => import("../views/TableView.vue"),
	},
	{
		path: "/edit",
		name: "EditView",
		component: () => import("../views/FileView.vue"),
	},
	{
		path: "/group",
		name: "GroupView",
		component: () => import("../views/GroupView.vue"),
	},
	{
		path: "/artifact",
		name: "ArtifactView",
		component: () => import("../views/ArtifactView.vue"),
	},
	{
		path: "/chat",
		name: "ChatView",
		component: () => import("../views/ChatView.vue"),
	},
	{
		path: "/setting",
		name: "SettingView",
		component: () => import("../views/SettingView.vue"),
		children: [
			{
				path: "mcp",
				name: "McpSettingView",
				component: () => import("../views/McpSettingView.vue"),
			},
		],
	},
];

const router = createRouter({
	history: createWebHistory(),
	routes,
});

export default router;
