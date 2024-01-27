//usr/bin/go run $0 $@ ; exit

package main
import (
		"fmt"
		"bufio"
		"os"
		"strings"
		"unicode/utf8"
	)

func check(arg string) bool {
	array := [4]string{"-l", "-c", "-w", "-m"}
	for _, b := range array {
		if ( arg == b ){
			return true
		}
	}
	return false
}


func main(){
	var arg string
	var argType string
	open := true
	
	if ( len(os.Args) == 2 && !check(os.Args[1])  ){
		arg = os.Args[1]
		argType = ""
	} else if ( len(os.Args) == 3){
		arg = os.Args[1]
		argType = os.Args[1]
	} else if ( len(os.Args) == 2 && check(os.Args[1]) ) {
		argType = os.Args[1]
		open = false;
	} else {
		fmt.Println("Invalid input.");
		return
	}

	var file *os.File
	if ( open ){
		file, _ = os.Open(arg)
	} else {
		file = os.Stdin
	}

	scanner := bufio.NewScanner(file)

	var totalword, totalbyte, totalchar, totalline int = 0, 0, 0, 0

	for scanner.Scan() {
		line := scanner.Text()
		fmt.Println("Line: " + line);
		
		var wordlen int = len(strings.Split(line, " "))
		var bytelen int = len(line) * 8
		totalline += 1

		charlen := utf8.RuneCountInString(line);
		totalword += wordlen;
		totalbyte += bytelen
		totalchar += charlen
	}
	
	if argType == "-c" {
		fmt.Printf("The byte count is %d\n", totalbyte)
	} else if argType == "-l" {
		fmt.Printf("The number of lines is %d\n", totalline)
	} else if argType == "-w" {
		fmt.Printf("The word count is %d\n", totalword)
	} else if argType == "-m" {
		fmt.Printf("The number of characters is %d\n", totalchar)
	} else {
		fmt.Printf("The byte, line, and word counts are %d, %d, %d\n", totalbyte, totalline, totalword);
	}

}
