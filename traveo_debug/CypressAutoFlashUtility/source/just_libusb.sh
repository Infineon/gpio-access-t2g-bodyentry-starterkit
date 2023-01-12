#!/bin/bash

#
# $ Copyright Cypress Semiconductor $
#

if [ "$PREREQUISITES_DONE" != "yes" ]; then
source ./prerequisites.sh
fi

###################################################################################################
#
# Download LibUSBTar balls
#
###################################################################################################


# Download libusb 1.x
if [ ! -e ./$DOWNLOAD_DIR/$LIBUSB_FILENAME ]; then
    echo "Downloading libUSB"
    cd $DOWNLOAD_DIR
    $WGET $LIBUSB_URL
    cd ..
fi




# Windows Specific
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "msys2" ]]; then

    # Download libusb 0.x on Windows
    if [ ! -e ./$DOWNLOAD_DIR/$LIBUSB_WIN32_FILENAME ]
    then
        echo "Downloading libUSB-win32"
    cd $DOWNLOAD_DIR
        $WGET $LIBUSB_WIN32_URL
    cd ..
    fi

fi


# Fetch libusb and libusb-compat on OS-X
if [[ "$OSTYPE" == *darwin* ]]; then

    if [ ! -e ./$DOWNLOAD_DIR/$LIBUSB_COMPAT_FILENAME ]; then
        echo "Downloading libUSB-compat"
                cd $DOWNLOAD_DIR
        $WGET $LIBUSB_COMPAT_URL
                cd ..
    fi
fi



###################################################################################################
#
# Build LibUSB
#
###################################################################################################


# Build libusb on Windows
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "msys2" ]]; then

    # NOTE: libusb-win32 is needed because many WICED users have the libusb-win32 windows driver installed
    #       and the libusb-compat layer  does not work with the old libusb-win32 driver.

    echo "Unzipping libUSB 0.x  (libUSB-win32)"

    rm -rf ./libusb-win32-src-$LIBUSB_WIN32_VER/

    # Unzip fails with segmentation violation first time when used in long directory path for some reason
    unzip -o ./$DOWNLOAD_DIR/$LIBUSB_WIN32_FILENAME || unzip -o ./$DOWNLOAD_DIR/$LIBUSB_WIN32_FILENAME

    echo "Patching libUSB 0.x  (libUSB-win32)"
    patch --ignore-whitespace -p1 -N -d libusb-win32-src-$LIBUSB_WIN32_VER < patches/libusb-win32-src-$LIBUSB_WIN32_VER.patch

    echo "Building libUSB 0.x  (libUSB-win32)"
   #CFLAGS="-g $EXTRA_CFLAGS" make -C ./libusb-win32-src-$LIBUSB_WIN32_VER/ static_lib
    CFLAGS="-g $EXTRA_CFLAGS" make -C ./libusb-win32-src-$LIBUSB_WIN32_VER

    # Create a package config file for LibUSB-Win32
    mkdir -p ./libusb-win32-src-$LIBUSB_WIN32_VER/pkgconfig/
    echo "Name: libusb" > ./libusb-win32-src-$LIBUSB_WIN32_VER/pkgconfig/libusb.pc
    echo "Description: Legacy C API for USB device access from Windows userspace" >> ./libusb-win32-src-$LIBUSB_WIN32_VER/pkgconfig/libusb.pc
    echo "Version: win32-1.0.18" >> ./libusb-win32-src-$LIBUSB_WIN32_VER/pkgconfig/libusb.pc
    echo "Libs: -lusb" >> ./libusb-win32-src-$LIBUSB_WIN32_VER/pkgconfig/libusb.pc
    echo "Libs.private: " >> ./libusb-win32-src-$LIBUSB_WIN32_VER/pkgconfig/libusb.pc
    echo "Cflags:" >> ./libusb-win32-src-$LIBUSB_WIN32_VER/pkgconfig/libusb.pc

    cp ./libusb-win32-src-$LIBUSB_WIN32_VER/src/lusb0_usb.h ./libusb-win32-src-$LIBUSB_WIN32_VER/src/usb.h


   #CONFIG_PARAMS="--enable-static=yes --enable-shared=no"
    CONFIG_PARAMS="--enable-static=no --enable-shared=yes"
else
    CONFIG_PARAMS="--enable-static=no"
fi

# Build libusb 1.x

echo "Unzipping libUSB 1.x"
rm -rf ./libusb-$LIBUSB_VER/
tar -jxvf ./$DOWNLOAD_DIR/$LIBUSB_FILENAME

echo "Patching libUSB 1.x"
# Note - this patch should not be needed when https://github.com/libusb/libusb/pull/140 is merged into a libusb release
# MYKT: commented as LibUSB is updated to 1.0.22, and the path is related to 1.0.20
#patch --ignore-whitespace -p1 -N -d libusb-$LIBUSB_VER < patches/libusb-${LIBUSB_VER}.patch


echo "Building libUSB 1.x"
mkdir -p libusb-build
mkdir -p libusb-install
cd libusb-build
../libusb-$LIBUSB_VER/configure --prefix=`pwd`/../libusb-install/ $CONFIG_PARAMS
make install
cd ..

# Build libusb & libusb-compat on OS-X
if [[ "$OSTYPE" == *darwin* ]]; then


    export PKG_CONFIG_PATH=$PKG_CONFIG_PATH:`pwd`/libusb-install/lib/pkgconfig/

    echo "Unzipping libUSB-compat"
    rm -rf ./libusb-$LIBUSB_COMPAT_VER/
    tar -jxvf ./$DOWNLOAD_DIR/$LIBUSB_COMPAT_FILENAME

    echo "Building libUSB-compat"
    mkdir -p libusb-compat-build
    mkdir -p libusb-compat-install
    cd libusb-compat-$LIBUSB_COMPAT_VER
    # Copy config.guess/config.sub files that allow compilation on MSYS2
    cp ../patches/msys2-config.guess ./config.guess
    cp ../patches/msys2-config.sub   ./config.sub

    touch configure.ac aclocal.m4 configure Makefile.am Makefile.in
    cd ../libusb-compat-build
    ../libusb-compat-$LIBUSB_COMPAT_VER/configure --prefix=`pwd`/../libusb-compat-install/ --enable-static=no CFLAGS="-g -I`pwd`/libusb-install/include/libusb-1.0 -L`pwd`/libusb-install/lib/"
    make install
    cd ..
    if [ "$OSTYPE" == *darwin* ]; then
        cp libusb-install/lib/libusb-1.0.0.dylib $INSTALL_DIR/$HOST_TYPE/
        cp libusb-compat-install/lib/libusb-0.1.4.dylib $INSTALL_DIR/$HOST_TYPE/
    fi
fi
