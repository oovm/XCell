import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import { execSync } from 'child_process';

// 主函数
async function main() {
    // 使用相对路径
    const __filename = fileURLToPath(import.meta.url);
    const scriptDir = path.dirname(__filename);
    const projectRoot = path.resolve(scriptDir, '..');
    const rpgDir = path.resolve(projectRoot, 'examples', 'rpg');
    
    console.log('Finding xcell executable...');
    
    // 检查 RPG 示例目录是否存在
    if (!fs.existsSync(rpgDir)) {
        console.error(`RPG example directory not found at: ${rpgDir}`);
        console.error('Please make sure the RPG example project exists.');
        return;
    }
    
    // 找到 debug 版本的 xcell 可执行文件
    const xcellPath = path.resolve(projectRoot, 'target', 'debug', 'xcell.exe');
    
    if (!fs.existsSync(xcellPath)) {
        console.error(`xcell.exe not found at: ${xcellPath}`);
        console.error('Please run "cargo build" first to compile xcell.');
        return;
    }
    
    console.log(`Found xcell.exe at: ${xcellPath}`);
    
    // 执行 xcell generate 命令
    console.log('Running xcell generate...');
    
    try {
        execSync(`"${xcellPath}"`, { cwd: rpgDir, stdio: 'inherit' });
        console.log('xcell generate completed successfully!');
        
        // 验证生成结果
        console.log('Verifying generated files...');
        const generatedDir = path.resolve(rpgDir, 'src', 'generated');
        
        if (fs.existsSync(generatedDir)) {
            console.log(`Generated files found at: ${generatedDir}`);
        } else {
            console.warn('Generated files directory not found. Please check the generation process.');
        }
    } catch (error) {
        console.error('xcell generate failed:', error.message);
        return;
    }
    
    console.log('\nAll tasks completed successfully!');
}

// 运行主函数
main().catch(console.error);
