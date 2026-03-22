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
			{
				id: "overview-dict",
				path: "/document/overview/dict",
				title: {
					"zh-hans": "dict 表",
					en: "Dict Table",
				},
				filePath: "concepts/dict.md",
				order: 2,
			},
			{
				id: "overview-language",
				path: "/document/overview/language",
				title: {
					"zh-hans": "language 表",
					en: "Language Table",
				},
				filePath: "concepts/language.md",
				order: 3,
			},
			{
				id: "overview-list",
				path: "/document/overview/list",
				title: {
					"zh-hans": "list 表",
					en: "List Table",
				},
				filePath: "concepts/list.md",
				order: 4,
			},
			{
				id: "overview-enum",
				path: "/document/overview/enum",
				title: {
					"zh-hans": "enum 表",
					en: "Enum Table",
				},
				filePath: "concepts/enum.md",
				order: 5,
			},
			{
				id: "overview-class",
				path: "/document/overview/class",
				title: {
					"zh-hans": "class 表",
					en: "Class Table",
				},
				filePath: "concepts/class.md",
				order: 6,
			},
			{
				id: "overview-merge",
				path: "/document/overview/merge",
				title: {
					"zh-hans": "合表",
					en: "Merge",
				},
				filePath: "concepts/merge.md",
				order: 7,
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
		order: 3,
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
		order: 4,
		children: [
			{
				id: "advanced-type-system",
				path: "/document/advanced/type-system",
				title: {
					"zh-hans": "类型系统",
					en: "Type System",
				},
				filePath: "advanced/type-system.md",
				order: 1,
			},
			{
				id: "advanced-key-field",
				path: "/document/advanced/key-field",
				title: {
					"zh-hans": "字段约束",
					en: "Field Constraints",
				},
				filePath: "advanced/key-field.md",
				order: 2,
			},
			{
				id: "advanced-ref-type",
				path: "/document/advanced/ref-type",
				title: {
					"zh-hans": "引用类型",
					en: "Reference Type",
				},
				filePath: "advanced/ref-type.md",
				order: 3,
			},
			{
				id: "advanced-meta-data",
				path: "/document/advanced/meta-data",
				title: {
					"zh-hans": "元属性",
					en: "Meta Data",
				},
				filePath: "advanced/meta-data.md",
				order: 4,
			},
			{
				id: "advanced-config",
				path: "/document/advanced/config",
				title: {
					"zh-hans": "配置",
					en: "Config",
				},
				filePath: "concepts/config.md",
				order: 5,
			},
			{
				id: "advanced-extensibility",
				path: "/document/advanced/extensibility",
				title: {
					"zh-hans": "可扩展性",
					en: "Extensibility",
				},
				filePath: "advanced/extensibility.md",
				order: 6,
			},
		],
	},
];
