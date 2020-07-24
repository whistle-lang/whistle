let list = [
  "import",
  "as",
  "from",
  "export",
  "fun",
  "return",
  "if",
  "else",
  "while",
  "loop",
  "break",
  "continue",
  "var",
  "val",
  "for",
  "in",
  "match",
  "type",
  "struct",
  "trait",
];

list = list.sort()
  .map((e) => e[0].toUpperCase() + e.substr(1));

for (const el of list) {
  console.log(`${el},`);
}
