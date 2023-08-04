# Image Effect Experiment

The name is a placeholder - this project started out as just me wanting to learn more about dithering and more image effects. After looking into algorithms and other code I've seen online, this is what I've got!

**Note:** You can technically use this as a library - but it's definitely not currently stable so you may experience major breakages. The `src/main.rs` and `src/lib.rs` should contain examples in this case. I *suggest* not using it for anything other than side projects. It might get published, but currently I don't think it's at *that* level of quality yet.

Feel free to open issues / pull requests / fork if you'd like.

## Dithering

The **2-bit** dithering is separated purely because it lacks the need of a colour distance function, which makes it faster by default.

For now, the colour distance function used is **weighted euclidean**, which looks like this:

$$
f(R, G, B) = \begin{cases}
    \sqrt{2\Delta R^2 + 4\Delta G^2 + 3\Delta B^2} & \overline{R} < 128, \\
    \sqrt{3\Delta R^2 + 4\Delta G^2 + 2\Delta B^2} & \textrm{otherwise},
\end{cases}
$$

Currently supports the following algorithms:

|            **Name** | *2-bit*                                         | *RGB (Web-safe)*                                    | *RGB (8-bit)*                                    |
| ------------------: | :---------------------------------------------- | :-------------------------------------------------- | :----------------------------------------------- |
|               Basic | ![](./data/dither/basic-mono.png)               | ![](./data/dither/basic-web-safe.png)               | ![](./data/dither/basic-8-bit.png)               |
|     Floyd-Steinberg | ![](./data/dither/floyd-steinberg-mono.png)     | ![](./data/dither/floyd-steinberg-web-safe.png)     | ![](./data/dither/floyd-steinberg-8-bit.png)     |
| Jarvis-Judice-Ninke | ![](./data/dither/jarvis-judice-ninke-mono.png) | ![](./data/dither/jarvis-judice-ninke-web-safe.png) | ![](./data/dither/jarvis-judice-ninke-8-bit.png) |
|              Stucki | ![](./data/dither/stucki-mono.png)              | ![](./data/dither/stucki-web-safe.png)              | ![](./data/dither/stucki-8-bit.png)              |
|            Atkinson | ![](./data/dither/atkinson-mono.png)            | ![](./data/dither/atkinson-web-safe.png)            | ![](./data/dither/atkinson-8-bit.png)            |
|              Burkes | ![](./data/dither/burkes-mono.png)              | ![](./data/dither/burkes-web-safe.png)              | ![](./data/dither/burkes-8-bit.png)              |
|              Sierra | ![](./data/dither/sierra-mono.png)              | ![](./data/dither/sierra-web-safe.png)              | ![](./data/dither/sierra-8-bit.png)              |
|        SierraTwoRow | ![](./data/dither/sierra-two-row-mono.png)      | ![](./data/dither/sierra-two-row-web-safe.png)      | ![](./data/dither/sierra-two-row-8-bit.png)      |
|          SierraLite | ![](./data/dither/sierra-lite-mono.png)         | ![](./data/dither/sierra-lite-web-safe.png)         | ![](./data/dither/sierra-lite-8-bit.png)         |
|           Bayer 2x2 | ![](./data/dither/bayer-2x2-mono.png)           | ![](./data/dither/bayer-2x2-web-safe.png)           | ![](./data/dither/bayer-2x2-8-bit.png)           |
|           Bayer 4x4 | ![](./data/dither/bayer-4x4-mono.png)           | ![](./data/dither/bayer-4x4-web-safe.png)           | ![](./data/dither/bayer-4x4-8-bit.png)           |
|           Bayer 8x8 | ![](./data/dither/bayer-8x8-mono.png)           | ![](./data/dither/bayer-8x8-web-safe.png)           | ![](./data/dither/bayer-8x8-8-bit.png)           |
|         Bayer 16x16 | ![](./data/dither/bayer-16x16-mono.png)         | ![](./data/dither/bayer-16x16-web-safe.png)         | ![](./data/dither/bayer-16x16-8-bit.png)         |

## Colour

For colour, certain filters such as *brightness, saturation, hue rotation*, are done by first mapping each RGB pixel to HSL or LCH.
Originally, HSL was used due to the ease of computation - however as LCH is significantly more accurate in representing each of its
components HSL was soon replaced with LCH.

However, `RGB -> LCH` requires more computation than `RGB -> HSL`. Currently the code requires you change it in order to use the right pixel,
but it may be worth looking into allowing the user to use HSL instead for maximal speed.

Currently supports the following effects:

|         **Name** | *Image*                                |
| ---------------: | -------------------------------------- |
|    brighten +0.2 | ![](./data/colour/brighten+10.0.png)    |
|    brighten -0.2 | ![](./data/colour/brighten-10.0.png)    |
|     contrast 0.5 | ![](./data/colour/contrast.0.5.png)    |
|     contrast 1.5 | ![](./data/colour/contrast.1.5.png)    |
| gradient mapping | ![](./data/colour/gradient-mapped.png) |
|   rotate hue 180 | ![](./data/colour/rotate-hue-180.png)  |
|    saturate +0.2 | ![](./data/colour/saturate+10.0.png)    |
|    saturate -0.2 | ![](./data/colour/saturate-10.0.png)    |
|     quantize hue | ![](./data/colour/quantize-hue.png)    |