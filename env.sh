# Not quite... but got me on track.
#export DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/Toolchains/XcodeDefault.xctoolchain/usr/lib/"

export DYLD_FALLBACK_LIBRARY_PATH=/Library/Developer/CommandLineTools/usr/lib/
export CV_BASE=/usr/local/opt/opencv
export OpenCV_DIR=${CV_BASE}
export OPENCV_LINK_PATHS=${CV_BASE}/lib 
export OPENCV_INCLUDE_PATHS=${CV_BASE}/include/opencv4
export PKG_CONFIG_PATH="${PKG_CONFIG_PATH}:${OPENCV_LINK_LIBS}/pkgconfig"
