#!/bin/sh

input=$HOME/Downloads/enwiki-20250801-pages-articles-multistream-index.txt.gz

inputtxt(){
	cat "${input}" | zcat
}

tail_cmd(){
	time inputtxt | tail -1
}

tail1(){
	time inputtxt | ./rs-tail1
}

echo tail1
tail1

echo
echo tail -1
tail_cmd

