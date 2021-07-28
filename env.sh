#export OPENCV_LINK_LIBS=/usr/local/opt/opencv@4/lib
# They keep fiddling with where this lives.
#export DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/usr/lib"
export DYLD_FALLBACK_LIBRARY_PATH=/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/:/usr/local/lib
export PKG_CONFIG_PATH="${PKG_CONFIG_PATH}:${OPENCV_LINK_LIBS}/pkgconfig"
