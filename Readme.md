XCell 配置表管理工具
==================






## 合表



### 合表规则

- 目录结构:

```markdown
LanguageTable/
  - CN/
    - Language_CN_UI
    - Language_CN_Item
  - EN/
    - Language_CN_UI
```

- 合表规则

```toml
[merge.10001]
mode = "row"
input = "Language_CN*"
target = "Language_CN"

[merge.10002]
mode = "row"
input = "Language_EN*"
target = "Language_EN"

[merge.20001]
mode = "colomn"
input = "Language*"
target = "Language"
```

## Unity


## Cocos