@.fstr = private constant [4 x i8] c"%f\0A\00", align 1
@.true_str = private constant [6 x i8] c"true\0A\00", align 1
@.false_str = private constant [7 x i8] c"false\0A\00", align 1
@.none_str = private constant [6 x i8] c"none\0A\00", align 1
declare i32 @printf(i8*, ...)
define i32 @main() {
entry:
%c.0 = alloca double, align 8
store double 0.0, double* %c.0, align 8
%a.0 = alloca double, align 8
store double 0.0, double* %a.0, align 8
%b.0 = alloca double, align 8
store double 0.0, double* %b.0, align 8
%fib.0 = alloca double, align 8
store double 1.0, double* %fib.0, align 8

%.0 = load double, double* %fib.0, align 8
%.1 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %.1, double %.0)

br label %loop.13
loop.13:
%.2 = load double, double* %c.0, align 8
%.3 = fcmp olt double %.2, 10.0
br i1 %.3, label %body.13, label %loop_exit.13
body.13:

%.4 = load double, double* %b.0, align 8
store double %.4, double* %a.0, align 8

%.5 = load double, double* %fib.0, align 8
store double %.5, double* %b.0, align 8

%.6 = load double, double* %a.0, align 8
%.7 = load double, double* %b.0, align 8
%.8 = fadd double %.6, %.7
store double %.8, double* %fib.0, align 8

%.9 = load double, double* %c.0, align 8
%.10 = fadd double %.9, 1.0
store double %.10, double* %c.0, align 8

%.11 = load double, double* %fib.0, align 8
%.12 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %.12, double %.11)
br label %loop.13
loop_exit.13:

ret i32 0
}
