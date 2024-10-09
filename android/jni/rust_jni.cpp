/* JNI calls. */

#include <jni.h>

#include "logging.h"

// include rust.h as a C ABI header
extern "C" {
#include "../../rust/rust.h"
}


// export all JNI functions with a C ABI
extern "C" {

// the entry() function
JNIEXPORT void JNICALL Java_example_rust_Rust_entry
  (JNIEnv *, jclass)
{
#ifdef DEBUG_LOGGING
    // Start the stdout -> android logging thread.
    start_logger("example.rust.native");
#endif
    entry();
}

}
