//! Constant array bounds retain their language-specific origin.
use crate::common;
use common::process;
use std::process::Command;

#[test]
fn fortran_array_lengths_use_both_bounds() {
    if process::optional_output(Command::new("gfortran").arg("--version")).is_none() {
        return;
    }
    let source = common::scratch_file("array-bounds", "f90");
    let object = source.with_extension("so");
    std::fs::write(&source, "subroutine implicit_origin(a)\nreal :: a(3)\na(3)=1\nend\nsubroutine negative_origin(a)\nreal :: a(-2:2)\na(2)=1\nend\nsubroutine negative_upper(a)\nreal :: a(-8:-4)\na(-4)=1\nend\n").unwrap();
    let output = Command::new("gfortran").args(["-O0", "-g", "-shared", "-fPIC"])
        .arg(&source).arg("-o").arg(&object).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    for (name, count) in [("implicit_origin", 3), ("negative_origin", 5), ("negative_upper", 5)] {
        let (body, err, status) = common::run_kuna(&["decompile", object.to_str().unwrap(), name]);
        assert_eq!(status, 0, "{err}");
        assert!(body.lines().next().unwrap().contains(&format!("[{count}]")), "{body}");
    }
    std::fs::remove_file(source).unwrap();
    std::fs::remove_file(object).unwrap();
}

#[test]
fn multidimensional_locals_recompile_with_their_full_extent() {
    if process::optional_output(Command::new("gfortran").arg("--version")).is_none() {
        return;
    }
    let compilers: Vec<_> = ["clang", "gcc"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    if compilers.is_empty() {
        return;
    }
    let dir = common::scratch_file("multidimensional-local", "dir");
    std::fs::create_dir_all(&dir).unwrap();
    let source = dir.join("sample.f90");
    std::fs::write(
        &source,
        r#"
subroutine fill_grid(seed,out) bind(C,name="fill_grid")
 use iso_c_binding
 implicit none
 real(c_float),intent(in)::seed
 real(c_float),intent(out)::out
 real(c_float)::grid(-2:0,5:11)
 integer::i,j
 do j=5,11
  do i=-2,0
   grid(i,j)=seed+real(3*i+5*j,c_float)
  end do
 end do
 out=sum(grid)
end subroutine
subroutine fill_cube(seed,out) bind(C,name="fill_cube")
 use iso_c_binding
 implicit none
 real(c_float),intent(in)::seed
 real(c_float),intent(out)::out
 real(c_float)::cube(2,3,4)
 integer::i,j,k
 do k=1,4
  do j=1,3
   do i=1,2
    cube(i,j,k)=seed+real(i+2*j+3*k,c_float)
   end do
  end do
 end do
 out=sum(cube)
end subroutine
"#,
    )
    .unwrap();
    let library = dir.join("sample.so");
    process::required_output(
        Command::new("gfortran")
            .args(["-O0", "-g", "-shared", "-fPIC"])
            .arg(&source)
            .arg("-o")
            .arg(&library),
    );
    let (printed, stderr, rc) = common::run_kuna(&[
        "decompile-all",
        library.to_str().unwrap(),
        "--functions",
        "fill_grid,fill_cube",
    ]);
    assert_eq!(rc, 0, "{stderr}\n{printed}");
    assert!(printed.contains("grid [21]"), "{printed}");
    assert!(printed.contains("cube [24]"), "{printed}");
    let regenerated = dir.join("regenerated.c");
    std::fs::write(&regenerated,format!("typedef float float4;\n#define fill_grid printed_fill_grid\n#define fill_cube printed_fill_cube\n{printed}")).unwrap();
    let driver = dir.join("driver.c");
    std::fs::write(
        &driver,
        r#"
#include <stdio.h>
void fill_grid(float*,float*);void printed_fill_grid(float*,float*);
void fill_cube(float*,float*);void printed_fill_cube(float*,float*);
int main(void){
 for(int i=-8;i<=8;i++){
  float seed=i*0.25f,expected,actual;
  fill_grid(&seed,&expected);printed_fill_grid(&seed,&actual);
  if(actual!=expected){fprintf(stderr,"grid: expected=%g actual=%g\n",expected,actual);return 1;}
  fill_cube(&seed,&expected);printed_fill_cube(&seed,&actual);
  if(actual!=expected){fprintf(stderr,"cube: expected=%g actual=%g\n",expected,actual);return 2;}
 }
 return 0;
}
"#,
    )
    .unwrap();
    for cc in compilers {
        for opt in ["-O0", "-O2"] {
            let exe = dir.join(format!("check-{cc}-{opt}"));
            process::required_output(
                Command::new(cc)
                    .args([opt, "-fno-fast-math"])
                    .arg(&regenerated)
                    .arg(&driver)
                    .arg(&library)
                    .arg("-o")
                    .arg(&exe),
            );
            process::required_output(&mut Command::new(exe));
        }
    }
    std::fs::remove_dir_all(dir).unwrap();
}
