const SCAN_SIZE = 0.0005;

// Function inspired by this old project
// https://github.com/victorhqc/chameleon/blob/master/chameleon.class.js
export function getColorFrom(imageBase64: string, scanSize = SCAN_SIZE): Promise<string> {
  return new Promise((resolve, reject) => {
    try {
      const img = new Image();
      img.src = imageBase64;

      img.onload = () => {
        const data = getImageData(img);
        const rgbData = getRGBDataFrom(data, scanSize);

        resolve(getMostPopularColor(rgbData));
      };
    } catch (e) {
      reject(e);
    }
  });
}

function getMostPopularColor(data: RGBData): string {
  const popular = Object.keys(data).reduce(
    (acc, hex) => {
      const current = data[hex];

      if (current.length > acc.len) {
        return { len: current.length, hex };
      }

      return acc;
    },
    { len: 0, hex: '' } as { len: number; hex: string }
  );

  return popular.hex;
}

function getImageData(img: HTMLImageElement): ImageData {
  const canvas = document.createElement('canvas');
  const context = canvas.getContext('2d');

  if (!context) {
    throw new Error('Could not get canvas context');
  }

  const height = (canvas.height = img.naturalHeight || img.offsetHeight || img.height);
  const width = (canvas.width = img.naturalWidth || img.offsetWidth || img.width);

  context.drawImage(img, 0, 0);
  const imgData = context.getImageData(0, 0, width, height);

  return imgData;
}

function getRGBDataFrom(img: ImageData, scanSize = SCAN_SIZE): RGBData {
  const length = img.data.length;
  const blockSize = Math.floor(length * scanSize);
  const rgbBlocks: RGB[] = [];

  let index = -4;
  while ((index += blockSize * 4) < length) {
    rgbBlocks.push({
      r: img.data[index],
      g: img.data[index + 1],
      b: img.data[index + 2],
    });
  }

  const result: RGBData = rgbBlocks.reduce((acc, rgb, i) => {
    if (!isNotWhiteOrBlack(rgb)) return acc;

    const hex = rgbToHex(rgb);
    if (!acc[hex]) {
      acc[hex] = [];
    }

    const values = rgbBlocks
      .filter((rgb, j) => i !== j && !acc[hex].includes(rgb))
      .filter((rgbBlock) => {
        const red = Math.abs(rgb.r - rgbBlock.r);
        const green = Math.abs(rgb.g - rgbBlock.g);
        const blue = Math.abs(rgb.b - rgbBlock.b);

        // if the colors are similar, then include it.
        return red <= 10 && green <= 10 && blue <= 10;
      });

    return {
      ...acc,
      [hex]: [...acc[hex], ...values],
    };
  }, {} as RGBData);

  return result;
}

function isNotWhiteOrBlack(rgb: RGB) {
  return rgb.r > 10 && rgb.r < 245 && rgb.g > 10 && rgb.g < 245 && rgb.b > 10 && rgb.b < 245;
}

function rgbToHex(rgb: RGB): string {
  return '#' + ((1 << 24) + (rgb.r << 16) + (rgb.g << 8) + rgb.b).toString(16).slice(1);
}

type RGBData = Record<string, RGB[]>;

type RGB = {
  r: number;
  g: number;
  b: number;
};
