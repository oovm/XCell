/**
 * 错误处理工具类
 * 用于处理前端错误并提供友好的用户反馈
 */

import { ElMessage, ElMessageBox } from 'element-plus';

/**
 * 错误类型枚举
 */
export enum ErrorType {
  NETWORK = 'network',
  SERVER = 'server',
  VALIDATION = 'validation',
  UNKNOWN = 'unknown'
}

/**
 * 错误信息接口
 */
export interface ErrorInfo {
  type: ErrorType;
  message: string;
  originalError?: any;
}

/**
 * 错误处理类
 */
export class ErrorHandler {
  /**
   * 处理错误
   * @param error 错误对象
   * @param customMessage 自定义错误信息
   * @returns 错误信息
   */
  static handleError(error: any, customMessage?: string): ErrorInfo {
    let errorInfo: ErrorInfo = {
      type: ErrorType.UNKNOWN,
      message: customMessage || '操作失败，请稍后重试',
      originalError: error
    };

    // 网络错误
    if (error.message && error.message.includes('Network Error')) {
      errorInfo.type = ErrorType.NETWORK;
      errorInfo.message = '网络连接失败，请检查网络设置';
    }
    // 服务器错误
    else if (error.response && error.response.status >= 500) {
      errorInfo.type = ErrorType.SERVER;
      errorInfo.message = '服务器错误，请稍后重试';
    }
    // 验证错误
    else if (error.response && error.response.status === 400) {
      errorInfo.type = ErrorType.VALIDATION;
      errorInfo.message = error.response.data.message || '数据验证失败';
    }
    // 其他错误
    else if (error.message) {
      errorInfo.message = error.message;
    }

    // 显示错误信息
    this.showError(errorInfo);
    
    return errorInfo;
  }

  /**
   * 显示错误信息
   * @param errorInfo 错误信息
   */
  static showError(errorInfo: ErrorInfo): void {
    ElMessage({
      message: errorInfo.message,
      type: 'error',
      duration: 5000
    });
  }

  /**
   * 显示成功信息
   * @param message 成功信息
   */
  static showSuccess(message: string): void {
    ElMessage({
      message,
      type: 'success',
      duration: 3000
    });
  }

  /**
   * 显示警告信息
   * @param message 警告信息
   */
  static showWarning(message: string): void {
    ElMessage({
      message,
      type: 'warning',
      duration: 4000
    });
  }

  /**
   * 显示确认对话框
   * @param title 对话框标题
   * @param message 对话框内容
   * @returns Promise<boolean> 确认结果
   */
  static async confirm(title: string, message: string): Promise<boolean> {
    try {
      await ElMessageBox.confirm(message, title, {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      });
      return true;
    } catch {
      return false;
    }
  }
}

export default ErrorHandler;