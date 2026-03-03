public class Main {
    public static void main(String[] args) {
        // ldc for String (single byte index)
        String s1 = "Hello, LDC!";
        System.out.println(s1);

        // ldc for Integer (single byte index)
        int i1 = 12345;
        System.out.println(i1);

        // ldc for Float (single byte index)
        float f1 = 3.14f;
        System.out.println(f1);

        // ldc_w for String (two-byte index, if constant pool is large enough)
        // To force ldc_w, we often need many constants before it.
        // Let's create a bunch of dummy strings to push the index.
        String dummy1 = "dummy1"; String dummy2 = "dummy2"; String dummy3 = "dummy3";
        String dummy4 = "dummy4"; String dummy5 = "dummy5"; String dummy6 = "dummy6";
        String dummy7 = "dummy7"; String dummy8 = "dummy8"; String dummy9 = "dummy9";
        String dummy10 = "dummy10"; String dummy11 = "dummy11"; String dummy12 = "dummy12";
        String dummy13 = "dummy13"; String dummy14 = "dummy14"; String dummy15 = "dummy15";
        String dummy16 = "dummy16"; String dummy17 = "dummy17"; String dummy18 = "dummy18";
        String dummy19 = "dummy19"; String dummy20 = "dummy20"; String dummy21 = "dummy21";
        String dummy22 = "dummy22"; String dummy23 = "dummy23"; String dummy24 = "dummy24";
        String dummy25 = "dummy25"; String dummy26 = "dummy26"; String dummy27 = "dummy27";
        String dummy28 = "dummy28"; String dummy29 = "dummy29"; String dummy30 = "dummy30";
        String dummy31 = "dummy31"; String dummy32 = "dummy32"; String dummy33 = "dummy33";
        String dummy34 = "dummy34"; String dummy35 = "dummy35"; String dummy36 = "dummy36";
        String dummy37 = "dummy37"; String dummy38 = "dummy38"; String dummy39 = "dummy39";
        String dummy40 = "dummy40"; String dummy41 = "dummy41"; String dummy42 = "dummy42";
        String dummy43 = "dummy43"; String dummy44 = "dummy44"; String dummy45 = "dummy45";
        String dummy46 = "dummy46"; String dummy47 = "dummy47"; String dummy48 = "dummy48";
        String dummy49 = "dummy49"; String dummy50 = "dummy50"; String dummy51 = "dummy51";
        String dummy52 = "dummy52"; String dummy53 = "dummy53"; String dummy54 = "dummy54";
        String dummy55 = "dummy55"; String dummy56 = "dummy56"; String dummy57 = "dummy57";
        String dummy58 = "dummy58"; String dummy59 = "dummy59"; String dummy60 = "dummy60";
        String dummy61 = "dummy61"; String dummy62 = "dummy62"; String dummy63 = "dummy63";
        String dummy64 = "dummy64"; String dummy65 = "dummy65"; String dummy66 = "dummy66";
        String dummy67 = "dummy67"; String dummy68 = "dummy68"; String dummy69 = "dummy69";
        String dummy70 = "dummy70"; String dummy71 = "dummy71"; String dummy72 = "dummy72";
        String dummy73 = "dummy73"; String dummy74 = "dummy74"; String dummy75 = "dummy75";
        String dummy76 = "dummy76"; String dummy77 = "dummy77"; String dummy78 = "dummy78";
        String dummy79 = "dummy79"; String dummy80 = "dummy80"; String dummy81 = "dummy81";
        String dummy82 = "dummy82"; String dummy83 = "dummy83"; String dummy84 = "dummy84";
        String dummy85 = "dummy85"; String dummy86 = "dummy86"; String dummy87 = "dummy87";
        String dummy88 = "dummy88"; String dummy89 = "dummy89"; String dummy90 = "dummy90";
        String dummy91 = "dummy91"; String dummy92 = "dummy92"; String dummy93 = "dummy93";
        String dummy94 = "dummy94"; String dummy95 = "dummy95"; String dummy96 = "dummy96";
        String dummy97 = "dummy97"; String dummy98 = "dummy98"; String dummy99 = "dummy99";
        String dummy100 = "dummy100";

        String s2 = "Another LDC constant, potentially LDC_W";
        System.out.println(s2);

        // ldc2_w for Long
        long l1 = 1234567890123456789L;
        System.out.println(l1);

        // ldc2_w for Double
        double d1 = 1.2345678901234567;
        System.out.println(d1);
    }
}