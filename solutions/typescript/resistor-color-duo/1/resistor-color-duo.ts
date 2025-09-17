export function decodedValue(colors: string[]): number {
  const colorsArray = ["black", "brown", "red", "orange", "yellow", "green", "blue", "violet", "grey", "white"]
  return parseInt(colorsArray.indexOf(colors[0]).toString() + colorsArray.indexOf(colors[1]).toString())
}
