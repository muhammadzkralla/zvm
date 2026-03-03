package com.zkrallah.bytez;

import org.objectweb.asm.*;
import java.io.FileOutputStream;
import java.io.IOException;
import static org.objectweb.asm.Opcodes.*;

public class GenerateLdcw {

    public static void generate() throws IOException {
        ClassWriter cw = new ClassWriter(ClassWriter.COMPUTE_FRAMES | ClassWriter.COMPUTE_MAXS);
        cw.visit(V17, ACC_PUBLIC, "LdcwTest", null, "java/lang/Object", null);

        // Default constructor
        MethodVisitor ctor = cw.visitMethod(ACC_PUBLIC, "<init>", "()V", null, null);
        ctor.visitCode();
        ctor.visitVarInsn(ALOAD, 0);
        ctor.visitMethodInsn(INVOKESPECIAL, "java/lang/Object", "<init>", "()V", false);
        ctor.visitInsn(RETURN);
        ctor.visitMaxs(1, 1);
        ctor.visitEnd();

        // Main method
        MethodVisitor mv = cw.visitMethod(ACC_PUBLIC | ACC_STATIC, "main", "([Ljava/lang/String;)V", null, null);
        mv.visitCode();

        // Create many dummy constants to push the constant pool index beyond 255
        for (int i = 0; i < 260; i++) {
            mv.visitLdcInsn("dummy_string_" + i);
            mv.visitInsn(POP); // Pop it off the stack to keep the stack clean
        }

        // This string should now have a constant pool index > 255, forcing LDC_W
        mv.visitLdcInsn("This string should be loaded with LDC_W");
        mv.visitFieldInsn(GETSTATIC, "java/lang/System", "out", "Ljava/io/PrintStream;");
        mv.visitInsn(SWAP);
        mv.visitMethodInsn(INVOKEVIRTUAL, "java/io/PrintStream", "println", "(Ljava/lang/String;)V", false);

        mv.visitLdcInsn(256); // Integer constant that might need LDC_W if index is high
        mv.visitFieldInsn(GETSTATIC, "java/lang/System", "out", "Ljava/io/PrintStream;");
        mv.visitInsn(SWAP);
        mv.visitMethodInsn(INVOKEVIRTUAL, "java/io/PrintStream", "println", "(I)V", false);

        mv.visitInsn(RETURN);
        mv.visitMaxs(0, 0);
        mv.visitEnd();

        cw.visitEnd();

        try (FileOutputStream fos = new FileOutputStream("LdcwTest.class")) {
            fos.write(cw.toByteArray());
        }

        System.out.println("Generated LdcwTest.class");
    }
}
