/* Synthetic X11 fixture and bounded PNG-only selection receiver. */
#define _POSIX_C_SOURCE 200809L
#include <X11/Xlib.h>
#include <X11/Xatom.h>
#include <errno.h>
#include <poll.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/resource.h>
#include <time.h>
#include <unistd.h>

#define PNG_MAX (16UL * 1024 * 1024)
#define RAW_MAX (64UL * 1024 * 1024)
static Display *display;
static Window window;
static Atom clipboard, image_png, property;

static void result(const char *name, unsigned long bytes, unsigned long requested) {
    printf("{\"name\":\"%s\",\"encoded_bytes\":%lu,\"largest_property_request\":%lu}\n", name, bytes, requested);
    fflush(stdout);
}
static long long now_ms(void) {
    struct timespec value;
    if (clock_gettime(CLOCK_MONOTONIC, &value)) exit(2);
    return value.tv_sec * 1000LL + value.tv_nsec / 1000000;
}
static int event_until(XEvent *event, long long deadline) {
    while (now_ms() < deadline) {
        if (XPending(display)) { XNextEvent(display, event); return 1; }
        struct pollfd fd = {ConnectionNumber(display), POLLIN, 0};
        if (poll(&fd, 1, 20) < 0 && errno != EINTR) return 0;
    }
    return 0;
}
static unsigned char *load(const char *path, unsigned long *length) {
    FILE *file = fopen(path, "rb");
    if (!file) return NULL;
    if (fseek(file, 0, SEEK_END) || ftell(file) < 0 || (unsigned long)ftell(file) > PNG_MAX + 1) {
        fclose(file); return NULL;
    }
    *length = (unsigned long)ftell(file);
    rewind(file);
    unsigned char *bytes = malloc(*length ? *length : 1);
    if (!bytes || fread(bytes, 1, *length, file) != *length) { free(bytes); fclose(file); return NULL; }
    fclose(file); return bytes;
}
static uint32_t be32(const unsigned char *p) {
    return ((uint32_t)p[0] << 24) | ((uint32_t)p[1] << 16) | ((uint32_t)p[2] << 8) | p[3];
}
static int capture(const char *path) {
    if (!XGetSelectionOwner(display, clipboard)) { result("CLIPBOARD_EMPTY", 0, 0); return 3; }
    XConvertSelection(display, clipboard, image_png, property, window, CurrentTime);
    XFlush(display);
    XEvent event;
    long long deadline = now_ms() + 3000;
    while (event_until(&event, deadline)) {
        if (event.type != SelectionNotify) continue;
        if (event.xselection.property == None) { result("CLIPBOARD_EMPTY", 0, 0); return 3; }
        Atom type = None; int format = 0; unsigned long count = 0, after = 0; unsigned char *data = NULL;
        /* Xlib receives at most 36 property bytes here, before image allocation. */
        int rc = XGetWindowProperty(display, window, property, 0, 9, False, AnyPropertyType,
                                    &type, &format, &count, &after, &data);
        if (rc != Success || type != image_png || format != 8 || count < 33) {
            if (data) XFree(data);
            result(type == XInternAtom(display, "INCR", False) ? "INCR_UNSUPPORTED" : "CORRUPT_IMAGE", 0, 36);
            return 3;
        }
        unsigned long length = count + after;
        uint32_t width = be32(data + 16), height = be32(data + 20);
        int valid = !memcmp(data, "\211PNG\r\n\032\n", 8) && be32(data + 8) == 13 &&
                    !memcmp(data + 12, "IHDR", 4) && width && height && width <= 8192 && height <= 8192 &&
                    (uint64_t)width * height <= 16777216 && (uint64_t)width * height * 4 <= RAW_MAX;
        XFree(data); data = NULL;
        if (!valid || length > PNG_MAX) { result("LIMIT_EXCEEDED", length, 36); return 3; }
        unsigned long requested = (length + 3) / 4;
        rc = XGetWindowProperty(display, window, property, 0, (long)requested, True, image_png,
                                &type, &format, &count, &after, &data);
        if (rc != Success || type != image_png || format != 8 || count != length || after) {
            if (data) XFree(data);
            result("CORRUPT_IMAGE", length, requested * 4); return 3;
        }
        FILE *file = fopen(path, "wb");
        if (!file) { XFree(data); return 2; }
        int ok = fwrite(data, 1, count, file) == count;
        if (fclose(file)) ok = 0;
        XFree(data);
        if (!ok) return 2;
        result("CAPTURED", length, requested * 4); return 0;
    }
    result("CAPTURE_TIMEOUT", 0, 0); return 3;
}
static int owner(const char *first, const char *second, int variant) {
    unsigned long length;
    unsigned char *data = load(first, &length);
    if (!data) return 2;
    XSetSelectionOwner(display, clipboard, window, CurrentTime);
    XFlush(display);
    if (XGetSelectionOwner(display, clipboard) != window) { free(data); return 2; }
    puts("READY"); fflush(stdout);
    long long deadline = now_ms() + 15000;
    for (;;) {
        if (now_ms() >= deadline) break;
        struct pollfd fd = {STDIN_FILENO, POLLIN, 0};
        if (poll(&fd, 1, 0) > 0) {
            char command[16];
            if (!fgets(command, sizeof command, stdin)) break;
            if (!strncmp(command, "next", 4) && second) {
                unsigned long next_length;
                unsigned char *next = load(second, &next_length);
                if (!next) { free(data); return 2; }
                free(data); data = next; length = next_length;
                puts("CHANGED"); fflush(stdout);
            } else if (!strncmp(command, "stop", 4)) break;
        }
        XEvent event;
        if (!event_until(&event, now_ms() + 20)) continue;
        if (event.type == SelectionClear) break;
        if (event.type != SelectionRequest) continue;
        XSelectionRequestEvent *request = &event.xselectionrequest;
        Atom destination = request->property ? request->property : request->target;
        XEvent response; memset(&response, 0, sizeof response);
        response.xselection.type = SelectionNotify;
        response.xselection.display = display;
        response.xselection.requestor = request->requestor;
        response.xselection.selection = request->selection;
        response.xselection.target = request->target;
        response.xselection.time = request->time;
        response.xselection.property = None;
        if (request->target == image_png && variant != 2) {
            if (variant == 1) {
                unsigned long size = length;
                XChangeProperty(display, request->requestor, destination, XInternAtom(display, "INCR", False),
                                32, PropModeReplace, (unsigned char *)&size, 1);
            } else {
                /* Chunk writes avoid the X11 maximum-request size, not INCR conversion. */
                XChangeProperty(display, request->requestor, destination, image_png, 8, PropModeReplace, data, 0);
                for (unsigned long offset = 0; offset < length; offset += 65536) {
                    unsigned long part = length - offset;
                    if (part > 65536) part = 65536;
                    XChangeProperty(display, request->requestor, destination, image_png, 8, PropModeAppend,
                                    data + offset, (int)part);
                }
            }
            response.xselection.property = destination;
        }
        XSendEvent(display, request->requestor, False, 0, &response);
        XFlush(display);
    }
    free(data); return 0;
}
int main(int argc, char **argv) {
    struct rlimit address = {256UL * 1024 * 1024, 256UL * 1024 * 1024};
    if (setrlimit(RLIMIT_AS, &address)) return 2;
    if (argc < 3) return 2;
    display = XOpenDisplay(NULL);
    if (!display) { result("CLIPBOARD_UNAVAILABLE", 0, 0); return 3; }
    window = XCreateSimpleWindow(display, DefaultRootWindow(display), 0, 0, 1, 1, 0, 0, 0);
    clipboard = XInternAtom(display, "CLIPBOARD", False);
    image_png = XInternAtom(display, "image/png", False);
    property = XInternAtom(display, "SSHX_PHASE0", False);
    int status;
    if (!strcmp(argv[1], "capture")) status = capture(argv[2]);
    else if (!strcmp(argv[1], "owner")) status = owner(argv[2], argc > 3 ? argv[3] : NULL, 0);
    else if (!strcmp(argv[1], "incr")) status = owner(argv[2], NULL, 1);
    else if (!strcmp(argv[1], "text")) status = owner(argv[2], NULL, 2);
    else status = 2;
    XDestroyWindow(display, window); XCloseDisplay(display); return status;
}
