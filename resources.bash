#!/bin/bash

unset error

for file in `find resources -mindepth 1 ! -name "*.log"`
do
  rm -f "$file".log "$file"_verif.log
  ./target/debug/krpsim --delay 1000 --file $file >> $file.log 2>&1 &&
    ./target/debug/krpsim_verif --delay 1000 --file $file --result_to_test "$file".log >> "$file"_verif.log 2>&1
  if [[ $? -ne 0 ]]; then
    echo "ERROR $file"
    echo "> cat $file.log"
    cat $file.log
    if [[ -e "$file"_verif.log ]]
    then
      echo "> cat $file""_verif.log"
      cat "$file"_verif.log
    fi
    error=1
  fi
done
if [[ $error -ne 0 ]]; then
  exit 1

fi
