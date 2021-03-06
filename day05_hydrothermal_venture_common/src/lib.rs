#[derive(PartialEq, Eq, Hash)]
pub struct Point {
    pub x: u32,
    pub y: u32
}

// Substitute with
// '<,'>s/\(\d\+\),\(\d\+\) -> \(\d\+\),\(\d\+\)/    (Point { x: \1, y: \2 }, Point { x: \3, y: \4 }),/
pub const SAMPLE_DATA: [(Point, Point); 10] = [
    (Point { x: 0, y: 9 }, Point { x: 5, y: 9 }),
    (Point { x: 8, y: 0 }, Point { x: 0, y: 8 }),
    (Point { x: 9, y: 4 }, Point { x: 3, y: 4 }),
    (Point { x: 2, y: 2 }, Point { x: 2, y: 1 }),
    (Point { x: 7, y: 0 }, Point { x: 7, y: 4 }),
    (Point { x: 6, y: 4 }, Point { x: 2, y: 0 }),
    (Point { x: 0, y: 9 }, Point { x: 2, y: 9 }),
    (Point { x: 3, y: 4 }, Point { x: 1, y: 4 }),
    (Point { x: 0, y: 0 }, Point { x: 8, y: 8 }),
    (Point { x: 5, y: 5 }, Point { x: 8, y: 2 }),
];

pub const REAL_DATA: [(Point, Point); 500] = [
    (Point { x: 60, y: 28 }, Point { x: 893, y: 861 }),
    (Point { x: 934, y: 945 }, Point { x: 222, y: 233 }),
    (Point { x: 125, y: 246 }, Point { x: 125, y: 306 }),
    (Point { x: 490, y: 255 }, Point { x: 490, y: 847 }),
    (Point { x: 457, y: 868 }, Point { x: 364, y: 961 }),
    (Point { x: 610, y: 46 }, Point { x: 610, y: 826 }),
    (Point { x: 338, y: 711 }, Point { x: 982, y: 67 }),
    (Point { x: 199, y: 581 }, Point { x: 295, y: 581 }),
    (Point { x: 578, y: 489 }, Point { x: 522, y: 545 }),
    (Point { x: 180, y: 516 }, Point { x: 180, y: 904 }),
    (Point { x: 354, y: 363 }, Point { x: 145, y: 363 }),
    (Point { x: 757, y: 471 }, Point { x: 692, y: 471 }),
    (Point { x: 896, y: 71 }, Point { x: 896, y: 185 }),
    (Point { x: 967, y: 744 }, Point { x: 967, y: 486 }),
    (Point { x: 166, y: 19 }, Point { x: 483, y: 19 }),
    (Point { x: 469, y: 22 }, Point { x: 529, y: 22 }),
    (Point { x: 774, y: 311 }, Point { x: 807, y: 311 }),
    (Point { x: 617, y: 308 }, Point { x: 203, y: 308 }),
    (Point { x: 694, y: 405 }, Point { x: 694, y: 43 }),
    (Point { x: 378, y: 176 }, Point { x: 378, y: 488 }),
    (Point { x: 989, y: 189 }, Point { x: 215, y: 189 }),
    (Point { x: 375, y: 96 }, Point { x: 612, y: 96 }),
    (Point { x: 505, y: 467 }, Point { x: 505, y: 246 }),
    (Point { x: 77, y: 832 }, Point { x: 77, y: 473 }),
    (Point { x: 686, y: 879 }, Point { x: 684, y: 879 }),
    (Point { x: 360, y: 593 }, Point { x: 151, y: 384 }),
    (Point { x: 387, y: 322 }, Point { x: 626, y: 322 }),
    (Point { x: 66, y: 784 }, Point { x: 66, y: 109 }),
    (Point { x: 100, y: 411 }, Point { x: 635, y: 946 }),
    (Point { x: 722, y: 14 }, Point { x: 722, y: 784 }),
    (Point { x: 724, y: 751 }, Point { x: 764, y: 751 }),
    (Point { x: 788, y: 844 }, Point { x: 32, y: 88 }),
    (Point { x: 905, y: 799 }, Point { x: 905, y: 713 }),
    (Point { x: 282, y: 502 }, Point { x: 238, y: 502 }),
    (Point { x: 685, y: 259 }, Point { x: 685, y: 768 }),
    (Point { x: 116, y: 578 }, Point { x: 477, y: 217 }),
    (Point { x: 115, y: 78 }, Point { x: 115, y: 458 }),
    (Point { x: 329, y: 569 }, Point { x: 66, y: 306 }),
    (Point { x: 817, y: 815 }, Point { x: 817, y: 466 }),
    (Point { x: 781, y: 909 }, Point { x: 872, y: 909 }),
    (Point { x: 62, y: 44 }, Point { x: 964, y: 946 }),
    (Point { x: 755, y: 307 }, Point { x: 593, y: 307 }),
    (Point { x: 436, y: 56 }, Point { x: 436, y: 869 }),
    (Point { x: 596, y: 815 }, Point { x: 49, y: 268 }),
    (Point { x: 465, y: 986 }, Point { x: 926, y: 525 }),
    (Point { x: 885, y: 254 }, Point { x: 39, y: 254 }),
    (Point { x: 947, y: 433 }, Point { x: 764, y: 433 }),
    (Point { x: 719, y: 787 }, Point { x: 200, y: 787 }),
    (Point { x: 380, y: 461 }, Point { x: 243, y: 461 }),
    (Point { x: 675, y: 434 }, Point { x: 675, y: 582 }),
    (Point { x: 106, y: 548 }, Point { x: 272, y: 714 }),
    (Point { x: 703, y: 143 }, Point { x: 703, y: 111 }),
    (Point { x: 238, y: 745 }, Point { x: 60, y: 745 }),
    (Point { x: 646, y: 235 }, Point { x: 646, y: 742 }),
    (Point { x: 243, y: 439 }, Point { x: 243, y: 964 }),
    (Point { x: 347, y: 763 }, Point { x: 321, y: 789 }),
    (Point { x: 322, y: 294 }, Point { x: 738, y: 294 }),
    (Point { x: 134, y: 361 }, Point { x: 946, y: 361 }),
    (Point { x: 223, y: 30 }, Point { x: 498, y: 305 }),
    (Point { x: 78, y: 721 }, Point { x: 288, y: 721 }),
    (Point { x: 792, y: 875 }, Point { x: 572, y: 875 }),
    (Point { x: 548, y: 380 }, Point { x: 637, y: 291 }),
    (Point { x: 85, y: 417 }, Point { x: 85, y: 296 }),
    (Point { x: 766, y: 81 }, Point { x: 766, y: 131 }),
    (Point { x: 340, y: 218 }, Point { x: 340, y: 271 }),
    (Point { x: 56, y: 962 }, Point { x: 974, y: 44 }),
    (Point { x: 415, y: 940 }, Point { x: 386, y: 940 }),
    (Point { x: 960, y: 60 }, Point { x: 46, y: 974 }),
    (Point { x: 719, y: 527 }, Point { x: 743, y: 527 }),
    (Point { x: 971, y: 986 }, Point { x: 313, y: 986 }),
    (Point { x: 415, y: 316 }, Point { x: 415, y: 57 }),
    (Point { x: 612, y: 556 }, Point { x: 612, y: 648 }),
    (Point { x: 421, y: 776 }, Point { x: 570, y: 776 }),
    (Point { x: 478, y: 533 }, Point { x: 478, y: 831 }),
    (Point { x: 758, y: 304 }, Point { x: 112, y: 950 }),
    (Point { x: 258, y: 950 }, Point { x: 79, y: 950 }),
    (Point { x: 329, y: 349 }, Point { x: 216, y: 349 }),
    (Point { x: 527, y: 38 }, Point { x: 755, y: 38 }),
    (Point { x: 517, y: 239 }, Point { x: 207, y: 239 }),
    (Point { x: 497, y: 944 }, Point { x: 898, y: 944 }),
    (Point { x: 418, y: 642 }, Point { x: 418, y: 557 }),
    (Point { x: 828, y: 750 }, Point { x: 456, y: 750 }),
    (Point { x: 632, y: 916 }, Point { x: 15, y: 299 }),
    (Point { x: 955, y: 973 }, Point { x: 970, y: 958 }),
    (Point { x: 474, y: 524 }, Point { x: 949, y: 49 }),
    (Point { x: 842, y: 690 }, Point { x: 842, y: 116 }),
    (Point { x: 203, y: 267 }, Point { x: 916, y: 980 }),
    (Point { x: 128, y: 562 }, Point { x: 936, y: 562 }),
    (Point { x: 682, y: 963 }, Point { x: 193, y: 963 }),
    (Point { x: 704, y: 822 }, Point { x: 44, y: 162 }),
    (Point { x: 371, y: 309 }, Point { x: 421, y: 359 }),
    (Point { x: 379, y: 240 }, Point { x: 510, y: 240 }),
    (Point { x: 547, y: 177 }, Point { x: 547, y: 20 }),
    (Point { x: 784, y: 968 }, Point { x: 784, y: 613 }),
    (Point { x: 75, y: 138 }, Point { x: 75, y: 724 }),
    (Point { x: 725, y: 48 }, Point { x: 32, y: 741 }),
    (Point { x: 141, y: 674 }, Point { x: 545, y: 674 }),
    (Point { x: 193, y: 922 }, Point { x: 193, y: 845 }),
    (Point { x: 916, y: 831 }, Point { x: 916, y: 401 }),
    (Point { x: 912, y: 232 }, Point { x: 923, y: 232 }),
    (Point { x: 127, y: 911 }, Point { x: 911, y: 127 }),
    (Point { x: 140, y: 381 }, Point { x: 140, y: 913 }),
    (Point { x: 472, y: 243 }, Point { x: 472, y: 134 }),
    (Point { x: 311, y: 548 }, Point { x: 311, y: 741 }),
    (Point { x: 415, y: 590 }, Point { x: 415, y: 409 }),
    (Point { x: 20, y: 984 }, Point { x: 983, y: 21 }),
    (Point { x: 572, y: 575 }, Point { x: 572, y: 756 }),
    (Point { x: 804, y: 188 }, Point { x: 20, y: 972 }),
    (Point { x: 485, y: 104 }, Point { x: 962, y: 104 }),
    (Point { x: 31, y: 966 }, Point { x: 986, y: 11 }),
    (Point { x: 584, y: 473 }, Point { x: 338, y: 473 }),
    (Point { x: 974, y: 962 }, Point { x: 34, y: 22 }),
    (Point { x: 176, y: 279 }, Point { x: 921, y: 279 }),
    (Point { x: 305, y: 863 }, Point { x: 305, y: 981 }),
    (Point { x: 137, y: 889 }, Point { x: 137, y: 47 }),
    (Point { x: 888, y: 259 }, Point { x: 888, y: 125 }),
    (Point { x: 794, y: 774 }, Point { x: 794, y: 455 }),
    (Point { x: 193, y: 118 }, Point { x: 966, y: 118 }),
    (Point { x: 429, y: 302 }, Point { x: 547, y: 184 }),
    (Point { x: 59, y: 49 }, Point { x: 984, y: 974 }),
    (Point { x: 978, y: 56 }, Point { x: 232, y: 802 }),
    (Point { x: 23, y: 142 }, Point { x: 456, y: 142 }),
    (Point { x: 532, y: 574 }, Point { x: 532, y: 265 }),
    (Point { x: 936, y: 263 }, Point { x: 933, y: 263 }),
    (Point { x: 773, y: 856 }, Point { x: 230, y: 313 }),
    (Point { x: 182, y: 809 }, Point { x: 913, y: 809 }),
    (Point { x: 364, y: 958 }, Point { x: 901, y: 958 }),
    (Point { x: 724, y: 290 }, Point { x: 64, y: 950 }),
    (Point { x: 312, y: 967 }, Point { x: 312, y: 166 }),
    (Point { x: 208, y: 286 }, Point { x: 566, y: 286 }),
    (Point { x: 828, y: 907 }, Point { x: 828, y: 729 }),
    (Point { x: 79, y: 692 }, Point { x: 753, y: 18 }),
    (Point { x: 235, y: 601 }, Point { x: 811, y: 601 }),
    (Point { x: 735, y: 206 }, Point { x: 301, y: 206 }),
    (Point { x: 246, y: 112 }, Point { x: 246, y: 423 }),
    (Point { x: 712, y: 439 }, Point { x: 712, y: 108 }),
    (Point { x: 570, y: 179 }, Point { x: 751, y: 179 }),
    (Point { x: 766, y: 22 }, Point { x: 816, y: 22 }),
    (Point { x: 51, y: 686 }, Point { x: 27, y: 686 }),
    (Point { x: 50, y: 954 }, Point { x: 50, y: 31 }),
    (Point { x: 762, y: 413 }, Point { x: 762, y: 601 }),
    (Point { x: 223, y: 812 }, Point { x: 670, y: 812 }),
    (Point { x: 391, y: 882 }, Point { x: 712, y: 882 }),
    (Point { x: 842, y: 332 }, Point { x: 922, y: 332 }),
    (Point { x: 540, y: 88 }, Point { x: 540, y: 124 }),
    (Point { x: 75, y: 312 }, Point { x: 161, y: 312 }),
    (Point { x: 979, y: 984 }, Point { x: 10, y: 15 }),
    (Point { x: 479, y: 856 }, Point { x: 823, y: 856 }),
    (Point { x: 690, y: 491 }, Point { x: 298, y: 883 }),
    (Point { x: 481, y: 401 }, Point { x: 481, y: 279 }),
    (Point { x: 870, y: 942 }, Point { x: 276, y: 348 }),
    (Point { x: 39, y: 935 }, Point { x: 937, y: 37 }),
    (Point { x: 706, y: 275 }, Point { x: 706, y: 948 }),
    (Point { x: 530, y: 892 }, Point { x: 535, y: 897 }),
    (Point { x: 743, y: 223 }, Point { x: 929, y: 223 }),
    (Point { x: 682, y: 917 }, Point { x: 490, y: 917 }),
    (Point { x: 616, y: 268 }, Point { x: 456, y: 268 }),
    (Point { x: 484, y: 72 }, Point { x: 429, y: 72 }),
    (Point { x: 61, y: 365 }, Point { x: 430, y: 365 }),
    (Point { x: 382, y: 741 }, Point { x: 910, y: 741 }),
    (Point { x: 710, y: 406 }, Point { x: 330, y: 406 }),
    (Point { x: 795, y: 770 }, Point { x: 55, y: 770 }),
    (Point { x: 117, y: 416 }, Point { x: 352, y: 651 }),
    (Point { x: 593, y: 151 }, Point { x: 20, y: 724 }),
    (Point { x: 238, y: 556 }, Point { x: 584, y: 556 }),
    (Point { x: 680, y: 583 }, Point { x: 680, y: 504 }),
    (Point { x: 678, y: 440 }, Point { x: 212, y: 440 }),
    (Point { x: 508, y: 222 }, Point { x: 508, y: 844 }),
    (Point { x: 435, y: 873 }, Point { x: 93, y: 873 }),
    (Point { x: 129, y: 607 }, Point { x: 468, y: 268 }),
    (Point { x: 280, y: 147 }, Point { x: 94, y: 147 }),
    (Point { x: 238, y: 872 }, Point { x: 971, y: 139 }),
    (Point { x: 881, y: 339 }, Point { x: 664, y: 339 }),
    (Point { x: 289, y: 960 }, Point { x: 289, y: 664 }),
    (Point { x: 70, y: 762 }, Point { x: 973, y: 762 }),
    (Point { x: 429, y: 24 }, Point { x: 429, y: 202 }),
    (Point { x: 907, y: 785 }, Point { x: 907, y: 190 }),
    (Point { x: 598, y: 548 }, Point { x: 598, y: 63 }),
    (Point { x: 324, y: 220 }, Point { x: 281, y: 220 }),
    (Point { x: 754, y: 980 }, Point { x: 79, y: 980 }),
    (Point { x: 568, y: 508 }, Point { x: 583, y: 508 }),
    (Point { x: 364, y: 712 }, Point { x: 503, y: 712 }),
    (Point { x: 655, y: 963 }, Point { x: 898, y: 963 }),
    (Point { x: 253, y: 359 }, Point { x: 46, y: 566 }),
    (Point { x: 989, y: 989 }, Point { x: 14, y: 14 }),
    (Point { x: 329, y: 924 }, Point { x: 380, y: 924 }),
    (Point { x: 248, y: 826 }, Point { x: 675, y: 826 }),
    (Point { x: 417, y: 428 }, Point { x: 417, y: 320 }),
    (Point { x: 13, y: 12 }, Point { x: 984, y: 983 }),
    (Point { x: 916, y: 53 }, Point { x: 916, y: 896 }),
    (Point { x: 247, y: 285 }, Point { x: 377, y: 155 }),
    (Point { x: 937, y: 588 }, Point { x: 710, y: 588 }),
    (Point { x: 473, y: 270 }, Point { x: 466, y: 277 }),
    (Point { x: 567, y: 74 }, Point { x: 567, y: 388 }),
    (Point { x: 371, y: 470 }, Point { x: 228, y: 470 }),
    (Point { x: 640, y: 96 }, Point { x: 766, y: 96 }),
    (Point { x: 725, y: 499 }, Point { x: 372, y: 499 }),
    (Point { x: 184, y: 561 }, Point { x: 184, y: 236 }),
    (Point { x: 654, y: 446 }, Point { x: 933, y: 446 }),
    (Point { x: 156, y: 153 }, Point { x: 978, y: 975 }),
    (Point { x: 811, y: 228 }, Point { x: 922, y: 339 }),
    (Point { x: 84, y: 861 }, Point { x: 878, y: 67 }),
    (Point { x: 622, y: 329 }, Point { x: 622, y: 425 }),
    (Point { x: 415, y: 186 }, Point { x: 450, y: 221 }),
    (Point { x: 109, y: 488 }, Point { x: 653, y: 488 }),
    (Point { x: 982, y: 16 }, Point { x: 33, y: 965 }),
    (Point { x: 100, y: 885 }, Point { x: 829, y: 156 }),
    (Point { x: 342, y: 914 }, Point { x: 342, y: 636 }),
    (Point { x: 177, y: 323 }, Point { x: 728, y: 874 }),
    (Point { x: 81, y: 414 }, Point { x: 406, y: 739 }),
    (Point { x: 889, y: 79 }, Point { x: 889, y: 698 }),
    (Point { x: 504, y: 450 }, Point { x: 148, y: 806 }),
    (Point { x: 961, y: 33 }, Point { x: 51, y: 943 }),
    (Point { x: 656, y: 21 }, Point { x: 100, y: 21 }),
    (Point { x: 32, y: 60 }, Point { x: 32, y: 562 }),
    (Point { x: 499, y: 174 }, Point { x: 499, y: 301 }),
    (Point { x: 162, y: 740 }, Point { x: 162, y: 906 }),
    (Point { x: 190, y: 183 }, Point { x: 811, y: 804 }),
    (Point { x: 93, y: 960 }, Point { x: 13, y: 960 }),
    (Point { x: 787, y: 681 }, Point { x: 866, y: 681 }),
    (Point { x: 254, y: 332 }, Point { x: 254, y: 79 }),
    (Point { x: 595, y: 873 }, Point { x: 595, y: 496 }),
    (Point { x: 151, y: 737 }, Point { x: 151, y: 390 }),
    (Point { x: 974, y: 429 }, Point { x: 990, y: 429 }),
    (Point { x: 295, y: 784 }, Point { x: 295, y: 513 }),
    (Point { x: 378, y: 942 }, Point { x: 378, y: 283 }),
    (Point { x: 152, y: 838 }, Point { x: 796, y: 838 }),
    (Point { x: 624, y: 630 }, Point { x: 881, y: 887 }),
    (Point { x: 90, y: 420 }, Point { x: 412, y: 420 }),
    (Point { x: 868, y: 69 }, Point { x: 46, y: 891 }),
    (Point { x: 75, y: 890 }, Point { x: 452, y: 513 }),
    (Point { x: 133, y: 460 }, Point { x: 133, y: 985 }),
    (Point { x: 970, y: 145 }, Point { x: 549, y: 145 }),
    (Point { x: 149, y: 462 }, Point { x: 916, y: 462 }),
    (Point { x: 92, y: 845 }, Point { x: 92, y: 268 }),
    (Point { x: 580, y: 99 }, Point { x: 250, y: 99 }),
    (Point { x: 618, y: 708 }, Point { x: 618, y: 652 }),
    (Point { x: 690, y: 948 }, Point { x: 690, y: 38 }),
    (Point { x: 808, y: 594 }, Point { x: 944, y: 730 }),
    (Point { x: 100, y: 359 }, Point { x: 312, y: 359 }),
    (Point { x: 546, y: 392 }, Point { x: 41, y: 897 }),
    (Point { x: 593, y: 413 }, Point { x: 593, y: 892 }),
    (Point { x: 602, y: 484 }, Point { x: 602, y: 144 }),
    (Point { x: 90, y: 863 }, Point { x: 90, y: 170 }),
    (Point { x: 888, y: 987 }, Point { x: 888, y: 162 }),
    (Point { x: 229, y: 932 }, Point { x: 960, y: 201 }),
    (Point { x: 919, y: 654 }, Point { x: 70, y: 654 }),
    (Point { x: 13, y: 684 }, Point { x: 13, y: 348 }),
    (Point { x: 743, y: 477 }, Point { x: 166, y: 477 }),
    (Point { x: 901, y: 113 }, Point { x: 936, y: 113 }),
    (Point { x: 167, y: 567 }, Point { x: 540, y: 567 }),
    (Point { x: 566, y: 729 }, Point { x: 566, y: 660 }),
    (Point { x: 102, y: 660 }, Point { x: 615, y: 660 }),
    (Point { x: 273, y: 241 }, Point { x: 273, y: 413 }),
    (Point { x: 512, y: 241 }, Point { x: 512, y: 643 }),
    (Point { x: 869, y: 695 }, Point { x: 614, y: 440 }),
    (Point { x: 356, y: 583 }, Point { x: 356, y: 408 }),
    (Point { x: 61, y: 345 }, Point { x: 61, y: 233 }),
    (Point { x: 973, y: 33 }, Point { x: 88, y: 918 }),
    (Point { x: 977, y: 130 }, Point { x: 771, y: 130 }),
    (Point { x: 422, y: 382 }, Point { x: 899, y: 382 }),
    (Point { x: 536, y: 517 }, Point { x: 914, y: 139 }),
    (Point { x: 563, y: 755 }, Point { x: 312, y: 755 }),
    (Point { x: 770, y: 581 }, Point { x: 770, y: 940 }),
    (Point { x: 103, y: 186 }, Point { x: 313, y: 186 }),
    (Point { x: 681, y: 490 }, Point { x: 77, y: 490 }),
    (Point { x: 676, y: 351 }, Point { x: 913, y: 588 }),
    (Point { x: 292, y: 700 }, Point { x: 862, y: 700 }),
    (Point { x: 445, y: 175 }, Point { x: 188, y: 175 }),
    (Point { x: 62, y: 490 }, Point { x: 173, y: 601 }),
    (Point { x: 530, y: 455 }, Point { x: 63, y: 455 }),
    (Point { x: 145, y: 85 }, Point { x: 832, y: 772 }),
    (Point { x: 273, y: 414 }, Point { x: 273, y: 240 }),
    (Point { x: 25, y: 888 }, Point { x: 25, y: 684 }),
    (Point { x: 599, y: 393 }, Point { x: 599, y: 232 }),
    (Point { x: 198, y: 296 }, Point { x: 584, y: 682 }),
    (Point { x: 217, y: 886 }, Point { x: 614, y: 886 }),
    (Point { x: 464, y: 598 }, Point { x: 362, y: 496 }),
    (Point { x: 874, y: 106 }, Point { x: 874, y: 227 }),
    (Point { x: 248, y: 511 }, Point { x: 940, y: 511 }),
    (Point { x: 501, y: 861 }, Point { x: 21, y: 381 }),
    (Point { x: 385, y: 232 }, Point { x: 341, y: 232 }),
    (Point { x: 258, y: 449 }, Point { x: 337, y: 449 }),
    (Point { x: 94, y: 46 }, Point { x: 910, y: 862 }),
    (Point { x: 946, y: 825 }, Point { x: 946, y: 341 }),
    (Point { x: 93, y: 836 }, Point { x: 93, y: 781 }),
    (Point { x: 170, y: 903 }, Point { x: 616, y: 457 }),
    (Point { x: 717, y: 333 }, Point { x: 717, y: 238 }),
    (Point { x: 404, y: 243 }, Point { x: 516, y: 243 }),
    (Point { x: 611, y: 579 }, Point { x: 217, y: 973 }),
    (Point { x: 76, y: 851 }, Point { x: 76, y: 255 }),
    (Point { x: 181, y: 780 }, Point { x: 661, y: 780 }),
    (Point { x: 316, y: 188 }, Point { x: 333, y: 188 }),
    (Point { x: 799, y: 92 }, Point { x: 779, y: 92 }),
    (Point { x: 955, y: 374 }, Point { x: 869, y: 374 }),
    (Point { x: 872, y: 792 }, Point { x: 280, y: 200 }),
    (Point { x: 337, y: 239 }, Point { x: 438, y: 239 }),
    (Point { x: 424, y: 706 }, Point { x: 273, y: 857 }),
    (Point { x: 501, y: 239 }, Point { x: 684, y: 239 }),
    (Point { x: 198, y: 671 }, Point { x: 882, y: 671 }),
    (Point { x: 790, y: 775 }, Point { x: 802, y: 775 }),
    (Point { x: 708, y: 624 }, Point { x: 361, y: 277 }),
    (Point { x: 547, y: 731 }, Point { x: 547, y: 621 }),
    (Point { x: 264, y: 449 }, Point { x: 293, y: 449 }),
    (Point { x: 496, y: 870 }, Point { x: 496, y: 396 }),
    (Point { x: 988, y: 959 }, Point { x: 988, y: 285 }),
    (Point { x: 19, y: 51 }, Point { x: 926, y: 958 }),
    (Point { x: 472, y: 537 }, Point { x: 127, y: 882 }),
    (Point { x: 188, y: 488 }, Point { x: 478, y: 198 }),
    (Point { x: 949, y: 376 }, Point { x: 797, y: 224 }),
    (Point { x: 448, y: 609 }, Point { x: 348, y: 609 }),
    (Point { x: 838, y: 285 }, Point { x: 838, y: 865 }),
    (Point { x: 796, y: 142 }, Point { x: 70, y: 868 }),
    (Point { x: 848, y: 91 }, Point { x: 972, y: 91 }),
    (Point { x: 722, y: 964 }, Point { x: 722, y: 409 }),
    (Point { x: 313, y: 156 }, Point { x: 313, y: 725 }),
    (Point { x: 925, y: 251 }, Point { x: 925, y: 687 }),
    (Point { x: 803, y: 815 }, Point { x: 113, y: 125 }),
    (Point { x: 505, y: 517 }, Point { x: 505, y: 337 }),
    (Point { x: 935, y: 920 }, Point { x: 235, y: 920 }),
    (Point { x: 674, y: 274 }, Point { x: 63, y: 885 }),
    (Point { x: 458, y: 981 }, Point { x: 626, y: 981 }),
    (Point { x: 928, y: 950 }, Point { x: 836, y: 950 }),
    (Point { x: 163, y: 453 }, Point { x: 695, y: 985 }),
    (Point { x: 57, y: 374 }, Point { x: 398, y: 374 }),
    (Point { x: 937, y: 327 }, Point { x: 937, y: 811 }),
    (Point { x: 975, y: 932 }, Point { x: 265, y: 222 }),
    (Point { x: 490, y: 583 }, Point { x: 490, y: 482 }),
    (Point { x: 170, y: 183 }, Point { x: 196, y: 183 }),
    (Point { x: 738, y: 978 }, Point { x: 738, y: 812 }),
    (Point { x: 914, y: 170 }, Point { x: 914, y: 202 }),
    (Point { x: 202, y: 885 }, Point { x: 499, y: 885 }),
    (Point { x: 270, y: 887 }, Point { x: 150, y: 887 }),
    (Point { x: 447, y: 783 }, Point { x: 831, y: 399 }),
    (Point { x: 66, y: 136 }, Point { x: 77, y: 136 }),
    (Point { x: 536, y: 703 }, Point { x: 662, y: 829 }),
    (Point { x: 297, y: 821 }, Point { x: 297, y: 792 }),
    (Point { x: 640, y: 572 }, Point { x: 321, y: 572 }),
    (Point { x: 244, y: 833 }, Point { x: 865, y: 212 }),
    (Point { x: 454, y: 672 }, Point { x: 454, y: 726 }),
    (Point { x: 133, y: 812 }, Point { x: 303, y: 642 }),
    (Point { x: 280, y: 589 }, Point { x: 184, y: 589 }),
    (Point { x: 977, y: 572 }, Point { x: 977, y: 42 }),
    (Point { x: 62, y: 247 }, Point { x: 215, y: 247 }),
    (Point { x: 427, y: 503 }, Point { x: 809, y: 885 }),
    (Point { x: 671, y: 85 }, Point { x: 671, y: 770 }),
    (Point { x: 296, y: 990 }, Point { x: 296, y: 558 }),
    (Point { x: 103, y: 19 }, Point { x: 971, y: 887 }),
    (Point { x: 263, y: 712 }, Point { x: 263, y: 329 }),
    (Point { x: 954, y: 897 }, Point { x: 954, y: 41 }),
    (Point { x: 278, y: 536 }, Point { x: 278, y: 346 }),
    (Point { x: 270, y: 620 }, Point { x: 983, y: 620 }),
    (Point { x: 229, y: 863 }, Point { x: 91, y: 863 }),
    (Point { x: 935, y: 413 }, Point { x: 394, y: 413 }),
    (Point { x: 709, y: 668 }, Point { x: 77, y: 668 }),
    (Point { x: 310, y: 853 }, Point { x: 310, y: 286 }),
    (Point { x: 534, y: 694 }, Point { x: 511, y: 717 }),
    (Point { x: 349, y: 726 }, Point { x: 349, y: 439 }),
    (Point { x: 113, y: 196 }, Point { x: 970, y: 196 }),
    (Point { x: 836, y: 340 }, Point { x: 709, y: 340 }),
    (Point { x: 38, y: 485 }, Point { x: 38, y: 14 }),
    (Point { x: 38, y: 278 }, Point { x: 569, y: 278 }),
    (Point { x: 862, y: 90 }, Point { x: 281, y: 671 }),
    (Point { x: 677, y: 124 }, Point { x: 405, y: 124 }),
    (Point { x: 399, y: 568 }, Point { x: 536, y: 705 }),
    (Point { x: 611, y: 839 }, Point { x: 188, y: 416 }),
    (Point { x: 570, y: 925 }, Point { x: 570, y: 251 }),
    (Point { x: 804, y: 368 }, Point { x: 284, y: 888 }),
    (Point { x: 262, y: 842 }, Point { x: 388, y: 842 }),
    (Point { x: 751, y: 800 }, Point { x: 751, y: 504 }),
    (Point { x: 762, y: 882 }, Point { x: 201, y: 321 }),
    (Point { x: 411, y: 421 }, Point { x: 807, y: 421 }),
    (Point { x: 654, y: 406 }, Point { x: 265, y: 795 }),
    (Point { x: 863, y: 558 }, Point { x: 625, y: 320 }),
    (Point { x: 451, y: 673 }, Point { x: 451, y: 354 }),
    (Point { x: 359, y: 239 }, Point { x: 566, y: 239 }),
    (Point { x: 259, y: 211 }, Point { x: 955, y: 907 }),
    (Point { x: 253, y: 506 }, Point { x: 542, y: 217 }),
    (Point { x: 547, y: 794 }, Point { x: 373, y: 620 }),
    (Point { x: 132, y: 263 }, Point { x: 581, y: 712 }),
    (Point { x: 168, y: 237 }, Point { x: 168, y: 142 }),
    (Point { x: 834, y: 296 }, Point { x: 152, y: 978 }),
    (Point { x: 156, y: 14 }, Point { x: 955, y: 14 }),
    (Point { x: 927, y: 22 }, Point { x: 285, y: 664 }),
    (Point { x: 384, y: 291 }, Point { x: 362, y: 269 }),
    (Point { x: 91, y: 561 }, Point { x: 91, y: 19 }),
    (Point { x: 472, y: 953 }, Point { x: 472, y: 576 }),
    (Point { x: 700, y: 666 }, Point { x: 723, y: 689 }),
    (Point { x: 447, y: 815 }, Point { x: 566, y: 815 }),
    (Point { x: 698, y: 411 }, Point { x: 698, y: 762 }),
    (Point { x: 427, y: 606 }, Point { x: 119, y: 298 }),
    (Point { x: 531, y: 401 }, Point { x: 669, y: 263 }),
    (Point { x: 681, y: 21 }, Point { x: 681, y: 111 }),
    (Point { x: 168, y: 360 }, Point { x: 168, y: 447 }),
    (Point { x: 74, y: 67 }, Point { x: 717, y: 67 }),
    (Point { x: 287, y: 88 }, Point { x: 345, y: 88 }),
    (Point { x: 234, y: 80 }, Point { x: 234, y: 848 }),
    (Point { x: 583, y: 251 }, Point { x: 33, y: 251 }),
    (Point { x: 200, y: 522 }, Point { x: 366, y: 356 }),
    (Point { x: 815, y: 936 }, Point { x: 27, y: 148 }),
    (Point { x: 139, y: 302 }, Point { x: 139, y: 768 }),
    (Point { x: 473, y: 69 }, Point { x: 473, y: 664 }),
    (Point { x: 42, y: 813 }, Point { x: 42, y: 918 }),
    (Point { x: 881, y: 188 }, Point { x: 881, y: 345 }),
    (Point { x: 457, y: 920 }, Point { x: 301, y: 764 }),
    (Point { x: 894, y: 662 }, Point { x: 779, y: 662 }),
    (Point { x: 750, y: 411 }, Point { x: 750, y: 368 }),
    (Point { x: 986, y: 167 }, Point { x: 246, y: 167 }),
    (Point { x: 914, y: 418 }, Point { x: 742, y: 590 }),
    (Point { x: 710, y: 110 }, Point { x: 63, y: 757 }),
    (Point { x: 353, y: 493 }, Point { x: 353, y: 473 }),
    (Point { x: 211, y: 700 }, Point { x: 181, y: 700 }),
    (Point { x: 492, y: 604 }, Point { x: 25, y: 604 }),
    (Point { x: 212, y: 174 }, Point { x: 362, y: 174 }),
    (Point { x: 801, y: 434 }, Point { x: 752, y: 385 }),
    (Point { x: 956, y: 861 }, Point { x: 469, y: 374 }),
    (Point { x: 197, y: 318 }, Point { x: 257, y: 378 }),
    (Point { x: 604, y: 594 }, Point { x: 604, y: 809 }),
    (Point { x: 716, y: 447 }, Point { x: 306, y: 857 }),
    (Point { x: 974, y: 974 }, Point { x: 24, y: 24 }),
    (Point { x: 925, y: 467 }, Point { x: 925, y: 311 }),
    (Point { x: 357, y: 381 }, Point { x: 769, y: 381 }),
    (Point { x: 714, y: 395 }, Point { x: 372, y: 395 }),
    (Point { x: 360, y: 718 }, Point { x: 728, y: 718 }),
    (Point { x: 161, y: 186 }, Point { x: 730, y: 755 }),
    (Point { x: 407, y: 316 }, Point { x: 407, y: 61 }),
    (Point { x: 466, y: 214 }, Point { x: 333, y: 347 }),
    (Point { x: 190, y: 955 }, Point { x: 190, y: 678 }),
    (Point { x: 969, y: 48 }, Point { x: 72, y: 945 }),
    (Point { x: 296, y: 153 }, Point { x: 833, y: 153 }),
    (Point { x: 930, y: 400 }, Point { x: 637, y: 400 }),
    (Point { x: 606, y: 953 }, Point { x: 541, y: 953 }),
    (Point { x: 978, y: 179 }, Point { x: 21, y: 179 }),
    (Point { x: 112, y: 49 }, Point { x: 112, y: 793 }),
    (Point { x: 346, y: 881 }, Point { x: 151, y: 881 }),
    (Point { x: 737, y: 404 }, Point { x: 737, y: 693 }),
    (Point { x: 98, y: 271 }, Point { x: 98, y: 144 }),
    (Point { x: 469, y: 830 }, Point { x: 46, y: 830 }),
    (Point { x: 246, y: 651 }, Point { x: 246, y: 243 }),
    (Point { x: 47, y: 129 }, Point { x: 880, y: 962 }),
    (Point { x: 449, y: 609 }, Point { x: 980, y: 78 }),
    (Point { x: 603, y: 307 }, Point { x: 603, y: 896 }),
    (Point { x: 121, y: 339 }, Point { x: 22, y: 240 }),
    (Point { x: 97, y: 726 }, Point { x: 274, y: 726 }),
    (Point { x: 527, y: 668 }, Point { x: 786, y: 409 }),
    (Point { x: 649, y: 162 }, Point { x: 321, y: 162 }),
    (Point { x: 253, y: 10 }, Point { x: 253, y: 690 }),
    (Point { x: 43, y: 748 }, Point { x: 590, y: 748 }),
    (Point { x: 245, y: 424 }, Point { x: 245, y: 495 }),
    (Point { x: 509, y: 595 }, Point { x: 261, y: 843 }),
    (Point { x: 924, y: 758 }, Point { x: 683, y: 758 }),
    (Point { x: 693, y: 516 }, Point { x: 684, y: 507 }),
    (Point { x: 654, y: 201 }, Point { x: 654, y: 840 }),
    (Point { x: 321, y: 543 }, Point { x: 315, y: 549 }),
    (Point { x: 840, y: 309 }, Point { x: 764, y: 233 }),
    (Point { x: 865, y: 183 }, Point { x: 177, y: 871 }),
    (Point { x: 700, y: 135 }, Point { x: 14, y: 821 }),
    (Point { x: 178, y: 178 }, Point { x: 971, y: 971 }),
    (Point { x: 88, y: 649 }, Point { x: 88, y: 899 }),
    (Point { x: 327, y: 37 }, Point { x: 327, y: 58 }),
    (Point { x: 252, y: 687 }, Point { x: 482, y: 457 }),
    (Point { x: 12, y: 771 }, Point { x: 754, y: 29 }),
    (Point { x: 309, y: 695 }, Point { x: 630, y: 695 }),
    (Point { x: 146, y: 671 }, Point { x: 146, y: 695 }),
    (Point { x: 226, y: 697 }, Point { x: 798, y: 697 }),
    (Point { x: 239, y: 736 }, Point { x: 239, y: 945 }),
    (Point { x: 483, y: 756 }, Point { x: 483, y: 965 }),
    (Point { x: 306, y: 475 }, Point { x: 800, y: 969 }),
    (Point { x: 580, y: 927 }, Point { x: 580, y: 102 }),
    (Point { x: 867, y: 83 }, Point { x: 830, y: 83 }),
    (Point { x: 635, y: 359 }, Point { x: 761, y: 233 }),
    (Point { x: 733, y: 851 }, Point { x: 180, y: 298 }),
    (Point { x: 478, y: 76 }, Point { x: 401, y: 76 }),
    (Point { x: 552, y: 581 }, Point { x: 552, y: 525 }),
    (Point { x: 842, y: 724 }, Point { x: 847, y: 724 }),
    (Point { x: 652, y: 76 }, Point { x: 385, y: 76 }),
    (Point { x: 695, y: 894 }, Point { x: 245, y: 894 }),
    (Point { x: 301, y: 487 }, Point { x: 301, y: 665 }),
    (Point { x: 412, y: 555 }, Point { x: 412, y: 80 }),
    (Point { x: 591, y: 311 }, Point { x: 289, y: 311 }),
    (Point { x: 961, y: 933 }, Point { x: 69, y: 41 }),
    (Point { x: 78, y: 266 }, Point { x: 14, y: 202 }),
    (Point { x: 255, y: 696 }, Point { x: 766, y: 696 }),
    (Point { x: 715, y: 246 }, Point { x: 508, y: 246 }),
    (Point { x: 756, y: 567 }, Point { x: 188, y: 567 }),
    (Point { x: 866, y: 377 }, Point { x: 652, y: 591 }),
    (Point { x: 267, y: 226 }, Point { x: 204, y: 163 }),
    (Point { x: 506, y: 104 }, Point { x: 506, y: 587 }),
    (Point { x: 270, y: 434 }, Point { x: 270, y: 395 }),
    (Point { x: 879, y: 127 }, Point { x: 879, y: 859 }),
    (Point { x: 65, y: 669 }, Point { x: 65, y: 747 }),
    (Point { x: 486, y: 745 }, Point { x: 612, y: 745 }),
    (Point { x: 276, y: 246 }, Point { x: 276, y: 41 }),
    (Point { x: 41, y: 840 }, Point { x: 226, y: 655 }),
    (Point { x: 207, y: 495 }, Point { x: 94, y: 495 }),
    (Point { x: 142, y: 970 }, Point { x: 285, y: 970 }),
    (Point { x: 73, y: 239 }, Point { x: 83, y: 239 }),
    (Point { x: 787, y: 409 }, Point { x: 527, y: 409 }),
    (Point { x: 678, y: 565 }, Point { x: 678, y: 582 }),
    (Point { x: 314, y: 185 }, Point { x: 67, y: 185 }),
];
