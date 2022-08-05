// Generate a colors.rs file based on the json
// colors taken from <https://cdn.tailwindcss.com/>
const text = await Deno.readTextFile("./colors.json");
const colors = JSON.parse(text);

function hexToRgb(hex: string) {
  const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
  return result
    ? {
      r: format(parseInt(result[1], 16) / 255),
      g: format(parseInt(result[2], 16) / 255),
      b: format(parseInt(result[3], 16) / 255),
    }
    : null;
}

function format(num: number) {
    return (Math.round(num * 100) / 100).toFixed(2)
}


const out = [];

for (const color of Object.keys(colors)) {
  for (const value in colors[color]) {
    const color_value = colors[color][value];
    const rgb = hexToRgb(color_value) || { r: 0, g: 0, b: 0 };
    out.push(
      `const ${color.toUpperCase()}_${value}: Color = Color::rgb(${rgb.r}, ${rgb.g}, ${rgb.b});`,
    );
  }
  out.push("");
}

await Deno.writeTextFile("./colors.rs", out.join("\n"));
