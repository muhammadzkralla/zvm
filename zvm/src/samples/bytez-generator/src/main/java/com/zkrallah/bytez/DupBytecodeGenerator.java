package com.zkrallah.bytez;

import org.objectweb.asm.*;
import java.io.FileOutputStream;
import java.io.IOException;

import static org.objectweb.asm.Opcodes.*;

public class DupBytecodeGenerator {

    public static void generateDupTestClass() throws IOException {
        ClassWriter cw = new ClassWriter(ClassWriter.COMPUTE_FRAMES | ClassWriter.COMPUTE_MAXS);

        cw.visit(V1_8, ACC_PUBLIC, "DupTest", null, "java/lang/Object", null);

        generateConstructor(cw);

        generateDupTest(cw);
        generateDupX1Test(cw);
        generateDupX2Test(cw);
        generateDup2Test(cw);
        generateDup2X1Test(cw);
        generateDup2X2Test(cw);

        generateMainMethod(cw);

        cw.visitEnd();

        byte[] classBytes = cw.toByteArray();
        try (FileOutputStream fos = new FileOutputStream("DupTest.class")) {
            fos.write(classBytes);
        }
    }

    private static void generateConstructor(ClassWriter cw) {
        MethodVisitor mv = cw.visitMethod(ACC_PUBLIC, "<init>", "()V", null, null);
        mv.visitCode();
        mv.visitVarInsn(ALOAD, 0);
        mv.visitMethodInsn(INVOKESPECIAL, "java/lang/Object", "<init>", "()V", false);
        mv.visitInsn(RETURN);
        mv.visitMaxs(0, 0);
        mv.visitEnd();
    }

    // Stack: value -> value, value
    private static void generateDupTest(ClassWriter cw) {
        MethodVisitor mv = cw.visitMethod(ACC_PUBLIC | ACC_STATIC, "testDup", "()V", null, null);
        mv.visitCode();

        mv.visitIntInsn(BIPUSH, 42);

        mv.visitInsn(DUP);

        // Now stack has [42, 42]
        // Pop and print both values
        mv.visitFieldInsn(GETSTATIC, "java/lang/System", "out", "Ljava/io/PrintStream;");
        mv.visitInsn(SWAP);
        mv.visitMethodInsn(INVOKEVIRTUAL, "java/io/PrintStream", "println", "(I)V", false);

        mv.visitFieldInsn(GETSTATIC, "java/lang/System", "out", "Ljava/io/PrintStream;");
        mv.visitInsn(SWAP);
        mv.visitMethodInsn(INVOKEVIRTUAL, "java/io/PrintStream", "println", "(I)V", false);

        mv.visitInsn(RETURN);
        mv.visitMaxs(0, 0);
        mv.visitEnd();
    }

    // Stack: value2, value1 -> value1, value2, value1
    private static void generateDupX1Test(ClassWriter cw) {
        MethodVisitor mv = cw.visitMethod(ACC_PUBLIC | ACC_STATIC, "testDupX1", "()V", null, null);
        mv.visitCode();

        mv.visitIntInsn(BIPUSH, 10);
        mv.visitIntInsn(BIPUSH, 20);

        mv.visitInsn(DUP_X1);

        // Stack now: [20, 10, 20]
        // Print all three
        mv.visitFieldInsn(GETSTATIC, "java/lang/System", "out", "Ljava/io/PrintStream;");
        mv.visitInsn(SWAP);
        mv.visitMethodInsn(INVOKEVIRTUAL, "java/io/PrintStream", "println", "(I)V", false);

        mv.visitFieldInsn(GETSTATIC, "java/lang/System", "out", "Ljava/io/PrintStream;");
        mv.visitInsn(SWAP);
        mv.visitMethodInsn(INVOKEVIRTUAL, "java/io/PrintStream", "println", "(I)V", false);

        mv.visitFieldInsn(GETSTATIC, "java/lang/System", "out", "Ljava/io/PrintStream;");
        mv.visitInsn(SWAP);
        mv.visitMethodInsn(INVOKEVIRTUAL, "java/io/PrintStream", "println", "(I)V", false);

        mv.visitInsn(RETURN);
        mv.visitMaxs(0, 0);
        mv.visitEnd();
    }

    // Stack: value3, value2, value1 -> value1, value3, value2, value1
    private static void generateDupX2Test(ClassWriter cw) {
        MethodVisitor mv = cw.visitMethod(ACC_PUBLIC | ACC_STATIC, "testDupX2", "()V", null, null);
        mv.visitCode();

        mv.visitIntInsn(BIPUSH, 10);
        mv.visitIntInsn(BIPUSH, 20);
        mv.visitIntInsn(BIPUSH, 30);

        mv.visitInsn(DUP_X2);

        // Stack now: [30, 10, 20, 30]
        // Print "DUP_X2 result:" and pop all values
        mv.visitFieldInsn(GETSTATIC, "java/lang/System", "out", "Ljava/io/PrintStream;");
        mv.visitLdcInsn("DUP_X2 test - top value:");
        mv.visitMethodInsn(INVOKEVIRTUAL, "java/io/PrintStream", "println", "(Ljava/lang/String;)V", false);

        for (int i = 0; i < 4; i++) {
            mv.visitFieldInsn(GETSTATIC, "java/lang/System", "out", "Ljava/io/PrintStream;");
            mv.visitInsn(SWAP);
            mv.visitMethodInsn(INVOKEVIRTUAL, "java/io/PrintStream", "println", "(I)V", false);
        }

        mv.visitInsn(RETURN);
        mv.visitMaxs(0, 0);
        mv.visitEnd();
    }

    // Form 1: value2, value1 -> value2, value1, value2, value1 (two cat1 values)
    private static void generateDup2Test(ClassWriter cw) {
        MethodVisitor mv = cw.visitMethod(ACC_PUBLIC | ACC_STATIC, "testDup2", "()V", null, null);
        mv.visitCode();

        mv.visitIntInsn(BIPUSH, 100);
        mv.visitIntInsn(BIPUSH, 200);

        mv.visitInsn(DUP2);

        // Stack now: [100, 200, 100, 200]
        mv.visitFieldInsn(GETSTATIC, "java/lang/System", "out", "Ljava/io/PrintStream;");
        mv.visitLdcInsn("DUP2 test:");
        mv.visitMethodInsn(INVOKEVIRTUAL, "java/io/PrintStream", "println", "(Ljava/lang/String;)V", false);

        for (int i = 0; i < 4; i++) {
            mv.visitFieldInsn(GETSTATIC, "java/lang/System", "out", "Ljava/io/PrintStream;");
            mv.visitInsn(SWAP);
            mv.visitMethodInsn(INVOKEVIRTUAL, "java/io/PrintStream", "println", "(I)V", false);
        }

        mv.visitInsn(RETURN);
        mv.visitMaxs(0, 0);
        mv.visitEnd();
    }

    // Form 1: value3, value2, value1 -> value2, value1, value3, value2, value1
    private static void generateDup2X1Test(ClassWriter cw) {
        MethodVisitor mv = cw.visitMethod(ACC_PUBLIC | ACC_STATIC, "testDup2X1", "()V", null, null);
        mv.visitCode();

        mv.visitIntInsn(BIPUSH, 1);
        mv.visitIntInsn(BIPUSH, 2);
        mv.visitIntInsn(BIPUSH, 3);

        mv.visitInsn(DUP2_X1);

        // Stack now: [2, 3, 1, 2, 3]
        mv.visitFieldInsn(GETSTATIC, "java/lang/System", "out", "Ljava/io/PrintStream;");
        mv.visitLdcInsn("DUP2_X1 test:");
        mv.visitMethodInsn(INVOKEVIRTUAL, "java/io/PrintStream", "println", "(Ljava/lang/String;)V", false);

        for (int i = 0; i < 5; i++) {
            mv.visitFieldInsn(GETSTATIC, "java/lang/System", "out", "Ljava/io/PrintStream;");
            mv.visitInsn(SWAP);
            mv.visitMethodInsn(INVOKEVIRTUAL, "java/io/PrintStream", "println", "(I)V", false);
        }

        mv.visitInsn(RETURN);
        mv.visitMaxs(0, 0);
        mv.visitEnd();
    }

    // Form 1: value4, value3, value2, value1 -> value2, value1, value4, value3,
    // value2, value1
    private static void generateDup2X2Test(ClassWriter cw) {
        MethodVisitor mv = cw.visitMethod(ACC_PUBLIC | ACC_STATIC, "testDup2X2", "()V", null, null);
        mv.visitCode();

        mv.visitIntInsn(BIPUSH, 1);
        mv.visitIntInsn(BIPUSH, 2);
        mv.visitIntInsn(BIPUSH, 3);
        mv.visitIntInsn(BIPUSH, 4);

        mv.visitInsn(DUP2_X2);

        // Stack now: [3, 4, 1, 2, 3, 4]
        mv.visitFieldInsn(GETSTATIC, "java/lang/System", "out", "Ljava/io/PrintStream;");
        mv.visitLdcInsn("DUP2_X2 test:");
        mv.visitMethodInsn(INVOKEVIRTUAL, "java/io/PrintStream", "println", "(Ljava/lang/String;)V", false);

        for (int i = 0; i < 6; i++) {
            mv.visitFieldInsn(GETSTATIC, "java/lang/System", "out", "Ljava/io/PrintStream;");
            mv.visitInsn(SWAP);
            mv.visitMethodInsn(INVOKEVIRTUAL, "java/io/PrintStream", "println", "(I)V", false);
        }

        mv.visitInsn(RETURN);
        mv.visitMaxs(0, 0);
        mv.visitEnd();
    }

    private static void generateMainMethod(ClassWriter cw) {
        MethodVisitor mv = cw.visitMethod(ACC_PUBLIC | ACC_STATIC, "main", "([Ljava/lang/String;)V", null, null);
        mv.visitCode();

        mv.visitFieldInsn(GETSTATIC, "java/lang/System", "out", "Ljava/io/PrintStream;");
        mv.visitLdcInsn("=== Testing DUP Instructions ===");
        mv.visitMethodInsn(INVOKEVIRTUAL, "java/io/PrintStream", "println", "(Ljava/lang/String;)V", false);

        // Call testDup
        mv.visitFieldInsn(GETSTATIC, "java/lang/System", "out", "Ljava/io/PrintStream;");
        mv.visitLdcInsn("\n--- DUP test ---");
        mv.visitMethodInsn(INVOKEVIRTUAL, "java/io/PrintStream", "println", "(Ljava/lang/String;)V", false);
        mv.visitMethodInsn(INVOKESTATIC, "DupTest", "testDup", "()V", false);

        // Call testDupX1
        mv.visitFieldInsn(GETSTATIC, "java/lang/System", "out", "Ljava/io/PrintStream;");
        mv.visitLdcInsn("\n--- DUP_X1 test ---");
        mv.visitMethodInsn(INVOKEVIRTUAL, "java/io/PrintStream", "println", "(Ljava/lang/String;)V", false);
        mv.visitMethodInsn(INVOKESTATIC, "DupTest", "testDupX1", "()V", false);

        // Call testDupX2
        mv.visitFieldInsn(GETSTATIC, "java/lang/System", "out", "Ljava/io/PrintStream;");
        mv.visitLdcInsn("\n--- DUP_X2 test ---");
        mv.visitMethodInsn(INVOKEVIRTUAL, "java/io/PrintStream", "println", "(Ljava/lang/String;)V", false);
        mv.visitMethodInsn(INVOKESTATIC, "DupTest", "testDupX2", "()V", false);

        // Call testDup2
        mv.visitFieldInsn(GETSTATIC, "java/lang/System", "out", "Ljava/io/PrintStream;");
        mv.visitLdcInsn("\n--- DUP2 test ---");
        mv.visitMethodInsn(INVOKEVIRTUAL, "java/io/PrintStream", "println", "(Ljava/lang/String;)V", false);
        mv.visitMethodInsn(INVOKESTATIC, "DupTest", "testDup2", "()V", false);

        // Call testDup2X1
        mv.visitFieldInsn(GETSTATIC, "java/lang/System", "out", "Ljava/io/PrintStream;");
        mv.visitLdcInsn("\n--- DUP2_X1 test ---");
        mv.visitMethodInsn(INVOKEVIRTUAL, "java/io/PrintStream", "println", "(Ljava/lang/String;)V", false);
        mv.visitMethodInsn(INVOKESTATIC, "DupTest", "testDup2X1", "()V", false);

        // Call testDup2X2
        mv.visitFieldInsn(GETSTATIC, "java/lang/System", "out", "Ljava/io/PrintStream;");
        mv.visitLdcInsn("\n--- DUP2_X2 test ---");
        mv.visitMethodInsn(INVOKEVIRTUAL, "java/io/PrintStream", "println", "(Ljava/lang/String;)V", false);
        mv.visitMethodInsn(INVOKESTATIC, "DupTest", "testDup2X2", "()V", false);

        mv.visitInsn(RETURN);
        mv.visitMaxs(0, 0);
        mv.visitEnd();
    }
}
