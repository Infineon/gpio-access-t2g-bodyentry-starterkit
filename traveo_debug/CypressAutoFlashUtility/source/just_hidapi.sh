#!/bin/bash

#
# $ Copyright Cypress Semiconductor $
#

if [ "$PREREQUISITES_DONE" != "yes" ]; then
source ./prerequisites.sh
fi

###################################################################################################
#
# Download HIDAPI Tar ball
#
###################################################################################################


# Fetch hidapi - clone it then zip the cloned directory for later use
if [ ! -e ./$DOWNLOAD_DIR/hidapi-$HIDAPI_REPO_HASH.tar.gz ]; then
    echo "Downloading hidapi"
    cd ./$DOWNLOAD_DIR
    rm -rf ./hidapi/
    git clone $HIDAPI_REPO_URL
    cd hidapi
    git checkout  $HIDAPI_REPO_HASH
    cd ..
    tar -zcvf hidapi-$HIDAPI_REPO_HASH.tar.gz hidapi/
    rm -rf hidapi/
    cd ..
fi


###################################################################################################
#
# Build HIDAPI
#
###################################################################################################

# Extract hidapi
echo "Extracting HIDAPI"
rm -rf ./hidapi/
tar -zxvf ./$DOWNLOAD_DIR/$HIDAPI_FILENAME

# Build HIDAPI
echo "Building HIDAPI"
rm -rf hidapi-build hidapi-install
mkdir -p hidapi-build
mkdir -p hidapi-install

cd hidapi
git am ../patches/hidapi.patch
patch -p1 < ../patches/hidapi_0001.patch
patch -p1 < ../patches/hidapi_0002.patch
./bootstrap
cd ../hidapi-build
../hidapi/configure --prefix=`pwd`/../hidapi-install/ --enable-shared=no CFLAGS="-g"
make install
cd ..
