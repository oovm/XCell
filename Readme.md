# XCell — shared contract hub

This repository is the **shared contract hub** for all XCell language implementations.
It holds documentation, examples, frontends, and (over time) shared schema fixtures —
**not** language backends.

Language ports clone or submodule **this** tree and treat it as the source of truth for
cross-impl tests and docs.

## Canonical remote

| Role | URL | Branch |
|------|-----|--------|
| **Consumer / documented** | [`https://github.com/game-gpt/XCell.git`](https://github.com/game-gpt/XCell.git) | **`dev`** |
| Historical (legacy clones / some package metadata) | `https://github.com/oovm/XCell` | — |

**Use `game-gpt/XCell` @ `dev` for all new clones, submodules, and CI.**
Do not assume `oovm/XCell` is current.

## Purpose

- **Documentation** — user and maintainer guides under `documentation/`
- **Examples** — sample workspaces under `examples/`
- **Frontends** — homepage, H5/desktop shells, TS/Cocos packages under `frontends/`
- **Shared schema fixtures** (planned) — golden `XCell.schema` / table cases for every port

This repo does **not** ship Rust/C#/TS analyzer or generator backends. Those live in the
implementation repos below.

## Consumers

| Implementation | Role |
|----------------|------|
| **XCell.rs** | Rust implementation |
| **XCell.cs** | C# / .NET implementation |
| **XCell.ts** | TypeScript implementation |
| *(future ports)* | Same contract |

Each impl should **clone or git-submodule** this repository at branch **`dev`**, then point
tests / fixtures / docs at that checkout.

### How implementations should reference it

**Submodule (recommended):**

```bash
git submodule add -b dev https://github.com/game-gpt/XCell.git vendor/XCell
# or: docs/XCell, fixtures/XCell, etc.
git submodule update --init --remote
```

**Shallow clone in CI:**

```bash
git clone --depth 1 --branch dev https://github.com/game-gpt/XCell.git
```

Pin to a known commit of `dev` in release CI if you need bit-reproducible fixtures.
Treat paths under this tree (examples, documentation, future schema fixtures) as the
shared contract — do not fork divergent copies inside an impl repo without upstreaming.

## Config surface

Project configuration for the new path is **`XCell.schema`**, with layout and output under:

```text
schema {
  layout { … }
  output { … }
}
```

See **[documentation/schema-and-layout.md](documentation/schema-and-layout.md)** for the
exact grammar, `layout` / `output` keys, and examples. Legacy TOML project configs are
cut for this path.

## Layout

```
XCell/
├── documentation/   # Docs (incl. schema & layout)
├── examples/        # Example game / project workspaces
└── frontends/       # Homepage, SDKs, H5 / desktop / Cocos UI
```

## Frontends / docs workspace (optional)

Root `package.json` + `pnpm-workspace.yaml` exist only so `frontends/*` and
`documentation` can install/build with pnpm. There is no backend build here.

```bash
pnpm install
pnpm --filter opencrab-homepage dev   # example
```

## License

See repository license files where present.
