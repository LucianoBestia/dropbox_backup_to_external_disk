initSidebarItems({"fn":[["at_line","move cursor to line"],["byte_pos_from_chars","it is used for substring, because string slice are counted in bytes and not chars. if we have multi-byte unicode characters we can get an error if the boundary is not on char boundary."],["get_pos","get cursor position from raw_mode, but return immediately to normal_mode"],["ns_elapsed","returns the elapsed nanoseconds"],["ns_print_ms","print elapsed time in milliseconds and returns the new now in nanoseconds"],["ns_print_ns","print elapsed time in nanoseconds and returns the new now in nanoseconds"],["ns_start","returns the now in nanoseconds"],["press_enter_to_continue_timeout_5_sec","waits 5 seconds for the user to press any key then continues It is usable to make visible some data before going to the next step where the screen is cleaned.  "],["shorten_string","shorten path for screen to avoid word-wrap"],["sort_string_lines","sort string lines case insensitive"],["start_hide_cursor_terminal","when changing cursor position it is good to hide the cursor"]],"struct":[["CLEAR_ALL","ansi clear all"],["CLEAR_LINE","ansi clear line"],["GREEN","ansi color"],["RED","ansi color"],["RESET","ansi reset color"],["UNHIDE_CURSOR","ansi unhide cursor"],["YELLOW","ansi color"]]});