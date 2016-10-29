revision=11243baad55c5116384414717736672a295f11dd

pushd $(dirname $0)
files_to_copy="teensy3/usb_dev.c teensy3/usb_mem.h teensy3/usb_mem.c
               teensy3/usb_dev.h teensy3/usb_desc.h teensy3/usb_desc.c
               teensy3/usb_names.h teensy3/usb_keyboard.h teensy3/usb_keyboard.c
               teensy3/kinetis.h"

echo "Cloning repo..."
git clone https://github.com/jgouly/cores.git
cd cores
git checkout ${revision}
echo "Copying files..."
for f in ${files_to_copy}; do
  cp ${files_to_copy} ../
done
cd ..
rm -rf cores
popd
