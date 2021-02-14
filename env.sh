export DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/usr/lib"
export PKG_CONFIG_PATH="${PKG_CONFIG_PATH}:${OPENCV_LINK_LIBS}/pkgconfig"
