find . -name 'geocities*.7z.001' -exec 7z -o/mnt/qnap/extracted x {} \;

find Directories -mindepth 4 -type d -print0 | parallel -0 -j0 ./MyScript -d {2} {1} ::: a b c d :::: -

find . -name 'geocities*.7z.001' | parallel -0 -j0 echo "yo {1}"

find . -name 'geocities*.7z.001' -type d -print | parallel -0 -j0 echo "yo"
find . -name 'geocities*.7z.001' | parallel -j0 7z -aos -o/mnt/qnap/extracted x {}
find . -name 'geocities-*' | parallel -j60 tar -xf {}
