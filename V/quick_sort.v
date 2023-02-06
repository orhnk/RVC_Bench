[translated]
module main

fn quicksort(number &int, first int, last int)  {
	i := 0
	j := 0
	pivot := 0
	temp := 0
	
	if first < last {
		pivot = first
		i = first
		j = last
		for i < j {
			for number [i]  <= number [pivot]  && i < last {
			i ++
			}
			for number [j]  > number [pivot]  {
			j --
			}
			if i < j {
				temp = number [i] 
				number [i]  = number [j] 
				number [j]  = temp
			}
		}
		temp = number [pivot] 
		number [pivot]  = number [j] 
		number [j]  = temp
		quicksort(number, first, j - 1)
		quicksort(number, j + 1, last)
	}
}

fn print_array(arr &int, size int)  {
	i := 0
	for i = 0 ; i < size ; i ++ {
	C.printf(c'%d ', arr [i] )
	}
	C.printf(c'\n')
}

fn initialize_array(arr &int, size int)  {
	i := 0
	for i = 0 ; i < size ; i ++ {
	arr [i]  = size - i
	}
}

fn main()  {
	count := 100000
	number := [100000]int{}
	
	initialize_array(number, count)
	C.printf(c'Before sorting')
	print_array(number, count)
	C.printf(c'\n')
	quicksort(number, 0, count - 1)
	print_array(number, count)
	return 
}

