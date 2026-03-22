import { readFileSync, writeFileSync, readdirSync } from 'fs';
import { join } from 'path';

const templatesDir = join(process.cwd(), 'backends', 'xcell-generator', 'templates');

const replacements = [
    { from: /<%\s*endif\s*%>/g, to: '<% end if %>' },
    { from: /<%-\s*endif\s*%>/g, to: '<%- end if %>' },
    { from: /<%\s*endfor\s*%>/g, to: '<% end for %>' },
    { from: /<%-\s*endfor\s*%>/g, to: '<%- end for %>' },
    { from: /<%\s*endloop\s*%>/g, to: '<% end loop %>' },
    { from: /<%-\s*endloop\s*%>/g, to: '<%- end loop %>' },
];

const files = readdirSync(templatesDir).filter(f => f.endsWith('.dejavu'));

let totalFixed = 0;

for (const file of files) {
    const filePath = join(templatesDir, file);
    let content = readFileSync(filePath, 'utf-8');
    let originalContent = content;
    
    for (const { from, to } of replacements) {
        content = content.replace(from, to);
    }
    
    if (content !== originalContent) {
        writeFileSync(filePath, content, 'utf-8');
        console.log(`Fixed: ${file}`);
        totalFixed++;
    }
}

console.log(`\nTotal files fixed: ${totalFixed}`);
