use xcell_types::{XError, XErrorKind};

/// 代码生成错误类型扩展
pub enum GeneratorErrorKind {
    /// 生成器未找到
    GeneratorNotFound(String),
    /// 配置错误
    ConfigError(String),
    /// 输出目录错误
    OutputDirError(String),
    /// 代码生成错误
    CodegenError(String),
    /// 并行处理错误
    ParallelError(String),
}

/// 代码生成错误
pub type GeneratorError = XError;

/// 错误构造器
pub trait GeneratorErrorExt {
    /// 创建生成器未找到错误
    fn generator_not_found(name: &str) -> Self;
    /// 创建配置错误
    fn config_error(message: &str) -> Self;
    /// 创建输出目录错误
    fn output_dir_error(message: &str) -> Self;
    /// 创建代码生成错误
    fn codegen_error(message: &str) -> Self;
    /// 创建并行处理错误
    fn parallel_error(message: &str) -> Self;
}

impl GeneratorErrorExt for GeneratorError {
    fn generator_not_found(name: &str) -> Self {
        let kind = XErrorKind::RuntimeError { message: format!("未找到生成器: {}", name) };
        Self::new(kind)
    }

    fn config_error(message: &str) -> Self {
        let kind = XErrorKind::RuntimeError { message: format!("配置错误: {}", message) };
        Self::new(kind)
    }

    fn output_dir_error(message: &str) -> Self {
        let kind = XErrorKind::IOError(format!("输出目录错误: {}", message));
        Self::new(kind)
    }

    fn codegen_error(message: &str) -> Self {
        let kind = XErrorKind::RuntimeError { message: format!("代码生成错误: {}", message) };
        Self::new(kind)
    }

    fn parallel_error(message: &str) -> Self {
        let kind = XErrorKind::RuntimeError { message: format!("并行处理错误: {}", message) };
        Self::new(kind)
    }
}

/// 结果类型
pub type GeneratorResult<T> = Result<T, GeneratorError>;
