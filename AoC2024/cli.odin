package main

import "core:flags"
import "core:fmt"
import "core:os"

Options :: struct {
	day:  u8 `args:"required" usage:"Choose the day"`,
	part: u8 `args:"required" usage:"Choose the part"`,
}

handle_args :: proc() -> (u8, u8) {
	if len(os.args) == 1 {
		fmt.eprintln("usage: -day [DAY] -part [PART]")
		os.exit(1)
	}

	opt: Options

	flags.parse_or_exit(&opt, os.args, .Unix)

	switch p := opt.part; {
	case p == 1, p == 2:
		break
	case:
		fmt.eprintln("There are only two parts")
		os.exit(1)
	}

	switch opt.day {
	case 1 ..= 25:
		return opt.day, opt.part
	case:
		fmt.eprintln("The day argument should be between 1 and 25")
		os.exit(1)
	}

	return 0, 0
}

