package main

import (
	"fmt"
	"os"
	"bufio"
	"sort"
)


type Node struct{
	  cnt int
		chr rune
		left *Node 
		right *Node 
}

type Pair struct {
	chr rune
	freq int
}

func balanceTree ( tree *Node ) *Node {
	return &Node{0, '@', nil, nil}
}

func buildHuffman( kvp []Pair ) *Node {
	var ret *Node = &Node {0, '@', nil, nil}
	i := 0
	for i < len(kvp) {
		if ( ret.left == nil && ret.right == nil){
			var one Node = Node { kvp[i].freq, kvp[i].chr, nil, nil }
			var two Node = Node { kvp[i+1].freq, kvp[i+1].chr, nil, nil }
			(*ret).left = &one
			(*ret).right = &two
			(*ret).cnt = kvp[i].freq + kvp[i+1].freq
			i += 1
		} else {
			var oldRet *Node = ret
			var newRet Node = Node{0, '@', nil, nil}
			(newRet).left = oldRet
			(newRet).right = &Node{kvp[i].freq, kvp[i].chr, nil, nil}
			newRet.cnt = (*oldRet).cnt + kvp[i].freq
			ret = &newRet
		}
		i += 1
	}
	return ret
}

func genEncode( huffman *Node, path string, encoding *map[string]rune )  {
	if ( (*huffman).left == nil && (*huffman).right == nil){
		(*encoding)[path] = (*huffman).chr
		fmt.Println( (*huffman).chr, path)
	}
	if ( (*huffman).left != nil ){ 
		path += "0"
		genEncode( (*huffman).left, path, encoding )
		path = path[0:len(path)-1]	
	}
	if ( (*huffman).right != nil){
		path += "1"
		genEncode( (*huffman).right, path, encoding )
		path = path[0:len(path)-1]
	}
}

func genDecode( str string, mp map[string]rune ) string {
	var pref string = ""
	var decode string = "" 
	//fmt.Printf("str: %s\n", str)
	i := 0
	for i < len(str) {
		val, ok := mp[pref]
		//fmt.Printf("pref: %s\n", pref)
		if ( ok ){
			//fmt.Printf("Decoded: %s\n", string(val))
			decode += string(val)
			pref = ""
			i -= 1
		}else {
			pref += string(str[i])
		}
		i += 1
	}
	
	val, ok := mp[pref]
	if ( ok ){
		decode += string(val)
	}
	return decode
}

func trav(tree *Node) {
	if ( tree.left != nil ){
		trav( (*tree).left )
	}
	fmt.Println( (*tree).cnt)
	if ( tree.right != nil ){
		trav( (*tree).right )
	}
}

func main(){
	var freq map[rune]int = make(map[rune]int)
	var kvp []Pair 
	file, _ := os.Open("../135-0.txt")
	scanner := bufio.NewScanner(file)
	text := ""
	for scanner.Scan() {
		line := scanner.Text()
		text += line
		for _, chr := range line {
			freq[chr] += 1
		}
	}

	fmt.Println("text: ", text[0:1000])
	for k, v := range freq {
		kvp = append(kvp, Pair{ k, v} )
	}
	sort.SliceStable( kvp, func (i, j int) bool {
		return kvp[i].freq < kvp[j].freq
	})
	
	huffman := buildHuffman(kvp)
	var decodeMap map[string]rune = make(map[string]rune)
	genEncode(huffman, "", &decodeMap)
	var encodeMap map[rune]string = make(map[rune]string)
	for k, v := range decodeMap {
		encodeMap[v] = k
	}
	encoding := ""
	for _, chr := range text[0:1000] {
		encoding += encodeMap[chr]
	}
	decode := genDecode(encoding, decodeMap)
	fmt.Println(decode)
	//trav(huffman)
	/*
	fmt.Println( (*huffman).cnt )
	fmt.Println( (*huffman).left )
	fmt.Println( (*huffman).right )
	fmt.Println( (*(*huffman).left).cnt)
	fmt.Println( (*(*huffman).left).left )
	fmt.Println( (*(*huffman).left).right )
	*/
}
