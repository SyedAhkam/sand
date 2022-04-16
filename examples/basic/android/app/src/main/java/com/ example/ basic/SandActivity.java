package com.example.basic;

import android.app.NativeActivity;

import android.os.Bundle;

public class SandActivity extends NativeActivity {

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        android.util.Log.i("SandActivity", "OnCreate");

        super.onCreate(savedInstanceState);
    }
}
