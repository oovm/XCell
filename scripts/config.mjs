// 配置文件

export const config = {
    // 路径配置
    paths: {
        // 可执行文件路径
        xcell: {
            debug: 'target/debug/xcell.exe',
            release: 'target/release/xcell.exe'
        },
        // 生成目录
        generated: {
            unity: 'Assets/Scripts/DataTable/Generated',
            cocos: 'assets/scripts/dataTable/generated'
        },
        // WASM 相关路径
        wasm: {
            dir: 'backends/xcell-wasi',
            output: 'target/wasm32-wasip1/release/xcell_wasi.wasm',
            js: 'xcell_wasi.js'
        },
        // 前端相关路径
        frontend: {
            xcell: 'frontends/xcell',
            lib: 'lib'
        }
    },
    // 示例目录
    examples: {
        rpg: 'examples/rpg',
        rpgUntyped: 'examples/rpg-untyped',
        rpgTyped: 'examples/rpg-typed',
        galgame: 'examples/galgame',
        slg: 'examples/slg'
    }
};
