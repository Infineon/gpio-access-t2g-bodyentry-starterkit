#!/bin/bash

#
# $ Copyright Cypress Semiconductor $
#

if [ "$PREREQUISITES_DONE" != "yes" ]; then
source ./prerequisites.sh
fi


if [ "$DEBUG_OPENOCD" == "yes" ]; then
    export EXTRA_CFLAGS="-g -O0"
else
    export EXTRA_CFLAGS="-O2"
fi

if [ "$OSTYPE" == "msys" ]; then
    export EXTRA_OPENOCD_CFGOPTS="--enable-parport-giveio"
else
    export EXTRA_OPENOCD_CFGOPTS=""
fi

if [[ "$OSTYPE" == *darwin* ]]; then
    #export EXTRA_CFLAGS="$EXTRA_CFLAGS -framework IOKit -framework CoreFoundation"
    #export EXTRA_CFLAGS="$EXTRA_CFLAGS -L`pwd`/libusb-install/lib/" # -L/opt/local/lib/" # -lusb-1.0"
    export EXTRA_CFLAGS="$EXTRA_CFLAGS -L`pwd`/libusb-compat-install/lib/ -Qunused-arguments"
else
    if [[ ! "$OSTYPE" == "msys2" ]]; then
        # These require linux/parport.h - hence do not work on OS-X
        export EXTRA_OPENOCD_CFGOPTS="$EXTRA_OPENOCD_CFGOPTS --enable-amtjtagaccel --enable-gw16012 --enable-parport "
    fi
    export EXTRA_CFLAGS="$EXTRA_CFLAGS -L`pwd`/libusb-win32-src-$LIBUSB_WIN32_VER/"
    export EXTRA_CFLAGS="$EXTRA_CFLAGS -Wl,-rpath,XORIGIN,--start-group "
fi

export CC_VAL="gcc"

if [ "$CONFIG" == "Coverage" ]; then
    export EXTRA_CFLAGS="$EXTRA_CFLAGS --coverage -finstrument-functions-exclude-file-list=msys64,jimtcl,src/jtag,src/helper,src/pld,src/rtos,src/server,src/svf,src/target,src/transport,src/xsvf,src/flash/hand"
    export EXTRA_OPENOCD_CFGOPTS="$EXTRA_OPENOCD_CFGOPTS --disable-werror"
fi

# Current branch
CURRENT_BRANCH=auto/2.0

# Get current branch from GitLab CI variable
if [ ! -z ${CI_BUILD_REF_NAME+x} ];  then
    CURRENT_BRANCH=${CI_BUILD_REF_NAME}
fi
echo "Current branch: $CURRENT_BRANCH"

# Build OpenOCD
echo "Building OpenOCD"
rm -rf openocd-build openocd-install
mkdir -p openocd-build
mkdir -p openocd-install

cd ..
git config core.autocrlf input
git checkout wiced-patches
git fetch
git reset --hard origin/wiced-patches
git checkout $CURRENT_BRANCH
git fetch
git reset --hard origin/$CURRENT_BRANCH
git branch -D build || true
git checkout -b build
#git merge -m "merge" origin/wiced-patches

./bootstrap

cd build/openocd-build

export PKG_CONFIG_PATH="`pwd`/../hidapi-install/lib/pkgconfig:`pwd`/../libusb-install/lib/pkgconfig:`pwd`/../libusb-compat-install/lib/pkgconfig:`pwd`/../libftdi-install/lib/pkgconfig:`pwd`/../libusb-win32-src-$LIBUSB_WIN32_VER/pkgconfig:$PKG_CONFIG_PATH"
export LD_LIBRARY_PATH=`pwd`/../libftdi-install/lib/:$LD_LIBRARY_PATH

if [ "x$PKG_LVERSION" != "x" ];
then
  EXTRA_CFLAGS="-URELSTR -DRELSTR=\\\"-${PKG_LVERSION}\\\" "${EXTRA_CFLAGS}
fi

if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "msys2" ]]; then
    echo "Patch jimtcl/auto.def ..."
    patch -p1 ../../jimtcl/auto.def < ../patches/jimtcl_msys2.patch
fi


../../configure $EXTRA_OPENOCD_CFGOPTS \
 --enable-dummy \
 --enable-ftdi \
 --enable-stlink \
 --enable-ti-icdi \
 --enable-ulink \
 --enable-usb-blaster-2 \
 --enable-vsllink \
 --enable-jlink \
 --enable-osbdm \
 --enable-opendous \
 --enable-aice \
 --disable-usbprog \
 --disable-rlink \
 --disable-armjtagew \
 --enable-cmsis-dap \
 --enable-legacy-ft2232_libftdi \
 --enable-jtag_vpi \
 --enable-usb_blaster_libftdi \
 --enable-ep93xx \
 --enable-at91rm9200 \
 --enable-bcm2835gpio \
 --enable-presto_libftdi \
 --enable-openjtag_ftdi \
 --disable-option-checking  \
 --prefix=`pwd`/../openocd-install/ \
 --enable-maintainer-mode  \
 PKG_CONFIG="pkg-config" \
 CC="$CC_VAL" \
 CFLAGS="$EXTRA_CFLAGS \
 -I`pwd`/../libusb-compat-install/include/ \
 -I`pwd`/../libusb-install/include/libusb-1.0 \
 -L`pwd`/../libusb-compat-install/lib/ \
 -L`pwd`/../libusb-install/lib/ \
 -lusb-1.0 \
 -L`pwd`/../libftdi-install/lib \
 -I`pwd`/../libusb-win32-src-$LIBUSB_WIN32_VER/src/ \
 -I`pwd`/../src/jtag/drivers/hndjtag/include/"


make -j6
make install

cd ..

# Copying OpenOCD into install directory
echo "Copying OpenOCD into install directory"
cp openocd-install/bin/openocd* $INSTALL_DIR/$HOST_TYPE/

# Strip OpenOCD
if [ ! "$DEBUG_OPENOCD" == "yes" ]; then
    echo "Stripping executable"
    for f in \
`find $INSTALL_DIR/$HOST_TYPE/ -name openocd`
    do
        strip $f
    done
fi

# OSX cannot be built static, so make a script to force it to find the dynamic libs
if [[ "$OSTYPE" == *darwin* ]]; then
    openocd_filename="$(find $INSTALL_DIR/$HOST_TYPE -iname 'openocd')"

    mv $openocd_filename ${openocd_filename}_run
    cp `which dirname` ${openocd_filename}_dirname

    echo "#!/bin/bash" > ${openocd_filename}
    echo "export DYLD_LIBRARY_PATH=\`\$0_dirname \$0\`:$DYLD_LIBRARY_PATH" >> ${openocd_filename}
    echo "\${0}_run \"\$@\"" >> ${openocd_filename}
    chmod a+x ${openocd_filename}
fi
# make proper rpath for linux
if [ "$OSTYPE" == "linux-gnu" -o "$OSTYPE" == "linux" ]; then
  openocd_filename="$(find $INSTALL_DIR/$HOST_TYPE/ -name openocd)"
  echo "Fix rpath in $openocd_filename"
  chrpath -r '$ORIGIN' $openocd_filename
fi

cp -r openocd-install/share/openocd/scripts $INSTALL_DIR/$HOST_TYPE
cp openocd-install/share/openocd/scripts/wiced/* $INSTALL_DIR/$HOST_TYPE
rm -rf $INSTALL_DIR/$HOST_TYPE/scripts/wiced

git checkout $CURRENT_BRANCH
git branch -D build


echo
echo "Done! - Output is in $INSTALL_DIR"
echo
