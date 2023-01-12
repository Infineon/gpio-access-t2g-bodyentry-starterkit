#!/bin/bash

#
# $ Copyright Cypress Semiconductor $
#

if [ "$PREREQUISITES_DONE" != "yes" ]; then
source ./prerequisites.sh
fi

###################################################################################################
#
# Download LibFTDI Tar ball
#
###################################################################################################


# Download libFTDI
if [ ! -e ./$DOWNLOAD_DIR/$LIBFTDI_FILENAME ]; then
    echo "Downloading libFTDI"
    cd $DOWNLOAD_DIR
    $WGET $LIBFTDI_URL
    cd ..
fi


###################################################################################################
#
# Build LibFTDI
#
###################################################################################################

# Extract libFTDI
echo "Extracting libFTDI"
rm -rf ./libftdi-$LIBFTDI_VER/
tar -zxvf ./$DOWNLOAD_DIR/$LIBFTDI_FILENAME
echo "Patching libFTDI"
patch --ignore-whitespace -p1 -N -d libftdi-$LIBFTDI_VER < patches/libftdi-${LIBFTDI_VER}.patch

# Build libFTDI
echo "Building libFTDI"
rm -rf libftdi-build libftdi-install
mkdir -p libftdi-build
mkdir -p libftdi-install

if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "msys2" ]]; then
    export PATH=`pwd`/libftdi-build/:$PATH
    cp libusb-config-msys libftdi-build/libusb-config
fi
if [[ "$OSTYPE" == *darwin* ]]; then
    export PATH=../libusb-compat-install/bin/:$PATH
    export EXTRA_CFLAGS="$EXTRA_CFLAGS -I`pwd`/libusb-compat-install/include/ -I`pwd`/libusb-install/include/libusb-1.0" # -L`pwd`/libusb-compat-install/lib/" # -L`pwd`/libusb-install/lib/" # -lusb-1.0"
fi

cd libftdi-build
../libftdi-$LIBFTDI_VER/configure --prefix=`pwd`/../libftdi-install/ --enable-shared=no
make CFLAGS="-g -L`pwd`/../libusb-win32-src-$LIBUSB_WIN32_VER/ -L`pwd`/../libusb-compat-install/lib/ -lusb -I`pwd`/../libusb-win32-src-$LIBUSB_WIN32_VER/src -I`pwd`/../libusb-compat-install/include/ -I/opt/local/include $EXTRA_CFLAGS" install
cd ..
