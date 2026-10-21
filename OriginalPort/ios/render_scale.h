#pragma once
#include <math.h>

// Preserve 320x480 coordinates. Use a power-of-two scale so the original
// repeating offscreen textures remain complete under GLES 1 texture rules.
static inline unsigned ChuzzleRenderScale(double pixelWidth, double pixelHeight) {
    double fit = fmin(pixelWidth / 320.0, pixelHeight / 480.0);
    if (!isfinite(fit) || fit <= 1.0) return 1;
    unsigned scale = 1;
    while (scale < 8 && scale < fit) scale *= 2;
    return scale;
}

// Enlarge the aspect-fit image uniformly to cover the window; never stretch it.
static inline double KoiPresentationZoom(double width, double height, int crop) {
    if (!crop || !isfinite(width) || !isfinite(height) || width <= 0 || height <= 0) return 1.0;
    return fmax(width / 320.0, height / 480.0) / fmin(width / 320.0, height / 480.0);
}
