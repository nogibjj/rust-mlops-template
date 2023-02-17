#This script create two files with random data of size 1MB each
#then it makes 25 copies of each file in a directory called testfiles
#then it creates a file called checksums.txt with the md5sum of each file

echo "Creating files"
mkdir -p testfiles

#Create the files
dd if=/dev/urandom of=file1 bs=1M count=1
dd if=/dev/urandom of=file2 bs=1M count=1

#Create 25 copies of each file
for i in {1..25}
do
    cp file1 testfiles/file1_$i
    cp file2 testfiles/file2_$i
done

#Create the checksums.txt file
md5sum testfiles/* > checksums.txt

