warning: unused import: `valid`
  --> armv6-m-examples/examples/enum_u16.rs:26:27
   |
26 | use symex_lib::{symbolic, valid, Validate};
   |                           ^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unreachable expression
  --> armv6-m-examples/examples/enum_u16.rs:86:5
   |
78 | /     match e {
79 | |         E::A => panic!(),
80 | |         E::B(u) => panic!(),
81 | |         E::C(u) => panic!(),
...  |
84 | |         E::F(_u) => panic!(),
85 | |     }
   | |_____- any code following this `match` expression is unreachable, as all arms diverge
86 |       panic!()
   |       ^^^^^^^^ unreachable expression
   |
   = note: `#[warn(unreachable_code)]` on by default
   = note: this warning originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `panic` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `u`
  --> armv6-m-examples/examples/enum_u16.rs:55:14
   |
55 |         E::B(u) => panic!(),
   |              ^ help: if this is intentional, prefix it with an underscore: `_u`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `u`
  --> armv6-m-examples/examples/enum_u16.rs:56:14
   |
56 |         E::C(u) => panic!(),
   |              ^ help: if this is intentional, prefix it with an underscore: `_u`

warning: unused variable: `u`
  --> armv6-m-examples/examples/enum_u16.rs:57:14
   |
57 |         E::D(u) => panic!(),
   |              ^ help: if this is intentional, prefix it with an underscore: `_u`

warning: unused variable: `u`
  --> armv6-m-examples/examples/enum_u16.rs:58:14
   |
58 |         E::E(u) => panic!(),
   |              ^ help: if this is intentional, prefix it with an underscore: `_u`

warning: unused variable: `u`
  --> armv6-m-examples/examples/enum_u16.rs:80:14
   |
80 |         E::B(u) => panic!(),
   |              ^ help: if this is intentional, prefix it with an underscore: `_u`

warning: unused variable: `u`
  --> armv6-m-examples/examples/enum_u16.rs:81:14
   |
81 |         E::C(u) => panic!(),
   |              ^ help: if this is intentional, prefix it with an underscore: `_u`

warning: unused variable: `u`
  --> armv6-m-examples/examples/enum_u16.rs:82:14
   |
82 |         E::D(u) => panic!(),
   |              ^ help: if this is intentional, prefix it with an underscore: `_u`

warning: unused variable: `u`
  --> armv6-m-examples/examples/enum_u16.rs:83:14
   |
83 |         E::E(u) => panic!(),
   |              ^ help: if this is intentional, prefix it with an underscore: `_u`

warning: 10 warnings emitted


enum_u16:	file format elf32-littlearm

Disassembly of section .text:

100001c0 <__stext>:
100001c0:      	bl	0x1000032e <__pre_init> @ imm = #0x16a
100001c4:      	ldr	r0, [pc, #0x20]         @ 0x100001e8 <__stext+0x28>
100001c6:      	ldr	r1, [pc, #0x24]         @ 0x100001ec <__stext+0x2c>
100001c8:      	movs	r2, #0x0
100001ca:      	cmp	r1, r0
100001cc:      	beq	0x100001d2 <__stext+0x12> @ imm = #0x2
100001ce:      	stm	r0!, {r2}
100001d0:      	b	0x100001ca <__stext+0xa> @ imm = #-0xa
100001d2:      	ldr	r0, [pc, #0x1c]         @ 0x100001f0 <__stext+0x30>
100001d4:      	ldr	r1, [pc, #0x1c]         @ 0x100001f4 <__stext+0x34>
100001d6:      	ldr	r2, [pc, #0x20]         @ 0x100001f8 <__stext+0x38>
100001d8:      	cmp	r1, r0
100001da:      	beq	0x100001e2 <__stext+0x22> @ imm = #0x4
100001dc:      	ldm	r2!, {r3}
100001de:      	stm	r0!, {r3}
100001e0:      	b	0x100001d8 <__stext+0x18> @ imm = #-0xc
100001e2:      	bl	0x100002cc <main>       @ imm = #0xe6
100001e6:      	udf	#0x0

100001e8 <$d.10>:
100001e8: 00 00 04 20  	.word	0x20040000
100001ec: 00 00 04 20  	.word	0x20040000
100001f0: 00 00 04 20  	.word	0x20040000
100001f4: 00 00 04 20  	.word	0x20040000
100001f8: 9c 0b 00 10  	.word	0x10000b9c

100001fc <symex_lib::symbolic_size::h26adc10ab1a533e2>:
100001fc:      	sub	sp, #0x4
100001fe:      	ldrh	r0, [r0]
10000200:      	movs	r0, #0x2
10000202:      	str	r0, [sp]
10000204:      	ldr	r0, [sp]
10000206:      	add	sp, #0x4
10000208:      	bx	lr

1000020a <symex_lib::symbolic::h7664a81e3f1d253c>:
1000020a:      	push	{r7, lr}
1000020c:      	add	r7, sp, #0x0
1000020e:      	sub	sp, #0x8
10000210:      	movs	r1, #0x2
10000212:      	str	r1, [sp, #0x4]
10000214:      	ldr	r1, [sp, #0x4]
10000216:      	bl	0x100001fc <symex_lib::symbolic_size::h26adc10ab1a533e2> @ imm = #-0x1e
1000021a:      	add	sp, #0x8
1000021c:      	pop	{r7, pc}
1000021e:      	bmi	0x100001ca <__stext+0xa> @ imm = #-0x58

10000220 <check2>:
10000220:      	push	{r7, lr}
10000222:      	add	r7, sp, #0x0
10000224:      	sub	sp, #0x8
10000226:      	add	r4, sp, #0x4
10000228:      	movs	r0, #0x0
1000022a:      	strh	r0, [r4]
1000022c:      	mov	r0, r4
1000022e:      	bl	0x1000020a <symex_lib::symbolic::h7664a81e3f1d253c> @ imm = #-0x28
10000232:      	ldrh	r0, [r4]
10000234:      	cmp	r0, #0x5
10000236:      	bhi	0x10000258 <check2+0x38> @ imm = #0x1e
10000238:      	add	r0, pc
1000023a:      	ldrb	r0, [r0, #0x4]
1000023c:      	lsls	r0, r0, #0x1
1000023e:      	add	pc, r0

10000240 <$d.6>:
10000240: 02 05 08 0b  	.word	0x0b080502
10000244: 0e 11	.short	0x110e

10000246 <$t.7>:
10000246:      	bl	0x1000026c <enum_u16::check2::panic_cold_explicit::hec2f007d252e7efd> @ imm = #0x22
1000024a:      	trap
1000024c:      	bl	0x1000027c <enum_u16::check2::panic_cold_explicit::h3c77cd040bafdbdc> @ imm = #0x2c
10000250:      	trap
10000252:      	bl	0x1000028c <enum_u16::check2::panic_cold_explicit::h6b78d0d64dd84fa7> @ imm = #0x36
10000256:      	trap
10000258:      	bl	0x100002bc <enum_u16::check2::panic_cold_explicit::hcb36ae08ece4b676> @ imm = #0x60
1000025c:      	trap
1000025e:      	bl	0x1000029c <enum_u16::check2::panic_cold_explicit::he7be493ee7c07e4d> @ imm = #0x3a
10000262:      	trap
10000264:      	bl	0x100002ac <enum_u16::check2::panic_cold_explicit::h279d018c8e80f6f6> @ imm = #0x44
10000268:      	trap
1000026a:      	bmi	0x10000216 <symex_lib::symbolic::h7664a81e3f1d253c+0xc> @ imm = #-0x58

1000026c <enum_u16::check2::panic_cold_explicit::hec2f007d252e7efd>:
1000026c:      	push	{r7, lr}
1000026e:      	add	r7, sp, #0x0
10000270:      	ldr	r0, [pc, #0x4]          @ 0x10000278 <enum_u16::check2::panic_cold_explicit::hec2f007d252e7efd+0xc>
10000272:      	bl	0x10000378 <core::panicking::panic_explicit::h78bd2b7c2484afb0> @ imm = #0x102
10000276:      	trap

10000278 <$d.21>:
10000278: 0c 0b 00 10  	.word	0x10000b0c

1000027c <enum_u16::check2::panic_cold_explicit::h3c77cd040bafdbdc>:
1000027c:      	push	{r7, lr}
1000027e:      	add	r7, sp, #0x0
10000280:      	ldr	r0, [pc, #0x4]          @ 0x10000288 <enum_u16::check2::panic_cold_explicit::h3c77cd040bafdbdc+0xc>
10000282:      	bl	0x10000378 <core::panicking::panic_explicit::h78bd2b7c2484afb0> @ imm = #0xf2
10000286:      	trap

10000288 <$d.23>:
10000288: 1c 0b 00 10  	.word	0x10000b1c

1000028c <enum_u16::check2::panic_cold_explicit::h6b78d0d64dd84fa7>:
1000028c:      	push	{r7, lr}
1000028e:      	add	r7, sp, #0x0
10000290:      	ldr	r0, [pc, #0x4]          @ 0x10000298 <enum_u16::check2::panic_cold_explicit::h6b78d0d64dd84fa7+0xc>
10000292:      	bl	0x10000378 <core::panicking::panic_explicit::h78bd2b7c2484afb0> @ imm = #0xe2
10000296:      	trap

10000298 <$d.25>:
10000298: 2c 0b 00 10  	.word	0x10000b2c

1000029c <enum_u16::check2::panic_cold_explicit::he7be493ee7c07e4d>:
1000029c:      	push	{r7, lr}
1000029e:      	add	r7, sp, #0x0
100002a0:      	ldr	r0, [pc, #0x4]          @ 0x100002a8 <enum_u16::check2::panic_cold_explicit::he7be493ee7c07e4d+0xc>
100002a2:      	bl	0x10000378 <core::panicking::panic_explicit::h78bd2b7c2484afb0> @ imm = #0xd2
100002a6:      	trap

100002a8 <$d.27>:
100002a8: 3c 0b 00 10  	.word	0x10000b3c

100002ac <enum_u16::check2::panic_cold_explicit::h279d018c8e80f6f6>:
100002ac:      	push	{r7, lr}
100002ae:      	add	r7, sp, #0x0
100002b0:      	ldr	r0, [pc, #0x4]          @ 0x100002b8 <enum_u16::check2::panic_cold_explicit::h279d018c8e80f6f6+0xc>
100002b2:      	bl	0x10000378 <core::panicking::panic_explicit::h78bd2b7c2484afb0> @ imm = #0xc2
100002b6:      	trap

100002b8 <$d.29>:
100002b8: 4c 0b 00 10  	.word	0x10000b4c

100002bc <enum_u16::check2::panic_cold_explicit::hcb36ae08ece4b676>:
100002bc:      	push	{r7, lr}
100002be:      	add	r7, sp, #0x0
100002c0:      	ldr	r0, [pc, #0x4]          @ 0x100002c8 <enum_u16::check2::panic_cold_explicit::hcb36ae08ece4b676+0xc>
100002c2:      	bl	0x10000378 <core::panicking::panic_explicit::h78bd2b7c2484afb0> @ imm = #0xb2
100002c6:      	trap

100002c8 <$d.31>:
100002c8: 5c 0b 00 10  	.word	0x10000b5c

100002cc <main>:
100002cc:      	push	{r7, lr}
100002ce:      	add	r7, sp, #0x0
100002d0:      	bl	0x100002d8 <enum_u16::__cortex_m_rt_main::h06b00e643b43cc85> @ imm = #0x4
100002d4:      	trap
100002d6:      	bmi	0x10000282 <enum_u16::check2::panic_cold_explicit::h3c77cd040bafdbdc+0x6> @ imm = #-0x58

100002d8 <enum_u16::__cortex_m_rt_main::h06b00e643b43cc85>:
100002d8:      	push	{r7, lr}
100002da:      	add	r7, sp, #0x0
100002dc:      	ldr	r0, [pc, #0x48]         @ 0x10000328 <enum_u16::__cortex_m_rt_main::h06b00e643b43cc85+0x50>
100002de:      	movs	r1, #0x1
100002e0:      	str	r1, [r0]
100002e2:      	str	r1, [r0, #0x4]
100002e4:      	str	r1, [r0, #0x8]
100002e6:      	str	r1, [r0, #0xc]
100002e8:      	str	r1, [r0, #0x10]
100002ea:      	str	r1, [r0, #0x14]
100002ec:      	str	r1, [r0, #0x18]
100002ee:      	str	r1, [r0, #0x1c]
100002f0:      	str	r1, [r0, #0x20]
100002f2:      	str	r1, [r0, #0x24]
100002f4:      	str	r1, [r0, #0x28]
100002f6:      	str	r1, [r0, #0x2c]
100002f8:      	str	r1, [r0, #0x30]
100002fa:      	str	r1, [r0, #0x34]
100002fc:      	str	r1, [r0, #0x38]
100002fe:      	str	r1, [r0, #0x3c]
10000300:      	str	r1, [r0, #0x40]
10000302:      	str	r1, [r0, #0x44]
10000304:      	str	r1, [r0, #0x48]
10000306:      	str	r1, [r0, #0x4c]
10000308:      	str	r1, [r0, #0x50]
1000030a:      	str	r1, [r0, #0x54]
1000030c:      	str	r1, [r0, #0x58]
1000030e:      	str	r1, [r0, #0x5c]
10000310:      	str	r1, [r0, #0x60]
10000312:      	str	r1, [r0, #0x64]
10000314:      	str	r1, [r0, #0x68]
10000316:      	str	r1, [r0, #0x6c]
10000318:      	str	r1, [r0, #0x70]
1000031a:      	str	r1, [r0, #0x74]
1000031c:      	str	r1, [r0, #0x78]
1000031e:      	str	r1, [r0, #0x7c]
10000320:      	bl	0x10000220 <check2>     @ imm = #-0x104
10000324:      	trap
10000326:      	mov	r8, r8

10000328 <$d.34>:
10000328: 00 01 00 d0  	.word	0xd0000100

1000032c <XIP_IRQ>:
1000032c:      	b	0x1000032c <XIP_IRQ>    @ imm = #-0x4

1000032e <__pre_init>:
1000032e:      	bx	lr

10000330 <rust_begin_unwind>:
10000330:      	b	0x10000330 <rust_begin_unwind> @ imm = #-0x4

10000332 <core::ptr::drop_in_place<core::fmt::Error>::h2f246cc06eb7d04f>:
10000332:      	bx	lr

10000334 <<T as core::any::Any>::type_id::hd883277038d44999>:
10000334:      	ldr	r0, [pc, #0x8]          @ 0x10000340 <<T as core::any::Any>::type_id::hd883277038d44999+0xc>
10000336:      	ldr	r1, [pc, #0xc]          @ 0x10000344 <<T as core::any::Any>::type_id::hd883277038d44999+0x10>
10000338:      	ldr	r2, [pc, #0xc]          @ 0x10000348 <<T as core::any::Any>::type_id::hd883277038d44999+0x14>
1000033a:      	ldr	r3, [pc, #0x10]         @ 0x1000034c <<T as core::any::Any>::type_id::hd883277038d44999+0x18>
1000033c:      	bx	lr
1000033e:      	mov	r8, r8

10000340 <$d.137>:
10000340: a6 7f 64 fd  	.word	0xfd647fa6
10000344: dc e4 c7 78  	.word	0x78c7e4dc
10000348: 94 f2 1d 46  	.word	0x461df294
1000034c: 84 cc ee f3  	.word	0xf3eecc84

10000350 <core::panicking::panic_fmt::hd2523d0706fb7bbc>:
10000350:      	push	{r7, lr}
10000352:      	add	r7, sp, #0x0
10000354:      	sub	sp, #0x18
10000356:      	add	r2, sp, #0x4
10000358:      	movs	r3, #0x1
1000035a:      	strh	r3, [r2, #0x10]
1000035c:      	str	r1, [sp, #0x10]
1000035e:      	str	r0, [sp, #0xc]
10000360:      	ldr	r0, [pc, #0xc]          @ 0x10000370 <core::panicking::panic_fmt::hd2523d0706fb7bbc+0x20>
10000362:      	str	r0, [sp, #0x8]
10000364:      	ldr	r0, [pc, #0xc]          @ 0x10000374 <core::panicking::panic_fmt::hd2523d0706fb7bbc+0x24>
10000366:      	str	r0, [sp, #0x4]
10000368:      	mov	r0, r2
1000036a:      	bl	0x10000330 <rust_begin_unwind> @ imm = #-0x3e
1000036e:      	trap

10000370 <$d.273>:
10000370: 74 0b 00 10  	.word	0x10000b74
10000374: 6c 0b 00 10  	.word	0x10000b6c

10000378 <core::panicking::panic_explicit::h78bd2b7c2484afb0>:
10000378:      	push	{r7, lr}
1000037a:      	add	r7, sp, #0x0
1000037c:      	sub	sp, #0x20
1000037e:      	mov	r1, r0
10000380:      	movs	r0, #0x0
10000382:      	str	r0, [sp, #0x10]
10000384:      	movs	r0, #0x1
10000386:      	str	r0, [sp, #0x4]
10000388:      	ldr	r2, [pc, #0x18]         @ 0x100003a4 <core::panicking::panic_explicit::h78bd2b7c2484afb0+0x2c>
1000038a:      	str	r2, [sp]
1000038c:      	str	r0, [sp, #0xc]
1000038e:      	add	r0, sp, #0x18
10000390:      	str	r0, [sp, #0x8]
10000392:      	ldr	r0, [pc, #0x14]         @ 0x100003a8 <core::panicking::panic_explicit::h78bd2b7c2484afb0+0x30>
10000394:      	str	r0, [sp, #0x1c]
10000396:      	ldr	r0, [pc, #0x14]         @ 0x100003ac <core::panicking::panic_explicit::h78bd2b7c2484afb0+0x34>
10000398:      	str	r0, [sp, #0x18]
1000039a:      	mov	r0, sp
1000039c:      	bl	0x10000350 <core::panicking::panic_fmt::hd2523d0706fb7bbc> @ imm = #-0x50
100003a0:      	trap
100003a2:      	mov	r8, r8

100003a4 <$d.283>:
100003a4: 6c 0b 00 10  	.word	0x10000b6c
100003a8: bd 0a 00 10  	.word	0x10000abd
100003ac: 94 0b 00 10  	.word	0x10000b94

100003b0 <core::fmt::Formatter::pad::hf5cda8bdf873db52>:
100003b0:      	push	{r4, r5, r6, r7, lr}
100003b2:      	add	r7, sp, #0xc
100003b4:      	sub	sp, #0x24
100003b6:      	mov	r5, r2
100003b8:      	mov	r4, r1
100003ba:      	mov	r3, r0
100003bc:      	ldr	r0, [r0, #0x8]
100003be:      	ldr	r1, [r3]
100003c0:      	str	r1, [sp, #0x10]
100003c2:      	orrs	r1, r0
100003c4:      	bne	0x100003c8 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x18> @ imm = #0x0
100003c6:      	b	0x10000570 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x1c0> @ imm = #0x1a6
100003c8:      	cmp	r0, #0x0
100003ca:      	str	r3, [sp, #0x18]
100003cc:      	bne	0x100003d0 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x20> @ imm = #0x0
100003ce:      	b	0x100004dc <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x12c> @ imm = #0x10a
100003d0:      	str	r5, [sp, #0x4]
100003d2:      	adds	r0, r4, r5
100003d4:      	str	r0, [sp, #0x1c]
100003d6:      	movs	r0, #0x11
100003d8:      	lsls	r0, r0, #0x10
100003da:      	str	r0, [sp, #0x20]
100003dc:      	ldr	r0, [r3, #0xc]
100003de:      	adds	r5, r0, #0x1
100003e0:      	movs	r1, #0x0
100003e2:      	mov	r6, r4
100003e4:      	str	r4, [sp, #0x14]
100003e6:      	b	0x100003f4 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x44> @ imm = #0xa
100003e8:      	adds	r6, r3, #0x1
100003ea:      	subs	r1, r1, r3
100003ec:      	adds	r1, r1, r6
100003ee:      	ldr	r2, [sp, #0x20]
100003f0:      	cmp	r0, r2
100003f2:      	beq	0x1000045c <core::fmt::Formatter::pad::hf5cda8bdf873db52+0xac> @ imm = #0x66
100003f4:      	mov	r3, r6
100003f6:      	subs	r5, r5, #0x1
100003f8:      	beq	0x10000468 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0xb8> @ imm = #0x6c
100003fa:      	ldr	r0, [sp, #0x1c]
100003fc:      	cmp	r3, r0
100003fe:      	beq	0x1000045c <core::fmt::Formatter::pad::hf5cda8bdf873db52+0xac> @ imm = #0x5a
10000400:      	movs	r0, #0x0
10000402:      	ldrsb	r2, [r3, r0]
10000404:      	uxtb	r0, r2
10000406:      	cmp	r2, #0x0
10000408:      	bpl	0x100003e8 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x38> @ imm = #-0x24
1000040a:      	ldrb	r4, [r3, #0x1]
1000040c:      	movs	r2, #0x3f
1000040e:      	ands	r4, r2
10000410:      	movs	r6, #0x1f
10000412:      	ands	r6, r0
10000414:      	cmp	r0, #0xdf
10000416:      	bls	0x10000446 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x96> @ imm = #0x2c
10000418:      	str	r1, [sp, #0x8]
1000041a:      	str	r2, [sp, #0xc]
1000041c:      	ldrb	r2, [r3, #0x2]
1000041e:      	ldr	r1, [sp, #0xc]
10000420:      	ands	r2, r1
10000422:      	lsls	r4, r4, #0x6
10000424:      	adds	r4, r4, r2
10000426:      	cmp	r0, #0xf0
10000428:      	blo	0x10000450 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0xa0> @ imm = #0x24
1000042a:      	ldrb	r0, [r3, #0x3]
1000042c:      	ldr	r2, [sp, #0xc]
1000042e:      	ands	r0, r2
10000430:      	lsls	r2, r4, #0x6
10000432:      	adds	r0, r2, r0
10000434:      	lsls	r2, r6, #0x1d
10000436:      	lsrs	r2, r2, #0xb
10000438:      	adds	r0, r0, r2
1000043a:      	ldr	r2, [sp, #0x20]
1000043c:      	cmp	r0, r2
1000043e:      	bne	0x10000442 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x92> @ imm = #0x0
10000440:      	b	0x1000063c <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x28c> @ imm = #0x1f8
10000442:      	adds	r6, r3, #0x4
10000444:      	b	0x10000456 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0xa6> @ imm = #0xe
10000446:      	lsls	r0, r6, #0x6
10000448:      	adds	r0, r0, r4
1000044a:      	adds	r6, r3, #0x2
1000044c:      	ldr	r4, [sp, #0x14]
1000044e:      	b	0x100003ea <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x3a> @ imm = #-0x68
10000450:      	lsls	r0, r6, #0xc
10000452:      	adds	r0, r4, r0
10000454:      	adds	r6, r3, #0x3
10000456:      	ldr	r4, [sp, #0x14]
10000458:      	ldr	r1, [sp, #0x8]
1000045a:      	b	0x100003ea <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x3a> @ imm = #-0x74
1000045c:      	ldr	r5, [sp, #0x4]
1000045e:      	ldr	r3, [sp, #0x18]
10000460:      	ldr	r0, [sp, #0x10]
10000462:      	cmp	r0, #0x0
10000464:      	bne	0x100004e2 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x132> @ imm = #0x7a
10000466:      	b	0x10000570 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x1c0> @ imm = #0x106
10000468:      	ldr	r0, [sp, #0x1c]
1000046a:      	cmp	r3, r0
1000046c:      	ldr	r5, [sp, #0x4]
1000046e:      	beq	0x100004be <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x10e> @ imm = #0x4c
10000470:      	movs	r4, #0x0
10000472:      	ldrsb	r0, [r3, r4]
10000474:      	cmp	r0, #0x0
10000476:      	bpl	0x100004a8 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0xf8> @ imm = #0x2e
10000478:      	uxtb	r0, r0
1000047a:      	cmp	r0, #0xe0
1000047c:      	blo	0x100004a8 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0xf8> @ imm = #0x28
1000047e:      	cmp	r0, #0xf0
10000480:      	blo	0x100004a8 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0xf8> @ imm = #0x24
10000482:      	ldrb	r2, [r3, #0x1]
10000484:      	ldrb	r5, [r3, #0x3]
10000486:      	movs	r6, #0x3f
10000488:      	ands	r6, r5
1000048a:      	ldr	r5, [sp, #0x4]
1000048c:      	lsls	r2, r2, #0x1a
1000048e:      	lsrs	r2, r2, #0xe
10000490:      	ldrb	r3, [r3, #0x2]
10000492:      	lsls	r3, r3, #0x1a
10000494:      	lsrs	r3, r3, #0x14
10000496:      	adds	r2, r3, r2
10000498:      	adds	r2, r2, r6
1000049a:      	lsls	r0, r0, #0x1d
1000049c:      	lsrs	r0, r0, #0xb
1000049e:      	adds	r0, r2, r0
100004a0:      	ldr	r2, [sp, #0x20]
100004a2:      	cmp	r0, r2
100004a4:      	bne	0x100004a8 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0xf8> @ imm = #0x0
100004a6:      	b	0x1000063e <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x28e> @ imm = #0x194
100004a8:      	cmp	r1, #0x0
100004aa:      	beq	0x100004ca <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x11a> @ imm = #0x1c
100004ac:      	cmp	r1, r5
100004ae:      	bhs	0x100004c8 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x118> @ imm = #0x16
100004b0:      	ldr	r0, [sp, #0x14]
100004b2:      	ldrsb	r0, [r0, r1]
100004b4:      	movs	r2, #0x3f
100004b6:      	mvns	r2, r2
100004b8:      	cmp	r0, r2
100004ba:      	bge	0x100004ca <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x11a> @ imm = #0xc
100004bc:      	b	0x100004cc <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x11c> @ imm = #0xc
100004be:      	ldr	r3, [sp, #0x18]
100004c0:      	ldr	r0, [sp, #0x10]
100004c2:      	cmp	r0, #0x0
100004c4:      	bne	0x100004e2 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x132> @ imm = #0x1a
100004c6:      	b	0x10000570 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x1c0> @ imm = #0xa6
100004c8:      	bne	0x100004cc <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x11c> @ imm = #0x0
100004ca:      	ldr	r4, [sp, #0x14]
100004cc:      	cmp	r4, #0x0
100004ce:      	beq	0x100004d2 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x122> @ imm = #0x0
100004d0:      	mov	r5, r1
100004d2:      	cmp	r4, #0x0
100004d4:      	ldr	r3, [sp, #0x18]
100004d6:      	beq	0x100004da <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x12a> @ imm = #0x0
100004d8:      	str	r4, [sp, #0x14]
100004da:      	ldr	r4, [sp, #0x14]
100004dc:      	ldr	r0, [sp, #0x10]
100004de:      	cmp	r0, #0x0
100004e0:      	beq	0x10000570 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x1c0> @ imm = #0x8c
100004e2:      	ldr	r0, [r3, #0x4]
100004e4:      	cmp	r5, #0x10
100004e6:      	str	r0, [sp, #0x1c]
100004e8:      	bhs	0x10000538 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x188> @ imm = #0x4c
100004ea:      	cmp	r5, #0x0
100004ec:      	beq	0x10000568 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x1b8> @ imm = #0x78
100004ee:      	movs	r6, #0x3
100004f0:      	mov	r1, r5
100004f2:      	ands	r1, r6
100004f4:      	str	r1, [sp, #0x14]
100004f6:      	cmp	r5, #0x4
100004f8:      	str	r5, [sp, #0x4]
100004fa:      	bhs	0x10000580 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x1d0> @ imm = #0x82
100004fc:      	movs	r0, #0x0
100004fe:      	mov	r2, r0
10000500:      	ldr	r1, [sp, #0x14]
10000502:      	cmp	r1, #0x0
10000504:      	ldr	r5, [sp, #0x4]
10000506:      	beq	0x10000542 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x192> @ imm = #0x38
10000508:      	ldrsb	r3, [r4, r2]
1000050a:      	movs	r1, #0x40
1000050c:      	mvns	r1, r1
1000050e:      	cmp	r3, r1
10000510:      	ble	0x10000514 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x164> @ imm = #0x0
10000512:      	adds	r0, r0, #0x1
10000514:      	ldr	r3, [sp, #0x14]
10000516:      	cmp	r3, #0x1
10000518:      	beq	0x10000540 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x190> @ imm = #0x24
1000051a:      	adds	r2, r2, r4
1000051c:      	movs	r3, #0x1
1000051e:      	ldrsb	r3, [r2, r3]
10000520:      	cmp	r3, r1
10000522:      	ble	0x10000526 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x176> @ imm = #0x0
10000524:      	adds	r0, r0, #0x1
10000526:      	ldr	r3, [sp, #0x14]
10000528:      	cmp	r3, #0x2
1000052a:      	beq	0x10000540 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x190> @ imm = #0x12
1000052c:      	movs	r3, #0x2
1000052e:      	ldrsb	r2, [r2, r3]
10000530:      	cmp	r2, r1
10000532:      	ble	0x10000540 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x190> @ imm = #0xa
10000534:      	adds	r0, r0, #0x1
10000536:      	b	0x10000540 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x190> @ imm = #0x6
10000538:      	mov	r0, r4
1000053a:      	mov	r1, r5
1000053c:      	bl	0x1000064c <core::str::count::do_count_chars::h419a4d2620ecb10e> @ imm = #0x10c
10000540:      	ldr	r3, [sp, #0x18]
10000542:      	ldr	r1, [sp, #0x1c]
10000544:      	cmp	r1, r0
10000546:      	bls	0x10000570 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x1c0> @ imm = #0x26
10000548:      	subs	r6, r1, r0
1000054a:      	movs	r0, #0x20
1000054c:      	ldrb	r1, [r3, r0]
1000054e:      	movs	r0, #0x0
10000550:      	str	r4, [sp, #0x14]
10000552:      	str	r5, [sp, #0x4]
10000554:      	add	r1, pc
10000556:      	ldrb	r1, [r1, #0x4]
10000558:      	lsls	r1, r1, #0x1
1000055a:      	add	pc, r1

1000055c <$d.379>:
1000055c: 41 01 3e 41  	.word	0x413e0141

10000560 <$t.380>:
10000560:      	movs	r1, #0x0
10000562:      	mov	r0, r6
10000564:      	mov	r6, r1
10000566:      	b	0x100005e0 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x230> @ imm = #0x76
10000568:      	movs	r0, #0x0
1000056a:      	ldr	r1, [sp, #0x1c]
1000056c:      	cmp	r1, r0
1000056e:      	bhi	0x10000548 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x198> @ imm = #-0x2a
10000570:      	ldr	r0, [r3, #0x14]
10000572:      	ldr	r1, [r3, #0x18]
10000574:      	ldr	r3, [r1, #0xc]
10000576:      	mov	r1, r4
10000578:      	mov	r2, r5
1000057a:      	blx	r3
1000057c:      	add	sp, #0x24
1000057e:      	pop	{r4, r5, r6, r7, pc}
10000580:      	bics	r5, r6
10000582:      	str	r5, [sp, #0x20]
10000584:      	movs	r0, #0x0
10000586:      	mov	r2, r0
10000588:      	b	0x10000596 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x1e6> @ imm = #0xa
1000058a:      	mov	r4, r3
1000058c:      	ldr	r3, [sp, #0x18]
1000058e:      	adds	r2, r2, #0x4
10000590:      	ldr	r1, [sp, #0x20]
10000592:      	cmp	r1, r2
10000594:      	beq	0x10000500 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x150> @ imm = #-0x98
10000596:      	mov	r5, r6
10000598:      	ldrsb	r6, [r4, r2]
1000059a:      	movs	r1, #0x40
1000059c:      	mvns	r1, r1
1000059e:      	cmp	r6, r1
100005a0:      	ble	0x100005a4 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x1f4> @ imm = #0x0
100005a2:      	adds	r0, r0, #0x1
100005a4:      	mov	r3, r4
100005a6:      	adds	r6, r4, r2
100005a8:      	movs	r4, #0x1
100005aa:      	ldrsb	r4, [r6, r4]
100005ac:      	cmp	r4, r1
100005ae:      	bgt	0x100005c2 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x212> @ imm = #0x10
100005b0:      	movs	r4, #0x2
100005b2:      	ldrsb	r4, [r6, r4]
100005b4:      	cmp	r4, r1
100005b6:      	bgt	0x100005cc <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x21c> @ imm = #0x12
100005b8:      	ldrsb	r4, [r6, r5]
100005ba:      	mov	r6, r5
100005bc:      	cmp	r4, r1
100005be:      	ble	0x1000058a <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x1da> @ imm = #-0x38
100005c0:      	b	0x100005d6 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x226> @ imm = #0x12
100005c2:      	adds	r0, r0, #0x1
100005c4:      	movs	r4, #0x2
100005c6:      	ldrsb	r4, [r6, r4]
100005c8:      	cmp	r4, r1
100005ca:      	ble	0x100005b8 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x208> @ imm = #-0x16
100005cc:      	adds	r0, r0, #0x1
100005ce:      	ldrsb	r4, [r6, r5]
100005d0:      	mov	r6, r5
100005d2:      	cmp	r4, r1
100005d4:      	ble	0x1000058a <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x1da> @ imm = #-0x4e
100005d6:      	adds	r0, r0, #0x1
100005d8:      	b	0x1000058a <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x1da> @ imm = #-0x52
100005da:      	lsrs	r0, r6, #0x1
100005dc:      	adds	r1, r6, #0x1
100005de:      	lsrs	r6, r1, #0x1
100005e0:      	str	r6, [sp, #0x1c]
100005e2:      	adds	r4, r0, #0x1
100005e4:      	ldr	r0, [r3, #0x10]
100005e6:      	str	r0, [sp, #0x20]
100005e8:      	ldr	r5, [r3, #0x14]
100005ea:      	ldr	r6, [r3, #0x18]
100005ec:      	subs	r4, r4, #0x1
100005ee:      	beq	0x100005fe <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x24e> @ imm = #0xc
100005f0:      	ldr	r2, [r6, #0x10]
100005f2:      	mov	r0, r5
100005f4:      	ldr	r1, [sp, #0x20]
100005f6:      	blx	r2
100005f8:      	cmp	r0, #0x0
100005fa:      	beq	0x100005ec <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x23c> @ imm = #-0x12
100005fc:      	b	0x10000636 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x286> @ imm = #0x36
100005fe:      	ldr	r3, [r6, #0xc]
10000600:      	mov	r0, r5
10000602:      	ldr	r1, [sp, #0x14]
10000604:      	ldr	r2, [sp, #0x4]
10000606:      	blx	r3
10000608:      	cmp	r0, #0x0
1000060a:      	bne	0x10000636 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x286> @ imm = #0x28
1000060c:      	movs	r4, #0x0
1000060e:      	ldr	r0, [sp, #0x1c]
10000610:      	cmp	r0, r4
10000612:      	beq	0x10000628 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x278> @ imm = #0x12
10000614:      	ldr	r2, [r6, #0x10]
10000616:      	mov	r0, r5
10000618:      	ldr	r1, [sp, #0x20]
1000061a:      	blx	r2
1000061c:      	adds	r4, r4, #0x1
1000061e:      	cmp	r0, #0x0
10000620:      	beq	0x1000060e <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x25e> @ imm = #-0x16
10000622:      	subs	r0, r4, #0x1
10000624:      	ldr	r1, [sp, #0x1c]
10000626:      	b	0x1000062c <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x27c> @ imm = #0x2
10000628:      	ldr	r1, [sp, #0x1c]
1000062a:      	mov	r0, r1
1000062c:      	cmp	r0, r1
1000062e:      	blo	0x10000636 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x286> @ imm = #0x4
10000630:      	movs	r0, #0x0
10000632:      	add	sp, #0x24
10000634:      	pop	{r4, r5, r6, r7, pc}
10000636:      	movs	r0, #0x1
10000638:      	add	sp, #0x24
1000063a:      	pop	{r4, r5, r6, r7, pc}
1000063c:      	ldr	r5, [sp, #0x4]
1000063e:      	ldr	r4, [sp, #0x14]
10000640:      	ldr	r3, [sp, #0x18]
10000642:      	ldr	r0, [sp, #0x10]
10000644:      	cmp	r0, #0x0
10000646:      	beq	0x10000570 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x1c0> @ imm = #-0xda
10000648:      	b	0x100004e2 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x132> @ imm = #-0x16a
1000064a:      	bmi	0x100005f6 <core::fmt::Formatter::pad::hf5cda8bdf873db52+0x246> @ imm = #-0x58

1000064c <core::str::count::do_count_chars::h419a4d2620ecb10e>:
1000064c:      	push	{r4, r5, r6, r7, lr}
1000064e:      	add	r7, sp, #0xc
10000650:      	sub	sp, #0x20
10000652:      	mov	r2, r0
10000654:      	adds	r3, r0, #0x3
10000656:      	movs	r0, #0x3
10000658:      	bics	r3, r0
1000065a:      	str	r3, [sp, #0x18]
1000065c:      	subs	r4, r3, r2
1000065e:      	cmp	r1, r4
10000660:      	str	r0, [sp, #0x1c]
10000662:      	bhs	0x10000666 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x1a> @ imm = #0x0
10000664:      	b	0x1000093c <core::str::count::do_count_chars::h419a4d2620ecb10e+0x2f0> @ imm = #0x2d4
10000666:      	subs	r3, r1, r4
10000668:      	lsrs	r0, r3, #0x2
1000066a:      	str	r0, [sp]
1000066c:      	bne	0x10000670 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x24> @ imm = #0x0
1000066e:      	b	0x1000093c <core::str::count::do_count_chars::h419a4d2620ecb10e+0x2f0> @ imm = #0x2ca
10000670:      	str	r3, [sp, #0xc]
10000672:      	mov	r0, r3
10000674:      	ldr	r1, [sp, #0x1c]
10000676:      	ands	r0, r1
10000678:      	str	r0, [sp, #0x10]
1000067a:      	movs	r3, #0x0
1000067c:      	ldr	r0, [sp, #0x18]
1000067e:      	cmp	r0, r2
10000680:      	str	r3, [sp, #0x14]
10000682:      	beq	0x100006c2 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x76> @ imm = #0x3c
10000684:      	mvns	r1, r2
10000686:      	adds	r1, r0, r1
10000688:      	movs	r3, #0x0
1000068a:      	cmp	r1, #0x3
1000068c:      	blo	0x10000690 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x44> @ imm = #0x0
1000068e:      	b	0x10000a44 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x3f8> @ imm = #0x3b2
10000690:      	mov	r1, r3
10000692:      	ldr	r0, [sp, #0x18]
10000694:      	cmp	r0, r2
10000696:      	beq	0x100006c2 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x76> @ imm = #0x28
10000698:      	ldrsb	r5, [r2, r1]
1000069a:      	movs	r0, #0x40
1000069c:      	mvns	r0, r0
1000069e:      	cmp	r5, r0
100006a0:      	ble	0x100006a4 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x58> @ imm = #0x0
100006a2:      	adds	r3, r3, #0x1
100006a4:      	cmp	r4, #0x1
100006a6:      	beq	0x100006c2 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x76> @ imm = #0x18
100006a8:      	adds	r1, r1, r2
100006aa:      	movs	r5, #0x1
100006ac:      	ldrsb	r5, [r1, r5]
100006ae:      	cmp	r5, r0
100006b0:      	ble	0x100006b4 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x68> @ imm = #0x0
100006b2:      	adds	r3, r3, #0x1
100006b4:      	cmp	r4, #0x2
100006b6:      	beq	0x100006c2 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x76> @ imm = #0x8
100006b8:      	movs	r5, #0x2
100006ba:      	ldrsb	r1, [r1, r5]
100006bc:      	cmp	r1, r0
100006be:      	ble	0x100006c2 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x76> @ imm = #0x0
100006c0:      	adds	r3, r3, #0x1
100006c2:      	adds	r2, r2, r4
100006c4:      	ldr	r0, [sp, #0x10]
100006c6:      	cmp	r0, #0x0
100006c8:      	ldr	r1, [sp, #0x1c]
100006ca:      	beq	0x10000710 <core::str::count::do_count_chars::h419a4d2620ecb10e+0xc4> @ imm = #0x42
100006cc:      	ldr	r0, [sp, #0xc]
100006ce:      	bics	r0, r1
100006d0:      	mov	r4, r2
100006d2:      	adds	r0, r2, r0
100006d4:      	movs	r1, #0x0
100006d6:      	str	r1, [sp, #0x14]
100006d8:      	ldrsb	r2, [r0, r1]
100006da:      	movs	r1, #0x40
100006dc:      	mvns	r1, r1
100006de:      	cmp	r2, r1
100006e0:      	ble	0x100006e6 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x9a> @ imm = #0x2
100006e2:      	movs	r2, #0x1
100006e4:      	str	r2, [sp, #0x14]
100006e6:      	ldr	r2, [sp, #0x10]
100006e8:      	cmp	r2, #0x1
100006ea:      	beq	0x1000070e <core::str::count::do_count_chars::h419a4d2620ecb10e+0xc2> @ imm = #0x20
100006ec:      	movs	r2, #0x1
100006ee:      	ldrsb	r2, [r0, r2]
100006f0:      	cmp	r2, r1
100006f2:      	ble	0x100006fa <core::str::count::do_count_chars::h419a4d2620ecb10e+0xae> @ imm = #0x4
100006f4:      	ldr	r2, [sp, #0x14]
100006f6:      	adds	r2, r2, #0x1
100006f8:      	str	r2, [sp, #0x14]
100006fa:      	ldr	r2, [sp, #0x10]
100006fc:      	cmp	r2, #0x2
100006fe:      	beq	0x1000070e <core::str::count::do_count_chars::h419a4d2620ecb10e+0xc2> @ imm = #0xc
10000700:      	movs	r2, #0x2
10000702:      	ldrsb	r0, [r0, r2]
10000704:      	cmp	r0, r1
10000706:      	ble	0x1000070e <core::str::count::do_count_chars::h419a4d2620ecb10e+0xc2> @ imm = #0x4
10000708:      	ldr	r0, [sp, #0x14]
1000070a:      	adds	r0, r0, #0x1
1000070c:      	str	r0, [sp, #0x14]
1000070e:      	mov	r2, r4
10000710:      	ldr	r0, [sp, #0x14]
10000712:      	adds	r0, r0, r3
10000714:      	str	r0, [sp, #0x18]
10000716:      	ldr	r0, [pc, #0x3a0]        @ 0x10000ab8 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x46c>
10000718:      	ldr	r5, [sp]
1000071a:      	b	0x10000746 <core::str::count::do_count_chars::h419a4d2620ecb10e+0xfa> @ imm = #0x28
1000071c:      	movs	r3, #0x0
1000071e:      	subs	r5, r5, r4
10000720:      	adds	r1, r2, r1
10000722:      	str	r1, [sp, #0x10]
10000724:      	lsrs	r1, r3, #0x8
10000726:      	mov	r6, r2
10000728:      	ldr	r2, [pc, #0x388]        @ 0x10000ab4 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x468>
1000072a:      	ands	r3, r2
1000072c:      	ands	r1, r2
1000072e:      	adds	r1, r1, r3
10000730:      	ldr	r2, [pc, #0x37c]        @ 0x10000ab0 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x464>
10000732:      	muls	r1, r2, r1
10000734:      	lsrs	r1, r1, #0x10
10000736:      	ldr	r2, [sp, #0x18]
10000738:      	adds	r2, r1, r2
1000073a:      	str	r2, [sp, #0x18]
1000073c:      	ldr	r2, [sp, #0x10]
1000073e:      	ldr	r1, [sp, #0x14]
10000740:      	cmp	r1, #0x0
10000742:      	beq	0x10000746 <core::str::count::do_count_chars::h419a4d2620ecb10e+0xfa> @ imm = #0x0
10000744:      	b	0x100009ce <core::str::count::do_count_chars::h419a4d2620ecb10e+0x382> @ imm = #0x286
10000746:      	cmp	r5, #0x0
10000748:      	bne	0x1000074c <core::str::count::do_count_chars::h419a4d2620ecb10e+0x100> @ imm = #0x0
1000074a:      	b	0x100009c8 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x37c> @ imm = #0x27a
1000074c:      	cmp	r5, #0xc0
1000074e:      	mov	r4, r5
10000750:      	blo	0x10000754 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x108> @ imm = #0x0
10000752:      	movs	r4, #0xc0
10000754:      	mov	r1, r4
10000756:      	ldr	r3, [sp, #0x1c]
10000758:      	ands	r1, r3
1000075a:      	str	r1, [sp, #0x14]
1000075c:      	lsls	r1, r4, #0x2
1000075e:      	cmp	r4, #0x4
10000760:      	blo	0x1000071c <core::str::count::do_count_chars::h419a4d2620ecb10e+0xd0> @ imm = #-0x48
10000762:      	str	r1, [sp, #0x8]
10000764:      	subs	r1, #0x10
10000766:      	lsrs	r3, r1, #0x4
10000768:      	adds	r3, r3, #0x1
1000076a:      	cmp	r1, #0x30
1000076c:      	str	r2, [sp, #0xc]
1000076e:      	str	r4, [sp, #0x4]
10000770:      	str	r3, [sp, #0x10]
10000772:      	bhs	0x1000077a <core::str::count::do_count_chars::h419a4d2620ecb10e+0x12e> @ imm = #0x4
10000774:      	movs	r3, #0x0
10000776:      	mov	r6, r2
10000778:      	b	0x10000870 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x224> @ imm = #0xf4
1000077a:      	str	r5, [sp]
1000077c:      	mov	r1, r3
1000077e:      	ldr	r3, [sp, #0x1c]
10000780:      	bics	r1, r3
10000782:      	movs	r3, #0x0
10000784:      	mov	r6, r2
10000786:      	ldr	r2, [r6, #0x4]
10000788:      	lsrs	r4, r2, #0x6
1000078a:      	mvns	r2, r2
1000078c:      	lsrs	r2, r2, #0x7
1000078e:      	orrs	r2, r4
10000790:      	ands	r2, r0
10000792:      	ldr	r4, [r6]
10000794:      	lsrs	r5, r4, #0x6
10000796:      	mvns	r4, r4
10000798:      	lsrs	r4, r4, #0x7
1000079a:      	orrs	r4, r5
1000079c:      	ands	r4, r0
1000079e:      	adds	r3, r4, r3
100007a0:      	adds	r2, r2, r3
100007a2:      	ldr	r3, [r6, #0x8]
100007a4:      	lsrs	r4, r3, #0x6
100007a6:      	mvns	r3, r3
100007a8:      	lsrs	r3, r3, #0x7
100007aa:      	orrs	r3, r4
100007ac:      	ands	r3, r0
100007ae:      	adds	r2, r3, r2
100007b0:      	ldr	r3, [r6, #0xc]
100007b2:      	lsrs	r4, r3, #0x6
100007b4:      	mvns	r3, r3
100007b6:      	lsrs	r3, r3, #0x7
100007b8:      	orrs	r3, r4
100007ba:      	ands	r3, r0
100007bc:      	adds	r2, r3, r2
100007be:      	ldr	r3, [r6, #0x10]
100007c0:      	lsrs	r4, r3, #0x6
100007c2:      	mvns	r3, r3
100007c4:      	lsrs	r3, r3, #0x7
100007c6:      	orrs	r3, r4
100007c8:      	ands	r3, r0
100007ca:      	adds	r2, r3, r2
100007cc:      	ldr	r3, [r6, #0x14]
100007ce:      	lsrs	r4, r3, #0x6
100007d0:      	mvns	r3, r3
100007d2:      	lsrs	r3, r3, #0x7
100007d4:      	orrs	r3, r4
100007d6:      	ands	r3, r0
100007d8:      	adds	r2, r3, r2
100007da:      	ldr	r3, [r6, #0x18]
100007dc:      	lsrs	r4, r3, #0x6
100007de:      	mvns	r3, r3
100007e0:      	lsrs	r3, r3, #0x7
100007e2:      	orrs	r3, r4
100007e4:      	ands	r3, r0
100007e6:      	adds	r2, r3, r2
100007e8:      	ldr	r3, [r6, #0x1c]
100007ea:      	lsrs	r4, r3, #0x6
100007ec:      	mvns	r3, r3
100007ee:      	lsrs	r3, r3, #0x7
100007f0:      	orrs	r3, r4
100007f2:      	ands	r3, r0
100007f4:      	adds	r2, r3, r2
100007f6:      	ldr	r3, [r6, #0x20]
100007f8:      	lsrs	r4, r3, #0x6
100007fa:      	mvns	r3, r3
100007fc:      	lsrs	r3, r3, #0x7
100007fe:      	orrs	r3, r4
10000800:      	ands	r3, r0
10000802:      	adds	r2, r3, r2
10000804:      	ldr	r3, [r6, #0x24]
10000806:      	lsrs	r4, r3, #0x6
10000808:      	mvns	r3, r3
1000080a:      	lsrs	r3, r3, #0x7
1000080c:      	orrs	r3, r4
1000080e:      	ands	r3, r0
10000810:      	adds	r2, r3, r2
10000812:      	ldr	r3, [r6, #0x28]
10000814:      	lsrs	r4, r3, #0x6
10000816:      	mvns	r3, r3
10000818:      	lsrs	r3, r3, #0x7
1000081a:      	orrs	r3, r4
1000081c:      	ands	r3, r0
1000081e:      	adds	r2, r3, r2
10000820:      	ldr	r3, [r6, #0x2c]
10000822:      	lsrs	r4, r3, #0x6
10000824:      	mvns	r3, r3
10000826:      	lsrs	r3, r3, #0x7
10000828:      	orrs	r3, r4
1000082a:      	ands	r3, r0
1000082c:      	adds	r2, r3, r2
1000082e:      	ldr	r3, [r6, #0x30]
10000830:      	lsrs	r4, r3, #0x6
10000832:      	mvns	r3, r3
10000834:      	lsrs	r3, r3, #0x7
10000836:      	orrs	r3, r4
10000838:      	ands	r3, r0
1000083a:      	adds	r2, r3, r2
1000083c:      	ldr	r3, [r6, #0x34]
1000083e:      	lsrs	r4, r3, #0x6
10000840:      	mvns	r3, r3
10000842:      	lsrs	r3, r3, #0x7
10000844:      	orrs	r3, r4
10000846:      	ands	r3, r0
10000848:      	adds	r2, r3, r2
1000084a:      	ldr	r3, [r6, #0x38]
1000084c:      	lsrs	r4, r3, #0x6
1000084e:      	mvns	r3, r3
10000850:      	lsrs	r3, r3, #0x7
10000852:      	orrs	r3, r4
10000854:      	ands	r3, r0
10000856:      	adds	r2, r3, r2
10000858:      	ldr	r3, [r6, #0x3c]
1000085a:      	lsrs	r4, r3, #0x6
1000085c:      	mvns	r3, r3
1000085e:      	lsrs	r3, r3, #0x7
10000860:      	orrs	r3, r4
10000862:      	ands	r3, r0
10000864:      	adds	r3, r3, r2
10000866:      	adds	r6, #0x40
10000868:      	subs	r1, r1, #0x4
1000086a:      	bne	0x10000786 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x13a> @ imm = #-0xe8
1000086c:      	ldr	r5, [sp]
1000086e:      	ldr	r4, [sp, #0x4]
10000870:      	ldr	r1, [sp, #0x10]
10000872:      	ldr	r2, [sp, #0x1c]
10000874:      	ands	r1, r2
10000876:      	str	r1, [sp, #0x10]
10000878:      	ldr	r1, [sp, #0x8]
1000087a:      	ldr	r2, [sp, #0xc]
1000087c:      	bne	0x10000880 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x234> @ imm = #0x0
1000087e:      	b	0x1000071e <core::str::count::do_count_chars::h419a4d2620ecb10e+0xd2> @ imm = #-0x164
10000880:      	ldr	r1, [r6, #0x4]
10000882:      	lsrs	r2, r1, #0x6
10000884:      	mvns	r1, r1
10000886:      	lsrs	r1, r1, #0x7
10000888:      	orrs	r1, r2
1000088a:      	ands	r1, r0
1000088c:      	ldr	r2, [r6]
1000088e:      	lsrs	r4, r2, #0x6
10000890:      	mvns	r2, r2
10000892:      	lsrs	r2, r2, #0x7
10000894:      	orrs	r2, r4
10000896:      	ands	r2, r0
10000898:      	adds	r2, r2, r3
1000089a:      	adds	r1, r1, r2
1000089c:      	ldr	r2, [r6, #0x8]
1000089e:      	lsrs	r3, r2, #0x6
100008a0:      	mvns	r2, r2
100008a2:      	lsrs	r2, r2, #0x7
100008a4:      	orrs	r2, r3
100008a6:      	ands	r2, r0
100008a8:      	adds	r1, r2, r1
100008aa:      	ldr	r2, [r6, #0xc]
100008ac:      	lsrs	r3, r2, #0x6
100008ae:      	mvns	r2, r2
100008b0:      	lsrs	r2, r2, #0x7
100008b2:      	orrs	r2, r3
100008b4:      	ands	r2, r0
100008b6:      	adds	r3, r2, r1
100008b8:      	ldr	r1, [sp, #0x10]
100008ba:      	cmp	r1, #0x1
100008bc:      	beq	0x10000934 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x2e8> @ imm = #0x74
100008be:      	ldr	r1, [r6, #0x14]
100008c0:      	lsrs	r2, r1, #0x6
100008c2:      	mvns	r1, r1
100008c4:      	lsrs	r1, r1, #0x7
100008c6:      	orrs	r1, r2
100008c8:      	ands	r1, r0
100008ca:      	ldr	r2, [r6, #0x10]
100008cc:      	lsrs	r4, r2, #0x6
100008ce:      	mvns	r2, r2
100008d0:      	lsrs	r2, r2, #0x7
100008d2:      	orrs	r2, r4
100008d4:      	ands	r2, r0
100008d6:      	adds	r2, r2, r3
100008d8:      	adds	r1, r1, r2
100008da:      	ldr	r2, [r6, #0x18]
100008dc:      	lsrs	r3, r2, #0x6
100008de:      	mvns	r2, r2
100008e0:      	lsrs	r2, r2, #0x7
100008e2:      	orrs	r2, r3
100008e4:      	ands	r2, r0
100008e6:      	adds	r1, r2, r1
100008e8:      	ldr	r2, [r6, #0x1c]
100008ea:      	lsrs	r3, r2, #0x6
100008ec:      	mvns	r2, r2
100008ee:      	lsrs	r2, r2, #0x7
100008f0:      	orrs	r2, r3
100008f2:      	ands	r2, r0
100008f4:      	adds	r3, r2, r1
100008f6:      	ldr	r1, [sp, #0x10]
100008f8:      	cmp	r1, #0x2
100008fa:      	beq	0x10000934 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x2e8> @ imm = #0x36
100008fc:      	ldr	r1, [r6, #0x24]
100008fe:      	lsrs	r2, r1, #0x6
10000900:      	mvns	r1, r1
10000902:      	lsrs	r1, r1, #0x7
10000904:      	orrs	r1, r2
10000906:      	ands	r1, r0
10000908:      	ldr	r2, [r6, #0x20]
1000090a:      	lsrs	r4, r2, #0x6
1000090c:      	mvns	r2, r2
1000090e:      	lsrs	r2, r2, #0x7
10000910:      	orrs	r2, r4
10000912:      	ands	r2, r0
10000914:      	adds	r2, r2, r3
10000916:      	adds	r1, r1, r2
10000918:      	ldr	r2, [r6, #0x28]
1000091a:      	lsrs	r3, r2, #0x6
1000091c:      	mvns	r2, r2
1000091e:      	lsrs	r2, r2, #0x7
10000920:      	orrs	r2, r3
10000922:      	ands	r2, r0
10000924:      	adds	r1, r2, r1
10000926:      	ldr	r2, [r6, #0x2c]
10000928:      	lsrs	r3, r2, #0x6
1000092a:      	mvns	r2, r2
1000092c:      	lsrs	r2, r2, #0x7
1000092e:      	orrs	r2, r3
10000930:      	ands	r2, r0
10000932:      	adds	r3, r2, r1
10000934:      	ldr	r2, [sp, #0xc]
10000936:      	ldr	r4, [sp, #0x4]
10000938:      	ldr	r1, [sp, #0x8]
1000093a:      	b	0x1000071e <core::str::count::do_count_chars::h419a4d2620ecb10e+0xd2> @ imm = #-0x220
1000093c:      	cmp	r1, #0x0
1000093e:      	beq	0x10000974 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x328> @ imm = #0x32
10000940:      	mov	r3, r1
10000942:      	ldr	r0, [sp, #0x1c]
10000944:      	ands	r3, r0
10000946:      	str	r3, [sp, #0x14]
10000948:      	cmp	r1, #0x4
1000094a:      	str	r2, [sp, #0x18]
1000094c:      	bhs	0x1000097a <core::str::count::do_count_chars::h419a4d2620ecb10e+0x32e> @ imm = #0x2a
1000094e:      	movs	r0, #0x0
10000950:      	mov	r4, r0
10000952:      	ldr	r1, [sp, #0x14]
10000954:      	cmp	r1, #0x0
10000956:      	beq	0x10000970 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x324> @ imm = #0x16
10000958:      	mov	r1, r2
1000095a:      	mov	r2, r0
1000095c:      	ldrsb	r0, [r1, r4]
1000095e:      	movs	r1, #0x40
10000960:      	mvns	r1, r1
10000962:      	cmp	r0, r1
10000964:      	bgt	0x10000a22 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x3d6> @ imm = #0xba
10000966:      	mov	r0, r2
10000968:      	ldr	r2, [sp, #0x14]
1000096a:      	cmp	r2, #0x1
1000096c:      	ldr	r2, [sp, #0x18]
1000096e:      	bne	0x10000a2e <core::str::count::do_count_chars::h419a4d2620ecb10e+0x3e2> @ imm = #0xbc
10000970:      	add	sp, #0x20
10000972:      	pop	{r4, r5, r6, r7, pc}
10000974:      	movs	r0, #0x0
10000976:      	add	sp, #0x20
10000978:      	pop	{r4, r5, r6, r7, pc}
1000097a:      	bics	r1, r0
1000097c:      	movs	r0, #0x0
1000097e:      	mov	r4, r0
10000980:      	b	0x1000098c <core::str::count::do_count_chars::h419a4d2620ecb10e+0x340> @ imm = #0x8
10000982:      	mov	r0, r2
10000984:      	ldr	r2, [sp, #0x18]
10000986:      	adds	r4, r4, #0x4
10000988:      	cmp	r1, r4
1000098a:      	beq	0x10000952 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x306> @ imm = #-0x3c
1000098c:      	ldrsb	r6, [r2, r4]
1000098e:      	movs	r5, #0x40
10000990:      	mvns	r5, r5
10000992:      	cmp	r6, r5
10000994:      	ble	0x10000998 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x34c> @ imm = #0x0
10000996:      	adds	r0, r0, #0x1
10000998:      	adds	r6, r2, r4
1000099a:      	movs	r3, #0x1
1000099c:      	ldrsb	r3, [r6, r3]
1000099e:      	cmp	r3, r5
100009a0:      	ble	0x100009a4 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x358> @ imm = #0x0
100009a2:      	adds	r0, r0, #0x1
100009a4:      	movs	r3, #0x2
100009a6:      	ldrsb	r3, [r6, r3]
100009a8:      	cmp	r3, r5
100009aa:      	bgt	0x100009b8 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x36c> @ imm = #0xa
100009ac:      	mov	r2, r0
100009ae:      	ldr	r0, [sp, #0x1c]
100009b0:      	ldrsb	r3, [r6, r0]
100009b2:      	cmp	r3, r5
100009b4:      	ble	0x10000982 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x336> @ imm = #-0x36
100009b6:      	b	0x100009c2 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x376> @ imm = #0x8
100009b8:      	adds	r2, r0, #0x1
100009ba:      	ldr	r0, [sp, #0x1c]
100009bc:      	ldrsb	r3, [r6, r0]
100009be:      	cmp	r3, r5
100009c0:      	ble	0x10000982 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x336> @ imm = #-0x42
100009c2:      	mov	r0, r2
100009c4:      	adds	r0, r2, #0x1
100009c6:      	b	0x10000984 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x338> @ imm = #-0x46
100009c8:      	ldr	r0, [sp, #0x18]
100009ca:      	add	sp, #0x20
100009cc:      	pop	{r4, r5, r6, r7, pc}
100009ce:      	movs	r1, #0xfc
100009d0:      	ands	r4, r1
100009d2:      	lsls	r2, r4, #0x2
100009d4:      	ldr	r1, [r6, r2]
100009d6:      	lsrs	r3, r1, #0x6
100009d8:      	mvns	r1, r1
100009da:      	lsrs	r1, r1, #0x7
100009dc:      	orrs	r1, r3
100009de:      	ldr	r3, [sp, #0x14]
100009e0:      	ands	r1, r0
100009e2:      	cmp	r3, #0x1
100009e4:      	beq	0x10000a0a <core::str::count::do_count_chars::h419a4d2620ecb10e+0x3be> @ imm = #0x22
100009e6:      	adds	r2, r6, r2
100009e8:      	mov	r5, r3
100009ea:      	ldr	r3, [r2, #0x4]
100009ec:      	lsrs	r4, r3, #0x6
100009ee:      	mvns	r3, r3
100009f0:      	lsrs	r3, r3, #0x7
100009f2:      	orrs	r3, r4
100009f4:      	ands	r3, r0
100009f6:      	adds	r1, r3, r1
100009f8:      	cmp	r5, #0x2
100009fa:      	beq	0x10000a0a <core::str::count::do_count_chars::h419a4d2620ecb10e+0x3be> @ imm = #0xc
100009fc:      	ldr	r2, [r2, #0x8]
100009fe:      	lsrs	r3, r2, #0x6
10000a00:      	mvns	r2, r2
10000a02:      	lsrs	r2, r2, #0x7
10000a04:      	orrs	r2, r3
10000a06:      	ands	r2, r0
10000a08:      	adds	r1, r2, r1
10000a0a:      	lsrs	r0, r1, #0x8
10000a0c:      	ldr	r2, [pc, #0xa4]         @ 0x10000ab4 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x468>
10000a0e:      	ands	r1, r2
10000a10:      	ands	r0, r2
10000a12:      	adds	r0, r0, r1
10000a14:      	ldr	r1, [pc, #0x98]         @ 0x10000ab0 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x464>
10000a16:      	muls	r1, r0, r1
10000a18:      	lsrs	r0, r1, #0x10
10000a1a:      	ldr	r1, [sp, #0x18]
10000a1c:      	adds	r0, r0, r1
10000a1e:      	add	sp, #0x20
10000a20:      	pop	{r4, r5, r6, r7, pc}
10000a22:      	mov	r0, r2
10000a24:      	adds	r0, r2, #0x1
10000a26:      	ldr	r2, [sp, #0x14]
10000a28:      	cmp	r2, #0x1
10000a2a:      	ldr	r2, [sp, #0x18]
10000a2c:      	beq	0x10000970 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x324> @ imm = #-0xc0
10000a2e:      	mov	r3, r0
10000a30:      	adds	r2, r4, r2
10000a32:      	movs	r0, #0x1
10000a34:      	ldrsb	r0, [r2, r0]
10000a36:      	cmp	r0, r1
10000a38:      	bgt	0x10000a8e <core::str::count::do_count_chars::h419a4d2620ecb10e+0x442> @ imm = #0x52
10000a3a:      	mov	r0, r3
10000a3c:      	ldr	r3, [sp, #0x14]
10000a3e:      	cmp	r3, #0x2
10000a40:      	beq	0x10000970 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x324> @ imm = #-0xd4
10000a42:      	b	0x10000a9a <core::str::count::do_count_chars::h419a4d2620ecb10e+0x44e> @ imm = #0x54
10000a44:      	mov	r1, r3
10000a46:      	b	0x10000a4e <core::str::count::do_count_chars::h419a4d2620ecb10e+0x402> @ imm = #0x4
10000a48:      	adds	r1, r1, #0x4
10000a4a:      	bne	0x10000a4e <core::str::count::do_count_chars::h419a4d2620ecb10e+0x402> @ imm = #0x0
10000a4c:      	b	0x10000692 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x46> @ imm = #-0x3be
10000a4e:      	ldrsb	r6, [r2, r1]
10000a50:      	movs	r5, #0x40
10000a52:      	mvns	r5, r5
10000a54:      	cmp	r6, r5
10000a56:      	ble	0x10000a5a <core::str::count::do_count_chars::h419a4d2620ecb10e+0x40e> @ imm = #0x0
10000a58:      	adds	r3, r3, #0x1
10000a5a:      	adds	r6, r2, r1
10000a5c:      	movs	r0, #0x1
10000a5e:      	ldrsb	r0, [r6, r0]
10000a60:      	cmp	r0, r5
10000a62:      	bgt	0x10000a76 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x42a> @ imm = #0x10
10000a64:      	movs	r0, #0x2
10000a66:      	ldrsb	r0, [r6, r0]
10000a68:      	cmp	r0, r5
10000a6a:      	bgt	0x10000a80 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x434> @ imm = #0x12
10000a6c:      	ldr	r0, [sp, #0x1c]
10000a6e:      	ldrsb	r0, [r6, r0]
10000a70:      	cmp	r0, r5
10000a72:      	ble	0x10000a48 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x3fc> @ imm = #-0x2e
10000a74:      	b	0x10000a8a <core::str::count::do_count_chars::h419a4d2620ecb10e+0x43e> @ imm = #0x12
10000a76:      	adds	r3, r3, #0x1
10000a78:      	movs	r0, #0x2
10000a7a:      	ldrsb	r0, [r6, r0]
10000a7c:      	cmp	r0, r5
10000a7e:      	ble	0x10000a6c <core::str::count::do_count_chars::h419a4d2620ecb10e+0x420> @ imm = #-0x16
10000a80:      	adds	r3, r3, #0x1
10000a82:      	ldr	r0, [sp, #0x1c]
10000a84:      	ldrsb	r0, [r6, r0]
10000a86:      	cmp	r0, r5
10000a88:      	ble	0x10000a48 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x3fc> @ imm = #-0x44
10000a8a:      	adds	r3, r3, #0x1
10000a8c:      	b	0x10000a48 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x3fc> @ imm = #-0x48
10000a8e:      	mov	r0, r3
10000a90:      	adds	r0, r3, #0x1
10000a92:      	ldr	r3, [sp, #0x14]
10000a94:      	cmp	r3, #0x2
10000a96:      	bne	0x10000a9a <core::str::count::do_count_chars::h419a4d2620ecb10e+0x44e> @ imm = #0x0
10000a98:      	b	0x10000970 <core::str::count::do_count_chars::h419a4d2620ecb10e+0x324> @ imm = #-0x12c
10000a9a:      	mov	r3, r0
10000a9c:      	movs	r0, #0x2
10000a9e:      	ldrsb	r0, [r2, r0]
10000aa0:      	cmp	r0, r1
10000aa2:      	bgt	0x10000aaa <core::str::count::do_count_chars::h419a4d2620ecb10e+0x45e> @ imm = #0x4
10000aa4:      	mov	r0, r3
10000aa6:      	add	sp, #0x20
10000aa8:      	pop	{r4, r5, r6, r7, pc}
10000aaa:      	adds	r0, r3, #0x1
10000aac:      	add	sp, #0x20
10000aae:      	pop	{r4, r5, r6, r7, pc}

10000ab0 <$d.471>:
10000ab0: 01 00 01 00  	.word	0x00010001
10000ab4: ff 00 ff 00  	.word	0x00ff00ff
10000ab8: 01 01 01 01  	.word	0x01010101

10000abc <<&T as core::fmt::Display>::fmt::ha285b7e5fc4ea0c7>:
10000abc:      	push	{r7, lr}
10000abe:      	add	r7, sp, #0x0
10000ac0:      	mov	r3, r1
10000ac2:      	ldm	r0!, {r1, r2}
10000ac4:      	mov	r0, r3
10000ac6:      	bl	0x100003b0 <core::fmt::Formatter::pad::hf5cda8bdf873db52> @ imm = #-0x71a
10000aca:      	pop	{r7, pc}

10000acc <HardFaultTrampoline>:
10000acc:      	mov	r0, lr
10000ace:      	movs	r1, #0x4
10000ad0:      	tst	r0, r1
10000ad2:      	bne	0x10000ada <HardFaultTrampoline+0xe> @ imm = #0x4
10000ad4:      	mrs	r0, msp
10000ad8:      	b	0x10000ae0 <HardFault_> @ imm = #0x4
10000ada:      	mrs	r0, psp
10000ade:      	b	0x10000ae0 <HardFault_> @ imm = #-0x2

10000ae0 <HardFault_>:
10000ae0:      	b	0x10000ae0 <HardFault_> @ imm = #-0x4
10000ae2:      	bmi	0x10000a8e <core::str::count::do_count_chars::h419a4d2620ecb10e+0x442> @ imm = #-0x58
