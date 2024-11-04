echo REM When mining to a local node, you can drop the -s option. > ${1}/mine.bat
echo echo ============================================================ >> ${1}/mine.bat
echo echo = Running Anuma Miner with default .bat. Edit to configure = >> ${1}/mine.bat
echo echo ============================================================ >> ${1}/mine.bat
echo :start >> ${1}/mine.bat
echo ${1}.exe -a anuma:qrg03ql6rnfvwd8a3z04j5a33l6f5d0ua8fkysftx8w7kacf0lsrqzxq2k2ud -s mainnet-dns.anuma.network >> ${1}/mine.bat
echo goto start >> ${1}/mine.bat