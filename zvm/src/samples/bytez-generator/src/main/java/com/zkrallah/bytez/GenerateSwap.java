package com.zkrallah.bytez;

import org.objectweb.asm.*;

import java.io.FileOutputStream;

import static org.objectweb.asm.Opcodes.*;

public class GenerateSwap {
    public static void generate() throws Exception {
        ClassWriter cw = new ClassWriter(ClassWriter.COMPUTE_FRAMES | ClassWriter.COMPUTE_MAXS);
        cw.visit(V17, ACC_PUBLIC, "SwapExample", null, "java/lang/Object", null);

        // default constructor
        MethodVisitor ctor = cw.visitMethod(ACC_PUBLIC, "<init>", "()V", null, null);
        ctor.visitCode();
        ctor.visitVarInsn(ALOAD, 0);
        ctor.visitMethodInsn(INVOKESPECIAL, "java/lang/Object", "<init>", "()V", false);
        ctor.visitInsn(RETURN);
        ctor.visitMaxs(1, 1);
        ctor.visitEnd();

        // main method
        MethodVisitor mv = cw.visitMethod(ACC_PUBLIC + ACC_STATIC, "main", "([Ljava/lang/String;)V", null, null);
        mv.visitCode();

        // Push two integers
        mv.visitLdcInsn(1); // push 1
        mv.visitLdcInsn(2); // push 2

        // Swap the top two stack values
        mv.visitInsn(SWAP);

        // Print top of stack
        mv.visitFieldInsn(GETSTATIC, "java/lang/System", "out", "Ljava/io/PrintStream;");
        mv.visitInsn(SWAP); // bring integer on top of println
        mv.visitMethodInsn(INVOKEVIRTUAL, "java/io/PrintStream", "println", "(I)V", false);

        mv.visitInsn(RETURN);
        mv.visitMaxs(2, 1);
        mv.visitEnd();

        cw.visitEnd();

        try (FileOutputStream fos = new FileOutputStream("SwapExample.class")) {
            fos.write(cw.toByteArray());
        }

        System.out.println("Generated SwapExample.class");
    }
}
