; ModuleID = 'probe1.2941139a-cgu.0'
source_filename = "probe1.2941139a-cgu.0"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx11.0.0"

; probe1::probe
; Function Attrs: uwtable
define void @_ZN6probe15probe17h95bad13b3f40676eE() unnamed_addr #0 {
start:
  ret void
}

attributes #0 = { uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-a14" }

!llvm.module.flags = !{!0, !1, !2, !3, !4}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 1, !"branch-target-enforcement", i32 0}
!2 = !{i32 1, !"sign-return-address", i32 0}
!3 = !{i32 1, !"sign-return-address-all", i32 0}
!4 = !{i32 1, !"sign-return-address-with-bkey", i32 0}
