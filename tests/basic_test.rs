use foxmulator::Foxmulator;
use foxmulator::HaltReason;

#[test]
fn test_nop() {
  let mut foxmulator = Foxmulator::singleton().unwrap();
  
  foxmulator.run_assembly(r#"
    block (end)
    nop
    halt
    end:
  "#);

  assert_eq!(foxmulator.state.halt_reason, HaltReason::Halt);
}

#[test]
fn test_add() {
  let mut foxmulator = Foxmulator::singleton().unwrap();

  foxmulator.state.r[0] = 2;
  foxmulator.state.r[1] = 1;
  foxmulator.state.r[2] = 3;
  foxmulator.run_assembly(r#"
    block (end)
    add r0, r1
    inc r2, r1
    halt
    end:
  "#);

  assert_eq!(foxmulator.state.halt_reason, HaltReason::Halt);
  assert_eq!(foxmulator.state.r[0], 3);
  assert_eq!(foxmulator.state.r[2], 4);

}

#[test]
fn test_sub() {
  let mut foxmulator = Foxmulator::singleton().unwrap();

  foxmulator.state.r[0] = 3;
  foxmulator.state.r[1] = 1;
  foxmulator.state.r[2] = 4;
  foxmulator.run_assembly(r#"
    block (end)
    sub r0, r1
    dec r2, r1
    halt
    end:
  "#);

  assert_eq!(foxmulator.state.halt_reason, HaltReason::Halt);
  assert_eq!(foxmulator.state.r[0], 2);
  assert_eq!(foxmulator.state.r[2], 3);
}

#[test]
fn test_and() {
  let mut foxmulator = Foxmulator::singleton().unwrap();

  foxmulator.state.r[0] = 0b1010;
  foxmulator.state.r[1] = 0b1100;
  foxmulator.run_assembly(r#"
    block (end)
    and r0, r1
    halt
    end:
  "#);

  assert_eq!(foxmulator.state.r[0], 0b1000);
  assert_eq!(foxmulator.state.halt_reason, HaltReason::Halt);
}

#[test]
fn test_or() {
  let mut foxmulator = Foxmulator::singleton().unwrap();

  foxmulator.state.r[0] = 0b1010;
  foxmulator.state.r[1] = 0b1100;
  foxmulator.run_assembly(r#"
    block (end)
    or r0, r1
    halt
    end:
  "#);

  assert_eq!(foxmulator.state.r[0], 0b1110);
  assert_eq!(foxmulator.state.halt_reason, HaltReason::Halt);
}

#[test]
fn test_xor() {
  let mut foxmulator = Foxmulator::singleton().unwrap();

  foxmulator.state.r[0] = 0b1010;
  foxmulator.state.r[1] = 0b1100;
  foxmulator.run_assembly(r#"
    block (end)
    xor r0, r1
    halt
    end:
  "#);

  assert_eq!(foxmulator.state.r[0], 0b0110);
  assert_eq!(foxmulator.state.halt_reason, HaltReason::Halt);
}

#[test]
fn test_andc() {
  let mut foxmulator = Foxmulator::singleton().unwrap();

  foxmulator.state.r[0] = 0b1010;
  foxmulator.state.r[1] = 0b1100;
  foxmulator.run_assembly(r#"
    block (end)
    andc r0, r1
    halt
    end:
  "#);

  assert_eq!(foxmulator.state.r[0], 0b0010);
  assert_eq!(foxmulator.state.halt_reason, HaltReason::Halt);
}

#[test]
fn test_not() {
  let mut foxmulator = Foxmulator::singleton().unwrap();

  foxmulator.state.r[0] = 0b1010_1010_1111_1111;
  foxmulator.state.r[1] = 0b1100_1100_0011_0011;
  foxmulator.run_assembly(r#"
    block (end)
    not r0, r1
    halt
    end:
  "#);

  assert_eq!(foxmulator.state.r[0], 0b0011_0011_1100_1100);
  assert_eq!(foxmulator.state.halt_reason, HaltReason::Halt);
}

#[test]
fn test_neg() {
  let mut foxmulator = Foxmulator::singleton().unwrap();

  foxmulator.state.r[0] = 54;
  foxmulator.state.r[1] = 28;
  foxmulator.run_assembly(r#"
    block (end)
    neg r0, r1
    halt
    end:
  "#);

  assert_eq!(foxmulator.state.r[0], -28_i16 as u16);
  assert_eq!(foxmulator.state.halt_reason, HaltReason::Halt);
}

#[test]
fn test_bswap() {
  let mut foxmulator = Foxmulator::singleton().unwrap();

  foxmulator.state.r[0] = 0xcafe;
  foxmulator.state.r[1] = 0xbeef;
  foxmulator.run_assembly(r#"
    block (end)
    byteswap r2, r0
    byteswap r1, r1
    halt
    end:
  "#);

  assert_eq!(foxmulator.state.halt_reason, HaltReason::Halt);
  assert_eq!(foxmulator.state.r[0], 0xcafe);
  assert_eq!(foxmulator.state.r[1], 0xefbe);
  assert_eq!(foxmulator.state.r[2], 0xfeca);
}

#[test]
fn test_set() {
  let mut foxmulator = Foxmulator::singleton().unwrap();

  foxmulator.state.r[0] = 0xcafe;
  foxmulator.run_assembly(r#"
    block (end)
    set r0, 0
    set r1, 1
    set r2, -1
    set r3, 15
    set r4, -15
    set r5, 16
    set r6, -16
    set r7, 0xffff
    set r8, 0xcafe
    set r9, 0xff
    set r10, 0xff00
    set r11, -100
    set r12, -30000
    halt
    end:
  "#);

  assert_eq!(foxmulator.state.r[0], 0);
  assert_eq!(foxmulator.state.r[1], 1);
  assert_eq!(foxmulator.state.r[2], (-1i16) as u16);
  assert_eq!(foxmulator.state.r[3], 15);
  assert_eq!(foxmulator.state.r[4], (-15i16) as u16);
  assert_eq!(foxmulator.state.r[5], 16);
  assert_eq!(foxmulator.state.r[6], (-16i16) as u16);
  assert_eq!(foxmulator.state.r[7], 0xffff);
  assert_eq!(foxmulator.state.r[8], 0xcafe);
  assert_eq!(foxmulator.state.r[9], 0x00ff);
  assert_eq!(foxmulator.state.r[10], 0xff00);
  assert_eq!(foxmulator.state.r[11], (-100i16) as u16);
  assert_eq!(foxmulator.state.r[12], (-30000i16) as u16);
  assert_eq!(foxmulator.state.halt_reason, HaltReason::Halt);
}

#[test]
fn test_block() {
  let mut foxmulator = Foxmulator::singleton().unwrap();

  foxmulator.state.r[0] = 3;
  foxmulator.state.r[1] = 1;
  foxmulator.run_assembly(r#"
    block (0, end) next
    nop
    sub r0, r1
    end:
    block (next)
    sub r0, r1
    next:
    block (#1)
    halt
  "#);

  assert_eq!(foxmulator.state.r[0], 1);
  assert_eq!(foxmulator.state.halt_reason, HaltReason::Halt);
}

#[test]
fn test_b() {
  let mut foxmulator = Foxmulator::singleton().unwrap();

  foxmulator.state.r[0] = 3;
  foxmulator.state.r[1] = 1;
  foxmulator.run_assembly(r#"
    block (1, end) next
    b t0
    sub r0, r1
    end:
    block (next)
    sub r0, r1
    next:
    block (#1)
    halt
  "#);

  assert_eq!(foxmulator.state.r[0], 2);
  assert_eq!(foxmulator.state.halt_reason, HaltReason::Halt);
}

#[test]
fn test_bz() {
  let mut foxmulator = Foxmulator::singleton().unwrap();

  foxmulator.state.r[0] = 0;
  foxmulator.state.r[1] = 1;
  foxmulator.run_assembly(r#"
    test:
    block (1, .end) eqzt
    b t0, if r0 eq 0
    .end:
    block (eqzt)
    set r2, 1
    eqzt:
    block (1, .end) neqzf
    set r3, 1
    b t0, if r0 neq 0
    .end:
    block (neqzf)
    set r4, 1
    neqzf:
    block (1, .end) eqzf
    set r5, 1
    b t0, if r1 eq 0
    .end:
    block (eqzf)
    set r6, 1
    eqzf:
    block (1, .end) neqzt
    set r7, 1
    b t0, if r1 neq 0
    .end:
    block (neqzt)
    set r8, 1
    neqzt:
    block (.end)
    halt
    .end:
  "#);

  assert_eq!(foxmulator.state.halt_reason, HaltReason::Halt);
  assert_eq!(foxmulator.state.r[0], 0);
  assert_eq!(foxmulator.state.r[1], 1);
  assert_eq!(foxmulator.state.r[2], 0);
  assert_eq!(foxmulator.state.r[3], 1);
  assert_eq!(foxmulator.state.r[4], 1);
  assert_eq!(foxmulator.state.r[5], 1);
  assert_eq!(foxmulator.state.r[6], 1);
  assert_eq!(foxmulator.state.r[7], 1);
  assert_eq!(foxmulator.state.r[8], 0);

}

#[test]
fn test_mov() {
  let mut foxmulator = Foxmulator::singleton().unwrap();

  foxmulator.state.r[0] = 0xcafe;
  foxmulator.state.r[1] = 0xbeef;

  foxmulator.run_assembly(r#"
    block (end)
    mov r2, r0
    mov r15, r2
    mov r2, r1
    mov r14, r2
    mov r3, r2
    mov r5, r1
    mov r10, r0
    halt
    end:
  "#);

  assert_eq!(foxmulator.state.halt_reason, HaltReason::Halt);
  assert_eq!(foxmulator.state.r[0], 0xcafe);
  assert_eq!(foxmulator.state.r[1], 0xbeef);
  assert_eq!(foxmulator.state.r[15], 0xcafe);
  assert_eq!(foxmulator.state.r[14], 0xbeef);
  assert_eq!(foxmulator.state.r[2], 0xbeef);
  assert_eq!(foxmulator.state.r[3], 0xbeef);
  assert_eq!(foxmulator.state.r[5], 0xbeef);
  assert_eq!(foxmulator.state.r[10], 0xcafe);
}

#[test]
fn test_movp() {
  let mut foxmulator = Foxmulator::singleton().unwrap();

  foxmulator.state.p[0] = true;
  foxmulator.state.p[1] = false;

  foxmulator.run_assembly(r#"
    block (end)
    mov p2, p0
    mov p3, p1
    mov p4, true
    mov p5, false
    mov p0, !p7
    mov p1, p7
    halt
    end:
  "#);

  assert_eq!(foxmulator.state.halt_reason, HaltReason::Halt);
  assert_eq!(foxmulator.state.p[0], false);
  assert_eq!(foxmulator.state.p[1], true);
  assert_eq!(foxmulator.state.p[2], true);
  assert_eq!(foxmulator.state.p[3], false);
  assert_eq!(foxmulator.state.p[4], true);
  assert_eq!(foxmulator.state.p[5], false);
}

#[test]
fn test_movt() {
  let mut foxmulator = Foxmulator::singleton().unwrap();

  foxmulator.state.t[0] = 0xcafe;
  foxmulator.state.t[1] = 0xbeef;

  foxmulator.run_assembly(r#"
    block (end)
    mov t2, t0
    mov t3, t1
    mov t4, t1
    mov t5, t0
    halt
    end:
  "#);

  assert_eq!(foxmulator.state.halt_reason, HaltReason::Halt);
  assert_eq!(foxmulator.state.t[0], 0xcafe);
  assert_eq!(foxmulator.state.t[1], 0xbeef);
  assert_eq!(foxmulator.state.t[2], 0xcafe);
  assert_eq!(foxmulator.state.t[3], 0xbeef);
  assert_eq!(foxmulator.state.t[4], 0xbeef);
  assert_eq!(foxmulator.state.t[5], 0xcafe);
}

#[test]
fn test_call() {
  let mut foxmulator = Foxmulator::singleton().unwrap();

  foxmulator.run_assembly(r#"
    block (1, end) test
    call t0
    end:
    block (end2)
    set r0, 1
    halt
    end2:
    test:
    block (end3)
    set r0, 2
    set r1, end
    halt
    end3:
  "#);

  assert_eq!(foxmulator.state.halt_reason, HaltReason::Halt);
  assert_eq!(foxmulator.state.r[0], 2);
  assert_eq!(foxmulator.state.t[5], 6);
  assert_eq!(foxmulator.state.t[5], foxmulator.state.r[1]);
}

#[test]
fn test_eq() {
  let mut foxmulator = Foxmulator::singleton().unwrap();

  foxmulator.state.r[0] = 0;
  foxmulator.state.r[1] = 1;
  foxmulator.state.r[2] = 54;
  foxmulator.state.r[3] = -32i16 as u16;
  foxmulator.state.r[4] = 0;
  foxmulator.state.r[5] = 1;
  foxmulator.state.r[6] = 54;
  foxmulator.state.r[7] = -32i16 as u16;
  foxmulator.run_assembly(r#"
    test:
    block (.end)
    eq p0, r0, r1
    mov p1, p0
    eq p0, r0, r4
    mov p2, p0
    eq p0, r2, r6
    mov p3, p0
    eq p0, r3, r7
    mov p4, p0
    eq p0, r3, r2
    mov p5, p0
    eq p0, r1, r5
    mov p6, p0
    eq p0, r1, r2
    halt
    .end:
  "#);

  assert_eq!(foxmulator.state.halt_reason, HaltReason::Halt);
  assert_eq!(foxmulator.state.p[0], false);
  assert_eq!(foxmulator.state.p[1], false);
  assert_eq!(foxmulator.state.p[2], true);
  assert_eq!(foxmulator.state.p[3], true);
  assert_eq!(foxmulator.state.p[4], true);
  assert_eq!(foxmulator.state.p[5], false);
  assert_eq!(foxmulator.state.p[6], true);

}

#[test]
fn test_lt_gt() {
  let mut foxmulator = Foxmulator::singleton().unwrap();

  foxmulator.state.r[0] = 0;
  foxmulator.state.r[1] = 1;
  foxmulator.state.r[2] = 54;
  foxmulator.state.r[3] = -32i16 as u16;
  foxmulator.state.r[4] = -54i16 as u16;
  foxmulator.run_assembly(r#"
    test:
    block (.end)
    gt.s p0, r0, r1
    mov p1, p0
    gt.u p0, r0, r1
    mov p2, p0
    gt.s p0, r1, r0
    mov p3, p0
    gt.u p0, r1, r0
    mov p4, p0
    gt.s p0, r3, r4
    mov p5, p0
    gt.u p0, r2, r4
    mov p6, p0
    gt.s p0, r2, r4
    halt
    .end:
  "#);

  assert_eq!(foxmulator.state.halt_reason, HaltReason::Halt);
  assert_eq!(foxmulator.state.p[0], true);
  assert_eq!(foxmulator.state.p[1], false);
  assert_eq!(foxmulator.state.p[2], false);
  assert_eq!(foxmulator.state.p[3], true);
  assert_eq!(foxmulator.state.p[4], true);
  assert_eq!(foxmulator.state.p[5], true);
  assert_eq!(foxmulator.state.p[6], false);

}

#[test]
fn test_inc_dec() {
  let mut foxmulator = Foxmulator::singleton().unwrap();

  foxmulator.run_assembly(r#"
    test:
    block (.end)
    inc r0, 1
    dec r1, 5
    inc r2, 16
    dec r3, 16
    inc r4, 6
    dec r4, 4
    inc r5, 10
    dec r5, 16
    halt
    .end:
  "#);

  assert_eq!(foxmulator.state.halt_reason, HaltReason::Halt);
  assert_eq!(foxmulator.state.r[0], 1);
  assert_eq!(foxmulator.state.r[1], -5_i16 as u16);
  assert_eq!(foxmulator.state.r[2], 16);
  assert_eq!(foxmulator.state.r[3], -16_i16 as u16);
  assert_eq!(foxmulator.state.r[4], 2);
  assert_eq!(foxmulator.state.r[5], -6_i16 as u16);

}

#[test]
fn test_read_word() {
  let mut foxmulator = Foxmulator::singleton().unwrap();

  foxmulator.run_assembly(r#"
    test:
    block (.end)
    set r0, data1
    set r1, data2

    ld.w r2, r0[0]
    ld.w r3, r0[1]
    ld.w r4, r1[0]
    ld.w r5, r1[1]
    ld.w r6, r1[-1]

    halt
    .end:

    data1:
    #d16 0xcafe, 0xbabe
    data2:
    #d16 0xbeef, 0xdead
  "#);

  assert_eq!(foxmulator.state.halt_reason, HaltReason::Halt);
  assert_eq!(foxmulator.state.r[2], 0xcafe);
  assert_eq!(foxmulator.state.r[3], 0xbabe);
  assert_eq!(foxmulator.state.r[4], 0xbeef);
  assert_eq!(foxmulator.state.r[5], 0xdead);
  assert_eq!(foxmulator.state.r[6], 0xbabe);

}

#[test]
fn test_write_word() {
  let mut foxmulator = Foxmulator::singleton().unwrap();

  foxmulator.run_assembly(r#"
    test:
    block (.end)
    set r0, data1
    set r1, data2
    set r2, 0xcafe
    set r3, 0xbabe
    set r4, 0xbeef
    set r5, 0xdead

    st.w r2, r0[0]
    st.w r3, r1[0]
    st.w r4, r1[1]
    st.w r5, r1[-1]

    ld.w r6, r0[0]
    ld.w r7, r0[1]
    ld.w r8, r0[2]
    ld.w r9, r0[3]

    halt
    .end:

    data1:
    #d16 0, 0
    data2:
    #d16 0, 0
  "#);

  assert_eq!(foxmulator.state.halt_reason, HaltReason::Halt);
  assert_eq!(foxmulator.state.r[6], 0xcafe);
  assert_eq!(foxmulator.state.r[7], 0xdead);
  assert_eq!(foxmulator.state.r[8], 0xbabe);
  assert_eq!(foxmulator.state.r[9], 0xbeef);

}