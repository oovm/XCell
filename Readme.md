Rust Template Project
=====================

Rust template project for monorepo






## 合表



### 合表规则
```toml
[merge.10001]
mode = "row"
input = "LanguageCN*"
target = "LanguageCN"

[merge.10002]
mode = "row"
input = "LanguageEN*"
target = "LanguageEN"

[merge.20001]
mode = "colomn"
input = "LanguageTable*"
target = "Language"
```

举例

```
LanguageTable/
LanguageTable/CN/LanguageUI
LanguageTable/EN/LanguageItem
```

## Unity


## Cocos