package com.zkrallah.bytez;

import java.io.File;
import java.io.IOException;
import java.io.PrintStream;

import net.bytebuddy.ByteBuddy;
import net.bytebuddy.description.modifier.Ownership;
import net.bytebuddy.description.modifier.Visibility;
import net.bytebuddy.implementation.MethodCall;

public class GenerateHelloByteBuddy {
    public static void generate() throws NoSuchMethodException, SecurityException, NoSuchFieldException, IOException {
        new ByteBuddy()
                .with(net.bytebuddy.ClassFileVersion.JAVA_V17)
                .subclass(Object.class)
                .name("HelloByteBuddy")
                .defineMethod("main", void.class, Visibility.PUBLIC, Ownership.STATIC)
                .withParameters(String[].class)
                .intercept(
                        MethodCall.invoke(PrintStream.class.getMethod("println", String.class))
                                .onField(System.class.getField("out"))
                                .with("Hello from ByteBuddy"))
                .make()
                .saveIn(new File("."));

        System.out.println("Generated HelloByteBuddy.class");

    }
}
