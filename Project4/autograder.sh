# put this file, add_test.py, and your netid.zip file in a new directory
for zipfile in *.zip; do
    netid=${zipfile%%.*}
    unzip -qq $zipfile -d $netid
	if [ -d $netid ]; then
		echo "student netid: $netid" >> log.txt
		python3 add_test.py $netid/ece598pv-sp2022-main
		cd $netid/ece598pv-sp2022-main
		cargo test sp2022autograder03 >> ../../log.txt 2>> build_log.txt
		cd ../..
	fi
done
#grep 'student netid\|test result' log.txt > result.txt