const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

// 测试目录路径
const testDir = path.join(__dirname, 'examples', 'test-rpg');

// 检查是否存在 ProjectSettings.toml 文件
const settingsPath = path.join(testDir, 'ProjectSettings.toml');

console.log('Testing ProjectSettings.toml creation...');

// 尝试运行 xcell 命令（即使构建失败，我们的修改应该已经编译进 xcell-config）
try {
    // 运行 xcell 命令
    execSync('pnpm xcell generate', { cwd: testDir, stdio: 'inherit' });
} catch (error) {
    console.log('Command failed, but checking if ProjectSettings.toml was created anyway...');
}

// 检查文件是否存在
if (fs.existsSync(settingsPath)) {
    console.log('✓ ProjectSettings.toml created successfully!');
    // 检查文件内容
    const content = fs.readFileSync(settingsPath, 'utf8');
    if (content.length > 0) {
        console.log('✓ ProjectSettings.toml has content');
        console.log('Content preview:');
        console.log(content.substring(0, 200) + '...');
    } else {
        console.log('⚠ ProjectSettings.toml is empty');
    }
} else {
    console.log('✗ ProjectSettings.toml was not created');
}
