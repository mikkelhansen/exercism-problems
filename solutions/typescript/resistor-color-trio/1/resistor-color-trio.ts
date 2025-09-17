const RESISTOR_COLOR_MAP: { [key: string]: number } = {
  "black": 0,
  "brown": 1,
  "red": 2,
  "orange": 3,
  "yellow": 4,
  "green": 5,
  "blue": 6,
  "violet": 7,
  "grey": 8,
  "white": 9,
};


export function decodedResistorValue([firstColor, secondColor, thirdColor]: string[]): string {
  let value = (RESISTOR_COLOR_MAP[firstColor] * 10 + RESISTOR_COLOR_MAP[secondColor]) * (10 ** RESISTOR_COLOR_MAP[thirdColor]);
  let unit = 'ohms';

  if (value >= 1000000000) {
    value /= 1000000000;
    unit = 'gigaohms';
  } else if (value >= 1000000) {
    value /= 1000000;
    unit = 'megaohms';
  } else if (value >= 1000) {
    value /= 1000;
    unit = 'kiloohms';
  }

  return `${value} ${unit}`;
}
