/* Java wrapper around our native C interface.
 */
package example.rust;

public class Rust {
    static {
        System.loadLibrary("app"); // same as name of android_binary target
    }
    public static native void entry();
}
