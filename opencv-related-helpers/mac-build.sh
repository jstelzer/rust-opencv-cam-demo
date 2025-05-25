#!/usr/bin/env bash
##
# on macos, brew doesn't link opencv to qt.
# this script builds opencv with qt support.
# I need qt to draw my help text.
#
# So, i'm managing my own opencv build.
# Brew defaults to qt6 but opencv barely supports 5.
#
# If you have them installed in parallel, 6 and 5, the cmake 
#stuff gets confused and generates completely invalid cflags files.
#
# On the linux side I can work around it easy via docker builds.
##

# Ensure Homebrew's qt@5 is installed
if ! brew list qt@5 &>/dev/null; then
    echo "qt@5 not found, installing with Homebrew..."
    brew install qt@5
fi

# Set Qt5 paths from Homebrew
export QT5PATH="$(brew --prefix qt@5)"
export Qt5_DIR="$QT5PATH/lib/cmake/Qt5"
export PATH="$QT5PATH/bin:$PATH"
export PKG_CONFIG_PATH="$QT5PATH/lib/pkgconfig:$PKG_CONFIG_PATH"

# Set deployment target for compatibility
export MACOSX_DEPLOYMENT_TARGET=10.15

# Optional: Clean previous build
rm -rf build

# Run CMake with Qt5 and optimizations
cmake -G Ninja \
    -D CMAKE_INSTALL_PREFIX=/opt/QtOpenCV \
    -D CMAKE_BUILD_TYPE=Release \
    -D ENABLE_LTO=ON \
    -D CMAKE_CXX_FLAGS="-march=native" \
    -D CMAKE_INSTALL_DO_STRIP=ON \
    -D BUILD_TESTS=OFF \
    -D BUILD_PERF_TESTS=OFF \
    -D BUILD_DOCS=OFF \
    -D BUILD_JAVA=OFF \
    -D BUILD_opencv_apps=OFF \
    -D OPENCV_EXTRA_MODULES_PATH=../opencv_contrib/modules \
    -D INSTALL_PYTHON_EXAMPLES=ON \
    -D INSTALL_C_EXAMPLES=OFF \
    -D BUILD_opencv_python2=OFF \
    -D BUILD_opencv_python3=OFF \
    -D WITH_QT=ON \
    -D OPENCV_ENABLE_NONFREE=ON \
    -D WITH_GSTREAMER=ON \
    -D WITH_OPENGL=ON \
    -D BUILD_EXAMPLES=ON \
    -D QT5_DIR="$Qt5_DIR" \
    -D CMAKE_PREFIX_PATH="$QT5PATH" \
    -S . -B ./build

ninja -C ./build

# Build
#cmake --build ./build -j"$(sysctl -n hw.logicalcpu)"

# To check library dependencies, use:
# find /opt/QtOpenCV -type f -perm +111 -exec otool -L {} \\;
