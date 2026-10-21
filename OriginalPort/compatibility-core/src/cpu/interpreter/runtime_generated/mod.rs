// Generated guarded static translation of bundled runtime libraries.
use super::{InterpreterCpu, Mem, CpuState};
mod arm_c0004;
mod arm_c0005;
mod arm_c0006;
mod arm_c0007;
mod arm_c0008;
mod arm_c0009;
mod arm_c000a;
mod arm_c000b;
mod arm_c000c;
mod arm_c000d;
mod arm_c000e;
mod arm_c000f;
mod arm_c0010;
mod arm_c0011;
mod arm_c0012;
mod arm_c0013;
mod arm_c0014;
mod arm_c0015;
mod arm_c0016;
mod arm_c0017;
mod arm_c0018;
mod arm_c0019;
mod arm_c001a;
mod arm_c001b;
mod arm_c001c;
mod arm_c001d;
mod arm_c001e;
mod arm_c001f;
mod arm_c0020;
mod arm_c0021;
mod arm_c0022;
mod arm_c0023;
mod arm_dd22c;
mod arm_dd22d;
mod arm_dd22e;
mod arm_dd22f;
mod arm_dd230;
mod arm_dd231;
mod arm_dd232;
mod arm_dd233;
mod arm_dd234;
mod arm_dd235;
mod arm_dd236;
mod arm_dd237;
mod arm_dd238;
mod arm_dd239;
mod arm_dd23a;
mod arm_dd23b;
mod arm_dd23c;
mod arm_dd23d;
mod arm_dd23e;
mod arm_dd23f;
mod arm_dd240;
mod arm_dd241;
mod arm_dd242;
mod arm_dd243;
mod arm_dd244;
mod arm_dd245;
mod arm_dd246;
mod arm_dd247;
mod arm_dd248;
mod arm_dd249;
mod arm_dd24a;
mod arm_dd24b;
mod arm_dd24c;
mod arm_dd24d;
mod arm_dd24e;
mod arm_dd24f;
mod arm_dd250;
mod arm_dd251;
mod arm_dd252;
mod arm_dd253;
mod arm_dd254;
mod arm_dd255;
mod arm_dd256;
mod arm_dd257;
mod arm_dd258;
mod arm_dd259;
mod arm_dd25a;
mod arm_dd25b;
mod arm_dd25c;
mod arm_dd25d;
mod arm_dd25e;
mod arm_dd25f;
mod arm_dd260;
mod arm_dd261;
mod arm_dd262;
mod arm_dd263;
mod arm_dd264;
mod arm_dd265;
mod arm_dd266;
mod arm_dd267;
mod arm_dd268;
mod arm_dd269;
mod arm_dd26a;
mod arm_dd26b;
mod arm_dd26c;
mod arm_dd26d;
mod arm_dd26e;
mod arm_dd26f;
mod arm_dd270;
mod arm_dd271;
mod arm_dd272;
mod arm_dd273;
mod arm_dd274;
mod arm_dd275;
mod arm_dd276;
mod arm_dd277;
mod arm_dd278;
mod arm_dd279;
mod arm_dd27a;
mod arm_dd27b;
mod arm_dd27c;
mod arm_dd27d;
mod arm_dd27e;
mod arm_dd27f;
mod arm_dd280;
mod arm_dd281;
mod arm_dd282;
mod arm_dd283;
mod arm_dd284;
mod arm_dd285;
mod arm_dd286;
mod arm_dd287;
mod arm_dd288;
mod arm_dd289;
mod arm_dd28a;
mod arm_dd28b;
mod arm_dd28c;
mod arm_dd28d;
mod arm_dd28e;
mod arm_dd28f;
mod arm_dd290;
mod arm_dd291;
mod arm_dd292;
mod arm_dd293;
mod arm_dd294;
mod arm_dd295;
mod arm_dd296;
mod arm_dd297;
mod arm_dd298;
mod arm_dd299;
mod arm_dd29a;
mod arm_dd29b;
mod arm_dd29c;
mod arm_dd29d;
mod arm_dd29e;
mod arm_dd29f;
mod arm_dd2a0;
mod arm_dd2a1;
mod arm_dd2a2;
mod arm_dd2a3;
mod arm_dd2a4;
mod arm_dd2a5;
mod arm_dd2a6;
mod arm_dd2a7;
mod arm_dd2a8;
mod arm_dd2a9;
mod arm_dd2aa;
mod arm_dd2ab;
mod arm_dd2ac;
mod arm_dd2ad;
mod arm_dd2ae;
mod arm_dd2af;
mod arm_dd2b0;
mod arm_dd2b1;
mod arm_dd2b2;
mod arm_dd2b3;
mod arm_dd2b4;
mod arm_dd2b5;
mod arm_dd2b6;
mod arm_dd2b7;
mod arm_dd2b8;
mod arm_dd2b9;
mod arm_dd2ba;
mod arm_dd2bb;
mod arm_dd2bc;
mod arm_dd2bd;
mod arm_dd2be;
mod arm_dd2bf;
mod arm_dd2c0;
mod arm_dd2c1;
mod arm_dd2c2;
mod arm_dd2c3;
mod arm_dd2c4;
mod arm_dd2c5;
mod arm_dd2c6;
mod arm_dd2c7;
mod arm_dd2c8;
mod arm_dd2c9;
mod arm_dd2ca;
mod arm_dd2cb;
mod arm_dd2cc;
mod arm_dd2cd;
mod arm_dd2ce;
mod arm_dd2cf;
mod arm_dd2d0;
mod arm_dd2d1;
mod arm_dd2d2;
mod arm_dd2d3;
mod arm_dd2d4;
mod arm_dd2d5;
mod arm_dd2d6;
mod arm_dd2d7;
mod arm_dd2d8;
mod arm_dd2d9;
mod arm_dd2da;
mod arm_dd2db;
mod arm_dd2dc;
mod arm_dd2dd;
mod arm_dd2de;
mod arm_dd2df;
mod arm_dd2e0;
mod arm_dd2e1;
mod arm_dd2e2;
mod arm_dd2e3;
mod arm_dd2e4;
mod arm_dd2e5;
mod arm_dd2e6;
mod arm_dd2e7;
mod arm_dd2e8;
mod arm_dd2e9;
mod arm_dd2ea;
mod arm_dd2eb;
mod arm_dd2ec;
mod arm_dd2ed;
mod arm_dd2ee;
mod arm_dd2ef;
mod arm_dd2f0;
mod arm_dd2f1;
mod arm_dd2f2;
mod arm_dd2f3;
mod arm_dd2f4;
mod arm_dd2f5;
mod arm_dd2f6;
mod arm_dd2f7;
mod arm_dd2f8;
mod arm_dd2f9;
mod arm_dd2fa;
mod arm_dd2fb;
mod arm_dd2fc;
mod arm_dd2fd;
mod arm_dd2fe;
mod arm_dd2ff;
mod arm_dd300;
mod arm_dd301;
mod arm_dd302;
mod arm_dd303;
mod arm_dd304;
mod arm_dd305;
mod arm_dd306;
mod arm_dd307;
mod arm_dd308;
mod arm_dd309;
mod arm_dd30a;
mod arm_dd30b;
mod arm_dd30c;
mod arm_dd30d;
mod arm_dd30e;
mod arm_dd30f;
mod arm_dd310;
mod arm_dd311;
mod arm_dd312;
mod arm_dd313;
mod arm_dd314;
mod arm_dd315;
mod arm_dd316;
mod arm_dd317;
mod arm_dd318;
mod arm_dd319;
mod arm_dd31a;
mod arm_dd31b;
mod arm_dd31c;
mod arm_dd31d;
mod arm_dd31e;
mod arm_dd31f;
mod arm_dd320;
mod arm_dd321;
mod arm_dd322;
mod arm_dd323;
mod arm_dd324;
mod arm_dd325;
mod arm_dd326;
mod arm_dd327;
mod arm_dd328;
mod arm_dd329;
mod arm_dd32a;
mod arm_dd32b;
mod arm_dd32c;
mod arm_dd32d;
mod arm_dd32e;
mod arm_dd32f;
mod arm_dd330;
mod arm_dd331;
mod arm_dd332;
mod arm_dd333;
mod arm_dd334;
mod arm_dd335;
mod arm_dd336;
mod arm_dd337;
mod arm_dd338;
mod arm_dd339;
mod arm_dd33a;
mod arm_dd33b;
mod arm_dd33c;
mod arm_dd34c;
mod arm_dd34d;
mod thumb_c0004;
mod thumb_c0005;
mod thumb_c0006;
mod thumb_c0007;
mod thumb_c0008;
mod thumb_c0009;
mod thumb_c000a;
mod thumb_c000b;
mod thumb_c000c;
mod thumb_c000d;
mod thumb_c000e;
mod thumb_c000f;
mod thumb_c0010;
mod thumb_c0011;
mod thumb_c0012;
mod thumb_c0013;
mod thumb_c0014;
mod thumb_c0015;
mod thumb_c0016;
mod thumb_c0017;
mod thumb_c0018;
mod thumb_c0019;
mod thumb_c001a;
mod thumb_c001b;
mod thumb_c001c;
mod thumb_c001d;
mod thumb_c001e;
mod thumb_c001f;
mod thumb_c0020;
mod thumb_c0021;
mod thumb_c0022;
mod thumb_c0023;
mod thumb_dd22c;
mod thumb_dd22d;
mod thumb_dd22e;
mod thumb_dd22f;
mod thumb_dd230;
mod thumb_dd231;
mod thumb_dd232;
mod thumb_dd233;
mod thumb_dd234;
mod thumb_dd235;
mod thumb_dd236;
mod thumb_dd237;
mod thumb_dd238;
mod thumb_dd239;
mod thumb_dd23a;
mod thumb_dd23b;
mod thumb_dd23c;
mod thumb_dd23d;
mod thumb_dd23e;
mod thumb_dd23f;
mod thumb_dd240;
mod thumb_dd241;
mod thumb_dd242;
mod thumb_dd243;
mod thumb_dd244;
mod thumb_dd245;
mod thumb_dd246;
mod thumb_dd247;
mod thumb_dd248;
mod thumb_dd249;
mod thumb_dd24a;
mod thumb_dd24b;
mod thumb_dd24c;
mod thumb_dd24d;
mod thumb_dd24e;
mod thumb_dd24f;
mod thumb_dd250;
mod thumb_dd251;
mod thumb_dd252;
mod thumb_dd253;
mod thumb_dd254;
mod thumb_dd255;
mod thumb_dd256;
mod thumb_dd257;
mod thumb_dd258;
mod thumb_dd259;
mod thumb_dd25a;
mod thumb_dd25b;
mod thumb_dd25c;
mod thumb_dd25d;
mod thumb_dd25e;
mod thumb_dd25f;
mod thumb_dd260;
mod thumb_dd261;
mod thumb_dd262;
mod thumb_dd263;
mod thumb_dd264;
mod thumb_dd265;
mod thumb_dd266;
mod thumb_dd267;
mod thumb_dd268;
mod thumb_dd269;
mod thumb_dd26a;
mod thumb_dd26b;
mod thumb_dd26c;
mod thumb_dd26d;
mod thumb_dd26e;
mod thumb_dd26f;
mod thumb_dd270;
mod thumb_dd271;
mod thumb_dd272;
mod thumb_dd273;
mod thumb_dd274;
mod thumb_dd275;
mod thumb_dd276;
mod thumb_dd277;
mod thumb_dd278;
mod thumb_dd279;
mod thumb_dd27a;
mod thumb_dd27b;
mod thumb_dd27c;
mod thumb_dd27d;
mod thumb_dd27e;
mod thumb_dd27f;
mod thumb_dd280;
mod thumb_dd281;
mod thumb_dd282;
mod thumb_dd283;
mod thumb_dd284;
mod thumb_dd285;
mod thumb_dd286;
mod thumb_dd287;
mod thumb_dd288;
mod thumb_dd289;
mod thumb_dd28a;
mod thumb_dd28b;
mod thumb_dd28c;
mod thumb_dd28d;
mod thumb_dd28e;
mod thumb_dd28f;
mod thumb_dd290;
mod thumb_dd291;
mod thumb_dd292;
mod thumb_dd293;
mod thumb_dd294;
mod thumb_dd295;
mod thumb_dd296;
mod thumb_dd297;
mod thumb_dd298;
mod thumb_dd299;
mod thumb_dd29a;
mod thumb_dd29b;
mod thumb_dd29c;
mod thumb_dd29d;
mod thumb_dd29e;
mod thumb_dd29f;
mod thumb_dd2a0;
mod thumb_dd2a1;
mod thumb_dd2a2;
mod thumb_dd2a3;
mod thumb_dd2a4;
mod thumb_dd2a5;
mod thumb_dd2a6;
mod thumb_dd2a7;
mod thumb_dd2a8;
mod thumb_dd2a9;
mod thumb_dd2aa;
mod thumb_dd2ab;
mod thumb_dd2ac;
mod thumb_dd2ad;
mod thumb_dd2ae;
mod thumb_dd2af;
mod thumb_dd2b0;
mod thumb_dd2b1;
mod thumb_dd2b2;
mod thumb_dd2b3;
mod thumb_dd2b4;
mod thumb_dd2b5;
mod thumb_dd2b6;
mod thumb_dd2b7;
mod thumb_dd2b8;
mod thumb_dd2b9;
mod thumb_dd2ba;
mod thumb_dd2bb;
mod thumb_dd2bc;
mod thumb_dd2bd;
mod thumb_dd2be;
mod thumb_dd2bf;
mod thumb_dd2c0;
mod thumb_dd2c1;
mod thumb_dd2c2;
mod thumb_dd2c3;
mod thumb_dd2c4;
mod thumb_dd2c5;
mod thumb_dd2c6;
mod thumb_dd2c7;
mod thumb_dd2c8;
mod thumb_dd2c9;
mod thumb_dd2ca;
mod thumb_dd2cb;
mod thumb_dd2cc;
mod thumb_dd2cd;
mod thumb_dd2ce;
mod thumb_dd2cf;
mod thumb_dd2d0;
mod thumb_dd2d1;
mod thumb_dd2d2;
mod thumb_dd2d3;
mod thumb_dd2d4;
mod thumb_dd2d5;
mod thumb_dd2d6;
mod thumb_dd2d7;
mod thumb_dd2d8;
mod thumb_dd2d9;
mod thumb_dd2da;
mod thumb_dd2db;
mod thumb_dd2dc;
mod thumb_dd2dd;
mod thumb_dd2de;
mod thumb_dd2df;
mod thumb_dd2e0;
mod thumb_dd2e1;
mod thumb_dd2e2;
mod thumb_dd2e3;
mod thumb_dd2e4;
mod thumb_dd2e5;
mod thumb_dd2e6;
mod thumb_dd2e7;
mod thumb_dd2e8;
mod thumb_dd2e9;
mod thumb_dd2ea;
mod thumb_dd2eb;
mod thumb_dd2ec;
mod thumb_dd2ed;
mod thumb_dd2ee;
mod thumb_dd2ef;
mod thumb_dd2f0;
mod thumb_dd2f1;
mod thumb_dd2f2;
mod thumb_dd2f3;
mod thumb_dd2f4;
mod thumb_dd2f5;
mod thumb_dd2f6;
mod thumb_dd2f7;
mod thumb_dd2f8;
mod thumb_dd2f9;
mod thumb_dd2fa;
mod thumb_dd2fb;
mod thumb_dd2fc;
mod thumb_dd2fd;
mod thumb_dd2fe;
mod thumb_dd2ff;
mod thumb_dd300;
mod thumb_dd301;
mod thumb_dd302;
mod thumb_dd303;
mod thumb_dd304;
mod thumb_dd305;
mod thumb_dd306;
mod thumb_dd307;
mod thumb_dd308;
mod thumb_dd309;
mod thumb_dd30a;
mod thumb_dd30b;
mod thumb_dd30c;
mod thumb_dd30d;
mod thumb_dd30e;
mod thumb_dd30f;
mod thumb_dd310;
mod thumb_dd311;
mod thumb_dd312;
mod thumb_dd313;
mod thumb_dd314;
mod thumb_dd315;
mod thumb_dd316;
mod thumb_dd317;
mod thumb_dd318;
mod thumb_dd319;
mod thumb_dd31a;
mod thumb_dd31b;
mod thumb_dd31c;
mod thumb_dd31d;
mod thumb_dd31e;
mod thumb_dd31f;
mod thumb_dd320;
mod thumb_dd321;
mod thumb_dd322;
mod thumb_dd323;
mod thumb_dd324;
mod thumb_dd325;
mod thumb_dd326;
mod thumb_dd327;
mod thumb_dd328;
mod thumb_dd329;
mod thumb_dd32a;
mod thumb_dd32b;
mod thumb_dd32c;
mod thumb_dd32d;
mod thumb_dd32e;
mod thumb_dd32f;
mod thumb_dd330;
mod thumb_dd331;
mod thumb_dd332;
mod thumb_dd333;
mod thumb_dd334;
mod thumb_dd335;
mod thumb_dd336;
mod thumb_dd337;
mod thumb_dd338;
mod thumb_dd339;
mod thumb_dd33a;
mod thumb_dd33b;
mod thumb_dd33c;
mod thumb_dd34c;
mod thumb_dd34d;
impl InterpreterCpu {
    pub(super) fn runtime_native_step(&mut self, mem: &mut Mem) -> Option<CpuState> {
        let pc = self.regs[15];
        let thumb = self.is_thumb();
        let insn = if thumb {
            let hw = self.read_code_u16(mem, pc)?;
            if hw & 0xf800 >= 0xe800 {
                ((hw as u32) << 16) | self.read_code_u16(mem, pc.wrapping_add(2))? as u32
            } else { hw as u32 }
        } else { self.read_code_u32(mem, pc)? };
        match (thumb, pc >> 10) {
            (false, 0xc0004) => self.runtime_arm_c0004(mem, pc, insn),
            (false, 0xc0005) => self.runtime_arm_c0005(mem, pc, insn),
            (false, 0xc0006) => self.runtime_arm_c0006(mem, pc, insn),
            (false, 0xc0007) => self.runtime_arm_c0007(mem, pc, insn),
            (false, 0xc0008) => self.runtime_arm_c0008(mem, pc, insn),
            (false, 0xc0009) => self.runtime_arm_c0009(mem, pc, insn),
            (false, 0xc000a) => self.runtime_arm_c000a(mem, pc, insn),
            (false, 0xc000b) => self.runtime_arm_c000b(mem, pc, insn),
            (false, 0xc000c) => self.runtime_arm_c000c(mem, pc, insn),
            (false, 0xc000d) => self.runtime_arm_c000d(mem, pc, insn),
            (false, 0xc000e) => self.runtime_arm_c000e(mem, pc, insn),
            (false, 0xc000f) => self.runtime_arm_c000f(mem, pc, insn),
            (false, 0xc0010) => self.runtime_arm_c0010(mem, pc, insn),
            (false, 0xc0011) => self.runtime_arm_c0011(mem, pc, insn),
            (false, 0xc0012) => self.runtime_arm_c0012(mem, pc, insn),
            (false, 0xc0013) => self.runtime_arm_c0013(mem, pc, insn),
            (false, 0xc0014) => self.runtime_arm_c0014(mem, pc, insn),
            (false, 0xc0015) => self.runtime_arm_c0015(mem, pc, insn),
            (false, 0xc0016) => self.runtime_arm_c0016(mem, pc, insn),
            (false, 0xc0017) => self.runtime_arm_c0017(mem, pc, insn),
            (false, 0xc0018) => self.runtime_arm_c0018(mem, pc, insn),
            (false, 0xc0019) => self.runtime_arm_c0019(mem, pc, insn),
            (false, 0xc001a) => self.runtime_arm_c001a(mem, pc, insn),
            (false, 0xc001b) => self.runtime_arm_c001b(mem, pc, insn),
            (false, 0xc001c) => self.runtime_arm_c001c(mem, pc, insn),
            (false, 0xc001d) => self.runtime_arm_c001d(mem, pc, insn),
            (false, 0xc001e) => self.runtime_arm_c001e(mem, pc, insn),
            (false, 0xc001f) => self.runtime_arm_c001f(mem, pc, insn),
            (false, 0xc0020) => self.runtime_arm_c0020(mem, pc, insn),
            (false, 0xc0021) => self.runtime_arm_c0021(mem, pc, insn),
            (false, 0xc0022) => self.runtime_arm_c0022(mem, pc, insn),
            (false, 0xc0023) => self.runtime_arm_c0023(mem, pc, insn),
            (false, 0xdd22c) => self.runtime_arm_dd22c(mem, pc, insn),
            (false, 0xdd22d) => self.runtime_arm_dd22d(mem, pc, insn),
            (false, 0xdd22e) => self.runtime_arm_dd22e(mem, pc, insn),
            (false, 0xdd22f) => self.runtime_arm_dd22f(mem, pc, insn),
            (false, 0xdd230) => self.runtime_arm_dd230(mem, pc, insn),
            (false, 0xdd231) => self.runtime_arm_dd231(mem, pc, insn),
            (false, 0xdd232) => self.runtime_arm_dd232(mem, pc, insn),
            (false, 0xdd233) => self.runtime_arm_dd233(mem, pc, insn),
            (false, 0xdd234) => self.runtime_arm_dd234(mem, pc, insn),
            (false, 0xdd235) => self.runtime_arm_dd235(mem, pc, insn),
            (false, 0xdd236) => self.runtime_arm_dd236(mem, pc, insn),
            (false, 0xdd237) => self.runtime_arm_dd237(mem, pc, insn),
            (false, 0xdd238) => self.runtime_arm_dd238(mem, pc, insn),
            (false, 0xdd239) => self.runtime_arm_dd239(mem, pc, insn),
            (false, 0xdd23a) => self.runtime_arm_dd23a(mem, pc, insn),
            (false, 0xdd23b) => self.runtime_arm_dd23b(mem, pc, insn),
            (false, 0xdd23c) => self.runtime_arm_dd23c(mem, pc, insn),
            (false, 0xdd23d) => self.runtime_arm_dd23d(mem, pc, insn),
            (false, 0xdd23e) => self.runtime_arm_dd23e(mem, pc, insn),
            (false, 0xdd23f) => self.runtime_arm_dd23f(mem, pc, insn),
            (false, 0xdd240) => self.runtime_arm_dd240(mem, pc, insn),
            (false, 0xdd241) => self.runtime_arm_dd241(mem, pc, insn),
            (false, 0xdd242) => self.runtime_arm_dd242(mem, pc, insn),
            (false, 0xdd243) => self.runtime_arm_dd243(mem, pc, insn),
            (false, 0xdd244) => self.runtime_arm_dd244(mem, pc, insn),
            (false, 0xdd245) => self.runtime_arm_dd245(mem, pc, insn),
            (false, 0xdd246) => self.runtime_arm_dd246(mem, pc, insn),
            (false, 0xdd247) => self.runtime_arm_dd247(mem, pc, insn),
            (false, 0xdd248) => self.runtime_arm_dd248(mem, pc, insn),
            (false, 0xdd249) => self.runtime_arm_dd249(mem, pc, insn),
            (false, 0xdd24a) => self.runtime_arm_dd24a(mem, pc, insn),
            (false, 0xdd24b) => self.runtime_arm_dd24b(mem, pc, insn),
            (false, 0xdd24c) => self.runtime_arm_dd24c(mem, pc, insn),
            (false, 0xdd24d) => self.runtime_arm_dd24d(mem, pc, insn),
            (false, 0xdd24e) => self.runtime_arm_dd24e(mem, pc, insn),
            (false, 0xdd24f) => self.runtime_arm_dd24f(mem, pc, insn),
            (false, 0xdd250) => self.runtime_arm_dd250(mem, pc, insn),
            (false, 0xdd251) => self.runtime_arm_dd251(mem, pc, insn),
            (false, 0xdd252) => self.runtime_arm_dd252(mem, pc, insn),
            (false, 0xdd253) => self.runtime_arm_dd253(mem, pc, insn),
            (false, 0xdd254) => self.runtime_arm_dd254(mem, pc, insn),
            (false, 0xdd255) => self.runtime_arm_dd255(mem, pc, insn),
            (false, 0xdd256) => self.runtime_arm_dd256(mem, pc, insn),
            (false, 0xdd257) => self.runtime_arm_dd257(mem, pc, insn),
            (false, 0xdd258) => self.runtime_arm_dd258(mem, pc, insn),
            (false, 0xdd259) => self.runtime_arm_dd259(mem, pc, insn),
            (false, 0xdd25a) => self.runtime_arm_dd25a(mem, pc, insn),
            (false, 0xdd25b) => self.runtime_arm_dd25b(mem, pc, insn),
            (false, 0xdd25c) => self.runtime_arm_dd25c(mem, pc, insn),
            (false, 0xdd25d) => self.runtime_arm_dd25d(mem, pc, insn),
            (false, 0xdd25e) => self.runtime_arm_dd25e(mem, pc, insn),
            (false, 0xdd25f) => self.runtime_arm_dd25f(mem, pc, insn),
            (false, 0xdd260) => self.runtime_arm_dd260(mem, pc, insn),
            (false, 0xdd261) => self.runtime_arm_dd261(mem, pc, insn),
            (false, 0xdd262) => self.runtime_arm_dd262(mem, pc, insn),
            (false, 0xdd263) => self.runtime_arm_dd263(mem, pc, insn),
            (false, 0xdd264) => self.runtime_arm_dd264(mem, pc, insn),
            (false, 0xdd265) => self.runtime_arm_dd265(mem, pc, insn),
            (false, 0xdd266) => self.runtime_arm_dd266(mem, pc, insn),
            (false, 0xdd267) => self.runtime_arm_dd267(mem, pc, insn),
            (false, 0xdd268) => self.runtime_arm_dd268(mem, pc, insn),
            (false, 0xdd269) => self.runtime_arm_dd269(mem, pc, insn),
            (false, 0xdd26a) => self.runtime_arm_dd26a(mem, pc, insn),
            (false, 0xdd26b) => self.runtime_arm_dd26b(mem, pc, insn),
            (false, 0xdd26c) => self.runtime_arm_dd26c(mem, pc, insn),
            (false, 0xdd26d) => self.runtime_arm_dd26d(mem, pc, insn),
            (false, 0xdd26e) => self.runtime_arm_dd26e(mem, pc, insn),
            (false, 0xdd26f) => self.runtime_arm_dd26f(mem, pc, insn),
            (false, 0xdd270) => self.runtime_arm_dd270(mem, pc, insn),
            (false, 0xdd271) => self.runtime_arm_dd271(mem, pc, insn),
            (false, 0xdd272) => self.runtime_arm_dd272(mem, pc, insn),
            (false, 0xdd273) => self.runtime_arm_dd273(mem, pc, insn),
            (false, 0xdd274) => self.runtime_arm_dd274(mem, pc, insn),
            (false, 0xdd275) => self.runtime_arm_dd275(mem, pc, insn),
            (false, 0xdd276) => self.runtime_arm_dd276(mem, pc, insn),
            (false, 0xdd277) => self.runtime_arm_dd277(mem, pc, insn),
            (false, 0xdd278) => self.runtime_arm_dd278(mem, pc, insn),
            (false, 0xdd279) => self.runtime_arm_dd279(mem, pc, insn),
            (false, 0xdd27a) => self.runtime_arm_dd27a(mem, pc, insn),
            (false, 0xdd27b) => self.runtime_arm_dd27b(mem, pc, insn),
            (false, 0xdd27c) => self.runtime_arm_dd27c(mem, pc, insn),
            (false, 0xdd27d) => self.runtime_arm_dd27d(mem, pc, insn),
            (false, 0xdd27e) => self.runtime_arm_dd27e(mem, pc, insn),
            (false, 0xdd27f) => self.runtime_arm_dd27f(mem, pc, insn),
            (false, 0xdd280) => self.runtime_arm_dd280(mem, pc, insn),
            (false, 0xdd281) => self.runtime_arm_dd281(mem, pc, insn),
            (false, 0xdd282) => self.runtime_arm_dd282(mem, pc, insn),
            (false, 0xdd283) => self.runtime_arm_dd283(mem, pc, insn),
            (false, 0xdd284) => self.runtime_arm_dd284(mem, pc, insn),
            (false, 0xdd285) => self.runtime_arm_dd285(mem, pc, insn),
            (false, 0xdd286) => self.runtime_arm_dd286(mem, pc, insn),
            (false, 0xdd287) => self.runtime_arm_dd287(mem, pc, insn),
            (false, 0xdd288) => self.runtime_arm_dd288(mem, pc, insn),
            (false, 0xdd289) => self.runtime_arm_dd289(mem, pc, insn),
            (false, 0xdd28a) => self.runtime_arm_dd28a(mem, pc, insn),
            (false, 0xdd28b) => self.runtime_arm_dd28b(mem, pc, insn),
            (false, 0xdd28c) => self.runtime_arm_dd28c(mem, pc, insn),
            (false, 0xdd28d) => self.runtime_arm_dd28d(mem, pc, insn),
            (false, 0xdd28e) => self.runtime_arm_dd28e(mem, pc, insn),
            (false, 0xdd28f) => self.runtime_arm_dd28f(mem, pc, insn),
            (false, 0xdd290) => self.runtime_arm_dd290(mem, pc, insn),
            (false, 0xdd291) => self.runtime_arm_dd291(mem, pc, insn),
            (false, 0xdd292) => self.runtime_arm_dd292(mem, pc, insn),
            (false, 0xdd293) => self.runtime_arm_dd293(mem, pc, insn),
            (false, 0xdd294) => self.runtime_arm_dd294(mem, pc, insn),
            (false, 0xdd295) => self.runtime_arm_dd295(mem, pc, insn),
            (false, 0xdd296) => self.runtime_arm_dd296(mem, pc, insn),
            (false, 0xdd297) => self.runtime_arm_dd297(mem, pc, insn),
            (false, 0xdd298) => self.runtime_arm_dd298(mem, pc, insn),
            (false, 0xdd299) => self.runtime_arm_dd299(mem, pc, insn),
            (false, 0xdd29a) => self.runtime_arm_dd29a(mem, pc, insn),
            (false, 0xdd29b) => self.runtime_arm_dd29b(mem, pc, insn),
            (false, 0xdd29c) => self.runtime_arm_dd29c(mem, pc, insn),
            (false, 0xdd29d) => self.runtime_arm_dd29d(mem, pc, insn),
            (false, 0xdd29e) => self.runtime_arm_dd29e(mem, pc, insn),
            (false, 0xdd29f) => self.runtime_arm_dd29f(mem, pc, insn),
            (false, 0xdd2a0) => self.runtime_arm_dd2a0(mem, pc, insn),
            (false, 0xdd2a1) => self.runtime_arm_dd2a1(mem, pc, insn),
            (false, 0xdd2a2) => self.runtime_arm_dd2a2(mem, pc, insn),
            (false, 0xdd2a3) => self.runtime_arm_dd2a3(mem, pc, insn),
            (false, 0xdd2a4) => self.runtime_arm_dd2a4(mem, pc, insn),
            (false, 0xdd2a5) => self.runtime_arm_dd2a5(mem, pc, insn),
            (false, 0xdd2a6) => self.runtime_arm_dd2a6(mem, pc, insn),
            (false, 0xdd2a7) => self.runtime_arm_dd2a7(mem, pc, insn),
            (false, 0xdd2a8) => self.runtime_arm_dd2a8(mem, pc, insn),
            (false, 0xdd2a9) => self.runtime_arm_dd2a9(mem, pc, insn),
            (false, 0xdd2aa) => self.runtime_arm_dd2aa(mem, pc, insn),
            (false, 0xdd2ab) => self.runtime_arm_dd2ab(mem, pc, insn),
            (false, 0xdd2ac) => self.runtime_arm_dd2ac(mem, pc, insn),
            (false, 0xdd2ad) => self.runtime_arm_dd2ad(mem, pc, insn),
            (false, 0xdd2ae) => self.runtime_arm_dd2ae(mem, pc, insn),
            (false, 0xdd2af) => self.runtime_arm_dd2af(mem, pc, insn),
            (false, 0xdd2b0) => self.runtime_arm_dd2b0(mem, pc, insn),
            (false, 0xdd2b1) => self.runtime_arm_dd2b1(mem, pc, insn),
            (false, 0xdd2b2) => self.runtime_arm_dd2b2(mem, pc, insn),
            (false, 0xdd2b3) => self.runtime_arm_dd2b3(mem, pc, insn),
            (false, 0xdd2b4) => self.runtime_arm_dd2b4(mem, pc, insn),
            (false, 0xdd2b5) => self.runtime_arm_dd2b5(mem, pc, insn),
            (false, 0xdd2b6) => self.runtime_arm_dd2b6(mem, pc, insn),
            (false, 0xdd2b7) => self.runtime_arm_dd2b7(mem, pc, insn),
            (false, 0xdd2b8) => self.runtime_arm_dd2b8(mem, pc, insn),
            (false, 0xdd2b9) => self.runtime_arm_dd2b9(mem, pc, insn),
            (false, 0xdd2ba) => self.runtime_arm_dd2ba(mem, pc, insn),
            (false, 0xdd2bb) => self.runtime_arm_dd2bb(mem, pc, insn),
            (false, 0xdd2bc) => self.runtime_arm_dd2bc(mem, pc, insn),
            (false, 0xdd2bd) => self.runtime_arm_dd2bd(mem, pc, insn),
            (false, 0xdd2be) => self.runtime_arm_dd2be(mem, pc, insn),
            (false, 0xdd2bf) => self.runtime_arm_dd2bf(mem, pc, insn),
            (false, 0xdd2c0) => self.runtime_arm_dd2c0(mem, pc, insn),
            (false, 0xdd2c1) => self.runtime_arm_dd2c1(mem, pc, insn),
            (false, 0xdd2c2) => self.runtime_arm_dd2c2(mem, pc, insn),
            (false, 0xdd2c3) => self.runtime_arm_dd2c3(mem, pc, insn),
            (false, 0xdd2c4) => self.runtime_arm_dd2c4(mem, pc, insn),
            (false, 0xdd2c5) => self.runtime_arm_dd2c5(mem, pc, insn),
            (false, 0xdd2c6) => self.runtime_arm_dd2c6(mem, pc, insn),
            (false, 0xdd2c7) => self.runtime_arm_dd2c7(mem, pc, insn),
            (false, 0xdd2c8) => self.runtime_arm_dd2c8(mem, pc, insn),
            (false, 0xdd2c9) => self.runtime_arm_dd2c9(mem, pc, insn),
            (false, 0xdd2ca) => self.runtime_arm_dd2ca(mem, pc, insn),
            (false, 0xdd2cb) => self.runtime_arm_dd2cb(mem, pc, insn),
            (false, 0xdd2cc) => self.runtime_arm_dd2cc(mem, pc, insn),
            (false, 0xdd2cd) => self.runtime_arm_dd2cd(mem, pc, insn),
            (false, 0xdd2ce) => self.runtime_arm_dd2ce(mem, pc, insn),
            (false, 0xdd2cf) => self.runtime_arm_dd2cf(mem, pc, insn),
            (false, 0xdd2d0) => self.runtime_arm_dd2d0(mem, pc, insn),
            (false, 0xdd2d1) => self.runtime_arm_dd2d1(mem, pc, insn),
            (false, 0xdd2d2) => self.runtime_arm_dd2d2(mem, pc, insn),
            (false, 0xdd2d3) => self.runtime_arm_dd2d3(mem, pc, insn),
            (false, 0xdd2d4) => self.runtime_arm_dd2d4(mem, pc, insn),
            (false, 0xdd2d5) => self.runtime_arm_dd2d5(mem, pc, insn),
            (false, 0xdd2d6) => self.runtime_arm_dd2d6(mem, pc, insn),
            (false, 0xdd2d7) => self.runtime_arm_dd2d7(mem, pc, insn),
            (false, 0xdd2d8) => self.runtime_arm_dd2d8(mem, pc, insn),
            (false, 0xdd2d9) => self.runtime_arm_dd2d9(mem, pc, insn),
            (false, 0xdd2da) => self.runtime_arm_dd2da(mem, pc, insn),
            (false, 0xdd2db) => self.runtime_arm_dd2db(mem, pc, insn),
            (false, 0xdd2dc) => self.runtime_arm_dd2dc(mem, pc, insn),
            (false, 0xdd2dd) => self.runtime_arm_dd2dd(mem, pc, insn),
            (false, 0xdd2de) => self.runtime_arm_dd2de(mem, pc, insn),
            (false, 0xdd2df) => self.runtime_arm_dd2df(mem, pc, insn),
            (false, 0xdd2e0) => self.runtime_arm_dd2e0(mem, pc, insn),
            (false, 0xdd2e1) => self.runtime_arm_dd2e1(mem, pc, insn),
            (false, 0xdd2e2) => self.runtime_arm_dd2e2(mem, pc, insn),
            (false, 0xdd2e3) => self.runtime_arm_dd2e3(mem, pc, insn),
            (false, 0xdd2e4) => self.runtime_arm_dd2e4(mem, pc, insn),
            (false, 0xdd2e5) => self.runtime_arm_dd2e5(mem, pc, insn),
            (false, 0xdd2e6) => self.runtime_arm_dd2e6(mem, pc, insn),
            (false, 0xdd2e7) => self.runtime_arm_dd2e7(mem, pc, insn),
            (false, 0xdd2e8) => self.runtime_arm_dd2e8(mem, pc, insn),
            (false, 0xdd2e9) => self.runtime_arm_dd2e9(mem, pc, insn),
            (false, 0xdd2ea) => self.runtime_arm_dd2ea(mem, pc, insn),
            (false, 0xdd2eb) => self.runtime_arm_dd2eb(mem, pc, insn),
            (false, 0xdd2ec) => self.runtime_arm_dd2ec(mem, pc, insn),
            (false, 0xdd2ed) => self.runtime_arm_dd2ed(mem, pc, insn),
            (false, 0xdd2ee) => self.runtime_arm_dd2ee(mem, pc, insn),
            (false, 0xdd2ef) => self.runtime_arm_dd2ef(mem, pc, insn),
            (false, 0xdd2f0) => self.runtime_arm_dd2f0(mem, pc, insn),
            (false, 0xdd2f1) => self.runtime_arm_dd2f1(mem, pc, insn),
            (false, 0xdd2f2) => self.runtime_arm_dd2f2(mem, pc, insn),
            (false, 0xdd2f3) => self.runtime_arm_dd2f3(mem, pc, insn),
            (false, 0xdd2f4) => self.runtime_arm_dd2f4(mem, pc, insn),
            (false, 0xdd2f5) => self.runtime_arm_dd2f5(mem, pc, insn),
            (false, 0xdd2f6) => self.runtime_arm_dd2f6(mem, pc, insn),
            (false, 0xdd2f7) => self.runtime_arm_dd2f7(mem, pc, insn),
            (false, 0xdd2f8) => self.runtime_arm_dd2f8(mem, pc, insn),
            (false, 0xdd2f9) => self.runtime_arm_dd2f9(mem, pc, insn),
            (false, 0xdd2fa) => self.runtime_arm_dd2fa(mem, pc, insn),
            (false, 0xdd2fb) => self.runtime_arm_dd2fb(mem, pc, insn),
            (false, 0xdd2fc) => self.runtime_arm_dd2fc(mem, pc, insn),
            (false, 0xdd2fd) => self.runtime_arm_dd2fd(mem, pc, insn),
            (false, 0xdd2fe) => self.runtime_arm_dd2fe(mem, pc, insn),
            (false, 0xdd2ff) => self.runtime_arm_dd2ff(mem, pc, insn),
            (false, 0xdd300) => self.runtime_arm_dd300(mem, pc, insn),
            (false, 0xdd301) => self.runtime_arm_dd301(mem, pc, insn),
            (false, 0xdd302) => self.runtime_arm_dd302(mem, pc, insn),
            (false, 0xdd303) => self.runtime_arm_dd303(mem, pc, insn),
            (false, 0xdd304) => self.runtime_arm_dd304(mem, pc, insn),
            (false, 0xdd305) => self.runtime_arm_dd305(mem, pc, insn),
            (false, 0xdd306) => self.runtime_arm_dd306(mem, pc, insn),
            (false, 0xdd307) => self.runtime_arm_dd307(mem, pc, insn),
            (false, 0xdd308) => self.runtime_arm_dd308(mem, pc, insn),
            (false, 0xdd309) => self.runtime_arm_dd309(mem, pc, insn),
            (false, 0xdd30a) => self.runtime_arm_dd30a(mem, pc, insn),
            (false, 0xdd30b) => self.runtime_arm_dd30b(mem, pc, insn),
            (false, 0xdd30c) => self.runtime_arm_dd30c(mem, pc, insn),
            (false, 0xdd30d) => self.runtime_arm_dd30d(mem, pc, insn),
            (false, 0xdd30e) => self.runtime_arm_dd30e(mem, pc, insn),
            (false, 0xdd30f) => self.runtime_arm_dd30f(mem, pc, insn),
            (false, 0xdd310) => self.runtime_arm_dd310(mem, pc, insn),
            (false, 0xdd311) => self.runtime_arm_dd311(mem, pc, insn),
            (false, 0xdd312) => self.runtime_arm_dd312(mem, pc, insn),
            (false, 0xdd313) => self.runtime_arm_dd313(mem, pc, insn),
            (false, 0xdd314) => self.runtime_arm_dd314(mem, pc, insn),
            (false, 0xdd315) => self.runtime_arm_dd315(mem, pc, insn),
            (false, 0xdd316) => self.runtime_arm_dd316(mem, pc, insn),
            (false, 0xdd317) => self.runtime_arm_dd317(mem, pc, insn),
            (false, 0xdd318) => self.runtime_arm_dd318(mem, pc, insn),
            (false, 0xdd319) => self.runtime_arm_dd319(mem, pc, insn),
            (false, 0xdd31a) => self.runtime_arm_dd31a(mem, pc, insn),
            (false, 0xdd31b) => self.runtime_arm_dd31b(mem, pc, insn),
            (false, 0xdd31c) => self.runtime_arm_dd31c(mem, pc, insn),
            (false, 0xdd31d) => self.runtime_arm_dd31d(mem, pc, insn),
            (false, 0xdd31e) => self.runtime_arm_dd31e(mem, pc, insn),
            (false, 0xdd31f) => self.runtime_arm_dd31f(mem, pc, insn),
            (false, 0xdd320) => self.runtime_arm_dd320(mem, pc, insn),
            (false, 0xdd321) => self.runtime_arm_dd321(mem, pc, insn),
            (false, 0xdd322) => self.runtime_arm_dd322(mem, pc, insn),
            (false, 0xdd323) => self.runtime_arm_dd323(mem, pc, insn),
            (false, 0xdd324) => self.runtime_arm_dd324(mem, pc, insn),
            (false, 0xdd325) => self.runtime_arm_dd325(mem, pc, insn),
            (false, 0xdd326) => self.runtime_arm_dd326(mem, pc, insn),
            (false, 0xdd327) => self.runtime_arm_dd327(mem, pc, insn),
            (false, 0xdd328) => self.runtime_arm_dd328(mem, pc, insn),
            (false, 0xdd329) => self.runtime_arm_dd329(mem, pc, insn),
            (false, 0xdd32a) => self.runtime_arm_dd32a(mem, pc, insn),
            (false, 0xdd32b) => self.runtime_arm_dd32b(mem, pc, insn),
            (false, 0xdd32c) => self.runtime_arm_dd32c(mem, pc, insn),
            (false, 0xdd32d) => self.runtime_arm_dd32d(mem, pc, insn),
            (false, 0xdd32e) => self.runtime_arm_dd32e(mem, pc, insn),
            (false, 0xdd32f) => self.runtime_arm_dd32f(mem, pc, insn),
            (false, 0xdd330) => self.runtime_arm_dd330(mem, pc, insn),
            (false, 0xdd331) => self.runtime_arm_dd331(mem, pc, insn),
            (false, 0xdd332) => self.runtime_arm_dd332(mem, pc, insn),
            (false, 0xdd333) => self.runtime_arm_dd333(mem, pc, insn),
            (false, 0xdd334) => self.runtime_arm_dd334(mem, pc, insn),
            (false, 0xdd335) => self.runtime_arm_dd335(mem, pc, insn),
            (false, 0xdd336) => self.runtime_arm_dd336(mem, pc, insn),
            (false, 0xdd337) => self.runtime_arm_dd337(mem, pc, insn),
            (false, 0xdd338) => self.runtime_arm_dd338(mem, pc, insn),
            (false, 0xdd339) => self.runtime_arm_dd339(mem, pc, insn),
            (false, 0xdd33a) => self.runtime_arm_dd33a(mem, pc, insn),
            (false, 0xdd33b) => self.runtime_arm_dd33b(mem, pc, insn),
            (false, 0xdd33c) => self.runtime_arm_dd33c(mem, pc, insn),
            (false, 0xdd34c) => self.runtime_arm_dd34c(mem, pc, insn),
            (false, 0xdd34d) => self.runtime_arm_dd34d(mem, pc, insn),
            (true, 0xc0004) => self.runtime_thumb_c0004(mem, pc, insn),
            (true, 0xc0005) => self.runtime_thumb_c0005(mem, pc, insn),
            (true, 0xc0006) => self.runtime_thumb_c0006(mem, pc, insn),
            (true, 0xc0007) => self.runtime_thumb_c0007(mem, pc, insn),
            (true, 0xc0008) => self.runtime_thumb_c0008(mem, pc, insn),
            (true, 0xc0009) => self.runtime_thumb_c0009(mem, pc, insn),
            (true, 0xc000a) => self.runtime_thumb_c000a(mem, pc, insn),
            (true, 0xc000b) => self.runtime_thumb_c000b(mem, pc, insn),
            (true, 0xc000c) => self.runtime_thumb_c000c(mem, pc, insn),
            (true, 0xc000d) => self.runtime_thumb_c000d(mem, pc, insn),
            (true, 0xc000e) => self.runtime_thumb_c000e(mem, pc, insn),
            (true, 0xc000f) => self.runtime_thumb_c000f(mem, pc, insn),
            (true, 0xc0010) => self.runtime_thumb_c0010(mem, pc, insn),
            (true, 0xc0011) => self.runtime_thumb_c0011(mem, pc, insn),
            (true, 0xc0012) => self.runtime_thumb_c0012(mem, pc, insn),
            (true, 0xc0013) => self.runtime_thumb_c0013(mem, pc, insn),
            (true, 0xc0014) => self.runtime_thumb_c0014(mem, pc, insn),
            (true, 0xc0015) => self.runtime_thumb_c0015(mem, pc, insn),
            (true, 0xc0016) => self.runtime_thumb_c0016(mem, pc, insn),
            (true, 0xc0017) => self.runtime_thumb_c0017(mem, pc, insn),
            (true, 0xc0018) => self.runtime_thumb_c0018(mem, pc, insn),
            (true, 0xc0019) => self.runtime_thumb_c0019(mem, pc, insn),
            (true, 0xc001a) => self.runtime_thumb_c001a(mem, pc, insn),
            (true, 0xc001b) => self.runtime_thumb_c001b(mem, pc, insn),
            (true, 0xc001c) => self.runtime_thumb_c001c(mem, pc, insn),
            (true, 0xc001d) => self.runtime_thumb_c001d(mem, pc, insn),
            (true, 0xc001e) => self.runtime_thumb_c001e(mem, pc, insn),
            (true, 0xc001f) => self.runtime_thumb_c001f(mem, pc, insn),
            (true, 0xc0020) => self.runtime_thumb_c0020(mem, pc, insn),
            (true, 0xc0021) => self.runtime_thumb_c0021(mem, pc, insn),
            (true, 0xc0022) => self.runtime_thumb_c0022(mem, pc, insn),
            (true, 0xc0023) => self.runtime_thumb_c0023(mem, pc, insn),
            (true, 0xdd22c) => self.runtime_thumb_dd22c(mem, pc, insn),
            (true, 0xdd22d) => self.runtime_thumb_dd22d(mem, pc, insn),
            (true, 0xdd22e) => self.runtime_thumb_dd22e(mem, pc, insn),
            (true, 0xdd22f) => self.runtime_thumb_dd22f(mem, pc, insn),
            (true, 0xdd230) => self.runtime_thumb_dd230(mem, pc, insn),
            (true, 0xdd231) => self.runtime_thumb_dd231(mem, pc, insn),
            (true, 0xdd232) => self.runtime_thumb_dd232(mem, pc, insn),
            (true, 0xdd233) => self.runtime_thumb_dd233(mem, pc, insn),
            (true, 0xdd234) => self.runtime_thumb_dd234(mem, pc, insn),
            (true, 0xdd235) => self.runtime_thumb_dd235(mem, pc, insn),
            (true, 0xdd236) => self.runtime_thumb_dd236(mem, pc, insn),
            (true, 0xdd237) => self.runtime_thumb_dd237(mem, pc, insn),
            (true, 0xdd238) => self.runtime_thumb_dd238(mem, pc, insn),
            (true, 0xdd239) => self.runtime_thumb_dd239(mem, pc, insn),
            (true, 0xdd23a) => self.runtime_thumb_dd23a(mem, pc, insn),
            (true, 0xdd23b) => self.runtime_thumb_dd23b(mem, pc, insn),
            (true, 0xdd23c) => self.runtime_thumb_dd23c(mem, pc, insn),
            (true, 0xdd23d) => self.runtime_thumb_dd23d(mem, pc, insn),
            (true, 0xdd23e) => self.runtime_thumb_dd23e(mem, pc, insn),
            (true, 0xdd23f) => self.runtime_thumb_dd23f(mem, pc, insn),
            (true, 0xdd240) => self.runtime_thumb_dd240(mem, pc, insn),
            (true, 0xdd241) => self.runtime_thumb_dd241(mem, pc, insn),
            (true, 0xdd242) => self.runtime_thumb_dd242(mem, pc, insn),
            (true, 0xdd243) => self.runtime_thumb_dd243(mem, pc, insn),
            (true, 0xdd244) => self.runtime_thumb_dd244(mem, pc, insn),
            (true, 0xdd245) => self.runtime_thumb_dd245(mem, pc, insn),
            (true, 0xdd246) => self.runtime_thumb_dd246(mem, pc, insn),
            (true, 0xdd247) => self.runtime_thumb_dd247(mem, pc, insn),
            (true, 0xdd248) => self.runtime_thumb_dd248(mem, pc, insn),
            (true, 0xdd249) => self.runtime_thumb_dd249(mem, pc, insn),
            (true, 0xdd24a) => self.runtime_thumb_dd24a(mem, pc, insn),
            (true, 0xdd24b) => self.runtime_thumb_dd24b(mem, pc, insn),
            (true, 0xdd24c) => self.runtime_thumb_dd24c(mem, pc, insn),
            (true, 0xdd24d) => self.runtime_thumb_dd24d(mem, pc, insn),
            (true, 0xdd24e) => self.runtime_thumb_dd24e(mem, pc, insn),
            (true, 0xdd24f) => self.runtime_thumb_dd24f(mem, pc, insn),
            (true, 0xdd250) => self.runtime_thumb_dd250(mem, pc, insn),
            (true, 0xdd251) => self.runtime_thumb_dd251(mem, pc, insn),
            (true, 0xdd252) => self.runtime_thumb_dd252(mem, pc, insn),
            (true, 0xdd253) => self.runtime_thumb_dd253(mem, pc, insn),
            (true, 0xdd254) => self.runtime_thumb_dd254(mem, pc, insn),
            (true, 0xdd255) => self.runtime_thumb_dd255(mem, pc, insn),
            (true, 0xdd256) => self.runtime_thumb_dd256(mem, pc, insn),
            (true, 0xdd257) => self.runtime_thumb_dd257(mem, pc, insn),
            (true, 0xdd258) => self.runtime_thumb_dd258(mem, pc, insn),
            (true, 0xdd259) => self.runtime_thumb_dd259(mem, pc, insn),
            (true, 0xdd25a) => self.runtime_thumb_dd25a(mem, pc, insn),
            (true, 0xdd25b) => self.runtime_thumb_dd25b(mem, pc, insn),
            (true, 0xdd25c) => self.runtime_thumb_dd25c(mem, pc, insn),
            (true, 0xdd25d) => self.runtime_thumb_dd25d(mem, pc, insn),
            (true, 0xdd25e) => self.runtime_thumb_dd25e(mem, pc, insn),
            (true, 0xdd25f) => self.runtime_thumb_dd25f(mem, pc, insn),
            (true, 0xdd260) => self.runtime_thumb_dd260(mem, pc, insn),
            (true, 0xdd261) => self.runtime_thumb_dd261(mem, pc, insn),
            (true, 0xdd262) => self.runtime_thumb_dd262(mem, pc, insn),
            (true, 0xdd263) => self.runtime_thumb_dd263(mem, pc, insn),
            (true, 0xdd264) => self.runtime_thumb_dd264(mem, pc, insn),
            (true, 0xdd265) => self.runtime_thumb_dd265(mem, pc, insn),
            (true, 0xdd266) => self.runtime_thumb_dd266(mem, pc, insn),
            (true, 0xdd267) => self.runtime_thumb_dd267(mem, pc, insn),
            (true, 0xdd268) => self.runtime_thumb_dd268(mem, pc, insn),
            (true, 0xdd269) => self.runtime_thumb_dd269(mem, pc, insn),
            (true, 0xdd26a) => self.runtime_thumb_dd26a(mem, pc, insn),
            (true, 0xdd26b) => self.runtime_thumb_dd26b(mem, pc, insn),
            (true, 0xdd26c) => self.runtime_thumb_dd26c(mem, pc, insn),
            (true, 0xdd26d) => self.runtime_thumb_dd26d(mem, pc, insn),
            (true, 0xdd26e) => self.runtime_thumb_dd26e(mem, pc, insn),
            (true, 0xdd26f) => self.runtime_thumb_dd26f(mem, pc, insn),
            (true, 0xdd270) => self.runtime_thumb_dd270(mem, pc, insn),
            (true, 0xdd271) => self.runtime_thumb_dd271(mem, pc, insn),
            (true, 0xdd272) => self.runtime_thumb_dd272(mem, pc, insn),
            (true, 0xdd273) => self.runtime_thumb_dd273(mem, pc, insn),
            (true, 0xdd274) => self.runtime_thumb_dd274(mem, pc, insn),
            (true, 0xdd275) => self.runtime_thumb_dd275(mem, pc, insn),
            (true, 0xdd276) => self.runtime_thumb_dd276(mem, pc, insn),
            (true, 0xdd277) => self.runtime_thumb_dd277(mem, pc, insn),
            (true, 0xdd278) => self.runtime_thumb_dd278(mem, pc, insn),
            (true, 0xdd279) => self.runtime_thumb_dd279(mem, pc, insn),
            (true, 0xdd27a) => self.runtime_thumb_dd27a(mem, pc, insn),
            (true, 0xdd27b) => self.runtime_thumb_dd27b(mem, pc, insn),
            (true, 0xdd27c) => self.runtime_thumb_dd27c(mem, pc, insn),
            (true, 0xdd27d) => self.runtime_thumb_dd27d(mem, pc, insn),
            (true, 0xdd27e) => self.runtime_thumb_dd27e(mem, pc, insn),
            (true, 0xdd27f) => self.runtime_thumb_dd27f(mem, pc, insn),
            (true, 0xdd280) => self.runtime_thumb_dd280(mem, pc, insn),
            (true, 0xdd281) => self.runtime_thumb_dd281(mem, pc, insn),
            (true, 0xdd282) => self.runtime_thumb_dd282(mem, pc, insn),
            (true, 0xdd283) => self.runtime_thumb_dd283(mem, pc, insn),
            (true, 0xdd284) => self.runtime_thumb_dd284(mem, pc, insn),
            (true, 0xdd285) => self.runtime_thumb_dd285(mem, pc, insn),
            (true, 0xdd286) => self.runtime_thumb_dd286(mem, pc, insn),
            (true, 0xdd287) => self.runtime_thumb_dd287(mem, pc, insn),
            (true, 0xdd288) => self.runtime_thumb_dd288(mem, pc, insn),
            (true, 0xdd289) => self.runtime_thumb_dd289(mem, pc, insn),
            (true, 0xdd28a) => self.runtime_thumb_dd28a(mem, pc, insn),
            (true, 0xdd28b) => self.runtime_thumb_dd28b(mem, pc, insn),
            (true, 0xdd28c) => self.runtime_thumb_dd28c(mem, pc, insn),
            (true, 0xdd28d) => self.runtime_thumb_dd28d(mem, pc, insn),
            (true, 0xdd28e) => self.runtime_thumb_dd28e(mem, pc, insn),
            (true, 0xdd28f) => self.runtime_thumb_dd28f(mem, pc, insn),
            (true, 0xdd290) => self.runtime_thumb_dd290(mem, pc, insn),
            (true, 0xdd291) => self.runtime_thumb_dd291(mem, pc, insn),
            (true, 0xdd292) => self.runtime_thumb_dd292(mem, pc, insn),
            (true, 0xdd293) => self.runtime_thumb_dd293(mem, pc, insn),
            (true, 0xdd294) => self.runtime_thumb_dd294(mem, pc, insn),
            (true, 0xdd295) => self.runtime_thumb_dd295(mem, pc, insn),
            (true, 0xdd296) => self.runtime_thumb_dd296(mem, pc, insn),
            (true, 0xdd297) => self.runtime_thumb_dd297(mem, pc, insn),
            (true, 0xdd298) => self.runtime_thumb_dd298(mem, pc, insn),
            (true, 0xdd299) => self.runtime_thumb_dd299(mem, pc, insn),
            (true, 0xdd29a) => self.runtime_thumb_dd29a(mem, pc, insn),
            (true, 0xdd29b) => self.runtime_thumb_dd29b(mem, pc, insn),
            (true, 0xdd29c) => self.runtime_thumb_dd29c(mem, pc, insn),
            (true, 0xdd29d) => self.runtime_thumb_dd29d(mem, pc, insn),
            (true, 0xdd29e) => self.runtime_thumb_dd29e(mem, pc, insn),
            (true, 0xdd29f) => self.runtime_thumb_dd29f(mem, pc, insn),
            (true, 0xdd2a0) => self.runtime_thumb_dd2a0(mem, pc, insn),
            (true, 0xdd2a1) => self.runtime_thumb_dd2a1(mem, pc, insn),
            (true, 0xdd2a2) => self.runtime_thumb_dd2a2(mem, pc, insn),
            (true, 0xdd2a3) => self.runtime_thumb_dd2a3(mem, pc, insn),
            (true, 0xdd2a4) => self.runtime_thumb_dd2a4(mem, pc, insn),
            (true, 0xdd2a5) => self.runtime_thumb_dd2a5(mem, pc, insn),
            (true, 0xdd2a6) => self.runtime_thumb_dd2a6(mem, pc, insn),
            (true, 0xdd2a7) => self.runtime_thumb_dd2a7(mem, pc, insn),
            (true, 0xdd2a8) => self.runtime_thumb_dd2a8(mem, pc, insn),
            (true, 0xdd2a9) => self.runtime_thumb_dd2a9(mem, pc, insn),
            (true, 0xdd2aa) => self.runtime_thumb_dd2aa(mem, pc, insn),
            (true, 0xdd2ab) => self.runtime_thumb_dd2ab(mem, pc, insn),
            (true, 0xdd2ac) => self.runtime_thumb_dd2ac(mem, pc, insn),
            (true, 0xdd2ad) => self.runtime_thumb_dd2ad(mem, pc, insn),
            (true, 0xdd2ae) => self.runtime_thumb_dd2ae(mem, pc, insn),
            (true, 0xdd2af) => self.runtime_thumb_dd2af(mem, pc, insn),
            (true, 0xdd2b0) => self.runtime_thumb_dd2b0(mem, pc, insn),
            (true, 0xdd2b1) => self.runtime_thumb_dd2b1(mem, pc, insn),
            (true, 0xdd2b2) => self.runtime_thumb_dd2b2(mem, pc, insn),
            (true, 0xdd2b3) => self.runtime_thumb_dd2b3(mem, pc, insn),
            (true, 0xdd2b4) => self.runtime_thumb_dd2b4(mem, pc, insn),
            (true, 0xdd2b5) => self.runtime_thumb_dd2b5(mem, pc, insn),
            (true, 0xdd2b6) => self.runtime_thumb_dd2b6(mem, pc, insn),
            (true, 0xdd2b7) => self.runtime_thumb_dd2b7(mem, pc, insn),
            (true, 0xdd2b8) => self.runtime_thumb_dd2b8(mem, pc, insn),
            (true, 0xdd2b9) => self.runtime_thumb_dd2b9(mem, pc, insn),
            (true, 0xdd2ba) => self.runtime_thumb_dd2ba(mem, pc, insn),
            (true, 0xdd2bb) => self.runtime_thumb_dd2bb(mem, pc, insn),
            (true, 0xdd2bc) => self.runtime_thumb_dd2bc(mem, pc, insn),
            (true, 0xdd2bd) => self.runtime_thumb_dd2bd(mem, pc, insn),
            (true, 0xdd2be) => self.runtime_thumb_dd2be(mem, pc, insn),
            (true, 0xdd2bf) => self.runtime_thumb_dd2bf(mem, pc, insn),
            (true, 0xdd2c0) => self.runtime_thumb_dd2c0(mem, pc, insn),
            (true, 0xdd2c1) => self.runtime_thumb_dd2c1(mem, pc, insn),
            (true, 0xdd2c2) => self.runtime_thumb_dd2c2(mem, pc, insn),
            (true, 0xdd2c3) => self.runtime_thumb_dd2c3(mem, pc, insn),
            (true, 0xdd2c4) => self.runtime_thumb_dd2c4(mem, pc, insn),
            (true, 0xdd2c5) => self.runtime_thumb_dd2c5(mem, pc, insn),
            (true, 0xdd2c6) => self.runtime_thumb_dd2c6(mem, pc, insn),
            (true, 0xdd2c7) => self.runtime_thumb_dd2c7(mem, pc, insn),
            (true, 0xdd2c8) => self.runtime_thumb_dd2c8(mem, pc, insn),
            (true, 0xdd2c9) => self.runtime_thumb_dd2c9(mem, pc, insn),
            (true, 0xdd2ca) => self.runtime_thumb_dd2ca(mem, pc, insn),
            (true, 0xdd2cb) => self.runtime_thumb_dd2cb(mem, pc, insn),
            (true, 0xdd2cc) => self.runtime_thumb_dd2cc(mem, pc, insn),
            (true, 0xdd2cd) => self.runtime_thumb_dd2cd(mem, pc, insn),
            (true, 0xdd2ce) => self.runtime_thumb_dd2ce(mem, pc, insn),
            (true, 0xdd2cf) => self.runtime_thumb_dd2cf(mem, pc, insn),
            (true, 0xdd2d0) => self.runtime_thumb_dd2d0(mem, pc, insn),
            (true, 0xdd2d1) => self.runtime_thumb_dd2d1(mem, pc, insn),
            (true, 0xdd2d2) => self.runtime_thumb_dd2d2(mem, pc, insn),
            (true, 0xdd2d3) => self.runtime_thumb_dd2d3(mem, pc, insn),
            (true, 0xdd2d4) => self.runtime_thumb_dd2d4(mem, pc, insn),
            (true, 0xdd2d5) => self.runtime_thumb_dd2d5(mem, pc, insn),
            (true, 0xdd2d6) => self.runtime_thumb_dd2d6(mem, pc, insn),
            (true, 0xdd2d7) => self.runtime_thumb_dd2d7(mem, pc, insn),
            (true, 0xdd2d8) => self.runtime_thumb_dd2d8(mem, pc, insn),
            (true, 0xdd2d9) => self.runtime_thumb_dd2d9(mem, pc, insn),
            (true, 0xdd2da) => self.runtime_thumb_dd2da(mem, pc, insn),
            (true, 0xdd2db) => self.runtime_thumb_dd2db(mem, pc, insn),
            (true, 0xdd2dc) => self.runtime_thumb_dd2dc(mem, pc, insn),
            (true, 0xdd2dd) => self.runtime_thumb_dd2dd(mem, pc, insn),
            (true, 0xdd2de) => self.runtime_thumb_dd2de(mem, pc, insn),
            (true, 0xdd2df) => self.runtime_thumb_dd2df(mem, pc, insn),
            (true, 0xdd2e0) => self.runtime_thumb_dd2e0(mem, pc, insn),
            (true, 0xdd2e1) => self.runtime_thumb_dd2e1(mem, pc, insn),
            (true, 0xdd2e2) => self.runtime_thumb_dd2e2(mem, pc, insn),
            (true, 0xdd2e3) => self.runtime_thumb_dd2e3(mem, pc, insn),
            (true, 0xdd2e4) => self.runtime_thumb_dd2e4(mem, pc, insn),
            (true, 0xdd2e5) => self.runtime_thumb_dd2e5(mem, pc, insn),
            (true, 0xdd2e6) => self.runtime_thumb_dd2e6(mem, pc, insn),
            (true, 0xdd2e7) => self.runtime_thumb_dd2e7(mem, pc, insn),
            (true, 0xdd2e8) => self.runtime_thumb_dd2e8(mem, pc, insn),
            (true, 0xdd2e9) => self.runtime_thumb_dd2e9(mem, pc, insn),
            (true, 0xdd2ea) => self.runtime_thumb_dd2ea(mem, pc, insn),
            (true, 0xdd2eb) => self.runtime_thumb_dd2eb(mem, pc, insn),
            (true, 0xdd2ec) => self.runtime_thumb_dd2ec(mem, pc, insn),
            (true, 0xdd2ed) => self.runtime_thumb_dd2ed(mem, pc, insn),
            (true, 0xdd2ee) => self.runtime_thumb_dd2ee(mem, pc, insn),
            (true, 0xdd2ef) => self.runtime_thumb_dd2ef(mem, pc, insn),
            (true, 0xdd2f0) => self.runtime_thumb_dd2f0(mem, pc, insn),
            (true, 0xdd2f1) => self.runtime_thumb_dd2f1(mem, pc, insn),
            (true, 0xdd2f2) => self.runtime_thumb_dd2f2(mem, pc, insn),
            (true, 0xdd2f3) => self.runtime_thumb_dd2f3(mem, pc, insn),
            (true, 0xdd2f4) => self.runtime_thumb_dd2f4(mem, pc, insn),
            (true, 0xdd2f5) => self.runtime_thumb_dd2f5(mem, pc, insn),
            (true, 0xdd2f6) => self.runtime_thumb_dd2f6(mem, pc, insn),
            (true, 0xdd2f7) => self.runtime_thumb_dd2f7(mem, pc, insn),
            (true, 0xdd2f8) => self.runtime_thumb_dd2f8(mem, pc, insn),
            (true, 0xdd2f9) => self.runtime_thumb_dd2f9(mem, pc, insn),
            (true, 0xdd2fa) => self.runtime_thumb_dd2fa(mem, pc, insn),
            (true, 0xdd2fb) => self.runtime_thumb_dd2fb(mem, pc, insn),
            (true, 0xdd2fc) => self.runtime_thumb_dd2fc(mem, pc, insn),
            (true, 0xdd2fd) => self.runtime_thumb_dd2fd(mem, pc, insn),
            (true, 0xdd2fe) => self.runtime_thumb_dd2fe(mem, pc, insn),
            (true, 0xdd2ff) => self.runtime_thumb_dd2ff(mem, pc, insn),
            (true, 0xdd300) => self.runtime_thumb_dd300(mem, pc, insn),
            (true, 0xdd301) => self.runtime_thumb_dd301(mem, pc, insn),
            (true, 0xdd302) => self.runtime_thumb_dd302(mem, pc, insn),
            (true, 0xdd303) => self.runtime_thumb_dd303(mem, pc, insn),
            (true, 0xdd304) => self.runtime_thumb_dd304(mem, pc, insn),
            (true, 0xdd305) => self.runtime_thumb_dd305(mem, pc, insn),
            (true, 0xdd306) => self.runtime_thumb_dd306(mem, pc, insn),
            (true, 0xdd307) => self.runtime_thumb_dd307(mem, pc, insn),
            (true, 0xdd308) => self.runtime_thumb_dd308(mem, pc, insn),
            (true, 0xdd309) => self.runtime_thumb_dd309(mem, pc, insn),
            (true, 0xdd30a) => self.runtime_thumb_dd30a(mem, pc, insn),
            (true, 0xdd30b) => self.runtime_thumb_dd30b(mem, pc, insn),
            (true, 0xdd30c) => self.runtime_thumb_dd30c(mem, pc, insn),
            (true, 0xdd30d) => self.runtime_thumb_dd30d(mem, pc, insn),
            (true, 0xdd30e) => self.runtime_thumb_dd30e(mem, pc, insn),
            (true, 0xdd30f) => self.runtime_thumb_dd30f(mem, pc, insn),
            (true, 0xdd310) => self.runtime_thumb_dd310(mem, pc, insn),
            (true, 0xdd311) => self.runtime_thumb_dd311(mem, pc, insn),
            (true, 0xdd312) => self.runtime_thumb_dd312(mem, pc, insn),
            (true, 0xdd313) => self.runtime_thumb_dd313(mem, pc, insn),
            (true, 0xdd314) => self.runtime_thumb_dd314(mem, pc, insn),
            (true, 0xdd315) => self.runtime_thumb_dd315(mem, pc, insn),
            (true, 0xdd316) => self.runtime_thumb_dd316(mem, pc, insn),
            (true, 0xdd317) => self.runtime_thumb_dd317(mem, pc, insn),
            (true, 0xdd318) => self.runtime_thumb_dd318(mem, pc, insn),
            (true, 0xdd319) => self.runtime_thumb_dd319(mem, pc, insn),
            (true, 0xdd31a) => self.runtime_thumb_dd31a(mem, pc, insn),
            (true, 0xdd31b) => self.runtime_thumb_dd31b(mem, pc, insn),
            (true, 0xdd31c) => self.runtime_thumb_dd31c(mem, pc, insn),
            (true, 0xdd31d) => self.runtime_thumb_dd31d(mem, pc, insn),
            (true, 0xdd31e) => self.runtime_thumb_dd31e(mem, pc, insn),
            (true, 0xdd31f) => self.runtime_thumb_dd31f(mem, pc, insn),
            (true, 0xdd320) => self.runtime_thumb_dd320(mem, pc, insn),
            (true, 0xdd321) => self.runtime_thumb_dd321(mem, pc, insn),
            (true, 0xdd322) => self.runtime_thumb_dd322(mem, pc, insn),
            (true, 0xdd323) => self.runtime_thumb_dd323(mem, pc, insn),
            (true, 0xdd324) => self.runtime_thumb_dd324(mem, pc, insn),
            (true, 0xdd325) => self.runtime_thumb_dd325(mem, pc, insn),
            (true, 0xdd326) => self.runtime_thumb_dd326(mem, pc, insn),
            (true, 0xdd327) => self.runtime_thumb_dd327(mem, pc, insn),
            (true, 0xdd328) => self.runtime_thumb_dd328(mem, pc, insn),
            (true, 0xdd329) => self.runtime_thumb_dd329(mem, pc, insn),
            (true, 0xdd32a) => self.runtime_thumb_dd32a(mem, pc, insn),
            (true, 0xdd32b) => self.runtime_thumb_dd32b(mem, pc, insn),
            (true, 0xdd32c) => self.runtime_thumb_dd32c(mem, pc, insn),
            (true, 0xdd32d) => self.runtime_thumb_dd32d(mem, pc, insn),
            (true, 0xdd32e) => self.runtime_thumb_dd32e(mem, pc, insn),
            (true, 0xdd32f) => self.runtime_thumb_dd32f(mem, pc, insn),
            (true, 0xdd330) => self.runtime_thumb_dd330(mem, pc, insn),
            (true, 0xdd331) => self.runtime_thumb_dd331(mem, pc, insn),
            (true, 0xdd332) => self.runtime_thumb_dd332(mem, pc, insn),
            (true, 0xdd333) => self.runtime_thumb_dd333(mem, pc, insn),
            (true, 0xdd334) => self.runtime_thumb_dd334(mem, pc, insn),
            (true, 0xdd335) => self.runtime_thumb_dd335(mem, pc, insn),
            (true, 0xdd336) => self.runtime_thumb_dd336(mem, pc, insn),
            (true, 0xdd337) => self.runtime_thumb_dd337(mem, pc, insn),
            (true, 0xdd338) => self.runtime_thumb_dd338(mem, pc, insn),
            (true, 0xdd339) => self.runtime_thumb_dd339(mem, pc, insn),
            (true, 0xdd33a) => self.runtime_thumb_dd33a(mem, pc, insn),
            (true, 0xdd33b) => self.runtime_thumb_dd33b(mem, pc, insn),
            (true, 0xdd33c) => self.runtime_thumb_dd33c(mem, pc, insn),
            (true, 0xdd34c) => self.runtime_thumb_dd34c(mem, pc, insn),
            (true, 0xdd34d) => self.runtime_thumb_dd34d(mem, pc, insn),
            _ => None,
        }
    }
}
