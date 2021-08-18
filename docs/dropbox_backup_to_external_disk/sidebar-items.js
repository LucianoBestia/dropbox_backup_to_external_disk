initSidebarItems({"enum":[["RemoteKind","with this enum I can send a completely different parameter to a function and then call a method on it, that processes differently depending on the kind. Smart."]],"fn":[["add2_just_downloaded_to_list_local",""],["add_just_downloaded_to_list_local",""],["at_line","move cursor to line"],["byte_pos_from_chars","it is used for substring, because string slice are counted in bytes and not chars. if we have multi-byte unicode characters we can get an error if the boundary is not on char boundary."],["compare2_lists",""],["compare_lists",""],["compare_lists_internal","compare list: the lists must be already sorted for this to work correctly"],["copy_from_list2_for_download","download files from list"],["correct_time_from_list",""],["correct_time_from_list_internal","modify the files from list_for_correct_time"],["download_from_list","download files from list"],["download_one_file","download one file"],["get_pos","get cursor position from raw_mode, but return immediately to normal_mode"],["get_short_lived_access_token","get token from env variable"],["list2_local","for second backup: list all local files and folders. It can take some time."],["list_and_sync","list and sync is the complete process for backup in one command"],["list_local","list all local files and folders. It can take some time."],["list_remote","get remote list in parallel first get the first level of folders and then request in parallel sub-folders recursively"],["list_remote_folder","list remote folder"],["move_or_rename_local_files","Files are often moved or renamed After compare, the same file (with different path or name) will be in the list_for_trash and in the list_for_download. First for every trash line, we search list_for_download for same size and modified. If found, get the remote_metadata with content_hash and calculate local_content_hash. If they are equal move or rename, else nothing: it will be trashed and downloaded eventually. Remove also the lines in files list_for_trash and list_for_download."],["ns_elapsed","returns the elapsed nanoseconds"],["ns_print_ms","print elapsed time in milliseconds and returns the new now in nanoseconds"],["ns_print_ns","print elapsed time in nanoseconds and returns the new now in nanoseconds"],["ns_start","returns the now in nanoseconds"],["press_enter_to_continue_timeout_5_sec","waits 5 seconds fr the user to press any key then continues It is usable to make visible some data before going to the next step where the screen is cleaned."],["remote_content_hash","get content_hash from remote"],["save2_base_path","saves the base local path for later commands like “/mnt/d/DropBoxBackup2”"],["save_base_path","saves the base local path for later commands like “/mnt/f/DropBoxBackup1”"],["second_backup","after the first backup from dropbox, we want to make a second backup from the first backup ideally, we put it somewhere safe in a distant location having 2 external disks on the same computer, is faster to just copy files then to question for calculating hash no need to move files or correct time. Just copy it. It is faster."],["shorten_string","shorten path for screen to avoid word-wrap"],["sort_remote_list_and_write_to_file","sort and write to file"],["sort_string_lines","sort string lines case insensitive"],["start_hide_cursor_terminal","when changing cursor position it is good to hide the cursor"],["sync_only","sync_only can be stopped and then restarted if downloading takes a lot of time. no need to repeat the “list” that takes a lot of timeS"],["test_connection","test authentication with dropbox.com"],["trash2_from_list",""],["trash_from_list",""],["trash_from_list_internal","move to trash folder the files from list_for_trash ignore if the file does not exist anymore"]],"mod":[["local_mod","local_mod.rs Module contains all functions for local external disk."],["remote_mod","remote_mod.rs Module contains all the communication with the remote dropbox storage."],["utils_mod","utils_mod.rs A module with often used functions. For every project I select only the functions I need for the project."]],"struct":[["CLEAR_ALL",""],["CLEAR_LINE","ansi terminal - clears the line on the terminal from cursor to end of line"],["GREEN",""],["RED",""],["RESET",""],["UNHIDE_CURSOR",""],["YELLOW",""]]});