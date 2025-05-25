#!/usr/bin/env bash
# https://gist.github.com/liviaerxin/6ee3d4faea1614572e621d81d0e114c8
# 
# On the linux side, I'm using arch which does link to qt.
# But if I ever need to go down this path, here's an outline
##
if [ -d build ]; then
    rm -rf build
fi

mkdir build



export LDFLAGS="-L/opt/homebrew/opt/qt@5/lib"
export CPPFLAGS="-I/opt/homebrew/opt/qt@5/include"



cmake -G Ninja \
    -D CMAKE_BUILD_TYPE=RELEASE \
    -D CMAKE_INSTALL_PREFIX=${HOME}/QtOpenCV \
    -D OPENCV_EXTRA_MODULES_PATH=../opencv_contrib/modules \
    -D ENABLE_LTO=ON \
    -D CMAKE_POLICY_VERSION_MINIMUM=3.5 \
    -D WITH_PKGCONFIG=ON \
    -D OPENCV_GENERATE_PKGCONFIG=ON \
    -D CMAKE_CXX_FLAGS="-march=native" \
    -D CMAKE_INSTALL_DO_STRIP=ON \
    -D BUILD_TESTS=OFF \
    -D BUILD_PERF_TESTS=OFF \
    -D BUILD_DOCS=OFF \
    -D BUILD_JAVA=OFF \
    -D INSTALL_PYTHON_EXAMPLES=OFF \
    -D INSTALL_C_EXAMPLES=OFF \
    -D BUILD_opencv_python2=OFF \
    -D BUILD_opencv_python3=OFF \
    -D WITH_QT=ON \
    -D OPENCV_ENABLE_NONFREE=ON \
    -D WITH_GSTREAMER=ON \
    -D WITH_OPENGL=ON \
    -D BUILD_EXAMPLES=ON -S . -B ./build

ninja -C ./build
# cd build
# make -j$(sysctl -n hw.physicalcpu)

# Please compile with -DACCELERATE_NEW_LAPACK to access the new headers and -DACCELERATE_LAPACK_ILP64 for ILP64 support.
