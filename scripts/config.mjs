// 配置文件

export const config = {
    // 示例目录
    examples: {
        rpgUntyped: 'examples/rpg',
        rpgTyped: 'examples/rpg',
        galgame: 'examples/galgame',
        slg: 'examples/slg'
    },
    // 路径配置
    paths: {
        // xcell 可执行文件路径
        xcell: {
            debug: 'backends/xcell/target/debug/xcell.exe'
        },
        // 生成文件路径
        generated: {
            unity: 'unity',
            cocos: 'cocos'
        }
    }
};