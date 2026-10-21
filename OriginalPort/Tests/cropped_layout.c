#include "../ios/render_scale.h"
#include <assert.h>
#include <stdio.h>
int main(void) {
    double cases[][2]={{1260,2736},{2853,2007},{1398,2034},{2400,3200},{320,480}};
    for (unsigned i=0;i<sizeof(cases)/sizeof(cases[0]);i++) {
        double w=cases[i][0],h=cases[i][1];
        assert(KoiPresentationZoom(w,h,0)==1);
        double fit=fmin(w/320,h/480), z=KoiPresentationZoom(w,h,1);
        double vw=320*fit*z,vh=480*fit*z;
        assert(vw+1e-7>=w && vh+1e-7>=h);
        assert(fabs(vw/vh-2.0/3.0)<1e-10);
        assert(fabs(vw-w)<1e-7 || fabs(vh-h)<1e-7);
        // Inverse touch transform keeps the middle of the window at pond center.
        assert(fabs(((w/2-(w-vw)/2)/vw)*320-160)<1e-7);
        assert(fabs(((h/2-(h-vh)/2)/vh)*480-240)<1e-7);
    }
    assert(KoiPresentationZoom(0,480,1)==1);
    assert(KoiPresentationZoom(NAN,480,1)==1);
    puts("Cropped layout: fit/fill, aspect, and touch-center checks passed for 5 displays");
}
