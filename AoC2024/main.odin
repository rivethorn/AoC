package main

import "core:fmt"
import "core:os"
import "days"

main :: proc() {
	day, part := handle_args()

	switch day {
	case 1:
		days.run_day1(part)
	case:
		fmt.eprintln("not implemented")
		os.exit(1)
	}
}

