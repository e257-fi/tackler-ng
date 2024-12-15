#!/bin/bash
# vim: tabstop=4 shiftwidth=4 smarttab expandtab softtabstop=4 autoindent
#
# Copyright 2018-2024 E257.FI
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
#############################################################################


usage () {
    echo "Initialize and populate git repository with test data"
    echo
	echo "Usage: $0 <repo name> <1E1 | 1E2 | 1E3 | 1E4 | 1E5 | 1E6>"
}

if [ $# != 2 ]; then
    usage
    exit 1
fi

repo_name=$1

name=txns-$2
store="../$name"

if [ ! -d $name ]; then
	echo "Error: $name not found"
	exit 1
fi

if [ ! -d $repo_name ]; then
    git init --bare "$repo_name.git"
    git clone "$repo_name.git"
    
    cd "$repo_name"
    
    git config user.name tackler
    git config user.email "accounting@example.com"
    git config commit.gpgSign no
    git config gc.autoDetach no
    
    git commit --allow-empty -m "init"
    cd ..
fi

cd $repo_name

if [ ! -e readme.txt ]; then
    echo "Tackler test repository for git storage backend" > readme.txt
    echo >> readme.txt
    echo "See different branches for available sets" >> readme.txt
    echo >> readme.txt

    git add readme.txt
    git commit -m 'Initial readme for main'
    git push --set-upstream origin main
fi

git checkout main


echo " * $name" >> readme.txt
git add readme.txt
git commit -m "$name" readme.txt
git push

git checkout -b $name

echo "set: $name" > "info.txt"
git add "info.txt"
git commit -m "$name: initial"
 
mkdir -p txns
mkdir -p txns/2016

for m in 01 02 03 04 05 06 07 08 09 10 11 12; do

    src="$store/2016/$m"

    echo "Perf: start $name, round: $m"

    # All sets does't have all months
    if [ ! -d "$src" ]; then
        echo "Perf: skip  $name, round: $m"
        echo
        continue
    fi
    cp -a "$src" txns/2016/
    git add txns
    git commit -m "$name: 2016/$m"
    echo
    echo "Do git gc next ..."
    git gc
    
    echo 
    echo "Perf: done $name, round: $m"
    # make sure that git timestamps are distinct
    sleep 3
done

git push --set-upstream origin $name

