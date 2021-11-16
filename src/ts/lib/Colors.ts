export type RGB = {
    r: number;
    g: number;
    b: number;
};

export type XYZ = {
    x: number;
    y: number;
    z: number;
};

export type LAB = {
    l: number;
    a: number;
    b: number;
};

/**
 * Based on https://stackoverflow.com/a/59602352
 */
export default class Colors
{
    // X, Y, Z of a "D65" light source.
    // "D65" is a standard 6500K Daylight light source.
    // https://en.wikipedia.org/wiki/Illuminant_D65
    public static readonly D65 = [95.047, 100, 108.883];

    /**
     * Converts RGB color to CIE 1931 XYZ color space.
     * https://www.image-engineering.de/library/technotes/958-how-to-convert-between-srgb-and-ciexyz
     */
    public static rgb2xyz (rgb: RGB): XYZ
    {
        rgb.r = this.sRGBtoLinearRGB(rgb.r / 255);
        rgb.g = this.sRGBtoLinearRGB(rgb.g / 255);
        rgb.b = this.sRGBtoLinearRGB(rgb.b / 255);

        const xyz: XYZ = {
            x: 100 * (0.4124 * rgb.r + 0.3576 * rgb.g + 0.1805 * rgb.b),
            y: 100 * (0.2126 * rgb.r + 0.7152 * rgb.g + 0.0722 * rgb.b),
            z: 100 * (0.0193 * rgb.r + 0.1192 * rgb.g + 0.9505 * rgb.b)
        };

        return xyz;
    }


    /**
     * Undoes gamma-correction from an RGB-encoded color.
     * https://en.wikipedia.org/wiki/SRGB#Specification_of_the_transformation
     * https://stackoverflow.com/questions/596216/formula-to-determine-brightness-of-rgb-color
     */
    public static sRGBtoLinearRGB (color: number): number
    {
        // Send this function a decimal sRGB gamma encoded color value
        // between 0.0 and 1.0, and it returns a linearized value.

        return color <= 0.04045 ?
            color / 12.92 :
            Math.pow((color + 0.055) / 1.055, 2.4);
    }

    /**
     * Converts CIE 1931 XYZ colors to CIE L*a*b*.
     * The conversion formula comes from <http://www.easyrgb.com/en/math.php>.
     * https://github.com/cangoektas/xyz-to-lab/blob/master/src/index.js
     */
    public static xyz2lab (xyz: XYZ): LAB
    {
        xyz.x /= this.D65[0];
        xyz.y /= this.D65[1];
        xyz.z /= this.D65[2];

        xyz.x = xyz.x > 0.008856 ? Math.pow(xyz.x, 1 / 3) : xyz.x * 7.787 + 16 / 116;
        xyz.y = xyz.y > 0.008856 ? Math.pow(xyz.y, 1 / 3) : xyz.y * 7.787 + 16 / 116;
        xyz.z = xyz.z > 0.008856 ? Math.pow(xyz.z, 1 / 3) : xyz.z * 7.787 + 16 / 116;

        return {
            l: 116 * xyz.y - 16,
            a: 500 * (xyz.x - xyz.y),
            b: 200 * (xyz.y - xyz.z)
        };
    }

    /**
     * Convert RGB to LAB
     */
    public static rgb2lab (rgb: RGB): LAB
    {
        return this.xyz2lab(this.rgb2xyz(rgb));
    }

    /**
     * Convert RGB to brightness level (0 - 100, black - white)
     */
    public static rgb2brightness (rgb: RGB): number
    {
        rgb.r = this.sRGBtoLinearRGB(rgb.r / 255);
        rgb.g = this.sRGBtoLinearRGB(rgb.g / 255);
        rgb.b = this.sRGBtoLinearRGB(rgb.b / 255);

        let y = 100 * (0.2126 * rgb.r + 0.7152 * rgb.g + 0.0722 * rgb.b) / this.D65[1];

        y = y > 0.008856 ? Math.pow(y, 1 / 3) : y * 7.787 + 16 / 116;

        return 116 * y - 16;
    }
}
