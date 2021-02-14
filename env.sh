# Not quite... but got me on track.
#export DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/Toolchains/XcodeDefault.xctoolchain/usr/lib/"

export DYLD_FALLBACK_LIBRARY_PATH=/Library/Developer/CommandLineTools/usr/lib/
export PKG_CONFIG_PATH="${PKG_CONFIG_PATH}:${OPENCV_LINK_LIBS}/pkgconfig"
