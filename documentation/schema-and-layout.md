# XCell.schema (new version)

**Canonical project file:** `XCell.schema` at the plugin-selected project root.
TOML configs (`xcell.toml` / `XCell.toml` / `xcell.config.toml` / `ProjectSettings.toml`)
are **cut** for the new path — layout and output live only under `schema { … }`.

Other `*.schema` files may declare shared types; the root `XCell.schema` is the
project definition surface.

## `schema { }` — layout + output

```text
SchemaFile      ::= SchemaTop*
SchemaTop       ::= SchemaProject | ClassDecl | SingletonDecl | EnumsDecl | FlagsDecl

SchemaProject   ::= 'schema' '{' ProjectItem* '}'
ProjectItem     ::= LayoutBlock | OutputBlock | Setting

LayoutBlock     ::= 'layout' '{' LayoutAssign* '}'
LayoutAssign    ::= LayoutKey '=' Integer ('; | ',')?
LayoutKey       ::= 'field' | 'typing' | 'type' | 'comment' | 'data'

OutputBlock     ::= 'output' '{' OutputItem* '}'
OutputItem      ::= TargetBlock | Setting
TargetBlock     ::= Ident '{' Setting* '}'
                 | 'target' Ident '{' Setting* '}'
Setting         ::= Ident '=' (Integer | String | Bool | Ident) ('; | ',')?

ClassDecl       ::= 'class' Ident '{' ClassField* '}'
SingletonDecl   ::= 'singleton' Ident '{' ClassField* '}'
…
```

### Exact `layout` fields

| Key | Meaning | Default |
|-----|---------|---------|
| `field` | Field-name row (1-based) | `1` |
| `typing` | Typing row (1-based); alias `type` | `2` |
| `comment` | Comment row (1-based); `0` = none | `3` |
| `data` | First data row (1-based) | `4` |

### Exact `output` fields

Block name is **`output`** (not `output_plan`).

**Top-level inside `output { }`:**

| Key | Meaning |
|-----|---------|
| `dir` | Optional default output directory under the XCell project root |
| *(target block)* | Named generator — see below |
| *(other `k = v`)* | Reserved extras |

**Target blocks** — `cocos { … }`, `unity { … }`, `typescript { … }`, `json { … }`,
`xlua { … }`, `sql { … }`, or `target Name { … }`. Custom names allowed.

| Key | Meaning |
|-----|---------|
| `enable` | `true`/`false` (default **true** if omitted) |
| `project` | Host engine project path, relative to XCell root |
| `code` | Generated **code** dir (relative to `project` if set, else XCell root) |
| `data` | Generated **data** dir (same relativity rules) |
| `dir` | Single output dir when code/data are not split (e.g. json) |
| `namespace` | Language namespace / package (Unity, …) |
| `template` | Optional template directory |

Paths use quoted strings: `project = "../MyCocos"`.

### Example

```text
schema {
  layout {
    field = 1
    typing = 2
    comment = 3
    data = 4
  }

  output {
    dir = "generated"

    cocos {
      enable = true
      project = "../MyCocos"
      code = "assets/scripts/dataTable/generated"
      data = "assets/resources/data"
    }

    unity {
      enable = true
      project = "../MyUnity"
      code = "Assets/Scripts/DataTable/Generated"
      data = "Assets/StreamingAssets/Data"
      namespace = "XCell"
    }

    typescript {
      enable = false
      project = "."
      code = "src/generated"
    }

    json {
      enable = true
      dir = "export/json"
    }
  }
}

class Foo { id: i32 }
singleton GameConfig { version: i32 }
enums Rarity { N = 1 }
flags Perk { A = 1 }
```

Plugins (Cocos / Unity): user picks the XCell project root → Rust loads
`XCell.schema` there → reads `schema.layout` and `schema.output` for generators.

## Field-name markers

| Marker | Meaning |
|--------|---------|
| `@@id` | Primary key |
| `@name` | Unique constraint |

If no `@@` field exists, the **first** field is the primary key by default.

## Typing references

| Cell | Meaning |
|------|---------|
| `&OtherTable` | Cross-table reference |
| `OtherTable` | Inline table class |
| `SchemaClass` / `SchemaSingleton` | Inline schema type |
| `&SchemaClass` | **Invalid** |
| `ItemKind` / `Permission` | Schema enums / flags |

## Cell values

| Kind | Accepted forms |
|------|----------------|
| enums | Member name **or** raw integer |
| flags | `A,B,C` names **or** raw integer |
| class / singleton | `key=value` / `key:value` pairs |
| `&Table` | Target table PK |
