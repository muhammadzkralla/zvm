package com.zkrallah.bytez;

import static org.objectweb.asm.Opcodes.ACC_PUBLIC;
import static org.objectweb.asm.Opcodes.ACC_STATIC;
import static org.objectweb.asm.Opcodes.ALOAD;
import static org.objectweb.asm.Opcodes.GETSTATIC;
import static org.objectweb.asm.Opcodes.INVOKESPECIAL;
import static org.objectweb.asm.Opcodes.INVOKEVIRTUAL;
import static org.objectweb.asm.Opcodes.RETURN;
import static org.objectweb.asm.Opcodes.V17;

import java.io.FileNotFoundException;
import java.io.FileOutputStream;
import java.io.IOException;

import org.objectweb.asm.ClassWriter;
import org.objectweb.asm.MethodVisitor;

public class GenerateHelloASM {
    public static void generate() throws FileNotFoundException, IOException {

        // 1) Create class writer
        ClassWriter cw = new ClassWriter(ClassWriter.COMPUTE_FRAMES);

        // 2) Define class
        cw.visit(
                V17, // Java 17 bytecode
                ACC_PUBLIC,
                "HelloAsm", // internal class name
                null,
                "java/lang/Object",
                null);

        // 3) Default constructor
        MethodVisitor ctor = cw.visitMethod(
                ACC_PUBLIC,
                "<init>",
                "()V",
                null,
                null);

        ctor.visitCode();
        ctor.visitVarInsn(ALOAD, 0);
        ctor.visitMethodInsn(
                INVOKESPECIAL,
                "java/lang/Object",
                "<init>",
                "()V",
                false);
        ctor.visitInsn(RETURN);
        ctor.visitMaxs(0, 0);
        ctor.visitEnd();

        // 4) main method
        MethodVisitor mv = cw.visitMethod(
                ACC_PUBLIC | ACC_STATIC,
                "main",
                "([Ljava/lang/String;)V",
                null,
                null);

        mv.visitCode();

        mv.visitFieldInsn(
                GETSTATIC,
                "java/lang/System",
                "out",
                "Ljava/io/PrintStream;");

        mv.visitLdcInsn("Hello from ASM");

        mv.visitMethodInsn(
                INVOKEVIRTUAL,
                "java/io/PrintStream",
                "println",
                "(Ljava/lang/String;)V",
                false);

        mv.visitInsn(RETURN);
        mv.visitMaxs(0, 0);
        mv.visitEnd();

        // 5) Finish class
        cw.visitEnd();

        // 6) Write class file
        try (FileOutputStream fos = new FileOutputStream("HelloAsm.class")) {
            fos.write(cw.toByteArray());
        }

        System.out.println("Generated HelloAsm.class");

    }
}
