const fs = require("fs");

// Using language list from https://gist.github.com/ppisarczyk/43962d06686722d26d176fad46879d41


let langs = JSON.parse(fs.readFileSync("langs.json"));


const makeCategory = (cat) => `FileType::${cat[0].toUpperCase()}${cat.slice(1)}`;

const makeLangs = (l) => `Lang{name: "${l.name}", category: ${makeCategory(l.category)}}`;

const newFile = Object.entries(langs).map(([ext, lang]) => `        ("${ext}", ${makeLangs(lang)}),`).join("\n");

const oldFile = fs.readFileSync("language_file_extensions.rs.in").toString();

fs.writeFileSync("language_file_extensions.rs", oldFile.replace("// INSERT", newFile));
