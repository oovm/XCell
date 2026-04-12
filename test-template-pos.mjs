const template = `// 代码生成, 修改无效! (XCell <% compiler_version %>)

<% if namespace %>
export namespace <% namespace %> {
<% end if %>

export enum <% class_name %> {
<% loop field in enumerate_ids %>
    <% loop line in field.document %>
    /** <% line %> */
    <% end loop %>
    <% field.key %> = <% field.value %>,
<% end loop %>
}

<% if namespace %>
}
<% end if %>`;

console.log("Template length:", template.length);
console.log("Position 69..79:", JSON.stringify(template.slice(69, 79)));
console.log("Position 65..85:", JSON.stringify(template.slice(65, 85)));

// Find all positions
for (let i = 0; i < template.length; i++) {
    if (template.slice(i, i+2) === '<%') {
        const end = template.indexOf('%>', i);
        console.log(`Position ${i}..${end+2}: ${JSON.stringify(template.slice(i, end+2))}`);
    }
}
