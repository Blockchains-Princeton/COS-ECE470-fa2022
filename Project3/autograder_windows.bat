@echo off
set /p netid="Please unzip your code manually in this directory, and put it in a directory named after your netid, (make sure that Cargo.toml and src/ is in your-netid/ece598pv-sp2022-main), and enter your netid:"
python3 add_test.py %netid%\ece598pv-sp2022-main
cd %netid%\ece598pv-sp2022-main
cargo test sp2022autograder02
pause