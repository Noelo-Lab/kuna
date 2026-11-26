package com.ex.fail;

public class Whole {
    @Mark('空')
    int broken;

    int size() {
        return broken;
    }
}
