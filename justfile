new day:
    # test that this day does not already exist
    test ! -d d{{day}}
    cp -r template d{{day}}
