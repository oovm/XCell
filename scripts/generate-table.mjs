import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import { execSync } from 'child_process';
import { config } from './config.mjs';

// 主函数
async function main() {
    // 使用相对路径
    const __filename = fileURLToPath(import.meta.url);
    const scriptDir = path.dirname(__filename);
    const projectRoot = path.resolve(scriptDir, '..');
    const rpgDirectories = [
        path.resolve(projectRoot, config.examples.rpgUntyped),
        path.resolve(projectRoot, config.examples.rpgTyped),
        path.resolve(projectRoot, config.examples.galgame),
        path.resolve(projectRoot, config.examples.slg),
        path.resolve(projectRoot, config.examples.mmorpg),
        path.resolve(projectRoot, config.examples.roguelike),
        path.resolve(projectRoot, config.examples.towerDefense),
        path.resolve(projectRoot, config.examples.tcg)
    ];
    
    console.log('Finding xcell executable...');
    
    // 找到 debug 版本的 xcell 可执行文件
    const xcellPath = path.resolve(projectRoot, config.paths.xcell.debug);
    
    if (!fs.existsSync(xcellPath)) {
        console.error(`xcell.exe not found at: ${xcellPath}`);
        console.error('Please run "cargo build" first to compile xcell.');
        return;
    }
    
    console.log(`Found xcell.exe at: ${xcellPath}`);
    
    // 处理每个 RPG 目录
    for (const rpgDir of rpgDirectories) {
        console.log(`\nProcessing directory: ${rpgDir}`);
        
        // 检查 RPG 示例目录是否存在
        if (!fs.existsSync(rpgDir)) {
            console.error(`RPG example directory not found at: ${rpgDir}`);
            console.error('Please make sure the RPG example project exists.');
            continue;
        }
        
        // 执行 xcell generate 命令
        console.log('Running xcell generate...');
        
        try {
            // 安全检查：确保路径是安全的
            const safeXcellPath = path.resolve(xcellPath);
            if (!safeXcellPath.includes('xcell.exe')) {
                console.error('Security warning: Invalid xcell executable path');
                continue;
            }
            execSync(`"${safeXcellPath}"`, { cwd: rpgDir, stdio: 'inherit' });
            console.log('xcell generate completed successfully!');
            
            // 验证生成结果
            console.log('Verifying generated files...');
            
            // 检查 Unity 生成文件
            const unityGeneratedDir = path.resolve(rpgDir, config.paths.generated.unity);
            if (fs.existsSync(unityGeneratedDir)) {
                console.log(`Unity generated files found at: ${unityGeneratedDir}`);
            } else {
                console.warn('Unity generated files directory not found. Please check the generation process.');
            }
            
            // 检查 Cocos 生成文件
            const cocosGeneratedDir = path.resolve(rpgDir, '..', config.paths.generated.cocos);
            if (fs.existsSync(cocosGeneratedDir)) {
                console.log(`Cocos generated files found at: ${cocosGeneratedDir}`);
            } else {
                console.warn('Cocos generated files directory not found. Please check the generation process.');
            }
        } catch (error) {
            console.error('xcell generate failed:', error.message);
            continue;
        }
    }
    
    console.log('\nAll tasks completed successfully!');
}

// 运行主函数
main().catch(console.error);
