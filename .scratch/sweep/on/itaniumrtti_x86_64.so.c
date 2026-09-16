// Function: _DT_INIT @ 0x1000
void _DT_INIT(void) // return-dupe
{
  if (!dat_3ff0)
    return;
  (*dat_3ff0)();
}

// Function: sub_1020 @ 0x1020
void sub_1020(void)
{
  (*dat_4010)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x1060
void __cxa_finalize(void)
{
  (*dat_3fe0)(); // jump-as-call
}

// Function: operator delete @ 0x1070
void operator delete(void)
{
  (*dat_4018)(); // jump-as-call
}

// Function: __stack_chk_fail @ 0x1080
void __stack_chk_fail(void)
{
  (*dat_4020)(); // jump-as-call
}

// Function: _Unwind_Resume @ 0x1090
void _Unwind_Resume(void)
{
  (*dat_4028)(); // jump-as-call
}

// Function: sub_10a0 @ 0x10a0
void sub_10a0(void)
{
  return;
}

// Function: _FINI_0 @ 0x1110
void _FINI_0(void)
{
  if (!dat_4040) {
    if (dat_3fe0)
      __cxa_finalize(dat_4030);
    sub_10a0();
    dat_4040 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x1150
void _INIT_0(void)
{
  return;
}

// Function: sub_115a @ 0x115a
void sub_115a(unsigned long *a0,unsigned int a1)
{
  *a0 = Hidden_vptr;
  *(unsigned int *)&a0[1] = a1;
}

// Function: anonymous_namespace::Hidden::vtable_0 @ 0x1180
void anonymous_namespace::Hidden::vtable_0(unsigned long *a0)
{
  *a0 = Hidden_vptr;
}

// Function: anonymous_namespace::Hidden::vtable_1 @ 0x119e
void anonymous_namespace::Hidden::vtable_1(unsigned long a0)
{
  anonymous_namespace::Hidden::vtable_0(a0);
  operator delete(a0,0x10);
}

// Function: anonymous_namespace::Hidden::vtable_2 @ 0x11ce
int anonymous_namespace::Hidden::vtable_2(long a0)
{
  return *(int *)(a0 + 8) * 9;
}

// Function: probe_shapes @ 0x11ea
int probe_shapes(unsigned int a0)
{
  int v1; // eax
  int v2; // eax
  long v3 [3]; // stack - 0x38
  
  sub_14ea(v3,2,5);
  v1 = (**(void **)(v3[0] + 0x10))(v3,a0);
  v2 = (**(void **)(v3[0] + 0x18))(v3);
  shapes::Circle::vtable_0(v3);
  return v1 + v2;
}

// Function: probe_widget @ 0x12a6
int probe_widget(unsigned int a0)
{
  int v1; // eax
  int v2; // eax
  char v3 [16];
  long v4 [3]; // stack - 0x38
  
  sub_16e2(v3,1,2,3);
  v1 = (**(void **)(v4[0] + 0x10))(v4,a0);
  v2 = shapes::Widget::vtable_2(v3,a0);
  shapes::Widget::vtable_0(v3);
  return v1 + v2;
}

// Function: probe_generic @ 0x1365
int probe_generic(unsigned int a0)
{
  int v1; // eax
  int v2; // eax
  char v3 [16];
  char v4 [16];
  char v5 [24];
  double v6; // xmm0_qa
  
  sub_1836(v3,a0);
  sub_18ae(v4);
  sub_115a(v5,a0);
  v1 = shapes::Vec_int::vtable_2(v3);
  shapes::Vec_double::vtable_2(v4);
  v2 = anonymous_namespace::Hidden::vtable_2(v5);
  anonymous_namespace::Hidden::vtable_0(v5);
  shapes::Vec_double::vtable_0(v4);
  shapes::Vec_int::vtable_0(v3);
  return v1 + (int)v6 + v2;
}

// Function: sub_1440 @ 0x1440
void sub_1440(unsigned long *a0,unsigned int a1)
{
  *a0 = Shape_vptr;
  *(unsigned int *)&a0[1] = a1;
}

// Function: shapes::Shape::vtable_0 @ 0x146a
void shapes::Shape::vtable_0(unsigned long *a0)
{
  *a0 = Shape_vptr;
}

// Function: shapes::Shape::vtable_1 @ 0x1488
void shapes::Shape::vtable_1(unsigned long a0)
{
  shapes::Shape::vtable_0(a0);
  operator delete(a0,0x10);
}

// Function: shapes::Shape::vtable_2 @ 0x14b8
int shapes::Shape::vtable_2(long a0,int a1)
{
  return *(int *)(a0 + 8) * a1;
}

// Function: shapes::Shape::vtable_3 @ 0x14d4
unsigned int shapes::Shape::vtable_3(long a0)
{
  return *(unsigned int *)(a0 + 8);
}

// Function: sub_14ea @ 0x14ea
void sub_14ea(unsigned long *a0,unsigned int a1,unsigned int a2)
{
  sub_1440(a0,a1);
  *a0 = Circle_vptr;
  *(unsigned int *)((long)a0 + 0xc) = a2;
}

// Function: shapes::Circle::vtable_0 @ 0x152c
void shapes::Circle::vtable_0(unsigned long *a0)
{
  *a0 = Circle_vptr;
  shapes::Shape::vtable_0(a0);
}

// Function: shapes::Circle::vtable_1 @ 0x155a
void shapes::Circle::vtable_1(unsigned long a0)
{
  shapes::Circle::vtable_0(a0);
  operator delete(a0,0x10);
}

// Function: shapes::Circle::vtable_2 @ 0x158a
int shapes::Circle::vtable_2(long a0,int a1)
{
  return *(int *)(a0 + 0xc) * a1 * *(int *)(a0 + 0xc) * 3;
}

// Function: sub_15b8 @ 0x15b8
void sub_15b8(unsigned long *a0,unsigned int a1)
{
  *a0 = Loggable_vptr;
  *(unsigned int *)&a0[1] = a1;
}

// Function: shapes::Loggable::vtable_0 @ 0x15e2
void shapes::Loggable::vtable_0(unsigned long *a0)
{
  *a0 = Loggable_vptr;
}

// Function: shapes::Loggable::vtable_1 @ 0x1600
void shapes::Loggable::vtable_1(unsigned long a0)
{
  shapes::Loggable::vtable_0(a0);
  operator delete(a0,0x10);
}

// Function: shapes::Loggable::vtable_2 @ 0x1630
int shapes::Loggable::vtable_2(long a0,int a1)
{
  return a1 + *(int *)(a0 + 8);
}

// Function: sub_164e @ 0x164e
void sub_164e(unsigned long *a0,unsigned int a1)
{
  *a0 = Drawable_vptr;
  *(unsigned int *)&a0[1] = a1;
}

// Function: shapes::Drawable::vtable_0 @ 0x1678
void shapes::Drawable::vtable_0(unsigned long *a0)
{
  *a0 = Drawable_vptr;
}

// Function: shapes::Drawable::vtable_1 @ 0x1696
void shapes::Drawable::vtable_1(unsigned long a0)
{
  shapes::Drawable::vtable_0(a0);
  operator delete(a0,0x10);
}

// Function: shapes::Drawable::vtable_2 @ 0x16c6
int shapes::Drawable::vtable_2(long a0,int a1)
{
  return *(int *)(a0 + 8) * a1;
}

// Function: sub_16e2 @ 0x16e2
void sub_16e2(unsigned long *a0,unsigned int a1,unsigned int a2,unsigned int a3)
{
  sub_15b8(a0,a1);
  sub_164e(&a0[2],a2);
  *a0 = Widget_vptr;
  a0[2] = Widget_vptr_for_Drawable;
  *(unsigned int *)((long)a0 + 0x1c) = a3;
}

// Function: shapes::Widget::vtable_0 @ 0x174c
void shapes::Widget::vtable_0(unsigned long *a0)
{
  *a0 = Widget_vptr;
  a0[2] = Widget_vptr_for_Drawable;
  shapes::Drawable::vtable_0(&a0[2]);
  shapes::Loggable::vtable_0(a0);
}

// Function: shapes::Widget::vtable_for_Drawable_0 @ 0x1798
void shapes::Widget::vtable_for_Drawable_0(long a0)
{
  shapes::Widget::vtable_0(a0 + -0x10); // tail-call
}

// Function: shapes::Widget::vtable_1 @ 0x17a2
void shapes::Widget::vtable_1(unsigned long a0)
{
  shapes::Widget::vtable_0(a0);
  operator delete(a0,0x20);
}

// Function: shapes::Widget::vtable_for_Drawable_1 @ 0x17d1
void shapes::Widget::vtable_for_Drawable_1(long a0)
{
  shapes::Widget::vtable_1(a0 + -0x10); // tail-call
}

// Function: shapes::Widget::vtable_2 @ 0x17dc
int shapes::Widget::vtable_2(long a0,int a1)
{
  return *(int *)(a0 + 0x1c) + *(int *)(a0 + 8) + a1;
}

// Function: shapes::Widget::vtable_3 @ 0x1802
int shapes::Widget::vtable_3(long a0,int a1)
{
  return *(int *)(a0 + 0x18) * a1 - *(int *)(a0 + 0x1c);
}

// Function: shapes::Widget::vtable_for_Drawable_2 @ 0x182b
void shapes::Widget::vtable_for_Drawable_2(long a0)
{
  shapes::Widget::vtable_3(a0 + -0x10); // tail-call
}

// Function: sub_1836 @ 0x1836
void sub_1836(unsigned long *a0,unsigned int a1)
{
  *a0 = Vec_int_vptr;
  *(unsigned int *)&a0[1] = a1;
}

// Function: shapes::Vec_int::vtable_0 @ 0x1860
void shapes::Vec_int::vtable_0(unsigned long *a0)
{
  *a0 = Vec_int_vptr;
}

// Function: shapes::Vec_int::vtable_1 @ 0x187e
void shapes::Vec_int::vtable_1(unsigned long a0)
{
  shapes::Vec_int::vtable_0(a0);
  operator delete(a0,0x10);
}

// Function: sub_18ae @ 0x18ae
void sub_18ae(unsigned long a0,unsigned long *a1)
{
  *a1 = Vec_double_vptr;
  a1[1] = a0;
}

// Function: shapes::Vec_double::vtable_0 @ 0x18de
void shapes::Vec_double::vtable_0(unsigned long *a0)
{
  *a0 = Vec_double_vptr;
}

// Function: shapes::Vec_double::vtable_1 @ 0x18fc
void shapes::Vec_double::vtable_1(unsigned long a0)
{
  shapes::Vec_double::vtable_0(a0);
  operator delete(a0,0x10);
}

// Function: shapes::Vec_int::vtable_2 @ 0x192c
unsigned int shapes::Vec_int::vtable_2(long a0)
{
  return *(unsigned int *)(a0 + 8);
}

// Function: shapes::Vec_double::vtable_2 @ 0x1942
unsigned long shapes::Vec_double::vtable_2(long a0)
{
  return *(unsigned long *)(a0 + 8);
}

// Function: _DT_FINI @ 0x1964
void _DT_FINI(void)
{
  return;
}

