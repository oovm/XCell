import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { apiService } from "../services/api";

/** 项目配置 */
export interface ProjectConfig {
  /** 版本号 */
  version: string;
  /** 包含的 Excel 路径 */
  include: string;
  /** 排除的 Excel 模式 */
  exclude: string;
  /** 行列排序模式 */
  line: string;
  /** 类型解析模式 */
  typing: string;
  /** 合表模式 */
  merge: string;
  /** 表头配置 */
  headers: {
    /** 第一行表头含义 */
    firstRow: string;
    /** 第二行表头含义 */
    secondRow: string;
    /** 第三行表头含义 */
    thirdRow: string;
  };
  /** 起始内容偏移量 */
  offset: {
    /** 列偏移 */
    x: number;
    /** 行偏移 */
    y: number;
  };
}

/** 项目状态 Store */
export const useProjectStore = defineStore("project", () => {
  /** 项目路径 */
  const projectPath = ref<string>("");

  /** 项目配置 */
  const projectConfig = ref<ProjectConfig>({
    version: "1.0.0",
    include: "excel/**/*.xlsx",
    exclude: "**/temp/**",
    line: "row",
    typing: "strict",
    merge: "directory",
    headers: {
      firstRow: "字段",
      secondRow: "类型",
      thirdRow: "注释",
    },
    offset: {
      x: 0,
      y: 3,
    },
  });

  /** 工作空间是否已加载 */
  const isLoaded = ref<boolean>(false);

  /** 加载状态 */
  const isLoading = ref<boolean>(false);

  /** 是否有项目打开 */
  const hasProject = computed(() => projectPath.value !== "");

  /** 打开项目对话框 */
  async function openProjectDialog(): Promise<string | null> {
    try {
      const path = await apiService.openProjectDialog();
      if (path) {
        projectPath.value = path;
        localStorage.setItem("projectPath", path);
      }
      return path;
    } catch {
      return null;
    }
  }

  /** 设置项目路径 */
  function setProjectPath(path: string): void {
    projectPath.value = path;
    localStorage.setItem("projectPath", path);
  }

  /** 从 localStorage 恢复项目路径 */
  function restoreProjectPath(): void {
    const saved = localStorage.getItem("projectPath");
    if (saved) {
      projectPath.value = saved;
    }
  }

  /** 清除项目 */
  function clearProject(): void {
    projectPath.value = "";
    isLoaded.value = false;
    localStorage.removeItem("projectPath");
  }

  return {
    projectPath,
    projectConfig,
    isLoaded,
    isLoading,
    hasProject,
    openProjectDialog,
    setProjectPath,
    restoreProjectPath,
    clearProject,
  };
});
