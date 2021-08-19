initSidebarItems({"enum":[["RemoteKind","The source file can be on dropbox or on external disk Backup_1"]],"fn":[["add2_just_downloaded_to_list_local","add just downloaded files to list_local (from external disk)"],["add_just_downloaded_to_list_local","add just downloaded files to list_local (from dropbox remote)"],["all_list_remote_and_local","all list remote and local"],["at_line","move cursor to line"],["byte_pos_from_chars","it is used for substring, because string slice are counted in bytes and not chars. if we have multi-byte unicode characters we can get an error if the boundary is not on char boundary."],["compare2_lists","compare list: the lists and produce list2_for_download, list2_for_trash, list2_for_correct_time"],["compare_lists","compare list: the lists and produce list_for_download, list_for_trash, list_for_correct_time"],["compare_lists_internal","compare list: the lists must be already sorted for this to work correctly"],["copy_from_list2_for_download","copies files from external disk backup_1 to backup_2"],["correct_time_from_list","modify the date od files from list_for_correct_time"],["download_from_list","download files from list"],["download_one_file","download one file"],["get_pos","get cursor position from raw_mode, but return immediately to normal_mode"],["get_short_lived_access_token","get token from env variable"],["list2_local","for second backup: list all local files and folders. It can take some time."],["list_and_sync","list and sync is the complete process for backup in one command"],["list_local","list all local files and folders. It can take some time."],["list_remote","get remote list in parallel first get the first level of folders and then request in parallel sub-folders recursively"],["list_remote_folder","list remote folder"],["move_or_rename_local_files","Files are often moved or renamed After compare, the same file (with different path or name) will be in the list_for_trash and in the list_for_download. First for every trash line, we search list_for_download for same size and modified. If found, get the remote_metadata with content_hash and calculate local_content_hash. If they are equal move or rename, else nothing: it will be trashed and downloaded eventually. Remove also the lines in files list_for_trash and list_for_download.  "],["ns_elapsed","returns the elapsed nanoseconds"],["ns_print_ms","print elapsed time in milliseconds and returns the new now in nanoseconds"],["ns_print_ns","print elapsed time in nanoseconds and returns the new now in nanoseconds"],["ns_start","returns the now in nanoseconds"],["press_enter_to_continue_timeout_5_sec","waits 5 seconds for the user to press any key then continues It is usable to make visible some data before going to the next step where the screen is cleaned.  "],["remote_content_hash","get content_hash from remote"],["save2_base_path","saves the base local path for later use like “/mnt/f/DropBoxBackup2”"],["save_base_path","saves the base local path for later use like “/mnt/d/DropBoxBackup1”"],["second_backup","After the first backup from dropbox, we want to make a second backup from the first backup. Ideally, we put it somewhere safe in a distant location. Having 2 external disks on the same computer, it is faster to just copy files then to question for calculating hash. No need to move files or correct time. Just copy it. It is faster.  "],["shorten_string","shorten path for screen to avoid word-wrap"],["sort_remote_list_and_write_to_file","sort and write to file"],["sort_string_lines","sort string lines case insensitive"],["start_hide_cursor_terminal","when changing cursor position it is good to hide the cursor"],["sync_only","sync_only can be stopped with ctrl+c and then restarted if downloading takes lots of time. No need to repeat the “list” that takes lots of times.  "],["test_connection","test authentication with dropbox.com"],["trash2_from_list","Move to trash folder the files from list_for_trash. Ignore if the file does not exist anymore.  "],["trash_from_list","Move to trash folder the files from list_for_trash. Ignore if the file does not exist anymore.  "],["trash_from_list_internal","internal"]],"mod":[["local_mod","Module contains all functions for local external disk."],["remote_mod","Module contains all the communication with the remote dropbox storage."],["utils_mod","A module with often used functions."]],"struct":[["CLEAR_ALL","ansi clear all"],["CLEAR_LINE","ansi clear line"],["GREEN","ansi color"],["RED","ansi color"],["RESET","ansi reset color"],["UNHIDE_CURSOR","ansi unhide cursor"],["YELLOW","ansi color"]]});