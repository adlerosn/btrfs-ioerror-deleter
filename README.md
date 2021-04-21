BTRFS I/O Error Deleter
=======================

## The problem
I've got a problem: my computer froze while moving a BTRFS partition.

Scrubbing did not fix my problem.

My scenario:
 - Metadata: RAID1C4
 - System: RAID1C4
 - Data: **Single**
 - Disks: 4

Scrubbing would not be able to fix my problem.

I knew files were already lost, despite them showing up in file manager with cached thumbnails.

## Observed behavior
Opening files for reading was not a problem, but reading a block with mismatching CSUM resulted in an I/O Error (OS Error code 5). All the times.

## Existing solutions
To remove those references to a content that doesn't exist anymore, [ArchWiki](https://wiki.archlinux.org/index.php/Identify_damaged_files#btrfs), [SuperUser](https://superuser.com/questions/1358686/after-btrfs-scrub-how-do-you-get-the-list-of-affected-files), and [SuperUser](https://superuser.com/questions/858237/finding-files-with-btrfs-uncorrectable-errors) suggested scrubbing, then checking kernel logs (that doesn't include all affected files), and, finally, removing each file.

Running as many scrubs as needed over 2TB data on 80MB/s (at best conditions) hard drives (not SSDs) would take... like... "forever". I don't have such a humongous amount of time.

## My solution
After scrubbing once after the disaster and realizing you have unrepairable data on disk, you run this program.

It recursively lists files in a directory, opens and reads each one of them to the end. All files that produce an OS Error with code 5 will be deleted (as its data isn't there anymore).

You need to run this only once.

## Shortcomings
This code doesn't handle corrupted metadata. (yet)

## License
MIT
