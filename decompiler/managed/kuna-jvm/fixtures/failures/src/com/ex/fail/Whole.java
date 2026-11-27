package com.ex.fail;

public class Whole {
    @Mark('\u7a7a')
    int broken;

    int size() {
        return broken;
    }
}
