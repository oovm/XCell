XCell 配置表管理工具
==================



```yaml
命令: xcell.exe [OPTIONS] [COMMAND]

指令:
  check:  检查配置表, 但不导出任何文件
  clear:  清除数据库与缓存

选项:
      --workspace <WORKSPACE>  手动设置工作目录, 不输入表示当前目录
  -w, --watch                  启用监听模式, 当有文件修改时只更新对应文件
      --disable-xml            强制关闭 xml 生成
      --disable-json           强制关闭 json 生成
  -h, --help                   显示帮助
  -V, --version                显示版本
```