#include <X11/X.h>
#include <X11/Xlib.h>
#include <X11/Xutil.h>
#include <cstring>

extern "C" unsigned int unix_vkey_to_keysym(int vkey)
{
    char str;
    KeySym keysym;

    XKeyEvent xkey = {};
    xkey.keycode = vkey;
    XLookupString(&xkey, &str, 1, &keysym, 0);
    return keysym;
}