#!/bin/bash

#
# $ Copyright Cypress Semiconductor $
#

source ./prerequisites.sh

# OpenOCD has some utf-8 filenames and MinGW cannot delete them.
# Use the Windows command console to delete the files (does not delete the empty directories).
if [ "$OSTYPE" == "msys" ]; then
    cmd /c "del /S /F /Q openocd"
fi

rm -rf install \
    hidapi \
    hidapi-build \
    hidapi-install \
    hidapi-externals \
    libftdi-$LIBFTDI_VER \
    libftdi-build \
    libftdi-install \
    openocd \
    openocd-build \
    openocd-install \
    libusb-win32-src-$LIBUSB_WIN32_VER \
    libusb-$LIBUSB_VER \
    libusb-build \
    libusb-install \
    libusb-compat-$LIBUSB_COMPAT_VER \
    libusb-compat-build \
    libusb-compat-install
